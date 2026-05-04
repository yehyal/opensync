use image::ImageError;
use tao::event_loop::EventLoop;
use tray_icon::{
    Error, Icon, TrayIcon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, SubmenuBuilder},
};

use crate::UserEvent;

pub enum TrayAction {
    None,
    OpenWindow,
    ToggleLogs(bool),
    Quit,
}

#[derive(Debug)]
enum LoadIconError {
    Image(ImageError),
    BadIcon(tray_icon::BadIcon),
}

impl From<ImageError> for LoadIconError {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}

impl From<tray_icon::BadIcon> for LoadIconError {
    fn from(value: tray_icon::BadIcon) -> Self {
        Self::BadIcon(value)
    }
}

fn icon_from_image_bytes(bytes: &[u8]) -> Result<Icon, LoadIconError> {
    // Same resizing strategy as `icon_from_image_file`, but for embedded assets.
    let target_size: u32 = if cfg!(target_os = "macos") { 32 } else { 48 };

    let dyn_img = image::load_from_memory(bytes)?;
    let rgba = dyn_img
        .resize(
            target_size,
            target_size,
            image::imageops::FilterType::Lanczos3,
        )
        .into_rgba8();

    Ok(Icon::from_rgba(rgba.into_raw(), target_size, target_size)?)
}

pub struct TrayApp {
    // Must be kept alive; dropping disconnects the tray icon and stops events.
    _tray_icon: TrayIcon,

    open_id: tray_icon::menu::MenuId,
    show_logs_id: tray_icon::menu::MenuId,
    debug_hello_id: tray_icon::menu::MenuId,
    debug_state_id: tray_icon::menu::MenuId,
    quit_id: tray_icon::menu::MenuId,

    show_logs_item: CheckMenuItem,
}

impl TrayApp {
    pub fn init(event_loop: &EventLoop<UserEvent>) -> Result<Self, Error> {
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
            tray_menu.append(&app_submenu).unwrap();
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

        // Embed the icon at compile-time so it ships with the binary.
        const ICON_PNG: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png"));
        let icon = icon_from_image_bytes(ICON_PNG).unwrap_or_else(|e| {
            eprintln!("failed to load embedded tray icon: {e:?}");
            Icon::from_rgba(rgba, icon_width, icon_height).expect("fallback RGBA icon buffer")
        });
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("system-tray")
            .with_icon(icon)
            .build()
            .unwrap();

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

        Ok(Self {
            _tray_icon: tray_icon,
            open_id,
            show_logs_id,
            debug_hello_id,
            debug_state_id,
            quit_id,
            show_logs_item,
        })
    }

    pub fn handle_tray_event(&self, _event: tray_icon::TrayIconEvent) -> TrayAction {
        TrayAction::None
    }

    pub fn handle_menu_event(&self, event: tray_icon::menu::MenuEvent) -> TrayAction {
        if event.id == self.open_id {
            return TrayAction::OpenWindow;
        }

        if event.id == self.show_logs_id {
            let new_value = !self.show_logs_item.is_checked();
            self.show_logs_item.set_checked(new_value);
            return TrayAction::ToggleLogs(new_value);
        }

        if event.id == self.debug_hello_id {
            println!("menu action: debug hello");
            return TrayAction::None;
        }

        if event.id == self.debug_state_id {
            println!("menu action: debug state");
            return TrayAction::None;
        }

        if event.id == self.quit_id {
            return TrayAction::Quit;
        }

        println!("menu action: unhandled id={}", event.id.0);
        TrayAction::None
    }
}
