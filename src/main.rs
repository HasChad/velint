use anyhow::Result;
use std::sync::Arc;
use vello::kurbo::{BezPath, Point};
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaConfig, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::window::Window;

mod brush_style;
mod draw_controller;

use draw_controller::*;

use crate::brush_style::brush_complete;

#[derive(Debug)]
enum RenderState {
    Active {
        surface: Box<RenderSurface<'static>>,
        valid_surface: bool,
        window: Arc<Window>,
    },
    Suspended(Option<Arc<Window>>),
}

struct ShapeProp {
    path: BezPath,
    color: Color,
    size: f32,
}

enum Shapes {
    Brush(ShapeProp),
    Rectangle(ShapeProp),
}

enum Tool {
    Brush,
    Rectangle,
}

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub held: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        Self {
            held: false,
            just_pressed: false,
            just_released: false,
        }
    }
    pub fn update(&mut self, new_held: bool) {
        self.just_pressed = new_held && !self.held;
        self.just_released = !new_held && self.held;
        self.held = new_held;
    }
}

struct AppState {
    mouse_pos: Option<Point>,
    mouse_left: ButtonState,
    mouse_right: ButtonState,
    shapes: Vec<Shapes>,
    points: Vec<Point>,
    current_tool: Tool,
    brush_size: f32,
    brush_color: Color,
    bg_color: Color,
}

impl AppState {
    fn new() -> Self {
        Self {
            mouse_pos: None,
            mouse_left: ButtonState::new(),
            mouse_right: ButtonState::new(),
            points: vec![],
            shapes: vec![],
            current_tool: Tool::Brush,
            brush_size: 5.0,
            brush_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

struct Velint {
    context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    render_state: RenderState,
    scene: Scene,
    app_state: AppState,
}

impl ApplicationHandler for Velint {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.render_state else {
            return;
        };

        // Get the winit window cached in a previous Suspended event or else create a new window
        let window = cached_window
            .take()
            .unwrap_or_else(|| create_winit_window(event_loop));

        // Create a vello Surface
        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Create a vello Renderer for the surface (using its device id)
        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id]
            .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

        // Save the Window and Surface to a state variable
        self.render_state = RenderState::Active {
            surface: Box::new(surface),
            valid_surface: true,
            window,
        };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.render_state {
            self.render_state = RenderState::Suspended(Some(window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Only process events for our window, and only when we have a surface.
        let (surface, valid_surface) = match &mut self.render_state {
            RenderState::Active {
                surface,
                valid_surface,
                window,
            } if window.id() == window_id => (surface, valid_surface),
            _ => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.app_state.mouse_pos = Some(Point {
                    x: position.x,
                    y: position.y,
                });

                // Request redraw. maybe i should find a better way
                if let RenderState::Active { window, .. } = &self.render_state {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                let mut raw_mouse_left = false;
                let mut raw_mouse_right = false;

                if button == MouseButton::Left {
                    raw_mouse_left = state == ElementState::Pressed;
                }
                if button == MouseButton::Right {
                    raw_mouse_right = state == ElementState::Pressed;
                }

                self.app_state.mouse_left.update(raw_mouse_left);
                self.app_state.mouse_right.update(raw_mouse_right);

                if let RenderState::Active { window, .. } = &self.render_state {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    if event.state.is_pressed() {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    self.context
                        .resize_surface(surface, size.width, size.height);
                    *valid_surface = true;
                } else {
                    *valid_surface = false;
                }
            }
            WindowEvent::RedrawRequested => {
                if !*valid_surface {
                    return;
                }

                self.scene.reset();

                process_mouse(&mut self.app_state);
                add_shapes_to_scene(&mut self.scene, &mut self.app_state);

                let width = surface.config.width;
                let height = surface.config.height;

                let device_handle = &self.context.devices[surface.dev_id];

                // Render to a texture, which we will later copy into the surface
                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_texture(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &surface.target_view,
                        &vello::RenderParams {
                            base_color: self.app_state.bg_color, // Background color
                            width,
                            height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .expect("failed to render to surface");

                // Get the surface's texture
                let surface_texture = surface
                    .surface
                    .get_current_texture()
                    .expect("failed to get surface texture");

                // Perform the copy
                let mut encoder =
                    device_handle
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Surface Blit"),
                        });
                surface.blitter.copy(
                    &device_handle.device,
                    &mut encoder,
                    &surface.target_view,
                    &surface_texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                );
                device_handle.queue.submit([encoder.finish()]);
                // Queue the texture to be presented on the surface
                surface_texture.present();

                device_handle.device.poll(wgpu::PollType::Poll).unwrap();
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let mut app = Velint {
        context: RenderContext::new(),
        renderers: vec![],
        render_state: RenderState::Suspended(None),
        scene: Scene::new(),
        app_state: AppState::new(),
    };

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}

fn create_winit_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    let attr = Window::default_attributes()
        .with_inner_size(LogicalSize::new(800, 600))
        .with_resizable(true)
        .with_title("Velint");
    Arc::new(event_loop.create_window(attr).unwrap())
}

fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}

fn process_mouse(app_state: &mut AppState) {
    if app_state.mouse_left.held {
        if let Some(mpos) = app_state.mouse_pos {
            app_state.points.push(mpos);
        }
    }

    if app_state.mouse_left.just_released {
        app_state.shapes.push(brush_complete(app_state));
        app_state.points.clear();
    }
}

fn add_shapes_to_scene(scene: &mut Scene, app_state: &mut AppState) {
    prev_shape_draw(scene, app_state);
    current_shape_draw(scene, app_state);
    cursor_draw(scene, app_state);
}
