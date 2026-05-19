use quickgpu::{
    device_descriptor, extent_3_d, request_adapter_options, texture_descriptor,
    texture_view_descriptor,
};
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureDimension, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

#[allow(unused_imports)]
use std::sync::mpsc;

use crate::scene::{GPUState, Scene};

pub const DEBUG_SVGS: bool = false;

#[derive(Copy, Clone)]
pub struct RenderTextures<'a> {
    pub view: &'a TextureView,
    pub resolve_target: Option<&'a TextureView>,
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
    pub async fn new(window: Arc<Window>, initial_size: PhysicalSize<u32>) -> App {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(
                &request_adapter_options()
                    .compatible_surface(&surface)
                    .build(),
            )
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &device_descriptor(None)
                    .required_limits(
                        wgpu::Limits::downlevel_webgl2_defaults()
                            .using_resolution(adapter.limits()),
                    )
                    .memory_hints(wgpu::MemoryHints::Performance)
                    .build(),
            )
            .await
            .expect("Failed to create device");

        let mut surface_config = surface
            .get_default_config(
                &adapter,
                initial_size.width.max(1),
                initial_size.height.max(1),
            )
            .unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        {
            surface_config.usage =
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        }

        surface.configure(&device, &surface_config);

        let format = surface_config.format;

        surface_config.view_formats.push(format);

        let (framebuffer_texture, framebuffer) = create_framebuffer(&device, &surface_config);

        let state = State {
            width: surface_config.width,
            height: surface_config.height,
        };

        let scene = Scene::new(
            &device,
            &queue,
            surface_config.width,
            surface_config.height,
            format,
        );

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

    let (framebuffer_texture, framebuffer) = create_framebuffer(&app.device, &app.surface_config);

    app.framebuffer_texture = framebuffer_texture;
    app.framebuffer = framebuffer;

    app.scene.resize(
        app.surface_config.width,
        app.surface_config.height,
        &app.device,
        &app.queue,
    );

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

    let render_textures = RenderTextures {
        view: &surface_view,
        resolve_target: None,
    };

    let scene_commands = app.scene.render(GPUState {
        render_textures,
        queue: &app.queue,
        device: &app.device,
    });

    app.queue.submit([scene_commands]);
    frame.present();
}

pub fn create_framebuffer(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> (wgpu::Texture, wgpu::TextureView) {
    #[cfg(target_arch = "wasm32")]
    let usage = wgpu::TextureUsages::RENDER_ATTACHMENT;

    #[cfg(not(target_arch = "wasm32"))]
    let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;

    let frame_descriptor = texture_descriptor(None)
        .size(extent_3_d().width(config.width).height(config.height))
        .mip_level_count(1)
        .sample_count(1)
        .dimension(TextureDimension::D2)
        .format(config.view_formats[0])
        .view_formats(&[])
        .usage(usage)
        .build();

    let texture = device.create_texture(&frame_descriptor);

    let descriptor = texture_view_descriptor(None)
        .format(texture.format())
        .build();

    let view = texture.create_view(&descriptor);

    (texture, view)
}
