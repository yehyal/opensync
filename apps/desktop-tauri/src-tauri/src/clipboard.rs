use tauri::{async_runtime as ar, AppHandle, Manager, Runtime};

use crate::state::{AuthState, CacheState};
use log::info;
use tokio::sync::broadcast;

pub fn start<R: Runtime>(
    app: AppHandle<R>,
    mut shutdown: broadcast::Receiver<()>,
) -> ar::JoinHandle<()> {
    ar::spawn(async move {
        if !app.state::<AuthState>().is_logged_in() {
            info!("clipboard service not started: user is not logged in");
            return;
        }

        let cache = app.state::<CacheState>().cache();
        tokio::select! {
            _ = clipboard::watch(cache) => {},
            _ = shutdown.recv() => {
                info!("clipboard service shutdown requested");
            }
        }
    })
}
