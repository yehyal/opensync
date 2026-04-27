use std::path::{Path, PathBuf};

use image::ImageError;
use tao::event_loop::EventLoop;
use tray_icon::{
    Error, Icon, TrayIcon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, SubmenuBuilder},
};

use crate::UserEvent;

pub enum TrayMenuAction {
    Open,
    ToggleLogs(bool),
    DebugHello,
    DebugState,
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

fn icon_from_image_file(path: &Path) -> Result<Icon, LoadIconError> {
    // Tray icons are rendered by the OS at fairly small sizes. If we pass a huge image
    // (e.g. 1024x1024), it can get downscaled poorly. Resize to a sensible target first.
    //
    // Note: exact rendered size is platform/desktop-environment dependent, but providing
    // a moderately-sized icon generally looks better than relying on implicit scaling.
    let target_size: u32 = if cfg!(target_os = "macos") { 200 } else { 48 };

    let dyn_img = image::open(path)?;
    let rgba = dyn_img
        // .crop(50, 100, 1024, 1024)
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

        let icon_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icon.png");
        let icon = icon_from_image_file(&icon_path).unwrap_or_else(|e| {
            eprintln!(
                "failed to load tray icon from {}: {e:?}",
                icon_path.display()
            );
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

    pub fn handle_tray_event(&self, event: tray_icon::TrayIconEvent) {
        println!("tray event proxied to event loop: {event:?}");
    }

    pub fn handle_menu_event(&self, event: tray_icon::menu::MenuEvent) -> Option<TrayMenuAction> {
        if event.id == self.open_id {
            return Some(TrayMenuAction::Open);
        }

        if event.id == self.show_logs_id {
            let new_value = !self.show_logs_item.is_checked();
            self.show_logs_item.set_checked(new_value);
            return Some(TrayMenuAction::ToggleLogs(new_value));
        }

        if event.id == self.debug_hello_id {
            return Some(TrayMenuAction::DebugHello);
        }

        if event.id == self.debug_state_id {
            return Some(TrayMenuAction::DebugState);
        }

        if event.id == self.quit_id {
            return Some(TrayMenuAction::Quit);
        }

        None
    }
}
