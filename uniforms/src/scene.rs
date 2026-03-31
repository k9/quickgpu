use std::{borrow::Cow, time::Instant};

use binder::bind_group_helper;
use quickgpu::*;
use wgpu::{
    BindGroup, BindingType, BufferBindingType, BufferUsages, Color, CommandBuffer, Device, LoadOp,
    Queue, RenderPipeline, SamplerBindingType, ShaderSource, ShaderStages, Texture,
    TextureDimension, TextureFormat, TextureUsages, TextureViewDimension,
};

use crate::{
    app::RenderTextures,
    bind::{BufferBind, SamplerBind, TextureBind, buffer::BoundBuffer},
    math::{cubic, graph, linear},
};

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
    pub queue: &'a Queue,
}

static SIZE: usize = 128;

type Offset = [f32; 2];

pub struct Scene {
    render_pipeline: RenderPipeline,
    gr_layout: SceneBinds,
    groups: [BindGroup; 2],
    offset_buffers: [BoundBuffer<Offset>; 2],
    size_buffer: BoundBuffer<u32>,
    textures: [Texture; 2],
    texels: [[u8; SIZE * SIZE]; 2],
    start: Instant,
}

#[bind_group_helper]
pub struct SceneBinds {
    pub points: BufferBind<[[f32; 4]; 6]>,
    pub offset: BufferBind<[f32; 2]>,
    pub size: BufferBind<u32>,
    pub pattern: TextureBind<[u8; super::SIZE * super::SIZE]>,
    pub pattern_sampler: SamplerBind,
}

impl Scene {
    pub fn new(device: &Device, format: TextureFormat, sample_count: u32) -> Self {
        let gr_layout = SceneBinds::new(
            None,
            device,
            BufferBind::new(
                BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                ShaderStages::VERTEX_FRAGMENT,
                "array<vec4<f32>>",
                "points",
            ),
            BufferBind::new(
                binding_type_buffer(),
                ShaderStages::VERTEX_FRAGMENT,
                "vec2<f32>",
                "offset",
            ),
            BufferBind::new(
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                ShaderStages::VERTEX_FRAGMENT,
                "u32",
                "size",
            ),
            TextureBind::new(
                BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                ShaderStages::VERTEX_FRAGMENT,
                "texture_2d<f32>",
                "pattern",
            ),
            SamplerBind::new(
                BindingType::Sampler(SamplerBindingType::Filtering),
                ShaderStages::VERTEX_FRAGMENT,
                "sampler",
                "pattern_sampler",
            ),
        );

        let points_buffer = gr_layout.points.make_buffer(
            None,
            [
                [0.0, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [1.0, 1.0, 0.0, 0.0],
            ],
            BufferUsages::STORAGE | BufferUsages::COPY_DST,
            device,
        );

        let offset_buffers = [
            gr_layout.offset.make_buffer(
                None,
                [1.0; 2],
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                device,
            ),
            gr_layout.offset.make_buffer(
                None,
                [1.0; 2],
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                device,
            ),
        ];

        let size_buffer = gr_layout.size.make_buffer(
            None,
            SIZE as u32,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            device,
        );

        let texels: [[u8; SIZE * SIZE]; 2] = [[0; SIZE * SIZE]; 2];

        let texture_extent = extent_3_d().width(SIZE as u32).height(SIZE as u32).build();

        let make_texture = || {
            texture_descriptor(None)
                .size(texture_extent)
                .mip_level_count(1)
                .sample_count(1)
                .dimension(TextureDimension::D2)
                .format(TextureFormat::R8Unorm)
                .usage(TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST)
                .view_formats(&[])
                .create_with(device)
        };

        let textures = [make_texture(), make_texture()];

        let pattern_views = [
            gr_layout
                .pattern
                .make_view(&textures[0], &texture_view_descriptor(None).build()),
            gr_layout
                .pattern
                .make_view(&textures[1], &texture_view_descriptor(None).build()),
        ];

        let pattern_sampler = gr_layout.pattern_sampler.make_sampler(
            sampler_descriptor(None)
                .address_mode_u(wgpu::AddressMode::Repeat)
                .address_mode_v(wgpu::AddressMode::Repeat)
                .create_with(device),
        );

        let groups = [
            gr_layout.group(
                None,
                SceneBindsResources {
                    points: &points_buffer,
                    offset: &offset_buffers[0],
                    size: &size_buffer,
                    pattern: &pattern_views[0],
                    pattern_sampler: &pattern_sampler,
                },
                SceneBindsEntries {
                    points: |binding, buffer| {
                        buffer_binding().buffer(buffer).offset(0).as_entry(binding)
                    },
                    offset: |binding, buffer| {
                        buffer_binding().buffer(buffer).offset(0).as_entry(binding)
                    },
                    size: |binding, buffer| {
                        buffer_binding().buffer(buffer).offset(0).as_entry(binding)
                    },
                },
                None,
                device,
            ),
            gr_layout.group(
                None,
                SceneBindsResources {
                    points: &points_buffer,
                    offset: &offset_buffers[1],
                    size: &size_buffer,
                    pattern: &pattern_views[1],
                    pattern_sampler: &pattern_sampler,
                },
                SceneBindsEntries {
                    points: |binding, buffer| {
                        buffer_binding().buffer(buffer).offset(0).as_entry(binding)
                    },
                    offset: |binding, buffer| {
                        buffer_binding().buffer(buffer).offset(0).as_entry(binding)
                    },
                    size: |binding, buffer| {
                        buffer_binding().buffer(buffer).offset(0).as_entry(binding)
                    },
                },
                None,
                device,
            ),
        ];

        let shader = shader_module_descriptor(None)
            .source(ShaderSource::Wgsl(Cow::Owned(shader_source(
                gr_layout.declarations(0),
            ))))
            .create_with(device);

        let layout = pipeline_layout_descriptor(Some("Layout"))
            .bind_group_layouts(&[&gr_layout.layout])
            .create_with(device);

        let render_pipeline = render_pipeline_descriptor(Some("Render Pipeline"))
            .layout(&layout)
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
                    .targets(&[Some(format.into())]),
            )
            .primitive(primitive_state().cull_mode(wgpu::Face::Back))
            .multisample(multisample_state().count(sample_count))
            .create_with(device);

        Scene {
            render_pipeline,
            gr_layout,
            groups,
            offset_buffers,
            size_buffer,
            textures,
            texels,
            start: Instant::now(),
        }
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
        let elapsed = self.start.elapsed().as_secs_f32();
        let t = (elapsed * 0.5).fract();

        let SceneBinds {
            offset,
            size,
            pattern,
            ..
        } = &self.gr_layout;

        offset.write(
            queue,
            &self.offset_buffers[0],
            None,
            &[-1.25, cubic(t) - 1.0],
        );
        offset.write(
            queue,
            &self.offset_buffers[1],
            None,
            &[0.25, linear(t) - 1.0],
        );
        size.write(queue, &self.size_buffer, None, &(SIZE as u32));

        graph(&mut self.texels[0], t, cubic, SIZE);
        pattern.write(
            queue,
            &self.textures[0],
            texel_copy_texture_info()
                .texture(&self.textures[0])
                .mip_level(0)
                .build(),
            &self.texels[0],
            None,
        );

        graph(&mut self.texels[1], t, linear, SIZE);
        pattern.write(
            queue,
            &self.textures[1],
            texel_copy_texture_info()
                .texture(&self.textures[1])
                .mip_level(0)
                .build(),
            &self.texels[1],
            None,
        );

        let mut encoder = command_encoder_descriptor(None).create_with(device);

        {
            let color_attachments = builders([Some(
                render_pass_color_attachment()
                    .view(render_textures.view)
                    .maybe_resolve_target(render_textures.resolve_target)
                    .ops(operations().load(LoadOp::Clear(Color {
                        r: 0.92,
                        g: 0.93,
                        b: 0.94,
                        a: 0.1,
                    }))),
            )]);

            let mut render_pass = render_pass_descriptor(Some("Render Pass"))
                .color_attachments(&color_attachments)
                .begin_with(&mut encoder);

            render_pass.set_bind_group(0, Some(&self.groups[0]), &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..6, 0..1);

            render_pass.set_bind_group(0, Some(&self.groups[1]), &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..6, 0..1);
        }

        encoder.finish()
    }
}

fn binding_type_buffer() -> BindingType {
    BindingType::Buffer {
        ty: BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    }
}

pub fn shader_source(
    SceneBindsDeclarations {
        points,
        offset,
        size,
        pattern,
        pattern_sampler,
    }: SceneBindsDeclarations,
) -> String {
    format!(
        "
{points}
{offset}
{size}
{pattern}
{pattern_sampler}

struct VertexOutput {{
    @location(0) uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
}};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {{
    var result: VertexOutput;

    var point = points[vertex_index].xy;
    result.position = vec4((point + offset) * 0.5, 0.0, 1.0);
    result.uv = point;

    return result;
}}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {{
    let tex = textureSample(pattern, pattern_sampler, vertex.uv);

    return vec4<f32>(
        0.2,
        mix(0.2, 1.0, tex.r),
        0.2,
        1.0
    );
}}
"
    )
}
