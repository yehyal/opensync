use tauri::{async_runtime::spawn, AppHandle, Manager};

use crate::state::{AuthState, CacheState};

pub fn start(app: AppHandle) {
    if !app.state::<AuthState>().is_logged_in() {
        println!("clipboard service not started: user is not logged in");
        return;
    }

    spawn(async move {
        let cache = app.state::<CacheState>().cache();
        clipboard::watch(cache).await;
    });
}
