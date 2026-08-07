use bytemuck::{Pod, Zeroable};
use nanorand::{Rng, WyRand};
use quickgpu::v29::*;
use wgpu::*;

use crate::app::RenderTextures;

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
    pub queue: &'a Queue,
}

create_binds!(GlobalBinds, uniforms, texture, sampler);
create_binds!(LocalBinds, bunny);

pub struct Scene {
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    global_group: wgpu::BindGroup,
    global_binds: GlobalBinds,
    local_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    bunnies: Vec<Bunny>,
    local_buffer: wgpu::Buffer,
    rng: WyRand,
    extent: [u32; 2],
}

const MAX_BUNNIES: usize = 1 << 20;
const BUNNY_SIZE: f32 = 0.15 * 256.0;
const GRAVITY: f32 = -9.8 * 100.0;
const MAX_VELOCITY: f32 = 750.0;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Globals {
    mvp: [[f32; 4]; 4],
    size: [f32; 2],
    pad: [f32; 2],
}

#[repr(C, align(256))]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Bunny {
    position: [f32; 2],
    velocity: [f32; 2],
    color: u32,
    _pad: [u32; (256 - 20) / 4],
}

impl Bunny {
    fn update_data(&mut self, delta: f32, extent: &[u32; 2]) {
        self.position[0] += self.velocity[0] * delta;
        self.position[1] += self.velocity[1] * delta;
        self.velocity[1] += GRAVITY * delta;

        if (self.velocity[0] > 0.0 && self.position[0] + 0.5 * BUNNY_SIZE > extent[0] as f32)
            || (self.velocity[0] < 0.0 && self.position[0] - 0.5 * BUNNY_SIZE < 0.0)
        {
            self.velocity[0] *= -1.0;
        }

        if self.velocity[1] < 0.0 && self.position[1] < 0.5 * BUNNY_SIZE {
            self.velocity[1] *= -1.0;
        }

        // Top boundary check
        if self.velocity[1] > 0.0 && self.position[1] + 0.5 * BUNNY_SIZE > extent[1] as f32 {
            self.velocity[1] *= -1.0;
        }
    }
}

impl Scene {
    pub fn spawn_bunnies(&mut self) {
        let spawn_count = 64;
        let color = self.rng.generate::<u32>();
        println!(
            "Spawning {} bunnies, total at {}",
            spawn_count,
            self.bunnies.len() + spawn_count
        );
        for _ in 0..spawn_count {
            let speed = self.rng.generate::<f32>() * MAX_VELOCITY - (MAX_VELOCITY * 0.5);
            self.bunnies.push(Bunny {
                position: [0.0, 0.5 * (self.extent[1] as f32)],
                velocity: [speed, 0.0],
                color,
                _pad: Zeroable::zeroed(),
            });
        }
    }

    pub fn new(
        device: &Device,
        queue: &Queue,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/base.wgsl"));

        let global_binds = GlobalBinds::builder()
            .uniforms(
                Binding::builder()
                    .binding(0)
                    .visibility(ShaderStages::VERTEX)
                    .ty(BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(size_of::<Globals>() as _),
                    }),
            )
            .texture(
                Binding::builder()
                    .binding(1)
                    .visibility(ShaderStages::FRAGMENT)
                    .ty(BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    }),
            )
            .sampler(
                Binding::builder()
                    .binding(2)
                    .visibility(ShaderStages::FRAGMENT)
                    .ty(BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
            )
            .build();

        let global_bind_group_layout = global_binds.layout(device);

        let local_binds = LocalBinds::builder()
            .bunny(
                Binding::builder()
                    .binding(0)
                    .visibility(ShaderStages::VERTEX)
                    .ty(BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(size_of::<Bunny>() as _),
                    }),
            )
            .build();

        let local_bind_group_layout = local_binds.layout(device);

        let pipeline_layout = pipeline_layout_descriptor(None)
            .bind_group_layouts(&[
                Some(&global_bind_group_layout),
                Some(&local_bind_group_layout),
            ])
            .create_with(device);

        let pipeline = render_pipeline_descriptor(None)
            .layout(&pipeline_layout)
            .vertex(
                vertex_state()
                    .module(&shader)
                    .entry_point("vs_main")
                    .buffers(&[]),
            )
            .fragment(
                fragment_state()
                    .module(&shader)
                    .entry_point("fs_main")
                    .targets(&arr_option![Some(
                        color_target_state()
                            .format(format)
                            .blend(BlendState::ALPHA_BLENDING)
                    )]),
            )
            .primitive(
                primitive_state()
                    .topology(PrimitiveTopology::TriangleStrip)
                    .strip_index_format(IndexFormat::Uint16),
            )
            .create_with(device);

        let texture = {
            let img_data = include_bytes!("../assets/logo.png");
            let decoder = png::Decoder::new(std::io::Cursor::new(img_data));
            let mut reader = decoder.read_info().unwrap();
            let buf_len = reader
                .output_buffer_size()
                .expect("output buffer would not fit in memory");
            let mut buf = vec![0; buf_len];
            let info = reader.next_frame(&mut buf).unwrap();

            let size = extent3d().width(info.width).height(info.height).build();

            let texture = texture_descriptor(None)
                .size(size)
                .mip_level_count(1)
                .sample_count(1)
                .dimension(wgpu::TextureDimension::D2)
                .format(wgpu::TextureFormat::Rgba8UnormSrgb)
                .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
                .view_formats(&[])
                .create_with(device);

            queue.write_texture(
                texture.as_image_copy(),
                &buf,
                texel_copy_buffer_layout()
                    .bytes_per_row(info.width * 4)
                    .build(),
                size,
            );

            texture
        };

        let sampler = sampler_descriptor(None)
            .mag_filter(FilterMode::Linear)
            .create_with(device);

        let globals = Globals {
            mvp: glam::Mat4::orthographic_rh(0.0, width as f32, 0.0, height as f32, -1.0, 1.0)
                .to_cols_array_2d(),
            size: [BUNNY_SIZE; 2],
            pad: [0.0; 2],
        };

        let global_buffer = buffer_init_descriptor(Some("global"))
            .contents(bytemuck::bytes_of(&globals))
            .usage(BufferUsages::COPY_DST | BufferUsages::UNIFORM)
            .create_with(device);

        let uniform_alignment =
            device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;

        let local_buffer = buffer_descriptor(Some("local"))
            .size((MAX_BUNNIES as wgpu::BufferAddress) * uniform_alignment)
            .usage(wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM)
            .mapped_at_creation(false)
            .create_with(device);

        let view = texture.create_view(&texture_view_descriptor(None).build());

        let global_group = global_binds
            .group()
            .uniforms(global_buffer.as_entire_binding())
            .texture(BindingResource::TextureView(&view))
            .sampler(BindingResource::Sampler(&sampler))
            .create(&global_bind_group_layout, device);

        let local_group = local_binds
            .group()
            .bunny(BindingResource::Buffer(
                buffer_binding()
                    .buffer(&local_buffer)
                    .offset(0)
                    .size(wgpu::BufferSize::new(size_of::<Bunny>() as _).unwrap())
                    .build(),
            ))
            .create(&local_bind_group_layout, device);

        let rng = WyRand::new_seed(42);

        let mut ex = Scene {
            view,
            sampler,
            pipeline,
            global_binds,
            global_group,
            local_group,
            bunnies: Vec::new(),
            local_buffer,
            rng,
            extent: [width, height],
        };

        ex.spawn_bunnies();

        ex
    }

    pub fn resize(
        &mut self,
        sc_desc: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        self.extent = [sc_desc.width, sc_desc.height];

        let globals = Globals {
            mvp: glam::Mat4::orthographic_rh(
                0.0,
                sc_desc.width as f32,
                0.0,
                sc_desc.height as f32,
                -1.0,
                1.0,
            )
            .to_cols_array_2d(),
            size: [BUNNY_SIZE; 2],
            pad: [0.0; 2],
        };

        let global_buffer = buffer_init_descriptor(Some("global"))
            .contents(bytemuck::bytes_of(&globals))
            .usage(wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM)
            .create_with(device);

        self.global_group = self
            .global_binds
            .group()
            .uniforms(global_buffer.as_entire_binding())
            .texture(BindingResource::TextureView(&self.view))
            .sampler(BindingResource::Sampler(&self.sampler))
            .create(&self.global_binds.layout(device), device);
    }

    #[must_use]
    pub fn render(
        &mut self,
        GPUState {
            render_textures,
            device,
            queue,
        }: GPUState,
    ) -> CommandBuffer {
        let delta = 0.01;
        for bunny in self.bunnies.iter_mut() {
            bunny.update_data(delta, &self.extent);
        }

        let uniform_alignment = device.limits().min_uniform_buffer_offset_alignment;
        queue.write_buffer(&self.local_buffer, 0, bytemuck::cast_slice(&self.bunnies));

        let mut encoder = command_encoder_descriptor(None).create_with(device);

        {
            let clear_color = color().r(0.1).g(0.2).b(0.3).a(1.0).build();

            let attachment = &[Some(
                render_pass_color_attachment()
                    .view(render_textures.view)
                    .ops(operations().load(wgpu::LoadOp::Clear(clear_color)))
                    .build(),
            )];

            let mut rpass = render_pass_descriptor(None)
                .color_attachments(attachment)
                .begin_with(&mut encoder);

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.global_group, &[]);
            for i in 0..self.bunnies.len() {
                let offset =
                    (i as wgpu::DynamicOffset) * (uniform_alignment as wgpu::DynamicOffset);
                rpass.set_bind_group(1, &self.local_group, &[offset]);
                rpass.draw(0..4, 0..1);
            }
        }

        encoder.finish()
    }
}
