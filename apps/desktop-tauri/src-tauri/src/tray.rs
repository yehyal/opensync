use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

const MENU_OPEN: &str = "tray.open";
const MENU_CLOSE: &str = "tray.close";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrayAction {
    None,
    ShowMainWindow,
    HideMainWindow,
}

pub fn setup<R: Runtime, M: Manager<R>>(app: &M) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, MENU_OPEN, "Open", true, None::<&str>)?;
    let close = MenuItem::with_id(app, MENU_CLOSE, "Close", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &close])?;

    let icon = app
        .app_handle()
        .default_window_icon()
        .cloned()
        .expect("default window icon missing");

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("Opensync")
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_event)
        .build(app)?;

    Ok(())
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    let action = match event.id.as_ref() {
        MENU_OPEN => TrayAction::ShowMainWindow,
        MENU_CLOSE => TrayAction::HideMainWindow,
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
    }
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        eprintln!("tray action requested main window, but no 'main' window exists");
    }
}

fn hide_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    } else {
        eprintln!("tray action requested hiding main window, but no 'main' window exists");
    }
}
