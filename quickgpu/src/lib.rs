#![doc=include_str!("../../INTRO.md")]
use wgpu::{CommandEncoder, Device, VertexState, util::DeviceExt};

#[doc(inline)]
pub use crate::inner::initializers::*;

pub use crate::inner::builders;

use crate::inner::{
    VertexStateBuilder, buffer_init_descriptor_builder, command_encoder_descriptor_builder,
    render_pass_descriptor_builder, render_pipeline_descriptor_builder,
    shader_module_descriptor_builder, vertex_state_builder,
};

mod inner;

impl<'a, S> builders::BufferInitDescriptorBuilder<'a, S>
where
    S: buffer_init_descriptor_builder::IsComplete,
{
    pub fn create_with(self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&self.build())
    }
}

impl<'a, S> builders::RenderPipelineDescriptorBuilder<'a, S>
where
    S: render_pipeline_descriptor_builder::IsComplete,
{
    pub fn create_with(self, device: &Device) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&self.build())
    }
}

impl<'a, S> builders::RenderPipelineDescriptorBuilder<'a, S>
where
    S: render_pipeline_descriptor_builder::State,
    S::Vertex: render_pipeline_descriptor_builder::IsUnset,
{
    pub fn vertex_builder<S2: vertex_state_builder::IsComplete>(
        self,
        v: VertexStateBuilder<'a, S2>,
    ) -> builders::RenderPipelineDescriptorBuilder<
        'a,
        render_pipeline_descriptor_builder::SetVertex<S>,
    > {
        self.vertex(v.build())
    }
}

impl<'a, S> builders::ShaderModuleDescriptorBuilder<'a, S>
where
    S: shader_module_descriptor_builder::IsComplete,
{
    pub fn create_with(self, device: &Device) -> wgpu::ShaderModule {
        device.create_shader_module(self.build())
    }
}

impl<'a, S> builders::CommandEncoderDescriptorBuilder<'a, S>
where
    S: command_encoder_descriptor_builder::IsComplete,
{
    pub fn create_with(self, device: &Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&self.build())
    }
}

impl<'a, 'encoder, S> builders::RenderPassDescriptorBuilder<'a, S>
where
    S: render_pass_descriptor_builder::IsComplete,
{
    pub fn begin_with(self, encoder: &'encoder mut CommandEncoder) -> wgpu::RenderPass<'encoder> {
        encoder.begin_render_pass(&self.build())
    }
}

// Experimental trait to use instead of generally implementing Into for builders
pub trait Nested<T> {
    fn nested(self) -> T;
}
