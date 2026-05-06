use std::time::{Instant, SystemTime};

use egui::{Button, Color32, Context, Vec2};
use egui_glow::Painter;
use glow::HasContext;
use glutin::{
    config::GlConfig,
    context::{NotCurrentGlContext, PossiblyCurrentContext},
    display::GlDisplay,
    prelude::GlSurface,
    surface::{Surface, WindowSurface},
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use tao::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::UserEvent;
enum AuthState {
    Waiting,
    Idle,
    Auth,
}

pub struct AppWindow {
    pub window: Window,
    gl: std::sync::Arc<glow::Context>,
    painter: Painter,
    egui_ctx: Context,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    last_frame_time: Instant,
    start_time: SystemTime,
    egui_events: Vec<egui::Event>,
    cursor_pos: Option<egui::Pos2>,
    visible: bool,
    needs_redraw: bool,
    auth_state: AuthState,
}

impl AppWindow {
    pub fn new(event_loop: &EventLoop<UserEvent>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Opensync")
            .with_inner_size(PhysicalSize::new(600, 800))
            .build(event_loop)
            .unwrap();

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

        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .build(Some(raw_window_handle.as_raw()));

        let context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .unwrap()
        };

        let surface_attributes = glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new()
            .build(
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
            glow::Context::from_loader_function(|name| {
                let c_str = std::ffi::CString::new(name).unwrap();
                display.get_proc_address(&c_str) as *const _
            })
        };

        let gl = std::sync::Arc::new(gl);
        let painter =
            Painter::new(gl.clone(), "", None, false).expect("failed to create egui glow painter");
        let egui_ctx = Context::default();
        egui_ctx.set_pixels_per_point(1.25);

        Self {
            window,
            gl,
            painter,
            egui_ctx,
            surface,
            context,
            last_frame_time: Instant::now(),
            start_time: SystemTime::now(),
            egui_events: Vec::new(),
            cursor_pos: None,
            visible: true,
            needs_redraw: true,
            auth_state: AuthState::Idle,
        }
    }

    pub fn open(&mut self) {
        self.window.set_visible(true);
        self.window.set_minimized(false);
        self.window.set_focus();
        self.visible = true;
        self.request_redraw();
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button was pressed; hiding window");
                self.window.set_visible(false);
                self.visible = false;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let scale_factor = self.window.scale_factor() as f32;
                let pos = egui::pos2(
                    position.x as f32 / scale_factor,
                    position.y as f32 / scale_factor,
                );
                self.cursor_pos = Some(pos);
                self.egui_events.push(egui::Event::PointerMoved(pos));
                self.request_redraw();
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor_pos = None;
                self.egui_events.push(egui::Event::PointerGone);
                self.request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let (Some(pos), Some(button)) = (self.cursor_pos, map_mouse_button(*button)) {
                    self.egui_events.push(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed: *state == ElementState::Pressed,
                        modifiers: egui::Modifiers::default(),
                    });
                    self.request_redraw();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    MouseScrollDelta::LineDelta(x, y) => egui::vec2(*x, *y) * 24.0,
                    MouseScrollDelta::PixelDelta(delta) => {
                        egui::vec2(delta.x as f32, delta.y as f32)
                    }
                    _ => egui::Vec2::ZERO,
                };
                self.egui_events.push(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta,
                    modifiers: egui::Modifiers::default(),
                });
                self.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.surface.resize(
                    &self.context,
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                );
                self.request_redraw();
            }
            _ => {}
        }
    }

    pub fn redraw(&mut self) {
        if !self.visible || !self.needs_redraw {
            return;
        }

        self.needs_redraw = false;

        let now = Instant::now();
        let delta = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        let scale_factor = self.window.scale_factor() as f32;
        let inner_size = self.window.inner_size();

        let raw_input = egui::RawInput {
            time: Some(
                self.start_time
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
            events: std::mem::take(&mut self.egui_events),
            ..Default::default()
        };

        self.egui_ctx.begin_pass(raw_input);

        egui::CentralPanel::default().show(&self.egui_ctx, |ui| {
            ui.heading("OpenSync");
            ui.separator();

            match self.auth_state {
                AuthState::Idle => {
                    let login = Button::new("test")
                        .fill(Color32::RED)
                        .min_size(Vec2::new(120_f32, 40_f32));
                    if ui.add(login).clicked() {
                        match webbrowser::open("http://localhost:3000/login") {
                            Ok(_) => println!("Login clicked"),
                            Err(e) => println!("Failed to open browser: {:?}", e),
                        }
                        println!("Login clicked");
                    }

                    if ui.button("Register").clicked() {
                        println!("Register clicked");
                    }

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.label("not logged in");
                    });
                }

                AuthState::Waiting => {
                    ui.label("Waiting for authentication...");
                    ui.add(egui::Spinner::new());
                }

                AuthState::Auth => {
                    ui.label("✅ Logged in");

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.label("not logged in");
                    });
                }
            }
        });

        let full_output = self.egui_ctx.end_pass();
        let clipped_primitives = self
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        let textures_delta = full_output.textures_delta;

        unsafe {
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }

        self.painter.paint_and_update_textures(
            [inner_size.width, inner_size.height],
            scale_factor,
            &clipped_primitives,
            &textures_delta,
        );

        self.surface.swap_buffers(&self.context).unwrap();
    }

    fn request_redraw(&mut self) {
        self.needs_redraw = true;
        self.window.request_redraw();
    }
}

fn map_mouse_button(button: MouseButton) -> Option<egui::PointerButton> {
    match button {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        MouseButton::Right => Some(egui::PointerButton::Secondary),
        MouseButton::Middle => Some(egui::PointerButton::Middle),
        _ => None,
    }
}
