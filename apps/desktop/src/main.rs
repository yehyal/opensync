use std::{sync::Arc, time::Duration};

use futures_util::FutureExt;

use rust_socketio::{Payload, asynchronous::ClientBuilder};
use sync::SuppresionCache;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tokio::{spawn, time::sleep};

mod tray;

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}
pub fn main() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    // let _window = WindowBuilder::new()
    //     .with_visible(false)
    //     .build(&event_loop)
    //     .unwrap();
    let tray = tray::TrayApp::init(&event_loop).expect("tray init failed");
    let rt = tokio::runtime::Runtime::new().unwrap();
    // let (tx, rx) = mpsc::channel::<AppEvent>(32);
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
        *control_flow = ControlFlow::Poll;
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                tray.handle_tray_event(event);
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                println!("menu event proxied to event loop: {event:?}");
                match tray.handle_menu_event(event) {
                    Some(tray::TrayMenuAction::Open) => println!("menu action: open"),
                    Some(tray::TrayMenuAction::ToggleLogs(new_value)) => {
                        println!("menu action: show logs toggled -> {new_value}")
                    }
                    Some(tray::TrayMenuAction::DebugHello) => println!("menu action: debug hello"),
                    Some(tray::TrayMenuAction::DebugState) => println!("menu action: debug state"),
                    Some(tray::TrayMenuAction::Quit) => {
                        println!("menu action: quit");
                        *control_flow = ControlFlow::Exit;
                    }
                    None => println!("menu action: unhandled"),
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close button was pressed");
                *control_flow = ControlFlow::Exit;
            }

            _ => (),
        }
    });
}

async fn socket_task(
    cache: Arc<SuppresionCache>, // tx: Sender<AppEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    // let tx_clone = tx.clone();
    // let _socket = ClientBuilder::new("http://localhost:3000")
    let _socket = ClientBuilder::new("https://hsiu-sociologistic-aliya.ngrok-free.dev")
        .on("event.created", move |payload, _| {
            let cache = cache.clone();
            async move {
                if let Payload::Text(values) = payload {
                    println!("socket event.created payload: {values:?}");

                    if let Some(first) = values.first() {
                        // `Payload::Text` is JSON values. Prefer unquoted string extraction.
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

    // Keep the socket client alive; dropping it disconnects.
    futures_util::future::pending::<()>().await;

    // while let Some(event) = rx.recv().await {
    //     if let AppEvent::Send(data) = event {
    //         let _ = socket.emit("event", data).await;
    //     }
    // }
    #[allow(unreachable_code)]
    Ok(())
}
