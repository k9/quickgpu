use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

#[allow(unused_imports)]
use winit::dpi::PhysicalSize;

use crate::app::{App, redraw, resize};

struct FrameCounter {
    // Instant of the last time we printed the frame time.
    last_printed_instant: web_time::Instant,
    // Number of frames since the last time we printed the frame time.
    frame_count: u32,
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            last_printed_instant: web_time::Instant::now(),
            frame_count: 0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let new_instant = web_time::Instant::now();
        let elapsed_secs = (new_instant - self.last_printed_instant).as_secs_f32();
        if elapsed_secs > 1.0 {
            let elapsed_ms = elapsed_secs * 1000.0;
            let frame_time = elapsed_ms / self.frame_count as f32;
            let fps = self.frame_count as f32 / elapsed_secs;
            log::info!("Frame time {frame_time:.2}ms ({fps:.1} FPS)");

            self.last_printed_instant = new_instant;
            self.frame_count = 0;
        }
    }
}

pub struct AppLoader {
    app: Rc<Mutex<Option<App>>>,
    frame_counter: FrameCounter,
}

impl Default for AppLoader {
    fn default() -> Self {
        AppLoader {
            app: Rc::new(Mutex::new(None)),
            frame_counter: FrameCounter::new(),
        }
    }
}

impl ApplicationHandler for AppLoader {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.lock().unwrap().is_none() {
            let attributes = Window::default_attributes().with_title("quickgpu demo");

            #[cfg(not(target_arch = "wasm32"))]
            {
                let window = Arc::new(event_loop.create_window(attributes).unwrap());

                let inner_size = window.inner_size();

                *self.app.lock().unwrap() = Some(pollster::block_on(App::new(window, inner_size)));
            }

            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;
                use winit::platform::web::WindowAttributesExtWebSys;
                let canvas = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("canvas")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();

                let ratio = web_sys::window().unwrap().device_pixel_ratio();
                let inner_size = PhysicalSize::new(
                    (canvas.client_width() as f64 * ratio) as u32,
                    (canvas.client_height() as f64 * ratio) as u32,
                );

                let attributes = attributes.with_canvas(Some(canvas));

                let window = Arc::new(event_loop.create_window(attributes).unwrap());

                let app_arc = self.app.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    *app_arc.lock().unwrap() = Some(App::new(window, inner_size).await);
                });
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Ok(mut guard) = self.app.lock() {
            if let Some(app) = &mut *guard {
                if app.surface_config.width == 1 {
                    resize(app, app.window.inner_size());
                }

                match event {
                    WindowEvent::Resized(new_size) => {
                        resize(app, new_size);
                    }
                    WindowEvent::RedrawRequested => {
                        redraw(app);
                        self.frame_counter.update();
                        app.window.request_redraw();
                    }
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Space),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        app.scene.spawn_bunnies();
                    }
                    _ => {}
                };
            }
        }
    }
}
