use std::sync::{atomic::{AtomicBool, Ordering}, Mutex};

use log::{info, warn};
use tauri::{async_runtime::spawn, AppHandle, Emitter, Manager, Runtime};
use tokio::sync::broadcast;

use crate::{clipboard, socket, state::AuthState};

pub struct ServicesState {
    enabled: AtomicBool,
    inner: Mutex<ServicesInner>,
}

struct ServicesInner {
    running: Option<RunningServices>,
}

struct RunningServices {
    shutdown_tx: broadcast::Sender<()>,
    socket_task: tauri::async_runtime::JoinHandle<()>,
    clipboard_task: tauri::async_runtime::JoinHandle<()>,
}

impl ServicesState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            inner: Mutex::new(ServicesInner { running: None }),
        }
    }

    /// Future-proof switch: allows disabling services even if authenticated.
    pub fn set_enabled<R: Runtime>(&self, app: &AppHandle<R>, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
        self.sync_with_auth(app);
    }

    /// Ensures background services reflect the current authentication state.
    pub fn sync_with_auth<R: Runtime>(&self, app: &AppHandle<R>) {
        let should_run = self.enabled.load(Ordering::SeqCst) && app.state::<AuthState>().is_logged_in();
        if should_run {
            self.start(app);
        } else {
            self.stop(app);
        }
    }

    pub fn start<R: Runtime>(&self, app: &AppHandle<R>) {
        let mut inner = self.inner.lock().unwrap();
        if inner.running.is_some() {
            return;
        }

        let (shutdown_tx, _) = broadcast::channel(1);

        info!("starting services");
        let socket_task = socket::start(app.clone(), shutdown_tx.subscribe());
        let clipboard_task = clipboard::start(app.clone(), shutdown_tx.subscribe());

        inner.running = Some(RunningServices {
            shutdown_tx,
            socket_task,
            clipboard_task,
        });

        let _ = app.emit("services://changed", true);
    }

    pub fn stop<R: Runtime>(&self, app: &AppHandle<R>) {
        let running = {
            let mut inner = self.inner.lock().unwrap();
            inner.running.take()
        };

        let Some(running) = running else {
            return;
        };

        info!("stopping services");
        let _ = running.shutdown_tx.send(());

        // Join tasks asynchronously; don't block the event loop.
        spawn(async move {
            let _ = running.socket_task.await;
            let _ = running.clipboard_task.await;
        });

        let _ = app.emit("services://changed", false);
    }
}

pub fn stop_on_exit<R: Runtime>(app: &AppHandle<R>) {
    if let Some(services) = app.try_state::<ServicesState>() {
        services.stop(app);
    } else {
        warn!("ServicesState not managed; nothing to stop");
    }
}
