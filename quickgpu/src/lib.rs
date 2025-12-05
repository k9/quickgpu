#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

mod generated;
use std::num::NonZeroU32;

pub use generated::*;
use wgpu::{BufferUsages, CommandEncoder, Device, util::DeviceExt};

use crate::generated::builders::{
    builder_buffer_init_descriptor::BufferInitDescriptorBuilder,
    builder_command_encoder_descriptor::CommandEncoderDescriptorBuilder,
    builder_render_pass_descriptor,
    builder_render_pipeline_descriptor::RenderPipelineDescriptorBuilder,
    common::{Get, GetOpt},
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
        RLabel: GetOpt<wgpu::Label<'a>>,
        RLayout: GetOpt<Option<&'a wgpu::PipelineLayout>>,
        RVertex: Get<wgpu::VertexState<'a>>,
        RPrimitive: GetOpt<wgpu::PrimitiveState>,
        RDepthStencil: GetOpt<Option<wgpu::DepthStencilState>>,
        RMultisample: GetOpt<wgpu::MultisampleState>,
        RFragment: GetOpt<Option<wgpu::FragmentState<'a>>>,
        RMultiview: GetOpt<Option<NonZeroU32>>,
        RCache: GetOpt<Option<&'a wgpu::PipelineCache>>,
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
        RLabel: GetOpt<wgpu::Label<'a>>,
        RContentsValue: Get<&'a [u8]>,
        RUsageValue: Get<BufferUsages>,
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
        RLabel: GetOpt<wgpu::Label<'a>>,
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
        RLabel: GetOpt<wgpu::Label<'a>>,
        RColorAttachments: GetOpt<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>,
        RDepthStencilAttachment: GetOpt<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>,
        RTimestampWrites: GetOpt<Option<wgpu::RenderPassTimestampWrites<'a>>>,
        ROcclusionQuerySet: GetOpt<Option<&'a wgpu::QuerySet>>,
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
