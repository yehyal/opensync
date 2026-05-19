mod clipboard;
mod commands;
mod services;
mod socket;
mod state;
mod tray;

use serde::Deserialize;
use serde_json::json;
use storage::db::{DeviceRegistration, LoginResponse};
use tauri::async_runtime::spawn;
use tauri::{Emitter, Manager};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_http::reqwest;
use url::Url;

use log::{error, info, warn};
use std::collections::HashMap;

use crate::services::ServicesState;
use crate::state::{AppState, AuthState};

const API_BASE_URL: &str = "http://localhost:3000";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: None,
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                ])
                .build(),
        )
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            info!("single-instance cwd: {cwd}");
            info!("single-instance args: {args:?}");

            // On Windows/Linux the deep link URL often comes as a CLI arg. Let the deep-link plugin process it.
            app.deep_link().handle_cli_arguments(args.iter());

            // If the OS delivered the deep link as a CLI arg, handle it here too.
            for arg in &args {
                if let Ok(url) = Url::parse(arg) {
                    handle_deep_link_url(app, url);
                }
            }

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            } else {
                warn!("single-instance invoked but no 'main' window exists");
            }
        }))
        .setup(|app| {
            let state = state::AppState::new().unwrap();
            let auth = state::AuthState::new(state.load_auth_session().unwrap());
            let cache = state::CacheState::new();
            let services = services::ServicesState::new();
            if let Some(window) = app.get_webview_window("main") {
                if auth.is_logged_in() {
                    let _ = window.hide();
                }
            }

            let state_inserted = app.manage(state);
            assert!(state_inserted, "AppState was already managed");

            let auth_inserted = app.manage(auth);
            assert!(auth_inserted, "AuthState was already managed");

            let cache_inserted = app.manage(cache);
            assert!(cache_inserted, "CacheState was already managed");

            let services_inserted = app.manage(services);
            assert!(services_inserted, "ServicesState was already managed");

            tray::setup(app)?;

            // macOS/iOS emit deep-link URLs as events (not CLI args).
            let app_handle = app.app_handle().clone();
            app.deep_link().on_open_url(move |event| {
                for url in event.urls() {
                    handle_deep_link_url(&app_handle, url);
                }
            });

            app.state::<ServicesState>()
                .sync_with_auth(app.app_handle());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::is_authenticated,
            commands::get_events,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        if matches!(
            event,
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit
        ) {
            services::stop_on_exit(app_handle);
        }
    });
}

#[derive(Debug, Clone)]
struct CallbackArgs {
    token: String,
    user_id: String,
    extra: HashMap<String, String>,
}

impl CallbackArgs {
    fn parse(url: &Url) -> Result<Self, String> {
        let query: HashMap<String, String> = url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let token = query
            .get("token")
            .cloned()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| "missing required query parameter: token".to_string())?;

        let user_id = query.get("user_id").cloned().unwrap_or_default();

        let mut extra = query;
        extra.remove("token");
        extra.remove("user_id");

        Ok(Self {
            token,
            user_id,
            extra,
        })
    }
}

fn handle_deep_link_url<R: tauri::Runtime>(app: &tauri::AppHandle<R>, url: Url) {
    if url.scheme() != "opensync" {
        return;
    }

    // Example: opensync://callback?token=test
    let is_callback = url.host_str() == Some("callback") || url.path() == "/callback";
    if !is_callback {
        warn!("Ignoring opensync URL (not callback): {url}");
        return;
    }

    let args = match CallbackArgs::parse(&url) {
        Ok(args) => args,
        Err(message) => {
            warn!("Callback URL invalid ({message}): {url}");
            return;
        }
    };

    info!(
        "Received callback args (token_len={}, user_id_present={}): storing session",
        args.token.len(),
        !args.user_id.is_empty()
    );

    if !args.extra.is_empty() {
        info!("Callback extra args: {:?}", args.extra);
    }

    // TODO: replace test values with real response when API is wired.
    let binding = app.state::<AppState>();
    let db = binding.db();
    if let Err(error) = db.lock().unwrap().insert(LoginResponse {
        user_id: args.user_id.clone(),
        name: "".to_string(),
        token: args.token.clone(),
        email: "".to_string(),
    }) {
        error!("DB insert error while handling deep link: {error}");
    }

    // Refresh in-memory auth state immediately.
    match binding.load_auth_session() {
        Ok(session) => {
            app.state::<AuthState>().set_session(session);
            app.state::<ServicesState>().sync_with_auth(app);
            let _ = app.emit("auth://changed", true);
        }
        Err(err) => {
            error!("failed to load auth session after callback: {err}");
        }
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    // Register this device with the server once we have a session.
    let app_handle = app.clone();
    let token = args.token;
    let user_id = args.user_id;
    spawn(async move {
        if user_id.is_empty() {
            warn!("device registration skipped: missing user_id");
            return;
        }

        if let Err(err) = register_device_if_needed(app_handle, token, user_id).await {
            warn!("device registration failed: {err}");
        }
    });
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceCreateResponse {
    device_id: Option<String>,
    device_name: Option<String>,
    platform: Option<String>,
}

async fn register_device_if_needed<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    token: String,
    user_id: String,
) -> Result<(), String> {
    let device_name = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());
    let platform = format!("{} {}", std::env::consts::OS, std::env::consts::ARCH);

    // Avoid creating duplicates if the user logs out and logs back in.
    {
        let state = app.state::<AppState>();
        let db = state
            .db()
            .lock()
            .map_err(|_| "db mutex poisoned".to_string())?;
        match db.get_device_for_user(&user_id) {
            Ok(Some(existing)) => {
                info!(
                    "device already registered for user_id={}: device_id={}",
                    existing.user_id, existing.device_id
                );
                return Ok(());
            }
            Ok(None) => {}
            Err(err) => return Err(format!("db error checking existing device: {err}")),
        };
    }

    let client = reqwest::Client::new();
    let url = format!("{API_BASE_URL}/devices");
    let res = client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
          "name": device_name,
          "platform": platform,
        }))
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("server returned {status}: {body}"));
    }

    let body: DeviceCreateResponse = res
        .json()
        .await
        .map_err(|e| format!("failed to parse response json: {e}"))?;

    let device_id = body
        .device_id
        .ok_or_else(|| "response missing device_id".to_string())?;

    let saved = DeviceRegistration {
        user_id,
        device_id,
        device_name: body.device_name.unwrap_or_else(|| {
            whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string())
        }),
        platform: body
            .platform
            .unwrap_or_else(|| format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)),
    };

    let state = app.state::<AppState>();
    let db = state.db();
    db.lock()
        .map_err(|_| "db mutex poisoned".to_string())?
        .upsert_device_for_user(saved)
        .map_err(|e| format!("db error saving device registration: {e}"))?;

    Ok(())
}
