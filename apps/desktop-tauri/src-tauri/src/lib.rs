mod clipboard;
mod socket;
mod state;
mod tray;
use tauri::{Manager, State};

use crate::state::AuthState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = state::AppState::new().unwrap();
            let auth = state::AuthState::new(state.load_auth_session().unwrap());
            let cache = state::CacheState::new();

            let state_inserted = app.manage(state);
            assert!(state_inserted, "AppState was already managed");

            let auth_inserted = app.manage(auth);
            assert!(auth_inserted, "AuthState was already managed");

            let cache_inserted = app.manage(cache);
            assert!(cache_inserted, "CacheState was already managed");

            tray::setup(app)?;

            if app.state::<state::AuthState>().is_logged_in() {
                socket::start(app.app_handle().clone());
                clipboard::start(app.app_handle().clone());
            } else {
                println!("auth session missing; socket and clipboard services not started");
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_authenticated])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn is_authenticated(state: State<'_, AuthState>) -> bool {
    state.is_logged_in()
}
