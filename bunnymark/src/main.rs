pub mod app;
pub mod app_loader;

#[macro_use]
pub mod binds;

pub mod scene;

use winit::event_loop::EventLoop;

use crate::app_loader::AppLoader;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = env_logger::builder()
            .filter(None, log::LevelFilter::Info)
            .parse_default_env()
            .try_init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        console_log::init().expect("could not initialize logger");
    }

    #[allow(unused_mut)]
    let mut event_loop_builder = &mut EventLoop::with_user_event();

    let event_loop = event_loop_builder.build().unwrap();

    let mut loop_state = AppLoader::default();

    event_loop.run_app(&mut loop_state).unwrap();
}
