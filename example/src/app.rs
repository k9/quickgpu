use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

#[allow(unused_imports)]
use std::sync::mpsc;

use crate::scene::{GPUState, Scene};

pub const DEBUG_SVGS: bool = false;

#[derive(Copy, Clone)]
pub struct RenderTextures<'a> {
    pub view: &'a TextureView,
    pub resolve_target: Option<&'a TextureView>,
    pub sample_count: u32,
}

pub struct State {
    pub width: u32,
    pub height: u32,
}

pub struct App {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub framebuffer: TextureView,
    pub framebuffer_texture: wgpu::Texture,
    pub state: State,
    pub scene: Scene,
    pub window: Arc<Window>,
    pub first_draw: bool,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        window: Arc<Window>,
        initial_size: PhysicalSize<u32>,
        sample_count: u32,
    ) -> App {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter,
                // so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Failed to create device");

        let mut surface_config = surface
            .get_default_config(
                &adapter,
                initial_size.width.max(1),
                initial_size.height.max(1),
            )
            .unwrap();

        surface_config.format = surface_config.format.remove_srgb_suffix();

        #[cfg(not(target_arch = "wasm32"))]
        {
            surface_config.usage =
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        }

        surface.configure(&device, &surface_config);

        let format = surface_config.format.remove_srgb_suffix();

        surface_config.view_formats.push(format);

        let (framebuffer_texture, framebuffer) =
            create_multisampled_framebuffer(&device, &surface_config, sample_count);

        let state = State {
            width: surface_config.width,
            height: surface_config.height,
        };

        let scene = Scene::new(&device, format, sample_count);

        App {
            device,
            queue,
            surface_config,
            framebuffer_texture,
            framebuffer,
            window,
            surface,
            state,
            scene,
            first_draw: true,
        }
    }
}

pub fn resize(app: &mut App, new_size: PhysicalSize<u32>) {
    // Reconfigure the surface with the new size
    app.surface_config.width = new_size.width.max(1);
    app.surface_config.height = new_size.height.max(1);
    app.surface.configure(&app.device, &app.surface_config);

    let sample_count = app.framebuffer_texture.sample_count();

    let (framebuffer_texture, framebuffer) =
        create_multisampled_framebuffer(&app.device, &app.surface_config, sample_count);

    app.framebuffer_texture = framebuffer_texture;
    app.framebuffer = framebuffer;

    // On macos the window needs to be redrawn manually after resizing
    app.window.request_redraw();
}

pub fn redraw(app: &mut App) {
    let frame = app
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    let surface_view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(app.surface_config.view_formats[0]),
        ..wgpu::TextureViewDescriptor::default()
    });

    let sample_count = app.framebuffer_texture.sample_count();

    let render_textures = if sample_count > 1 {
        RenderTextures {
            view: &app.framebuffer,
            resolve_target: Some(&surface_view),
            sample_count,
        }
    } else {
        RenderTextures {
            view: &surface_view,
            resolve_target: None,
            sample_count,
        }
    };

    let scene_commands = app.scene.render(GPUState {
        render_textures,
        device: &app.device,
        queue: &app.queue,
    });

    app.queue.submit([scene_commands]);
    frame.present();
}

pub fn create_multisampled_framebuffer(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let multisampled_texture_extent = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };

    #[cfg(target_arch = "wasm32")]
    let usage = wgpu::TextureUsages::RENDER_ATTACHMENT;

    #[cfg(not(target_arch = "wasm32"))]
    let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;

    let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        size: multisampled_texture_extent,
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.view_formats[0],
        usage,
        label: None,
        view_formats: &[],
    };

    let texture = device.create_texture(multisampled_frame_descriptor);

    let descriptor = wgpu::TextureViewDescriptor {
        format: Some(texture.format()),
        ..Default::default()
    };

    let view = texture.create_view(&descriptor);

    (texture, view)
}
