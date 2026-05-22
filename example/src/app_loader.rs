use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
};

#[allow(unused_imports)]
use winit::dpi::PhysicalSize;

use crate::app::{App, redraw, resize};

pub struct AppLoader {
    app: Rc<Mutex<Option<App>>>,
}

impl Default for AppLoader {
    fn default() -> Self {
        AppLoader {
            app: Rc::new(Mutex::new(None)),
        }
    }
}

pub const SAMPLE_COUNT: u32 = 4;

impl ApplicationHandler for AppLoader {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.lock().unwrap().is_none() {
            let attributes = Window::default_attributes().with_title("quickgpu demo");

            #[cfg(not(target_arch = "wasm32"))]
            {
                let attributes = attributes.with_maximized(true);

                let window = Arc::new(event_loop.create_window(attributes).unwrap());

                let inner_size = window.inner_size();

                *self.app.lock().unwrap() = Some(pollster::block_on(App::new(
                    window,
                    inner_size,
                    SAMPLE_COUNT,
                )));
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
                    *app_arc.lock().unwrap() =
                        Some(App::new(window, inner_size, SAMPLE_COUNT).await);
                });
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Ok(mut guard) = self.app.lock()
            && let Some(app) = &mut *guard
        {
            event_loop.set_control_flow(ControlFlow::Wait);
            if app.surface_config.width == 1 {
                resize(app, app.window.inner_size());
            }

            match event {
                WindowEvent::Resized(new_size) => {
                    resize(app, new_size);
                }
                WindowEvent::RedrawRequested => {
                    redraw(app);
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {}
            };
        }
    }
}
