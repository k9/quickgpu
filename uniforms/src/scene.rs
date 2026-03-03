use std::{borrow::Cow, time::Instant};

use quickgpu::*;
use wgpu::{
    BindGroup, BindingType, BufferBindingType, BufferUsages, Color, CommandBuffer, Device, LoadOp,
    Queue, RenderPipeline, ShaderSource, ShaderStages, TextureFormat,
};

use crate::{
    app::RenderTextures,
    bind::{Bind, BindBuffer},
    group::group_builder,
};

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
    pub queue: &'a Queue,
}

type Offset = [f32; 2];

pub struct Scene {
    render_pipeline: RenderPipeline,
    group: [BindGroup; 2],
    offset_bind: Bind<Offset>,
    offset_buffers: [BindBuffer<Offset>; 2],
    redness_bind: Bind<u32>,
    redness_buffer: BindBuffer<u32>,
    start: Instant,
}

pub fn shader_source(offset_declaration: String, redness_declaration: String) -> String {
    format!(
        "
{offset_declaration}
{redness_declaration}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
) -> @builtin(position) vec4f {{
    let pos = array(
          vec2f(0.0,  0.25), // top center
          vec2f(-0.25, -0.25), // bottom left
          vec2f(0.25, -0.25) // bottom right
    );
    
    return vec4f(pos[vertex_index] + offset, 0.0, 1.0);
}}

@fragment
fn fs_main() -> @location(0) vec4<f32> {{
    return vec4<f32>(
        f32(redness) / 100.0,
        0.3,
        0.3,
        1.0
    );
}}
"
    )
}

impl Scene {
    pub fn new(device: &Device, format: TextureFormat, sample_count: u32) -> Self {
        let offset_bind = Bind::new(
            BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            ShaderStages::VERTEX_FRAGMENT,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            "vec2<f32>".to_string(),
            "offset".to_string(),
        );

        let redness_bind = Bind::new(
            BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            ShaderStages::VERTEX_FRAGMENT,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            "u32".to_string(),
            "redness".to_string(),
        );

        let bgl = device.create_bind_group_layout(
            &bind_group_layout_descriptor(Some("bgl"))
                .entries(&[offset_bind.layout_entry(0), redness_bind.layout_entry(1)])
                .build(),
        );

        let offset_buffers = [
            offset_bind.make_buffer([1.0; 2], device),
            offset_bind.make_buffer([1.0; 2], device),
        ];

        let redness_buffer = redness_bind.make_buffer(10, device);

        let groups = [
            group_builder(None, &bgl)
                .entry(&offset_buffers[0])
                .entry(&redness_buffer)
                .make(device),
            group_builder(None, &bgl)
                .entry(&offset_buffers[1])
                .entry(&redness_buffer)
                .make(device),
        ];

        let shader = device.create_shader_module(
            shader_module_descriptor(None)
                .source(ShaderSource::Wgsl(Cow::Owned(shader_source(
                    offset_bind.wgsl_declaration(0, 0),
                    redness_bind.wgsl_declaration(0, 1),
                ))))
                .build(),
        );

        let layout = device.create_pipeline_layout(
            &pipeline_layout_descriptor(Some("Layout"))
                .bind_group_layouts(&[&bgl])
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
            group: groups,
            offset_buffers,
            redness_buffer,
            offset_bind,
            redness_bind,
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

        self.offset_bind.write(
            queue,
            &self.offset_buffers[0],
            &[-0.33, linear(t) * 0.66 - 0.33],
        );

        self.offset_bind.write(
            queue,
            &self.offset_buffers[1],
            &[0.33, cubic(t) * 0.66 - 0.33],
        );

        self.redness_bind.write(
            queue,
            &self.redness_buffer,
            &(((elapsed.sin() * 0.5 + 0.5) * 100.0) as u32),
        );

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

            render_pass.set_bind_group(0, Some(&self.group[0]), &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);

            render_pass.set_bind_group(0, Some(&self.group[1]), &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        encoder.finish()
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
