use std::{sync::Arc, time::Duration};

use futures_util::FutureExt;

use rust_socketio::{Payload, asynchronous::ClientBuilder};
use single_instance::SingleInstance;
use storage::Storage;
use sync::SuppresionCache;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tokio::{spawn, time::sleep};
use window::AppWindow;

mod tray;
mod window;

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

pub fn main() {
    let instance = SingleInstance::new("opensync").unwrap();
    if !instance.is_single() {
        println!("Another instance is already running!");
        std::process::exit(1);
    }
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let mut app_window = AppWindow::new(&event_loop);
    let storage = Storage::new();

    let tray = tray::TrayApp::init(&event_loop).expect("tray init failed");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = Arc::new(SuppresionCache::default());

    {
        let cache = cache.clone();
        rt.spawn(async {
            if let Err(e) = socket_task(cache).await {
                eprintln!("socket task exited with error: {e}");
            }
        });
    }

    {
        let cache = cache.clone();
        rt.spawn(async {
            clipboard::watch(cache).await;
        });
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                handle_tray_action(tray.handle_tray_event(event), &mut app_window, control_flow)
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                handle_tray_action(tray.handle_menu_event(event), &mut app_window, control_flow)
            }

            Event::WindowEvent { event, .. } => app_window.handle_window_event(&event),
            Event::RedrawRequested(_) => app_window.redraw(),
            _ => (),
        }
    });
}

fn handle_tray_action(
    action: tray::TrayAction,
    app_window: &mut AppWindow,
    control_flow: &mut ControlFlow,
) {
    match action {
        tray::TrayAction::None => {}
        tray::TrayAction::OpenWindow => {
            println!("menu action: open");
            app_window.open();
        }
        tray::TrayAction::ToggleLogs(new_value) => {
            println!("menu action: show logs toggled -> {new_value}");
        }
        tray::TrayAction::Quit => {
            println!("menu action: quit");
            *control_flow = ControlFlow::Exit;
        }
    }
}

async fn socket_task(cache: Arc<SuppresionCache>) -> Result<(), Box<dyn std::error::Error>> {
    let _socket = ClientBuilder::new("https://hsiu-sociologistic-aliya.ngrok-free.dev")
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
                            sleep(Duration::from_secs(5)).await;
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
