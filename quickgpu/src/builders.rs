use bon::builder;
use wgpu::*;

#[builder(state_mod(vis = "pub(crate)"))]
pub fn adapter_info(
    #[builder(default)] name: String,
    #[builder(default)] vendor: u32,
    #[builder(default)] device: u32,

    device_type: DeviceType,
    #[builder(default)] driver: String,
    #[builder(default)] driver_info: String,

    backend: Backend,
) -> AdapterInfo {
    AdapterInfo {
        name,

        vendor,

        device,

        device_type,

        driver,

        driver_info,

        backend,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn allocator_report(
    #[builder(default)] allocations: Vec<wgt::AllocationReport>,
    #[builder(default)] blocks: Vec<wgt::MemoryBlockReport>,
    #[builder(default)] total_allocated_bytes: u64,
    #[builder(default)] total_reserved_bytes: u64,
) -> AllocatorReport {
    AllocatorReport {
        allocations,

        blocks,

        total_allocated_bytes,

        total_reserved_bytes,
    }
}

/*
    #[derive(Clone, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn backend_options(
    #[builder(default)] gl: GlBackendOptions,
    #[builder(default)] dx12: Dx12BackendOptions,
    #[builder(default)] noop: NoopBackendOptions,
) -> BackendOptions {
    BackendOptions { gl, dx12, noop }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_descriptor<'a>(
    label: Option<&'a str>,

    layout: &'a BindGroupLayout,

    entries: &'a [BindGroupEntry<'a>],
) -> BindGroupDescriptor<'a> {
    BindGroupDescriptor {
        label,

        layout,

        entries,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_entry<'a>(
    #[builder(default)] binding: u32,

    resource: BindingResource<'a>,
) -> BindGroupEntry<'a> {
    BindGroupEntry { binding, resource }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_layout_descriptor<'a>(
    label: Option<&'a str>,

    entries: &'a [BindGroupLayoutEntry],
) -> BindGroupLayoutDescriptor<'a> {
    BindGroupLayoutDescriptor { label, entries }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_layout_entry(
    #[builder(default)] binding: u32,

    visibility: ShaderStages,

    ty: BindingType,

    count: Option<std::num::NonZero<u32>>,
) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,

        visibility,

        ty,

        count,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn blas_build_entry<'a>(blas: &'a Blas, geometry: BlasGeometries<'a>) -> BlasBuildEntry<'a> {
    BlasBuildEntry { blas, geometry }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn blas_triangle_geometry<'a>(
    size: &'a wgt::BlasTriangleGeometrySizeDescriptor,

    vertex_buffer: &'a Buffer,
    #[builder(default)] first_vertex: u32,
    #[builder(default)] vertex_stride: u64,

    index_buffer: Option<&'a Buffer>,

    first_index: Option<u32>,

    transform_buffer: Option<&'a Buffer>,

    transform_buffer_offset: Option<u64>,
) -> BlasTriangleGeometry<'a> {
    BlasTriangleGeometry {
        size,

        vertex_buffer,

        first_vertex,

        vertex_stride,

        index_buffer,

        first_index,

        transform_buffer,

        transform_buffer_offset,
    }
}

/*
    impl Default for BlendComponent {
    fn default() -> Self {
        Self::REPLACE
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn blend_component(
    src_factor: BlendFactor,

    dst_factor: BlendFactor,
    #[builder(default)] operation: BlendOperation,
) -> BlendComponent {
    BlendComponent {
        src_factor,

        dst_factor,

        operation,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn blend_state(
    #[builder(default)] color: BlendComponent,
    #[builder(default)] alpha: BlendComponent,
) -> BlendState {
    BlendState { color, alpha }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_binding<'a>(
    buffer: &'a Buffer,
    #[builder(default)] offset: u64,

    size: Option<std::num::NonZero<u64>>,
) -> BufferBinding<'a> {
    BufferBinding {
        buffer,

        offset,

        size,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_texture_copy_info(
    #[builder(default)] copy_width: u64,
    #[builder(default)] copy_height: u64,
    #[builder(default)] depth_or_array_layers: u64,
    #[builder(default)] offset: u64,
    #[builder(default)] block_size_bytes: u64,
    #[builder(default)] block_width_texels: u64,
    #[builder(default)] block_height_texels: u64,
    #[builder(default)] width_blocks: u64,
    #[builder(default)] height_blocks: u64,
    #[builder(default)] row_bytes_dense: u64,
    #[builder(default)] row_stride_bytes: u64,
    #[builder(default)] image_stride_rows: u64,
    #[builder(default)] image_stride_bytes: u64,
    #[builder(default)] image_rows_dense: u64,
    #[builder(default)] image_bytes_dense: u64,
    #[builder(default)] bytes_in_copy: u64,
) -> BufferTextureCopyInfo {
    BufferTextureCopyInfo {
        copy_width,

        copy_height,

        depth_or_array_layers,

        offset,

        block_size_bytes,

        block_width_texels,

        block_height_texels,

        width_blocks,

        height_blocks,

        row_bytes_dense,

        row_stride_bytes,

        image_stride_rows,

        image_stride_bytes,

        image_rows_dense,

        image_bytes_dense,

        bytes_in_copy,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_transition<T>(buffer: T, state: BufferUses) -> BufferTransition<T> {
    BufferTransition { buffer, state }
}

/*
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn color(
    #[builder(default)] r: f64,
    #[builder(default)] g: f64,
    #[builder(default)] b: f64,
    #[builder(default)] a: f64,
) -> Color {
    Color { r, g, b, a }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn color_target_state(
    format: TextureFormat,

    blend: Option<BlendState>,
    #[builder(default)] write_mask: ColorWrites,
) -> ColorTargetState {
    ColorTargetState {
        format,

        blend,

        write_mask,
    }
}

/*
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn command_buffer_descriptor<L>(label: L) -> CommandBufferDescriptor<L> {
    CommandBufferDescriptor { label }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn compilation_info(#[builder(default)] messages: Vec<CompilationMessage>) -> CompilationInfo {
    CompilationInfo { messages }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn compilation_message(
    #[builder(default)] message: String,

    message_type: CompilationMessageType,

    location: Option<SourceLocation>,
) -> CompilationMessage {
    CompilationMessage {
        message,

        message_type,

        location,
    }
}

/*
    #[derive(Clone, Default, Debug)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn compute_pass_descriptor<'a>(
    label: Option<&'a str>,

    timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
) -> ComputePassDescriptor<'a> {
    ComputePassDescriptor {
        label,

        timestamp_writes,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn compute_pass_timestamp_writes<'a>(
    query_set: &'a QuerySet,

    beginning_of_pass_write_index: Option<u32>,

    end_of_pass_write_index: Option<u32>,
) -> ComputePassTimestampWrites<'a> {
    ComputePassTimestampWrites {
        query_set,

        beginning_of_pass_write_index,

        end_of_pass_write_index,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn compute_pipeline_descriptor<'a>(
    label: Option<&'a str>,

    layout: Option<&'a PipelineLayout>,

    module: &'a ShaderModule,

    entry_point: Option<&'a str>,
    #[builder(default)] compilation_options: PipelineCompilationOptions<'a>,

    cache: Option<&'a PipelineCache>,
) -> ComputePipelineDescriptor<'a> {
    ComputePipelineDescriptor {
        label,

        layout,

        module,

        entry_point,

        compilation_options,

        cache,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn copy_external_image_dest_info<T>(
    texture: T,
    #[builder(default)] mip_level: u32,
    #[builder(default)] origin: Origin3d,
    #[builder(default)] aspect: TextureAspect,

    color_space: PredefinedColorSpace,
    #[builder(default)] premultiplied_alpha: bool,
) -> CopyExternalImageDestInfo<T> {
    CopyExternalImageDestInfo {
        texture,

        mip_level,

        origin,

        aspect,

        color_space,

        premultiplied_alpha,
    }
}

/*
    #[derive(Clone, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn core_counters() -> CoreCounters {
    CoreCounters {}
}

/*
    #[derive(Clone, Copy, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn depth_bias_state(
    #[builder(default)] constant: i32,
    #[builder(default)] slope_scale: f32,
    #[builder(default)] clamp: f32,
) -> DepthBiasState {
    DepthBiasState {
        constant,

        slope_scale,

        clamp,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn depth_stencil_state(
    format: TextureFormat,
    #[builder(default)] depth_write_enabled: bool,

    depth_compare: CompareFunction,
    #[builder(default)] stencil: StencilState,
    #[builder(default)] bias: DepthBiasState,
) -> DepthStencilState {
    DepthStencilState {
        format,

        depth_write_enabled,

        depth_compare,

        stencil,

        bias,
    }
}

/*
    impl Default for DownlevelCapabilities {
    fn default() -> Self {
        Self {
            flags: DownlevelFlags::all(),
            limits: DownlevelLimits::default(),
            shader_model: ShaderModel::Sm5,
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn downlevel_capabilities(
    flags: DownlevelFlags,
    #[builder(default)] limits: DownlevelLimits,

    shader_model: ShaderModel,
) -> DownlevelCapabilities {
    DownlevelCapabilities {
        flags,

        limits,

        shader_model,
    }
}

/*
    #[allow(clippy::derivable_impls)]
impl Default for DownlevelLimits {
    fn default() -> Self {
        DownlevelLimits {}
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn downlevel_limits() -> DownlevelLimits {
    DownlevelLimits {}
}

/*
    #[derive(Clone, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn dx_12_backend_options(
    #[builder(default)] shader_compiler: Dx12Compiler,
) -> Dx12BackendOptions {
    Dx12BackendOptions { shader_compiler }
}

/*
    impl Default for Extent3d {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn extent_3_d(
    #[builder(default)] width: u32,
    #[builder(default)] height: u32,
    #[builder(default)] depth_or_array_layers: u32,
) -> Extent3d {
    Extent3d {
        width,

        height,

        depth_or_array_layers,
    }
}

/*
    #[derive(Default,Debug,Copy,Clone,PartialEq,Eq,Hash)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn features(
    #[builder(default)] features_wgpu: FeaturesWGPU,
    #[builder(default)] features_webgpu: FeaturesWebGPU,
) -> Features {
    Features {
        features_wgpu,

        features_webgpu,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn fragment_state<'a>(
    module: &'a ShaderModule,

    entry_point: Option<&'a str>,
    #[builder(default)] compilation_options: PipelineCompilationOptions<'a>,

    targets: &'a [Option<ColorTargetState>],
) -> FragmentState<'a> {
    FragmentState {
        module,

        entry_point,

        compilation_options,

        targets,
    }
}

/*
    #[derive(Clone, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn gl_backend_options(
    #[builder(default)] gles_minor_version: Gles3MinorVersion,
    #[builder(default)] fence_behavior: GlFenceBehavior,
) -> GlBackendOptions {
    GlBackendOptions {
        gles_minor_version,

        fence_behavior,
    }
}

/*
    #[derive(Clone, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn hal_counters(
    #[builder(default)] buffers: wgt::InternalCounter,
    #[builder(default)] textures: wgt::InternalCounter,
    #[builder(default)] texture_views: wgt::InternalCounter,
    #[builder(default)] bind_groups: wgt::InternalCounter,
    #[builder(default)] bind_group_layouts: wgt::InternalCounter,
    #[builder(default)] render_pipelines: wgt::InternalCounter,
    #[builder(default)] compute_pipelines: wgt::InternalCounter,
    #[builder(default)] pipeline_layouts: wgt::InternalCounter,
    #[builder(default)] samplers: wgt::InternalCounter,
    #[builder(default)] command_encoders: wgt::InternalCounter,
    #[builder(default)] shader_modules: wgt::InternalCounter,
    #[builder(default)] query_sets: wgt::InternalCounter,
    #[builder(default)] fences: wgt::InternalCounter,
    #[builder(default)] buffer_memory: wgt::InternalCounter,
    #[builder(default)] texture_memory: wgt::InternalCounter,
    #[builder(default)] acceleration_structure_memory: wgt::InternalCounter,
    #[builder(default)] memory_allocations: wgt::InternalCounter,
) -> HalCounters {
    HalCounters {
        buffers,

        textures,

        texture_views,

        bind_groups,

        bind_group_layouts,

        render_pipelines,

        compute_pipelines,

        pipeline_layouts,

        samplers,

        command_encoders,

        shader_modules,

        query_sets,

        fences,

        buffer_memory,

        texture_memory,

        acceleration_structure_memory,

        memory_allocations,
    }
}

/*
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn image_subresource_range(
    #[builder(default)] aspect: TextureAspect,
    #[builder(default)] base_mip_level: u32,

    mip_level_count: Option<u32>,
    #[builder(default)] base_array_layer: u32,

    array_layer_count: Option<u32>,
) -> ImageSubresourceRange {
    ImageSubresourceRange {
        aspect,

        base_mip_level,

        mip_level_count,

        base_array_layer,

        array_layer_count,
    }
}

/*
    impl Default for InstanceDescriptor {
    fn default() -> Self {
        Self {
            backends: Backends::all(),
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn instance_descriptor(
    #[builder(default)] backends: Backends,
    #[builder(default)] flags: InstanceFlags,
    #[builder(default)] memory_budget_thresholds: MemoryBudgetThresholds,
    #[builder(default)] backend_options: BackendOptions,
) -> InstanceDescriptor {
    InstanceDescriptor {
        backends,

        flags,

        memory_budget_thresholds,

        backend_options,
    }
}

/*
    #[derive(Clone, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn internal_counters(
    #[builder(default)] core: CoreCounters,
    #[builder(default)] hal: HalCounters,
) -> InternalCounters {
    InternalCounters { core, hal }
}

/*
    impl Default for Limits {
    fn default() -> Self {
        Self::defaults()
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn limits(
    #[builder(default)] max_texture_dimension_1d: u32,
    #[builder(default)] max_texture_dimension_2d: u32,
    #[builder(default)] max_texture_dimension_3d: u32,
    #[builder(default)] max_texture_array_layers: u32,
    #[builder(default)] max_bind_groups: u32,
    #[builder(default)] max_bindings_per_bind_group: u32,
    #[builder(default)] max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    #[builder(default)] max_dynamic_storage_buffers_per_pipeline_layout: u32,
    #[builder(default)] max_sampled_textures_per_shader_stage: u32,
    #[builder(default)] max_samplers_per_shader_stage: u32,
    #[builder(default)] max_storage_buffers_per_shader_stage: u32,
    #[builder(default)] max_storage_textures_per_shader_stage: u32,
    #[builder(default)] max_uniform_buffers_per_shader_stage: u32,
    #[builder(default)] max_binding_array_elements_per_shader_stage: u32,
    #[builder(default)] max_binding_array_sampler_elements_per_shader_stage: u32,
    #[builder(default)] max_uniform_buffer_binding_size: u32,
    #[builder(default)] max_storage_buffer_binding_size: u32,
    #[builder(default)] max_vertex_buffers: u32,
    #[builder(default)] max_buffer_size: u64,
    #[builder(default)] max_vertex_attributes: u32,
    #[builder(default)] max_vertex_buffer_array_stride: u32,
    #[builder(default)] min_uniform_buffer_offset_alignment: u32,
    #[builder(default)] min_storage_buffer_offset_alignment: u32,
    #[builder(default)] max_inter_stage_shader_components: u32,
    #[builder(default)] max_color_attachments: u32,
    #[builder(default)] max_color_attachment_bytes_per_sample: u32,
    #[builder(default)] max_compute_workgroup_storage_size: u32,
    #[builder(default)] max_compute_invocations_per_workgroup: u32,
    #[builder(default)] max_compute_workgroup_size_x: u32,
    #[builder(default)] max_compute_workgroup_size_y: u32,
    #[builder(default)] max_compute_workgroup_size_z: u32,
    #[builder(default)] max_compute_workgroups_per_dimension: u32,
    #[builder(default)] min_subgroup_size: u32,
    #[builder(default)] max_subgroup_size: u32,
    #[builder(default)] max_push_constant_size: u32,
    #[builder(default)] max_non_sampler_bindings: u32,
    #[builder(default)] max_blas_primitive_count: u32,
    #[builder(default)] max_blas_geometry_count: u32,
    #[builder(default)] max_tlas_instance_count: u32,
    #[builder(default)] max_acceleration_structures_per_shader_stage: u32,
) -> Limits {
    Limits {
        max_texture_dimension_1d,

        max_texture_dimension_2d,

        max_texture_dimension_3d,

        max_texture_array_layers,

        max_bind_groups,

        max_bindings_per_bind_group,

        max_dynamic_uniform_buffers_per_pipeline_layout,

        max_dynamic_storage_buffers_per_pipeline_layout,

        max_sampled_textures_per_shader_stage,

        max_samplers_per_shader_stage,

        max_storage_buffers_per_shader_stage,

        max_storage_textures_per_shader_stage,

        max_uniform_buffers_per_shader_stage,

        max_binding_array_elements_per_shader_stage,

        max_binding_array_sampler_elements_per_shader_stage,

        max_uniform_buffer_binding_size,

        max_storage_buffer_binding_size,

        max_vertex_buffers,

        max_buffer_size,

        max_vertex_attributes,

        max_vertex_buffer_array_stride,

        min_uniform_buffer_offset_alignment,

        min_storage_buffer_offset_alignment,

        max_inter_stage_shader_components,

        max_color_attachments,

        max_color_attachment_bytes_per_sample,

        max_compute_workgroup_storage_size,

        max_compute_invocations_per_workgroup,

        max_compute_workgroup_size_x,

        max_compute_workgroup_size_y,

        max_compute_workgroup_size_z,

        max_compute_workgroups_per_dimension,

        min_subgroup_size,

        max_subgroup_size,

        max_push_constant_size,

        max_non_sampler_bindings,

        max_blas_primitive_count,

        max_blas_geometry_count,

        max_tlas_instance_count,

        max_acceleration_structures_per_shader_stage,
    }
}

/*
    #[derive(Default, Clone, Debug, Copy)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn memory_budget_thresholds(
    for_resource_creation: Option<u8>,

    for_device_loss: Option<u8>,
) -> MemoryBudgetThresholds {
    MemoryBudgetThresholds {
        for_resource_creation,

        for_device_loss,
    }
}

/*
    impl Default for MultisampleState {
    fn default() -> Self {
        MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn multisample_state(
    #[builder(default)] count: u32,
    #[builder(default)] mask: u64,
    #[builder(default)] alpha_to_coverage_enabled: bool,
) -> MultisampleState {
    MultisampleState {
        count,

        mask,

        alpha_to_coverage_enabled,
    }
}

/*
    #[derive(Clone, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn noop_backend_options(#[builder(default)] enable: bool) -> NoopBackendOptions {
    NoopBackendOptions { enable }
}

/*
    impl<V: Default> Default for Operations<V> {
    #[inline]
    fn default() -> Self {
        Self {
            load: LoadOp::<V>::default(),
            store: StoreOp::default(),
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn operations<V>(load: LoadOp<V>, #[builder(default, into)] store: StoreOp) -> Operations<V> {
    Operations { load, store }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn origin_2_d(#[builder(default)] x: u32, #[builder(default)] y: u32) -> Origin2d {
    Origin2d { x, y }
}

/*
    impl Default for Origin3d {
    fn default() -> Self {
        Self::ZERO
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn origin_3_d(
    #[builder(default)] x: u32,
    #[builder(default)] y: u32,
    #[builder(default)] z: u32,
) -> Origin3d {
    Origin3d { x, y, z }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn pipeline_cache_descriptor<'a>(
    label: Option<&'a str>,

    data: Option<&'a [u8]>,
    #[builder(default)] fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    PipelineCacheDescriptor {
        label,

        data,

        fallback,
    }
}

/*
    impl Default for PipelineCompilationOptions<'_> {
    fn default() -> Self {
        Self {
            constants: Default::default(),
            zero_initialize_workgroup_memory: true,
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn pipeline_compilation_options<'a>(
    constants: &'a [(&'a str, f64)],
    #[builder(default)] zero_initialize_workgroup_memory: bool,
) -> PipelineCompilationOptions<'a> {
    PipelineCompilationOptions {
        constants,

        zero_initialize_workgroup_memory,
    }
}

/*
    #[derive(Clone, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn pipeline_layout_descriptor<'a>(
    label: Option<&'a str>,

    bind_group_layouts: &'a [&'a BindGroupLayout],

    push_constant_ranges: &'a [PushConstantRange],
) -> PipelineLayoutDescriptor<'a> {
    PipelineLayoutDescriptor {
        label,

        bind_group_layouts,

        push_constant_ranges,
    }
}

/*
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn primitive_state(
    #[builder(default)] topology: PrimitiveTopology,

    strip_index_format: Option<IndexFormat>,
    #[builder(default)] front_face: FrontFace,

    cull_mode: Option<Face>,
    #[builder(default)] unclipped_depth: bool,
    #[builder(default)] polygon_mode: PolygonMode,
    #[builder(default)] conservative: bool,
) -> PrimitiveState {
    PrimitiveState {
        topology,

        strip_index_format,

        front_face,

        cull_mode,

        unclipped_depth,

        polygon_mode,

        conservative,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn push_constant_range(
    stages: ShaderStages,
    #[builder(default)] range: std::ops::Range<u32>,
) -> PushConstantRange {
    PushConstantRange { stages, range }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_bundle_depth_stencil(
    format: TextureFormat,
    #[builder(default)] depth_read_only: bool,
    #[builder(default)] stencil_read_only: bool,
) -> RenderBundleDepthStencil {
    RenderBundleDepthStencil {
        format,

        depth_read_only,

        stencil_read_only,
    }
}

/*
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_bundle_encoder_descriptor<'a>(
    label: Option<&'a str>,

    color_formats: &'a [Option<TextureFormat>],

    depth_stencil: Option<RenderBundleDepthStencil>,
    #[builder(default)] sample_count: u32,

    multiview: Option<std::num::NonZero<u32>>,
) -> RenderBundleEncoderDescriptor<'a> {
    RenderBundleEncoderDescriptor {
        label,

        color_formats,

        depth_stencil,

        sample_count,

        multiview,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_color_attachment<'tex>(
    view: &'tex TextureView,

    depth_slice: Option<u32>,

    resolve_target: Option<&'tex TextureView>,
    #[builder(default)] ops: Operations<Color>,
) -> RenderPassColorAttachment<'tex> {
    RenderPassColorAttachment {
        view,

        depth_slice,

        resolve_target,

        ops,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_depth_stencil_attachment<'tex>(
    view: &'tex TextureView,

    depth_ops: Option<Operations<f32>>,

    stencil_ops: Option<Operations<u32>>,
) -> RenderPassDepthStencilAttachment<'tex> {
    RenderPassDepthStencilAttachment {
        view,

        depth_ops,

        stencil_ops,
    }
}

/*
    #[derive(Clone, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_descriptor<'a>(
    label: Option<&'a str>,

    color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],

    depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,

    timestamp_writes: Option<RenderPassTimestampWrites<'a>>,

    occlusion_query_set: Option<&'a QuerySet>,
) -> RenderPassDescriptor<'a> {
    RenderPassDescriptor {
        label,

        color_attachments,

        depth_stencil_attachment,

        timestamp_writes,

        occlusion_query_set,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_timestamp_writes<'a>(
    query_set: &'a QuerySet,

    beginning_of_pass_write_index: Option<u32>,

    end_of_pass_write_index: Option<u32>,
) -> RenderPassTimestampWrites<'a> {
    RenderPassTimestampWrites {
        query_set,

        beginning_of_pass_write_index,

        end_of_pass_write_index,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn render_pipeline_descriptor<'a>(
    label: Option<&'a str>,

    layout: Option<&'a PipelineLayout>,

    vertex: VertexState<'a>,
    #[builder(default)] primitive: PrimitiveState,

    depth_stencil: Option<DepthStencilState>,
    #[builder(default)] multisample: MultisampleState,

    fragment: Option<FragmentState<'a>>,

    multiview: Option<std::num::NonZero<u32>>,

    cache: Option<&'a PipelineCache>,
) -> RenderPipelineDescriptor<'a> {
    RenderPipelineDescriptor {
        label,

        layout,

        vertex,

        primitive,

        depth_stencil,

        multisample,

        fragment,

        multiview,

        cache,
    }
}

/*
    impl<S> Default for RequestAdapterOptions<S> {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn request_adapter_options_base<S>(
    #[builder(default)] power_preference: PowerPreference,
    #[builder(default)] force_fallback_adapter: bool,

    compatible_surface: Option<S>,
) -> RequestAdapterOptionsBase<S> {
    RequestAdapterOptionsBase {
        power_preference,

        force_fallback_adapter,

        compatible_surface,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn shader_module_descriptor<'a>(
    label: Option<&'a str>,

    source: ShaderSource<'a>,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor { label, source }
}

/*
    impl Default for ShaderRuntimeChecks {
    fn default() -> Self {
        Self::checked()
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn shader_runtime_checks(
    #[builder(default)] bounds_checks: bool,
    #[builder(default)] force_loop_bounding: bool,
) -> ShaderRuntimeChecks {
    ShaderRuntimeChecks {
        bounds_checks,

        force_loop_bounding,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn source_location(
    #[builder(default)] line_number: u32,
    #[builder(default)] line_position: u32,
    #[builder(default)] offset: u32,
    #[builder(default)] length: u32,
) -> SourceLocation {
    SourceLocation {
        line_number,

        line_position,

        offset,

        length,
    }
}

/*
    impl Default for StencilFaceState {
    fn default() -> Self {
        Self::IGNORE
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn stencil_face_state(
    compare: CompareFunction,
    #[builder(default)] fail_op: StencilOperation,
    #[builder(default)] depth_fail_op: StencilOperation,
    #[builder(default)] pass_op: StencilOperation,
) -> StencilFaceState {
    StencilFaceState {
        compare,

        fail_op,

        depth_fail_op,

        pass_op,
    }
}

/*
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn stencil_state(
    #[builder(default)] front: StencilFaceState,
    #[builder(default)] back: StencilFaceState,
    #[builder(default)] read_mask: u32,
    #[builder(default)] write_mask: u32,
) -> StencilState {
    StencilState {
        front,

        back,

        read_mask,

        write_mask,
    }
}

/*
    impl Default for SurfaceCapabilities {
    fn default() -> Self {
        Self {
            formats: Vec::new(),
            present_modes: Vec::new(),
            alpha_modes: vec![CompositeAlphaMode::Opaque],
            usages: TextureUsages::RENDER_ATTACHMENT,
        }
    }
}
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn surface_capabilities(
    #[builder(default)] formats: Vec<TextureFormat>,
    #[builder(default)] present_modes: Vec<PresentMode>,
    #[builder(default)] alpha_modes: Vec<CompositeAlphaMode>,

    usages: TextureUsages,
) -> SurfaceCapabilities {
    SurfaceCapabilities {
        formats,

        present_modes,

        alpha_modes,

        usages,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn texel_copy_buffer_info_base<B>(
    buffer: B,
    #[builder(default)] layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfoBase<B> {
    TexelCopyBufferInfoBase { buffer, layout }
}

/*
    #[derive(Clone, Copy, Debug, Default)]
*/
#[builder(state_mod(vis = "pub(crate)"))]
pub fn texel_copy_buffer_layout(
    #[builder(default)] offset: u64,

    bytes_per_row: Option<u32>,

    rows_per_image: Option<u32>,
) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset,

        bytes_per_row,

        rows_per_image,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn texel_copy_texture_info_base<T>(
    texture: T,
    #[builder(default)] mip_level: u32,
    #[builder(default)] origin: Origin3d,
    #[builder(default)] aspect: TextureAspect,
) -> TexelCopyTextureInfoBase<T> {
    TexelCopyTextureInfoBase {
        texture,

        mip_level,

        origin,

        aspect,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn texture_format_features(
    allowed_usages: TextureUsages,

    flags: TextureFormatFeatureFlags,
) -> TextureFormatFeatures {
    TextureFormatFeatures {
        allowed_usages,

        flags,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn texture_transition<T>(
    texture: T,

    selector: Option<wgt::TextureSelector>,

    state: TextureUses,
) -> TextureTransition<T> {
    TextureTransition {
        texture,

        selector,

        state,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn vertex_attribute(
    format: VertexFormat,
    #[builder(default)] offset: u64,
    #[builder(default)] shader_location: u32,
) -> VertexAttribute {
    VertexAttribute {
        format,

        offset,

        shader_location,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn vertex_buffer_layout<'a>(
    #[builder(default)] array_stride: u64,
    #[builder(default)] step_mode: VertexStepMode,

    attributes: &'a [VertexAttribute],
) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        array_stride,

        step_mode,

        attributes,
    }
}

#[builder(state_mod(vis = "pub(crate)"))]
pub fn vertex_state<'a>(
    module: &'a ShaderModule,

    entry_point: Option<&'a str>,
    #[builder(default)] compilation_options: PipelineCompilationOptions<'a>,

    buffers: &'a [VertexBufferLayout<'a>],
) -> VertexState<'a> {
    VertexState {
        module,

        entry_point,

        compilation_options,

        buffers,
    }
}
