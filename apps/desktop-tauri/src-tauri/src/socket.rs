use std::time::Duration;

use crate::state::{AuthState, CacheState};
use futures_util::FutureExt;
use rust_socketio::{asynchronous::ClientBuilder, Payload};
use serde_json::json;
use tauri::{async_runtime::spawn, AppHandle, Manager};

pub fn start(app: AppHandle) {
    if !app.state::<AuthState>().is_logged_in() {
        println!("socket service not started: user is not logged in");
        return;
    }

    spawn(async move {
        if let Err(error) = socket_task(app).await {
            eprintln!("socket task exited with error: {error}");
        }
    });
}

async fn socket_task(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let cache = app.state::<CacheState>().cache();
    let session = app
        .state::<AuthState>()
        .session()
        .ok_or("socket service requires an authenticated session")?;
    // let _socket = ClientBuilder::new("https://hsiu-sociologistic-aliya.ngrok-free.dev")
    let _socket = ClientBuilder::new("http://localhost:3000")
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
                    println!("socket event.created payload: {values:?}");

                    if let Some(first) = values.first() {
                        let text = first
                            .as_str()
                            .map(str::to_owned)
                            .unwrap_or_else(|| first.to_string());

                        println!("socket event.created text: {text}");

                        if cache.contains(&text) {
                            return;
                        };

                        let hash = cache.insert(text.clone());
                        spawn(async move {
                            tokio::time::sleep(Duration::from_secs(5)).await;
                            cache.remove(&hash);
                        });

                        if let Err(e) = clipboard::add(&text) {
                            eprintln!("failed to set clipboard text: {e}");
                        }
                    }
                }
            }
            .boxed()
        })
        .connect()
        .await?;

    futures_util::future::pending::<()>().await;

    #[allow(unreachable_code)]
    Ok(())
}
