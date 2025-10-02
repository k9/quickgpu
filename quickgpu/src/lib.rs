use wgpu::{CommandEncoder, Device, RenderPassDepthStencilAttachment, util::DeviceExt};

use crate::builders::{
    RenderPassDepthStencilAttachmentBuilder, buffer_init_descriptor_builder,
    render_pass_depth_stencil_attachment_builder, render_pass_descriptor_builder,
    render_pipeline_descriptor_builder,
};

pub mod builders;

impl<'a, S> builders::BufferInitDescriptorBuilder<'a, S>
where
    S: buffer_init_descriptor_builder::IsComplete,
{
    pub fn create_with(self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&self.call())
    }
}

impl<'a, S> builders::RenderPipelineDescriptorBuilder<'a, S>
where
    S: render_pipeline_descriptor_builder::IsComplete,
{
    pub fn create_with(self, device: &Device) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&self.call())
    }
}

impl<'a, 'encoder, S> builders::RenderPassDescriptorBuilder<'a, S>
where
    S: render_pass_descriptor_builder::IsComplete,
{
    pub fn begin_with(self, encoder: &'encoder mut CommandEncoder) -> wgpu::RenderPass<'encoder> {
        encoder.begin_render_pass(&self.call())
    }
}

pub trait NestedBuilder {
    type Output;
    fn nested_build(self) -> Self::Output;
}

impl<'a, S> NestedBuilder for RenderPassDepthStencilAttachmentBuilder<'a, S>
where
    S: render_pass_depth_stencil_attachment_builder::IsComplete,
{
    type Output = RenderPassDepthStencilAttachment<'a>;

    fn nested_build(self) -> Self::Output {
        self.call()
    }
}

impl<'a> NestedBuilder for RenderPassDepthStencilAttachment<'a> {
    type Output = RenderPassDepthStencilAttachment<'a>;

    fn nested_build(self) -> Self::Output {
        self
    }
}
