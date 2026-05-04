use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

use futures_util::FutureExt;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rust_socketio::{Payload, asynchronous::ClientBuilder};
use sync::SuppresionCache;
use tao::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use tokio::{spawn, time::sleep};

mod tray;

use egui::{Button, Color32, Context};
use egui_glow::Painter;
use glow::HasContext;
use glutin::{
    config::GlConfig, context::NotCurrentGlContext, display::GlDisplay, prelude::GlSurface,
};

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn map_mouse_button(button: MouseButton) -> Option<egui::PointerButton> {
    match button {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        MouseButton::Right => Some(egui::PointerButton::Secondary),
        MouseButton::Middle => Some(egui::PointerButton::Middle),
        _ => None,
    }
}

pub fn main() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("OpenSync")
        .with_inner_size(PhysicalSize::new(1200, 600))
        .build(&event_loop)
        .unwrap();

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

    // Create OpenGL context using glutin
    let raw_display_handle = window.display_handle().unwrap();
    let raw_window_handle = window.window_handle().unwrap();

    let display = unsafe {
        glutin::display::Display::new(
            raw_display_handle.as_raw(),
            glutin::display::DisplayApiPreference::Cgl,
        )
        .unwrap()
    };

    let template = glutin::config::ConfigTemplateBuilder::new()
        .with_surface_type(glutin::config::ConfigSurfaceTypes::WINDOW)
        .build();

    let config = unsafe {
        display
            .find_configs(template)
            .unwrap()
            .reduce(|accum, config| {
                if config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .unwrap()
    };

    let context_attributes =
        glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle.as_raw()));

    let context = unsafe {
        display
            .create_context(&config, &context_attributes)
            .unwrap()
    };

    let surface_attributes =
        glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
            raw_window_handle.as_raw(),
            window.inner_size().width.try_into().unwrap(),
            window.inner_size().height.try_into().unwrap(),
        );

    let surface = unsafe {
        display
            .create_window_surface(&config, &surface_attributes)
            .unwrap()
    };

    let context = context.make_current(&surface).unwrap();

    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            let c_str = std::ffi::CString::new(s).unwrap();
            display.get_proc_address(&c_str) as *const _
        })
    };

    let gl = std::sync::Arc::new(gl);
    // `shader_prefix` must contain valid GLSL preprocessor text or be empty.
    // Passing the app title here makes shader compilation fail at startup.
    let mut painter = Painter::new(gl.clone(), "", None, false).expect("Failed to create painter");

    let ctx = Context::default();
    let mut last_time = Instant::now();
    let start_time = SystemTime::now();
    let mut egui_events = Vec::new();
    let mut cursor_pos = None;
    let mut window_visible = true;
    let mut needs_redraw = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                tray.handle_tray_event(event);
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                println!("menu event proxied to event loop: {event:?}");
                match tray.handle_menu_event(event) {
                    Some(tray::TrayMenuAction::Open) => {
                        println!("menu action: open");
                        window.set_visible(true);
                        window.set_minimized(false);
                        window.set_focus();
                        window_visible = true;
                        needs_redraw = true;
                        window.request_redraw();
                    }
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
                println!("Close button was pressed; hiding window");
                window.set_visible(false);
                window_visible = false;
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let scale_factor = window.scale_factor() as f32;
                let pos = egui::pos2(
                    position.x as f32 / scale_factor,
                    position.y as f32 / scale_factor,
                );
                cursor_pos = Some(pos);
                egui_events.push(egui::Event::PointerMoved(pos));
                needs_redraw = true;
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CursorLeft { .. },
                ..
            } => {
                cursor_pos = None;
                egui_events.push(egui::Event::PointerGone);
                needs_redraw = true;
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if let (Some(pos), Some(button)) = (cursor_pos, map_mouse_button(button)) {
                    egui_events.push(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed: state == ElementState::Pressed,
                        modifiers: egui::Modifiers::default(),
                    });
                    needs_redraw = true;
                    window.request_redraw();
                }
            }

            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                let delta = match delta {
                    MouseScrollDelta::LineDelta(x, y) => egui::vec2(x, y) * 24.0,
                    MouseScrollDelta::PixelDelta(delta) => {
                        egui::vec2(delta.x as f32, delta.y as f32)
                    }
                    _ => egui::Vec2::ZERO,
                };
                egui_events.push(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta,
                    modifiers: egui::Modifiers::default(),
                });
                needs_redraw = true;
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                surface.resize(
                    &context,
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                );
                needs_redraw = true;
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                if !window_visible || !needs_redraw {
                    return;
                }

                needs_redraw = false;
                let now = Instant::now();
                let delta = now.duration_since(last_time).as_secs_f32();
                last_time = now;
                let scale_factor = window.scale_factor() as f32;
                let inner_size = window.inner_size();

                let raw_input = egui::RawInput {
                    time: Some(
                        start_time
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64(),
                    ),
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::Vec2::new(
                            inner_size.width as f32 / scale_factor,
                            inner_size.height as f32 / scale_factor,
                        ),
                    )),
                    predicted_dt: delta,
                    events: std::mem::take(&mut egui_events),
                    ..Default::default()
                };

                ctx.begin_pass(raw_input);

                egui::CentralPanel::default().show(&ctx, |ui| {
                    ui.heading("OpenSync");
                    ui.separator();
                    let login = Button::new("test").fill(Color32::RED);
                    if ui.add(login).clicked() {
                        println!("Login clicked");
                    }
                    if ui.button("Register").clicked() {
                        println!("Register clicked");
                    }

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.label("Connected");
                    });
                });

                let full_output = ctx.end_pass();
                let clipped_primitives =
                    ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
                let textures_delta = full_output.textures_delta;

                unsafe {
                    gl.clear_color(0.0, 0.0, 0.0, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                painter.paint_and_update_textures(
                    [inner_size.width, inner_size.height],
                    scale_factor,
                    &clipped_primitives,
                    &textures_delta,
                );

                surface.swap_buffers(&context).unwrap();
            }

            _ => (),
        }
    });
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
