use crate::Nested;

impl<'a> Nested<wgpu::BufferBinding<'a>> for wgpu::BufferBinding<'a> {
    fn unnest(self) -> wgpu::BufferBinding<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::BufferBinding<'a>>
    for crate::builders::builder_buffer_binding::BufferBindingBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::buffer_binding_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BufferBinding<'a> {
        self.build()
    }
}

impl Nested<wgpu::GlBackendOptions> for wgpu::GlBackendOptions {
    fn unnest(self) -> wgpu::GlBackendOptions {
        self
    }
}
impl<BuilderState> Nested<wgpu::GlBackendOptions>
    for crate::builders::GlBackendOptionsBuilder<BuilderState>
where
    BuilderState: crate::builders::gl_backend_options_builder::IsComplete,
{
    fn unnest(self) -> wgpu::GlBackendOptions {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderPassTimestampWrites<'a>> for wgpu::RenderPassTimestampWrites<'a> {
    fn unnest(self) -> wgpu::RenderPassTimestampWrites<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::RenderPassTimestampWrites<'a>>
    for crate::builders::RenderPassTimestampWritesBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::render_pass_timestamp_writes_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderPassTimestampWrites<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::BindGroupEntry<'a>> for wgpu::BindGroupEntry<'a> {
    fn unnest(self) -> wgpu::BindGroupEntry<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::BindGroupEntry<'a>>
    for crate::builders::BindGroupEntryBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::bind_group_entry_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BindGroupEntry<'a> {
        self.build()
    }
}

impl Nested<wgpu::ColorTargetState> for wgpu::ColorTargetState {
    fn unnest(self) -> wgpu::ColorTargetState {
        self
    }
}
impl<NestedField1: Nested<wgpu::BlendState>, BuilderState> Nested<wgpu::ColorTargetState>
    for crate::builders::ColorTargetStateBuilder<NestedField1, BuilderState>
where
    BuilderState: crate::builders::color_target_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ColorTargetState {
        self.build()
    }
}

impl Nested<wgpu::BlendState> for wgpu::BlendState {
    fn unnest(self) -> wgpu::BlendState {
        self
    }
}
impl<
    NestedField0: Nested<wgpu::BlendComponent>,
    NestedField1: Nested<wgpu::BlendComponent>,
    BuilderState,
> Nested<wgpu::BlendState>
    for crate::builders::BlendStateBuilder<NestedField0, NestedField1, BuilderState>
where
    BuilderState: crate::builders::blend_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BlendState {
        self.build()
    }
}

impl<T> Nested<wgpu::TextureTransition<T>> for wgpu::TextureTransition<T> {
    fn unnest(self) -> wgpu::TextureTransition<T> {
        self
    }
}
impl<T, BuilderState> Nested<wgpu::TextureTransition<T>>
    for crate::builders::TextureTransitionBuilder<T, BuilderState>
where
    BuilderState: crate::builders::texture_transition_builder::IsComplete,
{
    fn unnest(self) -> wgpu::TextureTransition<T> {
        self.build()
    }
}

impl<'a> Nested<wgpu::util::BufferInitDescriptor<'a>> for wgpu::util::BufferInitDescriptor<'a> {
    fn unnest(self) -> wgpu::util::BufferInitDescriptor<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::util::BufferInitDescriptor<'a>>
    for crate::builders::BufferInitDescriptorBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::buffer_init_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::util::BufferInitDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::Origin2d> for wgpu::Origin2d {
    fn unnest(self) -> wgpu::Origin2d {
        self
    }
}
impl<BuilderState> Nested<wgpu::Origin2d> for crate::builders::Origin2DBuilder<BuilderState>
where
    BuilderState: crate::builders::origin2_d_builder::IsComplete,
{
    fn unnest(self) -> wgpu::Origin2d {
        self.build()
    }
}

impl Nested<wgpu::Color> for wgpu::Color {
    fn unnest(self) -> wgpu::Color {
        self
    }
}
impl<BuilderState> Nested<wgpu::Color> for crate::builders::ColorBuilder<BuilderState>
where
    BuilderState: crate::builders::color_builder::IsComplete,
{
    fn unnest(self) -> wgpu::Color {
        self.build()
    }
}

impl Nested<wgpu::ImageSubresourceRange> for wgpu::ImageSubresourceRange {
    fn unnest(self) -> wgpu::ImageSubresourceRange {
        self
    }
}
impl<BuilderState> Nested<wgpu::ImageSubresourceRange>
    for crate::builders::ImageSubresourceRangeBuilder<BuilderState>
where
    BuilderState: crate::builders::image_subresource_range_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ImageSubresourceRange {
        self.build()
    }
}

impl<'a> Nested<wgpu::ComputePipelineDescriptor<'a>> for wgpu::ComputePipelineDescriptor<'a> {
    fn unnest(self) -> wgpu::ComputePipelineDescriptor<'a> {
        self
    }
}
impl<'a, NestedField4: Nested<wgpu::PipelineCompilationOptions<'a>>, BuilderState>
    Nested<wgpu::ComputePipelineDescriptor<'a>>
    for crate::builders::ComputePipelineDescriptorBuilder<'a, NestedField4, BuilderState>
where
    BuilderState: crate::builders::compute_pipeline_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ComputePipelineDescriptor<'a> {
        self.build()
    }
}

impl<T> Nested<wgpu::CopyExternalImageDestInfo<T>> for wgpu::CopyExternalImageDestInfo<T> {
    fn unnest(self) -> wgpu::CopyExternalImageDestInfo<T> {
        self
    }
}
impl<T, NestedField2: Nested<wgpu::Origin3d>, BuilderState>
    Nested<wgpu::CopyExternalImageDestInfo<T>>
    for crate::builders::CopyExternalImageDestInfoBuilder<T, NestedField2, BuilderState>
where
    BuilderState: crate::builders::copy_external_image_dest_info_builder::IsComplete,
{
    fn unnest(self) -> wgpu::CopyExternalImageDestInfo<T> {
        self.build()
    }
}

impl Nested<wgpu::MemoryBudgetThresholds> for wgpu::MemoryBudgetThresholds {
    fn unnest(self) -> wgpu::MemoryBudgetThresholds {
        self
    }
}
impl<BuilderState> Nested<wgpu::MemoryBudgetThresholds>
    for crate::builders::MemoryBudgetThresholdsBuilder<BuilderState>
where
    BuilderState: crate::builders::memory_budget_thresholds_builder::IsComplete,
{
    fn unnest(self) -> wgpu::MemoryBudgetThresholds {
        self.build()
    }
}

impl<'a> Nested<wgpu::BindGroupDescriptor<'a>> for wgpu::BindGroupDescriptor<'a> {
    fn unnest(self) -> wgpu::BindGroupDescriptor<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::BindGroupDescriptor<'a>>
    for crate::builders::BindGroupDescriptorBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::bind_group_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BindGroupDescriptor<'a> {
        self.build()
    }
}

impl<T> Nested<wgpu::BufferTransition<T>> for wgpu::BufferTransition<T> {
    fn unnest(self) -> wgpu::BufferTransition<T> {
        self
    }
}
impl<T, BuilderState> Nested<wgpu::BufferTransition<T>>
    for crate::builders::BufferTransitionBuilder<T, BuilderState>
where
    BuilderState: crate::builders::buffer_transition_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BufferTransition<T> {
        self.build()
    }
}

impl Nested<wgpu::util::DrawIndexedIndirectArgs> for wgpu::util::DrawIndexedIndirectArgs {
    fn unnest(self) -> wgpu::util::DrawIndexedIndirectArgs {
        self
    }
}
impl<BuilderState> Nested<wgpu::util::DrawIndexedIndirectArgs>
    for crate::builders::DrawIndexedIndirectArgsBuilder<BuilderState>
where
    BuilderState: crate::builders::draw_indexed_indirect_args_builder::IsComplete,
{
    fn unnest(self) -> wgpu::util::DrawIndexedIndirectArgs {
        self.build()
    }
}

impl<'a> Nested<wgpu::BindGroupLayoutDescriptor<'a>> for wgpu::BindGroupLayoutDescriptor<'a> {
    fn unnest(self) -> wgpu::BindGroupLayoutDescriptor<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::BindGroupLayoutDescriptor<'a>>
    for crate::builders::BindGroupLayoutDescriptorBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::bind_group_layout_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BindGroupLayoutDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::Origin3d> for wgpu::Origin3d {
    fn unnest(self) -> wgpu::Origin3d {
        self
    }
}
impl<BuilderState> Nested<wgpu::Origin3d> for crate::builders::Origin3DBuilder<BuilderState>
where
    BuilderState: crate::builders::origin3_d_builder::IsComplete,
{
    fn unnest(self) -> wgpu::Origin3d {
        self.build()
    }
}

impl<'a> Nested<wgpu::MeshPipelineDescriptor<'a>> for wgpu::MeshPipelineDescriptor<'a> {
    fn unnest(self) -> wgpu::MeshPipelineDescriptor<'a> {
        self
    }
}
impl<
    'a,
    NestedField2: Nested<wgpu::TaskState<'a>>,
    NestedField3: Nested<wgpu::MeshState<'a>>,
    NestedField4: Nested<wgpu::PrimitiveState>,
    NestedField5: Nested<wgpu::DepthStencilState>,
    NestedField6: Nested<wgpu::MultisampleState>,
    NestedField7: Nested<wgpu::FragmentState<'a>>,
    BuilderState,
> Nested<wgpu::MeshPipelineDescriptor<'a>>
    for crate::builders::MeshPipelineDescriptorBuilder<
        'a,
        NestedField2,
        NestedField3,
        NestedField4,
        NestedField5,
        NestedField6,
        NestedField7,
        BuilderState,
    >
where
    BuilderState: crate::builders::mesh_pipeline_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::MeshPipelineDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::ShaderModuleDescriptor<'a>> for wgpu::ShaderModuleDescriptor<'a> {
    fn unnest(self) -> wgpu::ShaderModuleDescriptor<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::ShaderModuleDescriptor<'a>>
    for crate::builders::ShaderModuleDescriptorBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::shader_module_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ShaderModuleDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::PipelineCacheDescriptor<'a>> for wgpu::PipelineCacheDescriptor<'a> {
    fn unnest(self) -> wgpu::PipelineCacheDescriptor<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::PipelineCacheDescriptor<'a>>
    for crate::builders::PipelineCacheDescriptorBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::pipeline_cache_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::PipelineCacheDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::NoopBackendOptions> for wgpu::NoopBackendOptions {
    fn unnest(self) -> wgpu::NoopBackendOptions {
        self
    }
}
impl<BuilderState> Nested<wgpu::NoopBackendOptions>
    for crate::builders::NoopBackendOptionsBuilder<BuilderState>
where
    BuilderState: crate::builders::noop_backend_options_builder::IsComplete,
{
    fn unnest(self) -> wgpu::NoopBackendOptions {
        self.build()
    }
}

impl<'a> Nested<wgpu::BlasBuildEntry<'a>> for wgpu::BlasBuildEntry<'a> {
    fn unnest(self) -> wgpu::BlasBuildEntry<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::BlasBuildEntry<'a>>
    for crate::builders::BlasBuildEntryBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::blas_build_entry_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BlasBuildEntry<'a> {
        self.build()
    }
}

impl Nested<wgpu::ExternalTextureTransferFunction> for wgpu::ExternalTextureTransferFunction {
    fn unnest(self) -> wgpu::ExternalTextureTransferFunction {
        self
    }
}
impl<BuilderState> Nested<wgpu::ExternalTextureTransferFunction>
    for crate::builders::ExternalTextureTransferFunctionBuilder<BuilderState>
where
    BuilderState: crate::builders::external_texture_transfer_function_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ExternalTextureTransferFunction {
        self.build()
    }
}

impl<'a> Nested<wgpu::PipelineLayoutDescriptor<'a>> for wgpu::PipelineLayoutDescriptor<'a> {
    fn unnest(self) -> wgpu::PipelineLayoutDescriptor<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::PipelineLayoutDescriptor<'a>>
    for crate::builders::PipelineLayoutDescriptorBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::pipeline_layout_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::PipelineLayoutDescriptor<'a> {
        self.build()
    }
}

impl Nested<wgpu::StencilFaceState> for wgpu::StencilFaceState {
    fn unnest(self) -> wgpu::StencilFaceState {
        self
    }
}
impl<BuilderState> Nested<wgpu::StencilFaceState>
    for crate::builders::StencilFaceStateBuilder<BuilderState>
where
    BuilderState: crate::builders::stencil_face_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::StencilFaceState {
        self.build()
    }
}

impl<B> Nested<wgpu::TexelCopyBufferInfoBase<B>> for wgpu::TexelCopyBufferInfoBase<B> {
    fn unnest(self) -> wgpu::TexelCopyBufferInfoBase<B> {
        self
    }
}
impl<B, NestedField1: Nested<wgpu::TexelCopyBufferLayout>, BuilderState>
    Nested<wgpu::TexelCopyBufferInfoBase<B>>
    for crate::builders::TexelCopyBufferInfoBaseBuilder<B, NestedField1, BuilderState>
where
    BuilderState: crate::builders::texel_copy_buffer_info_base_builder::IsComplete,
{
    fn unnest(self) -> wgpu::TexelCopyBufferInfoBase<B> {
        self.build()
    }
}

impl<V: Default> Nested<wgpu::Operations<V: Default>> for wgpu::Operations<V: Default> {
    fn unnest(self) -> wgpu::Operations<V: Default> {
        self
    }
}
impl<V: Default, BuilderState> Nested<wgpu::Operations<V: Default>>
    for crate::builders::OperationsBuilder<V: Default, BuilderState>
where
    BuilderState: crate::builders::operations_builder::IsComplete,
{
    fn unnest(self) -> wgpu::Operations<V: Default> {
        self.build()
    }
}

impl Nested<wgpu::BindGroupLayoutEntry> for wgpu::BindGroupLayoutEntry {
    fn unnest(self) -> wgpu::BindGroupLayoutEntry {
        self
    }
}
impl<BuilderState> Nested<wgpu::BindGroupLayoutEntry>
    for crate::builders::BindGroupLayoutEntryBuilder<BuilderState>
where
    BuilderState: crate::builders::bind_group_layout_entry_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BindGroupLayoutEntry {
        self.build()
    }
}

impl Nested<wgpu::DepthStencilState> for wgpu::DepthStencilState {
    fn unnest(self) -> wgpu::DepthStencilState {
        self
    }
}
impl<
    NestedField3: Nested<wgpu::StencilState>,
    NestedField4: Nested<wgpu::DepthBiasState>,
    BuilderState,
> Nested<wgpu::DepthStencilState>
    for crate::builders::DepthStencilStateBuilder<NestedField3, NestedField4, BuilderState>
where
    BuilderState: crate::builders::depth_stencil_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::DepthStencilState {
        self.build()
    }
}

impl Nested<wgpu::util::DrawIndirectArgs> for wgpu::util::DrawIndirectArgs {
    fn unnest(self) -> wgpu::util::DrawIndirectArgs {
        self
    }
}
impl<BuilderState> Nested<wgpu::util::DrawIndirectArgs>
    for crate::builders::DrawIndirectArgsBuilder<BuilderState>
where
    BuilderState: crate::builders::draw_indirect_args_builder::IsComplete,
{
    fn unnest(self) -> wgpu::util::DrawIndirectArgs {
        self.build()
    }
}

impl Nested<wgpu::util::DispatchIndirectArgs> for wgpu::util::DispatchIndirectArgs {
    fn unnest(self) -> wgpu::util::DispatchIndirectArgs {
        self
    }
}
impl<BuilderState> Nested<wgpu::util::DispatchIndirectArgs>
    for crate::builders::DispatchIndirectArgsBuilder<BuilderState>
where
    BuilderState: crate::builders::dispatch_indirect_args_builder::IsComplete,
{
    fn unnest(self) -> wgpu::util::DispatchIndirectArgs {
        self.build()
    }
}

impl<'a> Nested<wgpu::ComputePassDescriptor<'a>> for wgpu::ComputePassDescriptor<'a> {
    fn unnest(self) -> wgpu::ComputePassDescriptor<'a> {
        self
    }
}
impl<'a, NestedField1: Nested<wgpu::ComputePassTimestampWrites<'a>>, BuilderState>
    Nested<wgpu::ComputePassDescriptor<'a>>
    for crate::builders::ComputePassDescriptorBuilder<'a, NestedField1, BuilderState>
where
    BuilderState: crate::builders::compute_pass_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ComputePassDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::MeshState<'a>> for wgpu::MeshState<'a> {
    fn unnest(self) -> wgpu::MeshState<'a> {
        self
    }
}
impl<'a, NestedField2: Nested<wgpu::PipelineCompilationOptions<'a>>, BuilderState>
    Nested<wgpu::MeshState<'a>>
    for crate::builders::MeshStateBuilder<'a, NestedField2, BuilderState>
where
    BuilderState: crate::builders::mesh_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::MeshState<'a> {
        self.build()
    }
}

impl Nested<wgpu::VertexAttribute> for wgpu::VertexAttribute {
    fn unnest(self) -> wgpu::VertexAttribute {
        self
    }
}
impl<BuilderState> Nested<wgpu::VertexAttribute>
    for crate::builders::VertexAttributeBuilder<BuilderState>
where
    BuilderState: crate::builders::vertex_attribute_builder::IsComplete,
{
    fn unnest(self) -> wgpu::VertexAttribute {
        self.build()
    }
}

impl Nested<wgpu::DownlevelLimits> for wgpu::DownlevelLimits {
    fn unnest(self) -> wgpu::DownlevelLimits {
        self
    }
}
impl<BuilderState> Nested<wgpu::DownlevelLimits>
    for crate::builders::DownlevelLimitsBuilder<BuilderState>
where
    BuilderState: crate::builders::downlevel_limits_builder::IsComplete,
{
    fn unnest(self) -> wgpu::DownlevelLimits {
        self.build()
    }
}

impl Nested<wgpu::RenderBundleDepthStencil> for wgpu::RenderBundleDepthStencil {
    fn unnest(self) -> wgpu::RenderBundleDepthStencil {
        self
    }
}
impl<BuilderState> Nested<wgpu::RenderBundleDepthStencil>
    for crate::builders::RenderBundleDepthStencilBuilder<BuilderState>
where
    BuilderState: crate::builders::render_bundle_depth_stencil_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderBundleDepthStencil {
        self.build()
    }
}

impl Nested<wgpu::BackendOptions> for wgpu::BackendOptions {
    fn unnest(self) -> wgpu::BackendOptions {
        self
    }
}
impl<
    NestedField0: Nested<wgpu::GlBackendOptions>,
    NestedField1: Nested<wgpu::Dx12BackendOptions>,
    NestedField2: Nested<wgpu::NoopBackendOptions>,
    BuilderState,
> Nested<wgpu::BackendOptions>
    for crate::builders::BackendOptionsBuilder<
        NestedField0,
        NestedField1,
        NestedField2,
        BuilderState,
    >
where
    BuilderState: crate::builders::backend_options_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BackendOptions {
        self.build()
    }
}

impl Nested<wgpu::StencilState> for wgpu::StencilState {
    fn unnest(self) -> wgpu::StencilState {
        self
    }
}
impl<
    NestedField0: Nested<wgpu::StencilFaceState>,
    NestedField1: Nested<wgpu::StencilFaceState>,
    BuilderState,
> Nested<wgpu::StencilState>
    for crate::builders::StencilStateBuilder<NestedField0, NestedField1, BuilderState>
where
    BuilderState: crate::builders::stencil_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::StencilState {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderPipelineDescriptor<'a>> for wgpu::RenderPipelineDescriptor<'a> {
    fn unnest(self) -> wgpu::RenderPipelineDescriptor<'a> {
        self
    }
}
impl<
    'a,
    NestedField2: Nested<wgpu::VertexState<'a>>,
    NestedField3: Nested<wgpu::PrimitiveState>,
    NestedField4: Nested<wgpu::DepthStencilState>,
    NestedField5: Nested<wgpu::MultisampleState>,
    NestedField6: Nested<wgpu::FragmentState<'a>>,
    BuilderState,
> Nested<wgpu::RenderPipelineDescriptor<'a>>
    for crate::builders::RenderPipelineDescriptorBuilder<
        'a,
        NestedField2,
        NestedField3,
        NestedField4,
        NestedField5,
        NestedField6,
        BuilderState,
    >
where
    BuilderState: crate::builders::render_pipeline_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderPipelineDescriptor<'a> {
        self.build()
    }
}

impl<S> Nested<wgpu::RequestAdapterOptionsBase<S>> for wgpu::RequestAdapterOptionsBase<S> {
    fn unnest(self) -> wgpu::RequestAdapterOptionsBase<S> {
        self
    }
}
impl<S, BuilderState> Nested<wgpu::RequestAdapterOptionsBase<S>>
    for crate::builders::RequestAdapterOptionsBaseBuilder<S, BuilderState>
where
    BuilderState: crate::builders::request_adapter_options_base_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RequestAdapterOptionsBase<S> {
        self.build()
    }
}

impl Nested<wgpu::TexelCopyBufferLayout> for wgpu::TexelCopyBufferLayout {
    fn unnest(self) -> wgpu::TexelCopyBufferLayout {
        self
    }
}
impl<BuilderState> Nested<wgpu::TexelCopyBufferLayout>
    for crate::builders::TexelCopyBufferLayoutBuilder<BuilderState>
where
    BuilderState: crate::builders::texel_copy_buffer_layout_builder::IsComplete,
{
    fn unnest(self) -> wgpu::TexelCopyBufferLayout {
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
impl<'a, NestedField2: Nested<wgpu::RenderBundleDepthStencil>, BuilderState>
    Nested<wgpu::RenderBundleEncoderDescriptor<'a>>
    for crate::builders::RenderBundleEncoderDescriptorBuilder<'a, NestedField2, BuilderState>
where
    BuilderState: crate::builders::render_bundle_encoder_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderBundleEncoderDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::RenderPassDescriptor<'a>> for wgpu::RenderPassDescriptor<'a> {
    fn unnest(self) -> wgpu::RenderPassDescriptor<'a> {
        self
    }
}
impl<
    'a,
    NestedField2: Nested<wgpu::RenderPassDepthStencilAttachment<'a>>,
    NestedField3: Nested<wgpu::RenderPassTimestampWrites<'a>>,
    BuilderState,
> Nested<wgpu::RenderPassDescriptor<'a>>
    for crate::builders::RenderPassDescriptorBuilder<'a, NestedField2, NestedField3, BuilderState>
where
    BuilderState: crate::builders::render_pass_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderPassDescriptor<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::VertexState<'a>> for wgpu::VertexState<'a> {
    fn unnest(self) -> wgpu::VertexState<'a> {
        self
    }
}
impl<'a, NestedField2: Nested<wgpu::PipelineCompilationOptions<'a>>, BuilderState>
    Nested<wgpu::VertexState<'a>>
    for crate::builders::VertexStateBuilder<'a, NestedField2, BuilderState>
where
    BuilderState: crate::builders::vertex_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::VertexState<'a> {
        self.build()
    }
}

impl Nested<wgpu::CompilationInfo> for wgpu::CompilationInfo {
    fn unnest(self) -> wgpu::CompilationInfo {
        self
    }
}
impl<BuilderState> Nested<wgpu::CompilationInfo>
    for crate::builders::CompilationInfoBuilder<BuilderState>
where
    BuilderState: crate::builders::compilation_info_builder::IsComplete,
{
    fn unnest(self) -> wgpu::CompilationInfo {
        self.build()
    }
}

impl Nested<wgpu::ShaderRuntimeChecks> for wgpu::ShaderRuntimeChecks {
    fn unnest(self) -> wgpu::ShaderRuntimeChecks {
        self
    }
}
impl<BuilderState> Nested<wgpu::ShaderRuntimeChecks>
    for crate::builders::ShaderRuntimeChecksBuilder<BuilderState>
where
    BuilderState: crate::builders::shader_runtime_checks_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ShaderRuntimeChecks {
        self.build()
    }
}

impl Nested<wgpu::BlendComponent> for wgpu::BlendComponent {
    fn unnest(self) -> wgpu::BlendComponent {
        self
    }
}
impl<BuilderState> Nested<wgpu::BlendComponent>
    for crate::builders::BlendComponentBuilder<BuilderState>
where
    BuilderState: crate::builders::blend_component_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BlendComponent {
        self.build()
    }
}

impl Nested<wgpu::PrimitiveState> for wgpu::PrimitiveState {
    fn unnest(self) -> wgpu::PrimitiveState {
        self
    }
}
impl<BuilderState> Nested<wgpu::PrimitiveState>
    for crate::builders::PrimitiveStateBuilder<BuilderState>
where
    BuilderState: crate::builders::primitive_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::PrimitiveState {
        self.build()
    }
}

impl<'a> Nested<wgpu::ComputePassTimestampWrites<'a>> for wgpu::ComputePassTimestampWrites<'a> {
    fn unnest(self) -> wgpu::ComputePassTimestampWrites<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::ComputePassTimestampWrites<'a>>
    for crate::builders::ComputePassTimestampWritesBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::compute_pass_timestamp_writes_builder::IsComplete,
{
    fn unnest(self) -> wgpu::ComputePassTimestampWrites<'a> {
        self.build()
    }
}

impl<T> Nested<wgpu::TexelCopyTextureInfoBase<T>> for wgpu::TexelCopyTextureInfoBase<T> {
    fn unnest(self) -> wgpu::TexelCopyTextureInfoBase<T> {
        self
    }
}
impl<T, NestedField2: Nested<wgpu::Origin3d>, BuilderState>
    Nested<wgpu::TexelCopyTextureInfoBase<T>>
    for crate::builders::TexelCopyTextureInfoBaseBuilder<T, NestedField2, BuilderState>
where
    BuilderState: crate::builders::texel_copy_texture_info_base_builder::IsComplete,
{
    fn unnest(self) -> wgpu::TexelCopyTextureInfoBase<T> {
        self.build()
    }
}

impl<'a> Nested<wgpu::PipelineCompilationOptions<'a>> for wgpu::PipelineCompilationOptions<'a> {
    fn unnest(self) -> wgpu::PipelineCompilationOptions<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::PipelineCompilationOptions<'a>>
    for crate::builders::PipelineCompilationOptionsBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::pipeline_compilation_options_builder::IsComplete,
{
    fn unnest(self) -> wgpu::PipelineCompilationOptions<'a> {
        self.build()
    }
}

impl Nested<wgpu::DepthBiasState> for wgpu::DepthBiasState {
    fn unnest(self) -> wgpu::DepthBiasState {
        self
    }
}
impl<BuilderState> Nested<wgpu::DepthBiasState>
    for crate::builders::DepthBiasStateBuilder<BuilderState>
where
    BuilderState: crate::builders::depth_bias_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::DepthBiasState {
        self.build()
    }
}

impl Nested<wgpu::CoreCounters> for wgpu::CoreCounters {
    fn unnest(self) -> wgpu::CoreCounters {
        self
    }
}
impl<BuilderState> Nested<wgpu::CoreCounters> for crate::builders::CoreCountersBuilder<BuilderState>
where
    BuilderState: crate::builders::core_counters_builder::IsComplete,
{
    fn unnest(self) -> wgpu::CoreCounters {
        self.build()
    }
}

impl Nested<wgpu::PushConstantRange> for wgpu::PushConstantRange {
    fn unnest(self) -> wgpu::PushConstantRange {
        self
    }
}
impl<BuilderState> Nested<wgpu::PushConstantRange>
    for crate::builders::PushConstantRangeBuilder<BuilderState>
where
    BuilderState: crate::builders::push_constant_range_builder::IsComplete,
{
    fn unnest(self) -> wgpu::PushConstantRange {
        self.build()
    }
}

impl<L: Default> Nested<wgpu::CommandBufferDescriptor<L: Default>>
    for wgpu::CommandBufferDescriptor<L: Default>
{
    fn unnest(self) -> wgpu::CommandBufferDescriptor<L: Default> {
        self
    }
}
impl<L: Default, BuilderState> Nested<wgpu::CommandBufferDescriptor<L: Default>>
    for crate::builders::CommandBufferDescriptorBuilder<L: Default, BuilderState>
where
    BuilderState: crate::builders::command_buffer_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::CommandBufferDescriptor<L: Default> {
        self.build()
    }
}

impl<'a> Nested<wgpu::TaskState<'a>> for wgpu::TaskState<'a> {
    fn unnest(self) -> wgpu::TaskState<'a> {
        self
    }
}
impl<'a, NestedField2: Nested<wgpu::PipelineCompilationOptions<'a>>, BuilderState>
    Nested<wgpu::TaskState<'a>>
    for crate::builders::TaskStateBuilder<'a, NestedField2, BuilderState>
where
    BuilderState: crate::builders::task_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::TaskState<'a> {
        self.build()
    }
}

impl<'tex> Nested<wgpu::RenderPassColorAttachment<'tex>> for wgpu::RenderPassColorAttachment<'tex> {
    fn unnest(self) -> wgpu::RenderPassColorAttachment<'tex> {
        self
    }
}
impl<'tex, NestedField3: Nested<wgpu::Operations<wgpu::Color>>, BuilderState>
    Nested<wgpu::RenderPassColorAttachment<'tex>>
    for crate::builders::RenderPassColorAttachmentBuilder<'tex, NestedField3, BuilderState>
where
    BuilderState: crate::builders::render_pass_color_attachment_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderPassColorAttachment<'tex> {
        self.build()
    }
}

impl Nested<wgpu::InstanceDescriptor> for wgpu::InstanceDescriptor {
    fn unnest(self) -> wgpu::InstanceDescriptor {
        self
    }
}
impl<
    NestedField2: Nested<wgpu::MemoryBudgetThresholds>,
    NestedField3: Nested<wgpu::BackendOptions>,
    BuilderState,
> Nested<wgpu::InstanceDescriptor>
    for crate::builders::InstanceDescriptorBuilder<NestedField2, NestedField3, BuilderState>
where
    BuilderState: crate::builders::instance_descriptor_builder::IsComplete,
{
    fn unnest(self) -> wgpu::InstanceDescriptor {
        self.build()
    }
}

impl Nested<wgpu::Extent3d> for wgpu::Extent3d {
    fn unnest(self) -> wgpu::Extent3d {
        self
    }
}
impl<BuilderState> Nested<wgpu::Extent3d> for crate::builders::Extent3DBuilder<BuilderState>
where
    BuilderState: crate::builders::extent3_d_builder::IsComplete,
{
    fn unnest(self) -> wgpu::Extent3d {
        self.build()
    }
}

impl<'a> Nested<wgpu::BlasTriangleGeometry<'a>> for wgpu::BlasTriangleGeometry<'a> {
    fn unnest(self) -> wgpu::BlasTriangleGeometry<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::BlasTriangleGeometry<'a>>
    for crate::builders::BlasTriangleGeometryBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::blas_triangle_geometry_builder::IsComplete,
{
    fn unnest(self) -> wgpu::BlasTriangleGeometry<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::VertexBufferLayout<'a>> for wgpu::VertexBufferLayout<'a> {
    fn unnest(self) -> wgpu::VertexBufferLayout<'a> {
        self
    }
}
impl<'a, BuilderState> Nested<wgpu::VertexBufferLayout<'a>>
    for crate::builders::VertexBufferLayoutBuilder<'a, BuilderState>
where
    BuilderState: crate::builders::vertex_buffer_layout_builder::IsComplete,
{
    fn unnest(self) -> wgpu::VertexBufferLayout<'a> {
        self.build()
    }
}

impl<'a> Nested<wgpu::FragmentState<'a>> for wgpu::FragmentState<'a> {
    fn unnest(self) -> wgpu::FragmentState<'a> {
        self
    }
}
impl<'a, NestedField2: Nested<wgpu::PipelineCompilationOptions<'a>>, BuilderState>
    Nested<wgpu::FragmentState<'a>>
    for crate::builders::FragmentStateBuilder<'a, NestedField2, BuilderState>
where
    BuilderState: crate::builders::fragment_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::FragmentState<'a> {
        self.build()
    }
}

impl Nested<wgpu::MultisampleState> for wgpu::MultisampleState {
    fn unnest(self) -> wgpu::MultisampleState {
        self
    }
}
impl<BuilderState> Nested<wgpu::MultisampleState>
    for crate::builders::MultisampleStateBuilder<BuilderState>
where
    BuilderState: crate::builders::multisample_state_builder::IsComplete,
{
    fn unnest(self) -> wgpu::MultisampleState {
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
impl<
    'tex,
    NestedField1: Nested<wgpu::Operations<f32>>,
    NestedField2: Nested<wgpu::Operations<u32>>,
    BuilderState,
> Nested<wgpu::RenderPassDepthStencilAttachment<'tex>>
    for crate::builders::RenderPassDepthStencilAttachmentBuilder<
        'tex,
        NestedField1,
        NestedField2,
        BuilderState,
    >
where
    BuilderState: crate::builders::render_pass_depth_stencil_attachment_builder::IsComplete,
{
    fn unnest(self) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
        self.build()
    }
}

impl Nested<wgpu::Dx12BackendOptions> for wgpu::Dx12BackendOptions {
    fn unnest(self) -> wgpu::Dx12BackendOptions {
        self
    }
}
impl<BuilderState> Nested<wgpu::Dx12BackendOptions>
    for crate::builders::Dx12BackendOptionsBuilder<BuilderState>
where
    BuilderState: crate::builders::dx12_backend_options_builder::IsComplete,
{
    fn unnest(self) -> wgpu::Dx12BackendOptions {
        self.build()
    }
}
