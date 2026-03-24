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
    gr_layout: BgHelper,
    groups: [BindGroup; 2],
    offset_buffers: [BoundBuffer<Offset>; 2],
    size_buffer: BoundBuffer<u32>,
    textures: [Texture; 2],
    texels: [[u8; SIZE * SIZE]; 2],
    start: Instant,
}

#[bind_group_helper]
pub struct BgHelper {
    pub offset: BufferBind<[f32; 2]>,
    pub size: BufferBind<u32>,
    pub pattern: TextureBind<[u8; super::SIZE * super::SIZE]>,
    pub pattern_sampler: SamplerBind,
}

impl Scene {
    pub fn new(device: &Device, format: TextureFormat, sample_count: u32) -> Self {
        let gr_layout = BgHelper::new(
            None,
            device,
            BufferBind::new(
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
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
            device.create_texture(
                &texture_descriptor(None)
                    .size(texture_extent)
                    .mip_level_count(1)
                    .sample_count(1)
                    .dimension(TextureDimension::D2)
                    .format(TextureFormat::R8Unorm)
                    .usage(TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST)
                    .view_formats(&[])
                    .build(),
            )
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
            device.create_sampler(
                &sampler_descriptor(None)
                    .address_mode_u(wgpu::AddressMode::Repeat)
                    .address_mode_v(wgpu::AddressMode::Repeat)
                    .build(),
            ),
        );

        let groups = [
            gr_layout.group(
                None,
                BgBuffers {
                    offset: &offset_buffers[0],
                    size: &size_buffer,
                    pattern: &pattern_views[0],
                    pattern_sampler: &pattern_sampler,
                },
                BgBindingEntries {
                    offset: |binding, buffer| {
                        bind_group_entry()
                            .binding(binding)
                            .resource(wgpu::BindingResource::Buffer(
                                buffer_binding().buffer(buffer).offset(0).build(),
                            ))
                    },
                    size: |binding, buffer| {
                        bind_group_entry()
                            .binding(binding)
                            .resource(wgpu::BindingResource::Buffer(
                                buffer_binding().buffer(buffer).offset(0).build(),
                            ))
                    },
                },
                None,
                device,
            ),
            gr_layout.group(
                None,
                BgBuffers {
                    offset: &offset_buffers[1],
                    size: &size_buffer,
                    pattern: &pattern_views[1],
                    pattern_sampler: &pattern_sampler,
                },
                BgBindingEntries {
                    offset: |binding, buffer| {
                        bind_group_entry()
                            .binding(binding)
                            .resource(wgpu::BindingResource::Buffer(
                                buffer_binding().buffer(buffer).offset(0).build(),
                            ))
                    },
                    size: |binding, buffer| {
                        bind_group_entry()
                            .binding(binding)
                            .resource(wgpu::BindingResource::Buffer(
                                buffer_binding().buffer(buffer).offset(0).build(),
                            ))
                    },
                },
                None,
                device,
            ),
        ];

        let shader = device.create_shader_module(
            shader_module_descriptor(None)
                .source(ShaderSource::Wgsl(Cow::Owned(shader_source(
                    gr_layout.declarations(0),
                ))))
                .build(),
        );

        let layout = device.create_pipeline_layout(
            &pipeline_layout_descriptor(Some("Layout"))
                .bind_group_layouts(&[&gr_layout.layout])
                .build(),
        );

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

        let BgHelper {
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
                        r: 0.95,
                        g: 0.95,
                        b: 0.95,
                        a: 1.0,
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

pub fn shader_source(
    BgDeclarations {
        offset,
        size,
        pattern,
        pattern_sampler,
    }: BgDeclarations,
) -> String {
    format!(
        "
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

    var points = array(
          vec2f(0.0, 0.0), // bottom left
          vec2f(1.0, 0.0), // bottom right
          vec2f(0.0, 1.0), // top left
          vec2f(0.0, 1.0), // top left
          vec2f(1.0, 0.0), // bottom right
          vec2f(1.0, 1.0), // top right
    );

    result.position = vec4((points[vertex_index] + offset) * 0.5, 0.0, 1.0);
    result.uv = points[vertex_index];

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
