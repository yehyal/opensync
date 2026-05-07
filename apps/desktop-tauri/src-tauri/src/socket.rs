use std::time::Duration;

use crate::state::{AuthState, CacheState};
use futures_util::FutureExt;
use log::{error, info, warn};
use rust_socketio::{asynchronous::ClientBuilder, Payload};
use serde_json::json;
use tauri::{async_runtime as ar, AppHandle, Manager, Runtime};
use tokio::sync::broadcast;

pub fn start<R: Runtime>(
    app: AppHandle<R>,
    mut shutdown: broadcast::Receiver<()>,
) -> ar::JoinHandle<()> {
    ar::spawn(async move {
        if !app.state::<AuthState>().is_logged_in() {
            info!("socket service not started: user is not logged in");
            return;
        }

        if let Err(error) = socket_task(app, &mut shutdown).await {
            error!("socket task exited with error: {error}");
        }
    })
}

async fn socket_task<R: Runtime>(
    app: AppHandle<R>,
    shutdown: &mut broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache = app.state::<CacheState>().cache();
    let session = app
        .state::<AuthState>()
        .session()
        .ok_or("socket service requires an authenticated session")?;
    // let _socket = ClientBuilder::new("https://hsiu-sociologistic-aliya.ngrok-free.dev")
    let socket = ClientBuilder::new("http://localhost:3000")
        .auth(json!({
            "token": session.token,
            "userId": session.user_id,
            "email": session.email,
            "name": session.name,
        }))
        .on("event.created", move |payload, _| {
            let cache = cache.clone();
            async move {
                if let Payload::Text(values) = payload {
                    info!("socket event.created payload: {values:?}");

                    if let Some(first) = values.first() {
                        let text = first
                            .as_str()
                            .map(str::to_owned)
                            .unwrap_or_else(|| first.to_string());

                        info!("socket event.created text: {text}");

                        if cache.contains(&text) {
                            return;
                        };

                        let hash = cache.insert(text.clone());
                        ar::spawn(async move {
                            tokio::time::sleep(Duration::from_secs(5)).await;
                            cache.remove(&hash);
                        });

                        if let Err(e) = clipboard::add(&text) {
                            warn!("failed to set clipboard text: {e}");
                        }
                    }
                }
            }
            .boxed()
        })
        .connect()
        .await?;

    tokio::select! {
        _ = futures_util::future::pending::<()>() => {},
        _ = shutdown.recv() => {
            info!("socket service shutdown requested");
        }
    }

    drop(socket);
    Ok(())
}
