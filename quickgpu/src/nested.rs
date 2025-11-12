use crate::Nested;

impl<'a> Nested<wgpu::ExternalTextureDescriptor<'a>> for wgpu::ExternalTextureDescriptor<'a> {
    fn unnest(self) -> wgpu::ExternalTextureDescriptor<'a> {
        self
    }
}
impl<'a, T, U> Nested<wgpu::ExternalTextureDescriptor<'a>>
    for crate::builders::ExternalTextureDescriptorBuilder<'a, T, U>
{
    fn unnest(self) -> wgpu::ExternalTextureDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::SamplerDescriptor<'a>> for wgpu::SamplerDescriptor<'a> {
    fn unnest(self) -> wgpu::SamplerDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::SamplerDescriptor<'a>> for crate::builders::SamplerDescriptorBuilder<'a> {
    fn unnest(self) -> wgpu::SamplerDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::CreateBlasDescriptor<'a>> for wgpu::CreateBlasDescriptor<'a> {
    fn unnest(self) -> wgpu::CreateBlasDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::CreateBlasDescriptor<'a>>
    for crate::builders::CreateBlasDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::CreateBlasDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::TexelCopyBufferLayout> for wgpu::TexelCopyBufferLayout {
    fn unnest(self) -> wgpu::TexelCopyBufferLayout {
        self
    }
}
impl Nested<wgpu::TexelCopyBufferLayout> for crate::builders::TexelCopyBufferLayoutBuilder {
    fn unnest(self) -> wgpu::TexelCopyBufferLayout {
        self.build()
    }
}

impl Nested<wgpu::PrimitiveState> for wgpu::PrimitiveState {
    fn unnest(self) -> wgpu::PrimitiveState {
        self
    }
}
impl Nested<wgpu::PrimitiveState> for crate::builders::PrimitiveStateBuilder {
    fn unnest(self) -> wgpu::PrimitiveState {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderBundleEncoderDescriptor<'a>>
    for wgpu::RenderBundleEncoderDescriptor<'a>
{
    fn unnest(self) -> wgpu::RenderBundleEncoderDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::RenderBundleEncoderDescriptor<'a>>
    for crate::builders::RenderBundleEncoderDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::RenderBundleEncoderDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::Origin2d> for wgpu::Origin2d {
    fn unnest(self) -> wgpu::Origin2d {
        self
    }
}
impl Nested<wgpu::Origin2d> for crate::builders::Origin2dBuilder {
    fn unnest(self) -> wgpu::Origin2d {
        self.build()
    }
}

impl Nested<wgpu::ImageSubresourceRange> for wgpu::ImageSubresourceRange {
    fn unnest(self) -> wgpu::ImageSubresourceRange {
        self
    }
}
impl Nested<wgpu::ImageSubresourceRange> for crate::builders::ImageSubresourceRangeBuilder {
    fn unnest(self) -> wgpu::ImageSubresourceRange {
        self.build()
    }
}

impl Nested<wgpu::GlBackendOptions> for wgpu::GlBackendOptions {
    fn unnest(self) -> wgpu::GlBackendOptions {
        self
    }
}
impl Nested<wgpu::GlBackendOptions> for crate::builders::GlBackendOptionsBuilder {
    fn unnest(self) -> wgpu::GlBackendOptions {
        self.build()
    }
}

impl Nested<wgpu::DepthBiasState> for wgpu::DepthBiasState {
    fn unnest(self) -> wgpu::DepthBiasState {
        self
    }
}
impl Nested<wgpu::DepthBiasState> for crate::builders::DepthBiasStateBuilder {
    fn unnest(self) -> wgpu::DepthBiasState {
        self.build()
    }
}

impl Nested<wgpu::RenderBundleDepthStencil> for wgpu::RenderBundleDepthStencil {
    fn unnest(self) -> wgpu::RenderBundleDepthStencil {
        self
    }
}
impl Nested<wgpu::RenderBundleDepthStencil> for crate::builders::RenderBundleDepthStencilBuilder {
    fn unnest(self) -> wgpu::RenderBundleDepthStencil {
        self.build()
    }
}

impl<'a> Nested<wgpu::MeshPipelineDescriptor<'a>> for wgpu::MeshPipelineDescriptor<'a> {
    fn unnest(self) -> wgpu::MeshPipelineDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::MeshPipelineDescriptor<'a>>
    for crate::builders::MeshPipelineDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::MeshPipelineDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::CreateTlasDescriptor<'a>> for wgpu::CreateTlasDescriptor<'a> {
    fn unnest(self) -> wgpu::CreateTlasDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::CreateTlasDescriptor<'a>>
    for crate::builders::CreateTlasDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::CreateTlasDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::MemoryBudgetThresholds> for wgpu::MemoryBudgetThresholds {
    fn unnest(self) -> wgpu::MemoryBudgetThresholds {
        self
    }
}
impl Nested<wgpu::MemoryBudgetThresholds> for crate::builders::MemoryBudgetThresholdsBuilder {
    fn unnest(self) -> wgpu::MemoryBudgetThresholds {
        self.build()
    }
}

impl<'a> Nested<wgpu::TextureViewDescriptor<'a>> for wgpu::TextureViewDescriptor<'a> {
    fn unnest(self) -> wgpu::TextureViewDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::TextureViewDescriptor<'a>>
    for crate::builders::TextureViewDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::TextureViewDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::Dx12BackendOptions> for wgpu::Dx12BackendOptions {
    fn unnest(self) -> wgpu::Dx12BackendOptions {
        self
    }
}
impl Nested<wgpu::Dx12BackendOptions> for crate::builders::Dx12BackendOptionsBuilder {
    fn unnest(self) -> wgpu::Dx12BackendOptions {
        self.build()
    }
}

impl Nested<wgpu::DownlevelLimits> for wgpu::DownlevelLimits {
    fn unnest(self) -> wgpu::DownlevelLimits {
        self
    }
}
impl Nested<wgpu::DownlevelLimits> for crate::builders::DownlevelLimitsBuilder {
    fn unnest(self) -> wgpu::DownlevelLimits {
        self.build()
    }
}

impl<'a> Nested<wgpu::BindGroupEntry<'a>> for wgpu::BindGroupEntry<'a> {
    fn unnest(self) -> wgpu::BindGroupEntry<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BindGroupEntry<'a>> for crate::builders::BindGroupEntryBuilder<'a> {
    fn unnest(self) -> wgpu::BindGroupEntry<'a> {
        self.build()
    }
}

impl Nested<wgpu::SurfaceConfiguration> for wgpu::SurfaceConfiguration {
    fn unnest(self) -> wgpu::SurfaceConfiguration {
        self
    }
}
impl Nested<wgpu::SurfaceConfiguration> for crate::builders::SurfaceConfigurationBuilder {
    fn unnest(self) -> wgpu::SurfaceConfiguration {
        self.build()
    }
}

impl Nested<wgpu::BackendOptions> for wgpu::BackendOptions {
    fn unnest(self) -> wgpu::BackendOptions {
        self
    }
}
impl Nested<wgpu::BackendOptions> for crate::builders::BackendOptionsBuilder {
    fn unnest(self) -> wgpu::BackendOptions {
        self.build()
    }
}

impl<V> Nested<wgpu::Operations<V>> for wgpu::Operations<V> {
    fn unnest(self) -> wgpu::Operations<V> {
        self
    }
}
impl<V> Nested<wgpu::Operations<V>> for crate::builders::OperationsBuilder<V> {
    fn unnest(self) -> wgpu::Operations<V> {
        self.build()
    }
}

impl<'a> Nested<wgpu::MeshState<'a>> for wgpu::MeshState<'a> {
    fn unnest(self) -> wgpu::MeshState<'a> {
        self
    }
}
impl<'a> Nested<wgpu::MeshState<'a>> for crate::builders::MeshStateBuilder<'a> {
    fn unnest(self) -> wgpu::MeshState<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::FragmentState<'a>> for wgpu::FragmentState<'a> {
    fn unnest(self) -> wgpu::FragmentState<'a> {
        self
    }
}
impl<'a> Nested<wgpu::FragmentState<'a>> for crate::builders::FragmentStateBuilder<'a> {
    fn unnest(self) -> wgpu::FragmentState<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::VertexBufferLayout<'a>> for wgpu::VertexBufferLayout<'a> {
    fn unnest(self) -> wgpu::VertexBufferLayout<'a> {
        self
    }
}
impl<'a> Nested<wgpu::VertexBufferLayout<'a>> for crate::builders::VertexBufferLayoutBuilder<'a> {
    fn unnest(self) -> wgpu::VertexBufferLayout<'a> {
        self.build()
    }
}

impl Nested<wgpu::Extent3d> for wgpu::Extent3d {
    fn unnest(self) -> wgpu::Extent3d {
        self
    }
}
impl Nested<wgpu::Extent3d> for crate::builders::Extent3dBuilder {
    fn unnest(self) -> wgpu::Extent3d {
        self.build()
    }
}

impl Nested<wgpu::CoreCounters> for wgpu::CoreCounters {
    fn unnest(self) -> wgpu::CoreCounters {
        self
    }
}
impl Nested<wgpu::CoreCounters> for crate::builders::CoreCountersBuilder {
    fn unnest(self) -> wgpu::CoreCounters {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderBundleDescriptor<'a>> for wgpu::RenderBundleDescriptor<'a> {
    fn unnest(self) -> wgpu::RenderBundleDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::RenderBundleDescriptor<'a>>
    for crate::builders::RenderBundleDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::RenderBundleDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::ComputePassDescriptor<'a>> for wgpu::ComputePassDescriptor<'a> {
    fn unnest(self) -> wgpu::ComputePassDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::ComputePassDescriptor<'a>>
    for crate::builders::ComputePassDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::ComputePassDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::TextureDescriptor<'a>> for wgpu::TextureDescriptor<'a> {
    fn unnest(self) -> wgpu::TextureDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::TextureDescriptor<'a>> for crate::builders::TextureDescriptorBuilder<'a> {
    fn unnest(self) -> wgpu::TextureDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::VertexState<'a>> for wgpu::VertexState<'a> {
    fn unnest(self) -> wgpu::VertexState<'a> {
        self
    }
}
impl<'a> Nested<wgpu::VertexState<'a>> for crate::builders::VertexStateBuilder<'a> {
    fn unnest(self) -> wgpu::VertexState<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::ComputePipelineDescriptor<'a>> for wgpu::ComputePipelineDescriptor<'a> {
    fn unnest(self) -> wgpu::ComputePipelineDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::ComputePipelineDescriptor<'a>>
    for crate::builders::ComputePipelineDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::ComputePipelineDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BufferDescriptor<'a>> for wgpu::BufferDescriptor<'a> {
    fn unnest(self) -> wgpu::BufferDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BufferDescriptor<'a>> for crate::builders::BufferDescriptorBuilder<'a> {
    fn unnest(self) -> wgpu::BufferDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::PipelineLayoutDescriptor<'a>> for wgpu::PipelineLayoutDescriptor<'a> {
    fn unnest(self) -> wgpu::PipelineLayoutDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::PipelineLayoutDescriptor<'a>>
    for crate::builders::PipelineLayoutDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::PipelineLayoutDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::Origin3d> for wgpu::Origin3d {
    fn unnest(self) -> wgpu::Origin3d {
        self
    }
}
impl Nested<wgpu::Origin3d> for crate::builders::Origin3dBuilder {
    fn unnest(self) -> wgpu::Origin3d {
        self.build()
    }
}

impl Nested<wgpu::StencilState> for wgpu::StencilState {
    fn unnest(self) -> wgpu::StencilState {
        self
    }
}
impl Nested<wgpu::StencilState> for crate::builders::StencilStateBuilder {
    fn unnest(self) -> wgpu::StencilState {
        self.build()
    }
}

impl Nested<wgpu::ShaderRuntimeChecks> for wgpu::ShaderRuntimeChecks {
    fn unnest(self) -> wgpu::ShaderRuntimeChecks {
        self
    }
}
impl Nested<wgpu::ShaderRuntimeChecks> for crate::builders::ShaderRuntimeChecksBuilder {
    fn unnest(self) -> wgpu::ShaderRuntimeChecks {
        self.build()
    }
}

impl Nested<wgpu::ColorTargetState> for wgpu::ColorTargetState {
    fn unnest(self) -> wgpu::ColorTargetState {
        self
    }
}
impl Nested<wgpu::ColorTargetState> for crate::builders::ColorTargetStateBuilder {
    fn unnest(self) -> wgpu::ColorTargetState {
        self.build()
    }
}

impl Nested<wgpu::BlendState> for wgpu::BlendState {
    fn unnest(self) -> wgpu::BlendState {
        self
    }
}
impl Nested<wgpu::BlendState> for crate::builders::BlendStateBuilder {
    fn unnest(self) -> wgpu::BlendState {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderPassDescriptor<'a>> for wgpu::RenderPassDescriptor<'a> {
    fn unnest(self) -> wgpu::RenderPassDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::RenderPassDescriptor<'a>>
    for crate::builders::RenderPassDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::RenderPassDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BufferBinding<'a>> for wgpu::BufferBinding<'a> {
    fn unnest(self) -> wgpu::BufferBinding<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BufferBinding<'a>> for crate::builders::BufferBindingBuilder<'a> {
    fn unnest(self) -> wgpu::BufferBinding<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BlasTriangleGeometry<'a>> for wgpu::BlasTriangleGeometry<'a> {
    fn unnest(self) -> wgpu::BlasTriangleGeometry<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BlasTriangleGeometry<'a>>
    for crate::builders::BlasTriangleGeometryBuilder<'a>
{
    fn unnest(self) -> wgpu::BlasTriangleGeometry<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::PipelineCacheDescriptor<'a>> for wgpu::PipelineCacheDescriptor<'a> {
    fn unnest(self) -> wgpu::PipelineCacheDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::PipelineCacheDescriptor<'a>>
    for crate::builders::PipelineCacheDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::PipelineCacheDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::StencilFaceState> for wgpu::StencilFaceState {
    fn unnest(self) -> wgpu::StencilFaceState {
        self
    }
}
impl Nested<wgpu::StencilFaceState> for crate::builders::StencilFaceStateBuilder {
    fn unnest(self) -> wgpu::StencilFaceState {
        self.build()
    }
}

impl<B> Nested<wgpu::TexelCopyBufferInfoBase<B>> for wgpu::TexelCopyBufferInfoBase<B> {
    fn unnest(self) -> wgpu::TexelCopyBufferInfoBase<B> {
        self
    }
}
impl<B> Nested<wgpu::TexelCopyBufferInfoBase<B>>
    for crate::builders::TexelCopyBufferInfoBaseBuilder<B>
{
    fn unnest(self) -> wgpu::TexelCopyBufferInfoBase<B> {
        self.build()
    }
}

impl Nested<wgpu::util::DrawIndirectArgs> for wgpu::util::DrawIndirectArgs {
    fn unnest(self) -> wgpu::util::DrawIndirectArgs {
        self
    }
}
impl Nested<wgpu::util::DrawIndirectArgs> for crate::builders::DrawIndirectArgsBuilder {
    fn unnest(self) -> wgpu::util::DrawIndirectArgs {
        self.build()
    }
}

impl<'a> Nested<wgpu::DeviceDescriptor<'a>> for wgpu::DeviceDescriptor<'a> {
    fn unnest(self) -> wgpu::DeviceDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::DeviceDescriptor<'a>> for crate::builders::DeviceDescriptorBuilder<'a> {
    fn unnest(self) -> wgpu::DeviceDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BindGroupDescriptor<'a>> for wgpu::BindGroupDescriptor<'a> {
    fn unnest(self) -> wgpu::BindGroupDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BindGroupDescriptor<'a>> for crate::builders::BindGroupDescriptorBuilder<'a> {
    fn unnest(self) -> wgpu::BindGroupDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::VertexAttribute> for wgpu::VertexAttribute {
    fn unnest(self) -> wgpu::VertexAttribute {
        self
    }
}
impl Nested<wgpu::VertexAttribute> for crate::builders::VertexAttributeBuilder {
    fn unnest(self) -> wgpu::VertexAttribute {
        self.build()
    }
}

impl<'a> Nested<wgpu::TexelCopyBufferInfo<'a>> for wgpu::TexelCopyBufferInfo<'a> {
    fn unnest(self) -> wgpu::TexelCopyBufferInfo<'a> {
        self
    }
}
impl<'a> Nested<wgpu::TexelCopyBufferInfo<'a>> for crate::builders::TexelCopyBufferInfoBuilder<'a> {
    fn unnest(self) -> wgpu::TexelCopyBufferInfo<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::ShaderModuleDescriptorPassthrough<'a>>
    for wgpu::ShaderModuleDescriptorPassthrough<'a>
{
    fn unnest(self) -> wgpu::ShaderModuleDescriptorPassthrough<'a> {
        self
    }
}
impl<'a> Nested<wgpu::ShaderModuleDescriptorPassthrough<'a>>
    for crate::builders::ShaderModuleDescriptorPassthroughBuilder<'a>
{
    fn unnest(self) -> wgpu::ShaderModuleDescriptorPassthrough<'a> {
        self.build()
    }
}

impl<T> Nested<wgpu::BufferTransition<T>> for wgpu::BufferTransition<T> {
    fn unnest(self) -> wgpu::BufferTransition<T> {
        self
    }
}
impl<T> Nested<wgpu::BufferTransition<T>> for crate::builders::BufferTransitionBuilder<T> {
    fn unnest(self) -> wgpu::BufferTransition<T> {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderPassTimestampWrites<'a>> for wgpu::RenderPassTimestampWrites<'a> {
    fn unnest(self) -> wgpu::RenderPassTimestampWrites<'a> {
        self
    }
}
impl<'a> Nested<wgpu::RenderPassTimestampWrites<'a>>
    for crate::builders::RenderPassTimestampWritesBuilder<'a>
{
    fn unnest(self) -> wgpu::RenderPassTimestampWrites<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::util::BufferInitDescriptor<'a>> for wgpu::util::BufferInitDescriptor<'a> {
    fn unnest(self) -> wgpu::util::BufferInitDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::util::BufferInitDescriptor<'a>>
    for crate::builders::BufferInitDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::util::BufferInitDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::NoopBackendOptions> for wgpu::NoopBackendOptions {
    fn unnest(self) -> wgpu::NoopBackendOptions {
        self
    }
}
impl Nested<wgpu::NoopBackendOptions> for crate::builders::NoopBackendOptionsBuilder {
    fn unnest(self) -> wgpu::NoopBackendOptions {
        self.build()
    }
}

impl<'tex> Nested<wgpu::RenderPassColorAttachment<'tex>> for wgpu::RenderPassColorAttachment<'tex> {
    fn unnest(self) -> wgpu::RenderPassColorAttachment<'tex> {
        self
    }
}
impl<'tex> Nested<wgpu::RenderPassColorAttachment<'tex>>
    for crate::builders::RenderPassColorAttachmentBuilder<'tex>
{
    fn unnest(self) -> wgpu::RenderPassColorAttachment<'tex> {
        self.build()
    }
}

impl<L> Nested<wgpu::CommandBufferDescriptor<L>> for wgpu::CommandBufferDescriptor<L> {
    fn unnest(self) -> wgpu::CommandBufferDescriptor<L> {
        self
    }
}
impl<L> Nested<wgpu::CommandBufferDescriptor<L>>
    for crate::builders::CommandBufferDescriptorBuilder<L>
{
    fn unnest(self) -> wgpu::CommandBufferDescriptor<L> {
        self.build()
    }
}

impl Nested<wgpu::util::DrawIndexedIndirectArgs> for wgpu::util::DrawIndexedIndirectArgs {
    fn unnest(self) -> wgpu::util::DrawIndexedIndirectArgs {
        self
    }
}
impl Nested<wgpu::util::DrawIndexedIndirectArgs>
    for crate::builders::DrawIndexedIndirectArgsBuilder
{
    fn unnest(self) -> wgpu::util::DrawIndexedIndirectArgs {
        self.build()
    }
}

impl<T> Nested<wgpu::CopyExternalImageDestInfo<T>> for wgpu::CopyExternalImageDestInfo<T> {
    fn unnest(self) -> wgpu::CopyExternalImageDestInfo<T> {
        self
    }
}
impl<T> Nested<wgpu::CopyExternalImageDestInfo<T>>
    for crate::builders::CopyExternalImageDestInfoBuilder<T>
{
    fn unnest(self) -> wgpu::CopyExternalImageDestInfo<T> {
        self.build()
    }
}

impl Nested<wgpu::BlendComponent> for wgpu::BlendComponent {
    fn unnest(self) -> wgpu::BlendComponent {
        self
    }
}
impl Nested<wgpu::BlendComponent> for crate::builders::BlendComponentBuilder {
    fn unnest(self) -> wgpu::BlendComponent {
        self.build()
    }
}

impl Nested<wgpu::ExternalTextureTransferFunction> for wgpu::ExternalTextureTransferFunction {
    fn unnest(self) -> wgpu::ExternalTextureTransferFunction {
        self
    }
}
impl Nested<wgpu::ExternalTextureTransferFunction>
    for crate::builders::ExternalTextureTransferFunctionBuilder
{
    fn unnest(self) -> wgpu::ExternalTextureTransferFunction {
        self.build()
    }
}

impl Nested<wgpu::Color> for wgpu::Color {
    fn unnest(self) -> wgpu::Color {
        self
    }
}
impl Nested<wgpu::Color> for crate::builders::ColorBuilder {
    fn unnest(self) -> wgpu::Color {
        self.build()
    }
}

impl<T> Nested<wgpu::TextureTransition<T>> for wgpu::TextureTransition<T> {
    fn unnest(self) -> wgpu::TextureTransition<T> {
        self
    }
}
impl<T> Nested<wgpu::TextureTransition<T>> for crate::builders::TextureTransitionBuilder<T> {
    fn unnest(self) -> wgpu::TextureTransition<T> {
        self.build()
    }
}

impl Nested<wgpu::util::DispatchIndirectArgs> for wgpu::util::DispatchIndirectArgs {
    fn unnest(self) -> wgpu::util::DispatchIndirectArgs {
        self
    }
}
impl Nested<wgpu::util::DispatchIndirectArgs> for crate::builders::DispatchIndirectArgsBuilder {
    fn unnest(self) -> wgpu::util::DispatchIndirectArgs {
        self.build()
    }
}

impl<'a> Nested<wgpu::ComputePassTimestampWrites<'a>> for wgpu::ComputePassTimestampWrites<'a> {
    fn unnest(self) -> wgpu::ComputePassTimestampWrites<'a> {
        self
    }
}
impl<'a> Nested<wgpu::ComputePassTimestampWrites<'a>>
    for crate::builders::ComputePassTimestampWritesBuilder<'a>
{
    fn unnest(self) -> wgpu::ComputePassTimestampWrites<'a> {
        self.build()
    }
}

impl Nested<wgpu::CompilationInfo> for wgpu::CompilationInfo {
    fn unnest(self) -> wgpu::CompilationInfo {
        self
    }
}
impl Nested<wgpu::CompilationInfo> for crate::builders::CompilationInfoBuilder {
    fn unnest(self) -> wgpu::CompilationInfo {
        self.build()
    }
}

impl Nested<wgpu::PushConstantRange> for wgpu::PushConstantRange {
    fn unnest(self) -> wgpu::PushConstantRange {
        self
    }
}
impl Nested<wgpu::PushConstantRange> for crate::builders::PushConstantRangeBuilder {
    fn unnest(self) -> wgpu::PushConstantRange {
        self.build()
    }
}

impl<'a> Nested<wgpu::CommandEncoderDescriptor<'a>> for wgpu::CommandEncoderDescriptor<'a> {
    fn unnest(self) -> wgpu::CommandEncoderDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::CommandEncoderDescriptor<'a>>
    for crate::builders::CommandEncoderDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::CommandEncoderDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::DepthStencilState> for wgpu::DepthStencilState {
    fn unnest(self) -> wgpu::DepthStencilState {
        self
    }
}
impl Nested<wgpu::DepthStencilState> for crate::builders::DepthStencilStateBuilder {
    fn unnest(self) -> wgpu::DepthStencilState {
        self.build()
    }
}

impl Nested<wgpu::MultisampleState> for wgpu::MultisampleState {
    fn unnest(self) -> wgpu::MultisampleState {
        self
    }
}
impl Nested<wgpu::MultisampleState> for crate::builders::MultisampleStateBuilder {
    fn unnest(self) -> wgpu::MultisampleState {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderPipelineDescriptor<'a>> for wgpu::RenderPipelineDescriptor<'a> {
    fn unnest(self) -> wgpu::RenderPipelineDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::RenderPipelineDescriptor<'a>>
    for crate::builders::RenderPipelineDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::RenderPipelineDescriptor<'a> {
        self.build()
    }
}

impl<'tex> Nested<wgpu::RenderPassDepthStencilAttachment<'tex>>
    for wgpu::RenderPassDepthStencilAttachment<'tex>
{
    fn unnest(self) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
        self
    }
}
impl<'tex> Nested<wgpu::RenderPassDepthStencilAttachment<'tex>>
    for crate::builders::RenderPassDepthStencilAttachmentBuilder<'tex>
{
    fn unnest(self) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
        self.build()
    }
}

impl<'a, 'b> Nested<wgpu::RequestAdapterOptions<'a, 'b>> for wgpu::RequestAdapterOptions<'a, 'b> {
    fn unnest(self) -> wgpu::RequestAdapterOptions<'a, 'b> {
        self
    }
}
impl<'a, 'b> Nested<wgpu::RequestAdapterOptions<'a, 'b>>
    for crate::builders::RequestAdapterOptionsBuilder<'a, 'b>
{
    fn unnest(self) -> wgpu::RequestAdapterOptions<'a, 'b> {
        self.build()
    }
}

impl Nested<wgpu::InstanceDescriptor> for wgpu::InstanceDescriptor {
    fn unnest(self) -> wgpu::InstanceDescriptor {
        self
    }
}
impl Nested<wgpu::InstanceDescriptor> for crate::builders::InstanceDescriptorBuilder {
    fn unnest(self) -> wgpu::InstanceDescriptor {
        self.build()
    }
}

impl<S> Nested<wgpu::RequestAdapterOptionsBase<S>> for wgpu::RequestAdapterOptionsBase<S> {
    fn unnest(self) -> wgpu::RequestAdapterOptionsBase<S> {
        self
    }
}
impl<S> Nested<wgpu::RequestAdapterOptionsBase<S>>
    for crate::builders::RequestAdapterOptionsBaseBuilder<S>
{
    fn unnest(self) -> wgpu::RequestAdapterOptionsBase<S> {
        self.build()
    }
}

impl<'a> Nested<wgpu::ShaderModuleDescriptor<'a>> for wgpu::ShaderModuleDescriptor<'a> {
    fn unnest(self) -> wgpu::ShaderModuleDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::ShaderModuleDescriptor<'a>>
    for crate::builders::ShaderModuleDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::ShaderModuleDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BindGroupLayoutDescriptor<'a>> for wgpu::BindGroupLayoutDescriptor<'a> {
    fn unnest(self) -> wgpu::BindGroupLayoutDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BindGroupLayoutDescriptor<'a>>
    for crate::builders::BindGroupLayoutDescriptorBuilder<'a>
{
    fn unnest(self) -> wgpu::BindGroupLayoutDescriptor<'a> {
        self.build()
    }
}

impl<T> Nested<wgpu::TexelCopyTextureInfoBase<T>> for wgpu::TexelCopyTextureInfoBase<T> {
    fn unnest(self) -> wgpu::TexelCopyTextureInfoBase<T> {
        self
    }
}
impl<T> Nested<wgpu::TexelCopyTextureInfoBase<T>>
    for crate::builders::TexelCopyTextureInfoBaseBuilder<T>
{
    fn unnest(self) -> wgpu::TexelCopyTextureInfoBase<T> {
        self.build()
    }
}

impl<'a> Nested<wgpu::TexelCopyTextureInfo<'a>> for wgpu::TexelCopyTextureInfo<'a> {
    fn unnest(self) -> wgpu::TexelCopyTextureInfo<'a> {
        self
    }
}
impl<'a> Nested<wgpu::TexelCopyTextureInfo<'a>>
    for crate::builders::TexelCopyTextureInfoBuilder<'a>
{
    fn unnest(self) -> wgpu::TexelCopyTextureInfo<'a> {
        self.build()
    }
}

impl Nested<wgpu::BindGroupLayoutEntry> for wgpu::BindGroupLayoutEntry {
    fn unnest(self) -> wgpu::BindGroupLayoutEntry {
        self
    }
}
impl Nested<wgpu::BindGroupLayoutEntry> for crate::builders::BindGroupLayoutEntryBuilder {
    fn unnest(self) -> wgpu::BindGroupLayoutEntry {
        self.build()
    }
}

impl<'a> Nested<wgpu::PipelineCompilationOptions<'a>> for wgpu::PipelineCompilationOptions<'a> {
    fn unnest(self) -> wgpu::PipelineCompilationOptions<'a> {
        self
    }
}
impl<'a> Nested<wgpu::PipelineCompilationOptions<'a>>
    for crate::builders::PipelineCompilationOptionsBuilder<'a>
{
    fn unnest(self) -> wgpu::PipelineCompilationOptions<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BlasBuildEntry<'a>> for wgpu::BlasBuildEntry<'a> {
    fn unnest(self) -> wgpu::BlasBuildEntry<'a> {
        self
    }
}
impl<'a> Nested<wgpu::BlasBuildEntry<'a>> for crate::builders::BlasBuildEntryBuilder<'a> {
    fn unnest(self) -> wgpu::BlasBuildEntry<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::TaskState<'a>> for wgpu::TaskState<'a> {
    fn unnest(self) -> wgpu::TaskState<'a> {
        self
    }
}
impl<'a> Nested<wgpu::TaskState<'a>> for crate::builders::TaskStateBuilder<'a> {
    fn unnest(self) -> wgpu::TaskState<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::QuerySetDescriptor<'a>> for wgpu::QuerySetDescriptor<'a> {
    fn unnest(self) -> wgpu::QuerySetDescriptor<'a> {
        self
    }
}
impl<'a> Nested<wgpu::QuerySetDescriptor<'a>> for crate::builders::QuerySetDescriptorBuilder<'a> {
    fn unnest(self) -> wgpu::QuerySetDescriptor<'a> {
        self.build()
    }
}
