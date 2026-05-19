use tauri::{
    menu::{IconMenuItem, Menu, MenuEvent, MenuItem, NativeIcon, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

use log::warn;

pub const TRAY_ID: &str = "main-tray";
const MENU_OPEN: &str = "tray.open";
const MENU_CLOSE: &str = "tray.close";
const MENU_CONNECTION: &str = "tray.connection";
const MENU_QUIT: &str = "tray.quit";

pub struct TrayState<R: Runtime> {
    connection: IconMenuItem<R>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrayAction {
    None,
    ShowMainWindow,
    HideMainWindow,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Offline,
    Connecting,
    Connected,
}

pub fn setup<R: Runtime, M: Manager<R>>(app: &M) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, MENU_OPEN, "Open", true, None::<&str>)?;
    let close = MenuItem::with_id(app, MENU_CLOSE, "Close", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;
    let connection = IconMenuItem::with_id_and_native_icon(
        app,
        MENU_CONNECTION,
        "Offline",
        false,
        Some(NativeIcon::StatusUnavailable),
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&connection, &open, &close, &separator, &quit])?;

    let icon = app
        .app_handle()
        .default_window_icon()
        .cloned()
        .expect("default window icon missing");

    let inserted = app.manage(TrayState {
        connection: connection.clone(),
    });
    assert!(inserted, "TrayState was already managed");

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .menu(&menu)
        .tooltip("Opensync")
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_event)
        .build(app)?;

    set_connection_state(app.app_handle(), ConnectionState::Offline);

    Ok(())
}

pub fn set_connection_state<R: Runtime>(app: &AppHandle<R>, state: ConnectionState) {
    let item = &app.state::<TrayState<R>>().connection;

    let (label, icon) = match state {
        ConnectionState::Offline => ("Offline", Some(NativeIcon::StatusUnavailable)),
        ConnectionState::Connecting => {
            ("Connecting...", Some(NativeIcon::StatusPartiallyAvailable))
        }
        ConnectionState::Connected => ("Connected", Some(NativeIcon::StatusAvailable)),
    };

    let _ = item.set_text(label);
    let _ = item.set_native_icon(icon);
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    let action = match event.id.as_ref() {
        MENU_OPEN => TrayAction::ShowMainWindow,
        MENU_CLOSE => TrayAction::HideMainWindow,
        MENU_QUIT => TrayAction::Quit,

        _ => TrayAction::None,
    };

    dispatch_action(app, action);
}

fn handle_tray_event<R: Runtime>(tray: &tauri::tray::TrayIcon<R>, event: TrayIconEvent) {
    let action = match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => TrayAction::ShowMainWindow,
        _ => TrayAction::None,
    };

    dispatch_action(tray.app_handle(), action);
}

fn dispatch_action<R: Runtime>(app: &AppHandle<R>, action: TrayAction) {
    match action {
        TrayAction::None => {}
        TrayAction::ShowMainWindow => show_main_window(app),
        TrayAction::HideMainWindow => hide_main_window(app),
        TrayAction::Quit => app.exit(0),
    }
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        warn!("tray action requested main window, but no 'main' window exists");
    }
}

fn hide_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    } else {
        warn!("tray action requested hiding main window, but no 'main' window exists");
    }
}
