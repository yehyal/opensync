use std::path::PathBuf;

use futures_util::FutureExt;
use rust_socketio::{
    Payload, Socket,
    asynchronous::{Client, ClientBuilder},
};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::error::Error,
};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, SubmenuBuilder},
};

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}
#[derive(Debug)]
enum AppEvent {
    Send(String),
    Receive(String),
}

pub fn main() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    // let _window = WindowBuilder::new()
    //     .with_visible(false)
    //     .build(&event_loop)
    //     .unwrap();
    let tray_menu = Menu::new();

    // Example menu items.
    let open_item = MenuItem::new("&Open", true, None);
    let show_logs_item = CheckMenuItem::new("Show &Logs", true, true, None);
    let sep_1 = PredefinedMenuItem::separator();
    let sep_2 = PredefinedMenuItem::separator();
    let quit_item = MenuItem::new("&Quit", true, None);

    let debug_hello_item = MenuItem::new("Print &Hello", true, None);
    let debug_state_item = MenuItem::new("Print &State", true, None);
    let debug_submenu = SubmenuBuilder::new()
        .text("&Debug")
        .enabled(true)
        .items(&[&debug_hello_item, &debug_state_item])
        .build()
        .unwrap();

    // macOS only allows `Submenu` items at the root `Menu`.
    #[cfg(target_os = "macos")]
    {
        let app_submenu = SubmenuBuilder::new()
            .text("OpenSync")
            .enabled(true)
            .items(&[
                &open_item,
                &sep_1,
                &show_logs_item,
                &debug_submenu,
                &sep_2,
                &quit_item,
            ])
            .build()
            .unwrap();
        let test = MenuItem::new("test", true, None);
        tray_menu.append_items(&[&test]).unwrap();
    }

    #[cfg(not(target_os = "macos"))]
    tray_menu
        .append_items(&[
            &open_item,
            &sep_1,
            &show_logs_item,
            &debug_submenu,
            &sep_2,
            &quit_item,
        ])
        .unwrap();

    let open_id = open_item.id().clone();
    let show_logs_id = show_logs_item.id().clone();
    let debug_hello_id = debug_hello_item.id().clone();
    let debug_state_id = debug_state_item.id().clone();
    let quit_id = quit_item.id().clone();

    let (icon_width, icon_height) = (22, 22);
    let mut rgba = Vec::with_capacity((icon_width * icon_height * 4) as usize);
    for _ in 0..(icon_width * icon_height) {
        rgba.extend_from_slice(&[255, 255, 255, 255]);
    }

    let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content.ico");
    println!("icon path: {:?}", icon_path.as_path());
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("system-tray")
        .with_icon(Icon::from_path(icon_path, None).expect("ICON FAILED"))
        .build()
        .unwrap();
    // .with_icon(Icon::from_rgba(rgba, icon_width, icon_height).expect("valid RGBA icon buffer"))

    let proxy = event_loop.create_proxy();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(UserEvent::TrayIconEvent(event)) {
            eprintln!("failed to proxy TrayIconEvent to event loop: {e:?}");
        }
    }));

    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(UserEvent::MenuEvent(event)) {
            eprintln!("failed to proxy MenuEvent to event loop: {e:?}");
        }
    }));

    let rt = tokio::runtime::Runtime::new().unwrap();
    // let (tx, rx) = mpsc::channel::<AppEvent>(32);

    rt.spawn(async {
        if let Err(e) = socket_task().await {
            eprintln!("socket task exited with error: {e}");
        }
    });
    rt.spawn(async {
        clipboard::watch().await;
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                println!("tray event proxied to event loop: {event:?}");
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                println!("menu event proxied to event loop: {event:?}");

                if event.id == open_id {
                    println!("menu action: open");
                } else if event.id == show_logs_id {
                    let new_value = !show_logs_item.is_checked();
                    show_logs_item.set_checked(new_value);
                    println!("menu action: show logs toggled -> {new_value}");
                } else if event.id == debug_hello_id {
                    println!("menu action: debug hello");
                } else if event.id == debug_state_id {
                    println!("menu action: debug state");
                } else if event.id == quit_id {
                    println!("menu action: quit");
                    *control_flow = ControlFlow::Exit;
                } else {
                    println!("menu action: unhandled id={}", event.id.0);
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

async fn socket_task(// mut rx: Receiver<AppEvent>,
    // tx: Sender<AppEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    // let tx_clone = tx.clone();
    let _socket = ClientBuilder::new("http://localhost:3000")
        .on("event.created", move |payload, _| {
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
