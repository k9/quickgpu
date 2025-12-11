#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

mod generated;

pub use generated::*;
use wgpu::{CommandEncoder, Device, util::DeviceExt};

use crate::generated::builders::{
    builder_buffer_init_descriptor::BufferInitDescriptorBuilder,
    builder_command_encoder_descriptor::CommandEncoderDescriptorBuilder,
    builder_render_pass_descriptor::RenderPassDescriptorBuilder,
    builder_render_pipeline_descriptor::RenderPipelineDescriptorBuilder,
};

pub trait Nested<T> {
    fn unnest(self) -> T;
}

impl<T, N: Nested<T>> Nested<Option<T>> for Option<N> {
    fn unnest(self) -> Option<T> {
        self.map(|o| o.unnest())
    }
}

impl<'a, CurrentState: builders::builder_render_pipeline_descriptor::Complete<'a>>
    RenderPipelineDescriptorBuilder<'a, CurrentState>
{
    pub fn create_with(self, device: &Device) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&self.build())
    }
}

impl<'a, CurrentState: builders::builder_buffer_init_descriptor::Complete<'a>>
    BufferInitDescriptorBuilder<'a, CurrentState>
{
    pub fn create_with(self, device: &Device) -> wgpu::Buffer {
        device.create_buffer_init(&self.build())
    }
}

impl<'a, CurrentState: builders::builder_command_encoder_descriptor::Complete<'a>>
    CommandEncoderDescriptorBuilder<'a, CurrentState>
{
    pub fn create_with(self, device: &Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&self.build())
    }
}

impl<'a, CurrentState: builders::builder_render_pass_descriptor::Complete<'a>>
    RenderPassDescriptorBuilder<'a, CurrentState>
{
    pub fn begin_with(self, encoder: &'a mut CommandEncoder) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&self.build())
    }
}
