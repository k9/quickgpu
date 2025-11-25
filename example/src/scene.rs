use bytemuck::{Pod, Zeroable};
use quickgpu::*;
use wgpu::{
    Buffer, Color, CommandBuffer, Device, LoadOp, PipelineCompilationOptions, RenderPipeline,
    TextureFormat, VertexFormat, include_wgsl,
};

use crate::app::RenderTextures;

pub struct GPUState<'a> {
    pub render_textures: RenderTextures<'a>,
    pub device: &'a Device,
}

pub struct Scene {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    render_pipeline: RenderPipeline,
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

impl Scene {
    pub fn new(device: &Device, format: TextureFormat, sample_count: u32) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../shaders/base.wgsl"));

        let render_pipeline = render_pipeline_descriptor(Some("Render Pipeline"))
            .vertex(
                vertex_state()
                    .module(&shader)
                    .entry_point("vs_main")
                    .buffers(&[vertex_buffer_layout()
                        .array_stride(size_of::<VertexInput>() as wgpu::BufferAddress)
                        .attributes(&[
                            vertex_attribute()
                                .format(VertexFormat::Float32x4)
                                .offset(0u64)
                                .shader_location(0u32)
                                .build(),
                            vertex_attribute()
                                .format(VertexFormat::Float32x2)
                                .offset(4 * 4u64)
                                .shader_location(1u32)
                                .build(),
                        ])
                        .build()]),
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
        }
    }

    #[must_use]
    pub fn render(
        &mut self,
        GPUState {
            render_textures,
            device,
        }: GPUState,
    ) -> CommandBuffer {
        let mut encoder = command_encoder_descriptor(None).create_with(device);

        {
            let color_attachments = &[Some(
                render_pass_color_attachment()
                    .view(render_textures.view)
                    .maybe_resolve_target(render_textures.resolve_target)
                    .ops(operations().load(LoadOp::Clear(Color::WHITE)))
                    .build(),
            )];

            let mut render_pass = render_pass_descriptor(Some("Render Pass"))
                .color_attachments(color_attachments)
                .begin_with(&mut encoder);

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        encoder.finish()
    }
}
