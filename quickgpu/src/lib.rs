mod builders;
use std::num::NonZeroU32;

pub use builders::*;
use wgpu::{BufferUsages, CommandEncoder, Device, util::DeviceExt};

use crate::builders::{
    builder_buffer_init_descriptor::BufferInitDescriptorBuilder,
    builder_command_encoder_descriptor::CommandEncoderDescriptorBuilder,
    builder_render_pipeline_descriptor::RenderPipelineDescriptorBuilder,
    common::{Resolve, ResolveOptional},
};

pub trait Nested<T> {
    fn unnest(self) -> T;
}

impl<
    'a,
    RLabel,
    RLayout,
    RVertex,
    RPrimitive,
    RDepthStencil,
    RMultisample,
    RFragment,
    RMultiview,
    RCache,
>
    RenderPipelineDescriptorBuilder<
        RLabel,
        RLayout,
        RVertex,
        RPrimitive,
        RDepthStencil,
        RMultisample,
        RFragment,
        RMultiview,
        RCache,
    >
{
    pub fn create_with(self, device: &Device) -> wgpu::RenderPipeline
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
        RVertex: Resolve<wgpu::VertexState<'a>>,
        RPrimitive: ResolveOptional<wgpu::PrimitiveState>,
        RDepthStencil: ResolveOptional<Option<wgpu::DepthStencilState>>,
        RMultisample: ResolveOptional<wgpu::MultisampleState>,
        RFragment: ResolveOptional<Option<wgpu::FragmentState<'a>>>,
        RMultiview: ResolveOptional<Option<NonZeroU32>>,
        RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
    {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label.resolve(),
            layout: self.layout.resolve(),
            vertex: self.vertex.resolve(),
            primitive: self.primitive.resolve(),
            depth_stencil: self.depth_stencil.resolve(),
            multisample: self.multisample.resolve(),
            fragment: self.fragment.resolve(),
            multiview: self.multiview.resolve(),
            cache: self.cache.resolve(),
        })
    }
}

impl<'a, RLabel, RContentsValue, RUsageValue>
    BufferInitDescriptorBuilder<RLabel, RContentsValue, RUsageValue>
{
    pub fn create_with(self, device: &Device) -> wgpu::Buffer
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RContentsValue: Resolve<&'a [u8]>,
        RUsageValue: Resolve<BufferUsages>,
    {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: self.label.resolve(),
            contents: self.contents.resolve(),
            usage: self.usage.resolve(),
        })
    }
}

impl<RLabel> CommandEncoderDescriptorBuilder<RLabel> {
    pub fn create_with<'a>(self, device: &Device) -> wgpu::CommandEncoder
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
    {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: self.label.resolve(),
        })
    }
}

impl<RLabel, RColorAttachments, RDepthStencilAttachment, RTimestampWrites, ROcclusionQuerySet>
    builder_render_pass_descriptor::RenderPassDescriptorBuilder<
        RLabel,
        RColorAttachments,
        RDepthStencilAttachment,
        RTimestampWrites,
        ROcclusionQuerySet,
    >
{
    pub fn begin_with<'a>(self, encoder: &'a mut CommandEncoder) -> wgpu::RenderPass<'a>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RColorAttachments: ResolveOptional<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>,
        RDepthStencilAttachment:
            ResolveOptional<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>,
        RTimestampWrites: ResolveOptional<Option<wgpu::RenderPassTimestampWrites<'a>>>,
        ROcclusionQuerySet: ResolveOptional<Option<&'a wgpu::QuerySet>>,
    {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.label.resolve(),
            color_attachments: self.color_attachments.resolve(),
            depth_stencil_attachment: self.depth_stencil_attachment.resolve(),
            timestamp_writes: self.timestamp_writes.resolve(),
            occlusion_query_set: self.occlusion_query_set.resolve(),
        })
    }
}
