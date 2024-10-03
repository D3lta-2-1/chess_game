use std::sync::Arc;
use std::time::{Duration, Instant};
use vello::{wgpu, AaConfig, Renderer, RendererOptions, Scene};
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;

pub trait LogicHandler {
    fn on_mouse_click(&mut self, x: f64, y: f64);
    fn on_exit_press(&mut self);
    fn draw(&mut self, scene: &mut Scene, duration: Duration);
}
pub struct ActiveRenderState<'s> {
    surface: RenderSurface<'s>,
    window: Arc<Window>,
}

enum RenderState<'s> {
    Active(ActiveRenderState<'s>),
    // Cache a window so that it can be reused when the app is resumed after being suspended
    Suspended(Option<Arc<Window>>),
}

pub struct SimpleVelloApp<'s, T: LogicHandler> {
    // The vello RenderContext which is a global context that lasts for the
    // lifetime of the application
    context: RenderContext,

    // An array of renderers, one per wgpu device
    renderers: Vec<Option<Renderer>>,

    // State for our example where we store the winit Window and the wgpu Surface
    state: RenderState<'s>,

    // A vello Scene which is a data structure which allows one to build up a
    // description a scene to be drawn (with paths, fills, images, text, etc)
    // which is then passed to a renderer for rendering
    scene: Scene,
    last_frame_time: Instant,
    last_cursor_pos: PhysicalPosition<f64>,
    logic_handler: T
}

impl<'s, T: LogicHandler> SimpleVelloApp<'s, T> {
    pub fn new(logic_handler: T) -> Self {
        Self {
            context: RenderContext::new(),
            renderers: vec![],
            state: RenderState::Suspended(None),
            scene: Default::default(),
            last_frame_time: Instant::now(),
            last_cursor_pos: Default::default(),
            logic_handler
        }
    }

    fn create_winit_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
        let attr = Window::default_attributes()
            //.with_inner_size(LogicalSize::new(720, 480))
            .with_resizable(true)
            .with_title("Chess");
        Arc::new(event_loop.create_window(attr).unwrap())
    }

    /// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
    fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface) -> Renderer {
        Renderer::new(
            &render_cx.devices[surface.dev_id].device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )
            .expect("Couldn't create renderer")
    }
}

impl<'s, T: LogicHandler> ApplicationHandler for SimpleVelloApp<'s, T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        // Get the winit window cached in a previous Suspended event or else create a new window
        let window = cached_window
            .take()
            .unwrap_or_else(|| Self::create_winit_window(event_loop));

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
            .get_or_insert_with(|| Self::create_vello_renderer(&self.context, &surface));

        // Save the Window and Surface to a state variable
        self.state = RenderState::Active(ActiveRenderState { window, surface });

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Ignore the event (return from the function) if
        //   - we have no render_state
        //   - OR the window id of the event doesn't match the window id of our render_state
        //
        // Else extract a mutable reference to the render state from its containing option for use below
        let render_state = match &mut self.state {
            RenderState::Active(state) if state.window.id() == window_id => state,
            _ => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                self.last_cursor_pos = position;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    self.logic_handler.on_mouse_click(self.last_cursor_pos.x, self.last_cursor_pos.y);
                }
            }
            WindowEvent::Touch(touch) => {
                if touch.phase == winit::event::TouchPhase::Ended {
                    self.logic_handler.on_mouse_click(touch.location.x, touch.location.y);
                }
            }

            WindowEvent::KeyboardInput { event,.. } => {
                if event.logical_key == Key::Named(NamedKey::Escape) && event.state == ElementState::Released {
                    self.logic_handler.on_exit_press()
                }
            }
            WindowEvent::Resized(size) => {
                self.context
                    .resize_surface(&mut render_state.surface, size.width, size.height);
                render_state.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                // Empty the scene of objects to draw. You could create a new Scene each time, but in this case
                // the same Scene is reused so that the underlying memory allocation can also be reused.
                self.scene.reset();

                // Get the RenderSurface (surface + config)
                let surface = &render_state.surface;

                // Get the window size
                let width = surface.config.width;
                let height = surface.config.height;

                // Re-add the objects to draw to the scene.
                let now = Instant::now();
                self.logic_handler.draw(&mut self.scene, now - self.last_frame_time);
                self.last_frame_time = now;

                // Get a handle to the device
                let device_handle = &self.context.devices[surface.dev_id];

                // Get the surface's texture
                let surface_texture = surface
                    .surface
                    .get_current_texture()
                    .expect("failed to get surface texture");

                // Render to the surface's texture
                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_surface(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &surface_texture,
                        &vello::RenderParams {
                            base_color: Color::GRAY, // Background color
                            width,
                            height,
                            antialiasing_method: AaConfig::Msaa8,
                        },
                    )
                    .expect("failed to render to surface");

                // Queue the texture to be presented on the surface
                surface_texture.present();
                device_handle.device.poll(wgpu::Maintain::Poll);
                if let RenderState::Active(context) = &self.state {
                    context.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        if let RenderState::Active(state) = &self.state {
            self.state = RenderState::Suspended(Some(state.window.clone()));
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }
}