use std::{borrow::Cow, time::Instant};

use binder::bind_group_helper;
use quickgpu::*;
use wgpu::{
    BindGroup, BindingType, BufferBindingType, BufferUsages, Color, CommandBuffer, Device, LoadOp,
    Queue, RenderPipeline, ShaderSource, ShaderStages, Texture, TextureDimension, TextureFormat,
    TextureUsages, TextureViewDimension,
};

use crate::{
    app::RenderTextures,
    bind::{Bind, BufferBind, buffer::BoundBuffer},
};

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
    pub queue: &'a Queue,
}

static SIZE: usize = 256;

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
            Bind::new(
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                ShaderStages::VERTEX_FRAGMENT,
                "u32",
                "size",
            ),
            Bind::new(
                BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                ShaderStages::VERTEX_FRAGMENT,
                "texture_2d<f32>",
                "pattern",
            ),
        );

        let offset_buffers = [
            gr_layout.offset.make_buffer(
                [1.0; 2],
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                device,
            ),
            gr_layout.offset.make_buffer(
                [1.0; 2],
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                device,
            ),
        ];

        let size_buffer = gr_layout.size.make_buffer(
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

        let groups = [
            gr_layout.group(
                None,
                BgBuffers {
                    offset: &offset_buffers[0],
                    size: &size_buffer,
                    pattern: &pattern_views[0],
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

        self.gr_layout
            .offset
            .write(queue, &self.offset_buffers[0], &[-1.25, linear(t) - 1.0]);

        self.gr_layout
            .offset
            .write(queue, &self.offset_buffers[1], &[0.25, cubic(t) - 1.0]);

        self.gr_layout
            .size
            .write(queue, &self.size_buffer, &(SIZE as u32));

        self.graph(queue, 0, t, cubic);
        self.graph(queue, 1, t, linear);

        self.gr_layout
            .pattern
            .write(queue, &self.textures[0], &self.texels[0]);

        let mut encoder = command_encoder_descriptor(None).create_with(device);

        {
            let color_attachments = builders([Some(
                render_pass_color_attachment()
                    .view(render_textures.view)
                    .maybe_resolve_target(render_textures.resolve_target)
                    .ops(operations().load(LoadOp::Clear(Color::WHITE))),
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

    fn graph(&mut self, queue: &Queue, index: usize, t: f32, f: fn(f32) -> f32) {
        for x_base in 0..SIZE {
            let fsize = SIZE as f32;

            for offset in [-0.325, -0.25, -0.125, 0.0, 0.125, 0.25, 0.325] {
                let x = x_base as f32 + offset;
                let texture_t = (x as f32) / (fsize - 1.0);
                let y = ((f(texture_t) * fsize) as usize).min(SIZE - 1);

                self.texels[index][y * SIZE + x_base] = if (t - texture_t).abs() < 0.005 {
                    255
                } else {
                    100
                };
            }
        }

        self.gr_layout
            .pattern
            .write(queue, &self.textures[index], &self.texels[index]);
    }
}

pub fn linear(t: f32) -> f32 {
    let value = t * 2.0;
    if value < 1.0 { value } else { 2.0 - value }
}

pub fn cubic(t: f32) -> f32 {
    let t = linear(t);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn shader_source(
    BgDeclarations {
        offset,
        size,
        pattern,
    }: BgDeclarations,
) -> String {
    format!(
        "
{offset}
{size}
{pattern}

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
    let tex = textureLoad(pattern, vec2<i32>(vertex.uv * f32(size)), 0);

    return vec4<f32>(
        0.0,
        tex.r,
        0.0,
        1.0
    );
}}
"
    )
}
