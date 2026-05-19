use bytemuck::{Pod, Zeroable};
use nanorand::{Rng, WyRand};
use quickgpu::binder::*;
use quickgpu::{
    bind_group_helper, color, color_target_state, command_encoder_descriptor, extent_3_d,
    fragment_state, operations, pipeline_layout_descriptor, primitive_state,
    render_pass_color_attachment, render_pass_descriptor, render_pipeline_descriptor,
    sampler_descriptor, texel_copy_buffer_layout, texture_descriptor, texture_view_descriptor,
    vertex_state,
};
use wgpu::{
    BindingType, BlendState, BufferBindingType, CommandBuffer, Device, FilterMode,
    MipmapFilterMode, Queue, SamplerBindingType, ShaderStages, TextureFormat, TextureSampleType,
    TextureViewDimension, include_wgsl,
};

use crate::app::RenderTextures;

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
    pub queue: &'a Queue,
}

#[bind_group_helper]
pub struct GlobalBinds {
    pub globals: BufferBind<super::Globals>,
    pub tex: TextureBind<()>,
    pub sam: SamplerBind,
}

#[bind_group_helper]
pub struct LocalBinds {
    pub locals: BufferBind<super::Bunny>,
}

pub struct Scene {
    texture_view: BoundTextureView<()>,
    sampler: BoundSampler,
    global_binds: GlobalBinds,
    global_group: wgpu::BindGroup,
    local_binds: LocalBinds,
    local_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    bunnies: Vec<Bunny>,
    local_buffer: BoundBuffer<Bunny>,
    rng: WyRand,
    extent: [u32; 2],
}

const MAX_BUNNIES: usize = 1 << 20;
const BUNNY_SIZE: f32 = 0.15 * 256.0;
const GRAVITY: f32 = -9.8 * 100.0;
const MAX_VELOCITY: f32 = 750.0;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Globals {
    mvp: [[f32; 4]; 4],
    size: [f32; 2],
    pad: [f32; 2],
}

#[repr(C, align(256))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Bunny {
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

fn make_globals(
    device: &Device,
    binds: &GlobalBinds,
    width: u32,
    height: u32,
) -> BoundBuffer<Globals> {
    let globals = Globals {
        mvp: glam::Mat4::orthographic_rh(0.0, width as f32, 0.0, height as f32, -1.0, 1.0)
            .to_cols_array_2d(),
        size: [BUNNY_SIZE; 2],
        pad: [0.0; 2],
    };

    binds.globals.make_buffer(
        Some("global"),
        globals,
        wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        device,
    )
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

        let global_binds = GlobalBinds::new(
            None,
            device,
            BufferBind::new(
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<Globals>() as _),
                },
                ShaderStages::VERTEX,
                "Globals",
                "globals",
            ),
            TextureBind::new(
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                ShaderStages::FRAGMENT,
                "texture_2d<f32>",
                "tex",
            ),
            SamplerBind::new(
                BindingType::Sampler(SamplerBindingType::Filtering),
                ShaderStages::FRAGMENT,
                "sampler",
                "sam",
            ),
        );

        let local_binds = LocalBinds::new(
            None,
            device,
            BufferBind::new(
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(size_of::<Bunny>() as _),
                },
                ShaderStages::VERTEX,
                "Bunny",
                "locals",
            ),
        );

        let pipeline_layout = pipeline_layout_descriptor(None)
            .bind_group_layouts(&[&global_binds.layout, &local_binds.layout])
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
                    .targets(&[Some(
                        color_target_state()
                            .format(format)
                            .blend(BlendState::ALPHA_BLENDING)
                            .build(),
                    )]),
            )
            .primitive(
                primitive_state()
                    .topology(wgpu::PrimitiveTopology::TriangleStrip)
                    .strip_index_format(wgpu::IndexFormat::Uint16),
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

            let size = extent_3_d().width(info.width).height(info.height).build();

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

        let texture_view = global_binds
            .tex
            .make_view(&texture, &texture_view_descriptor(None).build());

        let sampler = global_binds.sam.make_sampler(
            sampler_descriptor(None)
                .mag_filter(FilterMode::Linear)
                .min_filter(FilterMode::Nearest)
                .mipmap_filter(MipmapFilterMode::Nearest)
                .create_with(device),
        );

        let uniform_alignment =
            device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;

        let local_buffer = local_binds.locals.make_buffer_sized(
            Some("local"),
            (MAX_BUNNIES as wgpu::BufferAddress) * uniform_alignment,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            device,
        );

        let global_buffer = make_globals(device, &global_binds, width, height);
        let global_group = global_binds.group(
            None,
            GlobalBindsResources {
                globals: &global_buffer,
                tex: &texture_view,
                sam: &sampler,
            },
            GlobalBindsEntries { globals: None },
            None,
            device,
        );

        let local_group = local_binds.group(
            None,
            LocalBindsResources {
                locals: &local_buffer,
            },
            LocalBindsEntries {
                locals: Some(|binding, buffer| wgpu::BindGroupEntry {
                    binding,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer,
                        offset: 0,
                        size: wgpu::BufferSize::new(size_of::<Bunny>() as _),
                    }),
                }),
            },
            None,
            device,
        );

        let rng = WyRand::new_seed(42);

        let mut scene = Scene {
            texture_view,
            sampler,
            pipeline,
            global_binds,
            global_group,
            local_binds,
            local_group,
            bunnies: Vec::new(),
            local_buffer,
            rng,
            extent: [width, height],
        };

        scene.spawn_bunnies();

        scene
    }

    pub fn resize(&mut self, width: u32, height: u32, device: &wgpu::Device, _queue: &wgpu::Queue) {
        self.extent = [width, height];

        let global_buffer = make_globals(device, &self.global_binds, width, height);

        self.global_group = self.global_binds.group(
            None,
            GlobalBindsResources {
                globals: &global_buffer,
                tex: &self.texture_view,
                sam: &self.sampler,
            },
            GlobalBindsEntries { globals: None },
            None,
            device,
        );
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
        queue.write_buffer(
            &self.local_buffer.buffer,
            0,
            bytemuck::cast_slice(&self.bunnies),
        );

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
