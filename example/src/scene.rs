use std::borrow::Cow;

use binder::QBind;
use bytemuck::{NoUninit, Pod, Zeroable};
use quickgpu::*;
use wgpu::{
    BindGroup, BindingResource, Buffer, BufferBindingType, BufferUsages, Color, CommandBuffer,
    Device, LoadOp, PipelineCompilationOptions, Queue, RenderPipeline, ShaderSource, ShaderStages,
    TextureFormat, VertexFormat,
};

use crate::app::RenderTextures;

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
    pub queue: &'a Queue,
}

pub struct Scene {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    render_pipeline: RenderPipeline,
    group: BindGroup,
    resources: GreenResources,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct VertexInput {
    pub position: [f32; 4],
    pub uv: [f32; 2],
}

impl VertexInput {
    const fn new(position: [f32; 4], uv: [f32; 2]) -> Self {
        Self { position, uv }
    }
}

const VERTICES: &[VertexInput] = &[
    VertexInput::new([1.0, 1.0, 0.0, 0.0], [1.0, 1.0]),
    VertexInput::new([-1.0, 1.0, 0.0, 0.0], [0.0, 1.0]),
    VertexInput::new([-1.0, -1.0, 0.0, 0.0], [0.0, 0.0]),
    VertexInput::new([1.0, -1.0, 0.0, 0.0], [1.0, 0.0]),
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

#[derive(QBind, Clone, Copy, NoUninit)]
#[repr(C)]
pub struct Green {
    #[qbind(ty(u32))]
    pub red: u32,
    #[qbind(ty(f32))]
    pub green: f32,
    #[qbind(ty(f32))]
    pub blue: f32,
}

pub fn shader_source() -> String {
    let green_wgsl = Green::WGSL;
    format!(
        "
{green_wgsl}

struct VertexInput {{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}};

struct VertexOutput {{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {{
    var out: VertexOutput;
    out.uv = model.uv;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {{
    return vec4<f32>(
        f32(red) / 10.0,
        green,
        blue,
        1.0
    );
}}
"
    )
}

impl Scene {
    pub fn new(device: &Device, format: TextureFormat, sample_count: u32) -> Self {
        let shader = device.create_shader_module(
            shader_module_descriptor(None)
                .source(ShaderSource::Wgsl(Cow::Owned(shader_source())))
                .build(),
        );

        let bgl = Green::layout(device);
        let data = GreenData {
            red: vec![0],
            green: vec![0.0],
            blue: vec![0.0],
        };

        let resources = Green::resources(device, &data.slices());
        let group = Green::bind_group(device, &bgl, &resources);

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
                    .buffers(&builders([vertex_buffer_layout()
                        .array_stride(size_of::<VertexInput>() as wgpu::BufferAddress)
                        .attributes(&builders([
                            vertex_attribute()
                                .format(VertexFormat::Float32x4)
                                .offset(0u64)
                                .shader_location(0u32),
                            vertex_attribute()
                                .format(VertexFormat::Float32x2)
                                .offset(4 * 4u64)
                                .shader_location(1u32),
                        ]))])),
            )
            .fragment(
                fragment_state()
                    .module(&shader)
                    .entry_point("fs_main")
                    .targets(&[Some(format.into())])
                    .compilation_options(PipelineCompilationOptions::default()),
            )
            .primitive(primitive_state().cull_mode(wgpu::Face::Back))
            .multisample(multisample_state().count(sample_count))
            .create_with(device);

        let vertex_buffer = buffer_init_descriptor(Some("Vertex Buffer"))
            .contents(bytemuck::cast_slice(VERTICES))
            .usage(wgpu::BufferUsages::VERTEX)
            .create_with(device);

        let index_buffer = buffer_init_descriptor(Some("Index Buffer"))
            .contents(bytemuck::cast_slice(INDICES))
            .usage(wgpu::BufferUsages::INDEX)
            .create_with(device);

        Scene {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            group,
            resources,
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
        self.resources.write_red(queue, &[10]);

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

            render_pass.set_bind_group(0, Some(&self.group), &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        encoder.finish()
    }
}
