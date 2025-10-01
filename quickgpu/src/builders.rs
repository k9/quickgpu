use wgpu::util::*;
use wgpu::*;

/*
Default from: wgpu-types/src/lib.rs:7415
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn dispatch_indirect_args(x: u64, y: u64, z: u64) -> DispatchIndirectArgs {
    DispatchIndirectArgs { x, y, z }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_layout_entry(
    binding: u64,
    visibility: u64,
    ty: u64,
    count: u64,
) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn depth_stencil_state(
    format: u64,
    depth_write_enabled: u64,
    depth_compare: u64,
    #[builder(default)] stencil: u64,
    #[builder(default)] bias: u64,
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
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn copy_external_image_dest_info(
    texture: u64,
    mip_level: u64,
    #[builder(default)] origin: u64,
    #[builder(default)] aspect: u64,
    color_space: u64,
    premultiplied_alpha: u64,
) -> CopyExternalImageDestInfo {
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
Default from: wgpu-types/src/lib.rs:1846
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn multisample_state(
    count: u64,
    mask: u64,
    alpha_to_coverage_enabled: u64,
) -> MultisampleState {
    MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
    }
}

/*
Default from: wgpu-types/src/lib.rs:7498
impl Default for ShaderRuntimeChecks {
    fn default() -> Self {
        Self::checked()
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn shader_runtime_checks(bounds_checks: u64, force_loop_bounding: u64) -> ShaderRuntimeChecks {
    ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

/*
Default from: wgpu-types/src/lib.rs:5515
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn surface_capabilities(
    formats: u64,
    present_modes: u64,
    alpha_modes: u64,
    usages: u64,
) -> SurfaceCapabilities {
    SurfaceCapabilities {
        formats,
        present_modes,
        alpha_modes,
        usages,
    }
}

/*
Default from: wgpu-types/src/lib.rs:7365
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn draw_indirect_args(
    vertex_count: u64,
    instance_count: u64,
    first_vertex: u64,
    first_instance: u64,
) -> DrawIndirectArgs {
    DrawIndirectArgs {
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    }
}

/*
Default from: wgpu-types/src/lib.rs:5809
impl Default for Origin3d {
    fn default() -> Self {
        Self::ZERO
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn origin_3_d(x: u64, y: u64, z: u64) -> Origin3d {
    Origin3d { x, y, z }
}

/*
Default from: wgpu-types/src/lib.rs:5669
#[derive(Clone, Copy, Debug, Default, PartialEq)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn color(r: u64, g: u64, b: u64, a: u64) -> Color {
    Color { r, g, b, a }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn texture_format_features(allowed_usages: u64, flags: u64) -> TextureFormatFeatures {
    TextureFormatFeatures {
        allowed_usages,
        flags,
    }
}

/*
Default from: wgpu-types/src/lib.rs:7389
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn draw_indexed_indirect_args(
    index_count: u64,
    instance_count: u64,
    first_index: u64,
    base_vertex: u64,
    first_instance: u64,
) -> DrawIndexedIndirectArgs {
    DrawIndexedIndirectArgs {
        index_count,
        instance_count,
        first_index,
        base_vertex,
        first_instance,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_texture_copy_info(
    copy_width: u64,
    copy_height: u64,
    depth_or_array_layers: u64,
    offset: u64,
    block_size_bytes: u64,
    block_width_texels: u64,
    block_height_texels: u64,
    width_blocks: u64,
    height_blocks: u64,
    row_bytes_dense: u64,
    row_stride_bytes: u64,
    image_stride_rows: u64,
    image_stride_bytes: u64,
    image_rows_dense: u64,
    image_bytes_dense: u64,
    bytes_in_copy: u64,
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

/*
Default from: wgpu-types/src/counters.rs:107
#[derive(Clone, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn hal_counters(
    #[builder(default)] buffers: u64,
    #[builder(default)] textures: u64,
    #[builder(default)] texture_views: u64,
    #[builder(default)] bind_groups: u64,
    #[builder(default)] bind_group_layouts: u64,
    #[builder(default)] render_pipelines: u64,
    #[builder(default)] compute_pipelines: u64,
    #[builder(default)] pipeline_layouts: u64,
    #[builder(default)] samplers: u64,
    #[builder(default)] command_encoders: u64,
    #[builder(default)] shader_modules: u64,
    #[builder(default)] query_sets: u64,
    #[builder(default)] fences: u64,
    #[builder(default)] buffer_memory: u64,
    #[builder(default)] texture_memory: u64,
    #[builder(default)] acceleration_structure_memory: u64,
    #[builder(default)] memory_allocations: u64,
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
Default from: wgpu-types/src/lib.rs:628
impl Default for Limits {
    fn default() -> Self {
        Self::defaults()
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn limits(
    max_texture_dimension_1d: u64,
    max_texture_dimension_2d: u64,
    max_texture_dimension_3d: u64,
    max_texture_array_layers: u64,
    max_bind_groups: u64,
    max_bindings_per_bind_group: u64,
    max_dynamic_uniform_buffers_per_pipeline_layout: u64,
    max_dynamic_storage_buffers_per_pipeline_layout: u64,
    max_sampled_textures_per_shader_stage: u64,
    max_samplers_per_shader_stage: u64,
    max_storage_buffers_per_shader_stage: u64,
    max_storage_textures_per_shader_stage: u64,
    max_uniform_buffers_per_shader_stage: u64,
    max_binding_array_elements_per_shader_stage: u64,
    max_binding_array_sampler_elements_per_shader_stage: u64,
    max_uniform_buffer_binding_size: u64,
    max_storage_buffer_binding_size: u64,
    max_vertex_buffers: u64,
    max_buffer_size: u64,
    max_vertex_attributes: u64,
    max_vertex_buffer_array_stride: u64,
    min_uniform_buffer_offset_alignment: u64,
    min_storage_buffer_offset_alignment: u64,
    max_inter_stage_shader_components: u64,
    max_color_attachments: u64,
    max_color_attachment_bytes_per_sample: u64,
    max_compute_workgroup_storage_size: u64,
    max_compute_invocations_per_workgroup: u64,
    max_compute_workgroup_size_x: u64,
    max_compute_workgroup_size_y: u64,
    max_compute_workgroup_size_z: u64,
    max_compute_workgroups_per_dimension: u64,
    min_subgroup_size: u64,
    max_subgroup_size: u64,
    max_push_constant_size: u64,
    max_non_sampler_bindings: u64,
    max_blas_primitive_count: u64,
    max_blas_geometry_count: u64,
    max_tlas_instance_count: u64,
    max_acceleration_structures_per_shader_stage: u64,
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
Default from: wgpu-types/src/lib.rs:4751
impl Default for StencilFaceState {
    fn default() -> Self {
        Self::IGNORE
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn stencil_face_state(
    compare: u64,
    #[builder(default)] fail_op: u64,
    #[builder(default)] depth_fail_op: u64,
    #[builder(default)] pass_op: u64,
) -> StencilFaceState {
    StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn texture_transition(texture: u64, selector: u64, state: u64) -> TextureTransition {
    TextureTransition {
        texture,
        selector,
        state,
    }
}

/*
Default from: wgpu-types/src/counters.rs:136
#[derive(Clone, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn core_counters() -> CoreCounters {
    CoreCounters {}
}

/*
Default from: wgpu-types/src/instance.rs:253
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn backend_options(
    #[builder(default)] gl: u64,
    #[builder(default)] dx12: u64,
    #[builder(default)] noop: u64,
) -> BackendOptions {
    BackendOptions { gl, dx12, noop }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn color_target_state(
    format: u64,
    blend: u64,
    #[builder(default)] write_mask: u64,
) -> ColorTargetState {
    ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

/*
Default from: wgpu-types/src/lib.rs:1016
impl Default for DownlevelLimits {
    fn default() -> Self {
        DownlevelLimits {}
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn downlevel_limits() -> DownlevelLimits {
    DownlevelLimits {}
}

/*
Default from: wgpu-types/src/lib.rs:4419
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn stencil_state(
    #[builder(default)] front: u64,
    #[builder(default)] back: u64,
    read_mask: u64,
    write_mask: u64,
) -> StencilState {
    StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

/*
Default from: wgpu-types/src/instance.rs:292
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn gl_backend_options(
    #[builder(default)] gles_minor_version: u64,
    #[builder(default)] fence_behavior: u64,
) -> GlBackendOptions {
    GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn adapter_info(
    name: u64,
    vendor: u64,
    device: u64,
    device_type: u64,
    driver: u64,
    driver_info: u64,
    backend: u64,
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

/*
Default from: wgpu-types/src/instance.rs:330
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn dx_12_backend_options(#[builder(default)] shader_compiler: u64) -> Dx12BackendOptions {
    Dx12BackendOptions { shader_compiler }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_bundle_depth_stencil(
    format: u64,
    depth_read_only: u64,
    stencil_read_only: u64,
) -> RenderBundleDepthStencil {
    RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

/*
Default from: wgpu-types/src/instance.rs:361
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn noop_backend_options(enable: u64) -> NoopBackendOptions {
    NoopBackendOptions { enable }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn vertex_attribute(format: u64, offset: u64, shader_location: u64) -> VertexAttribute {
    VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn origin_2_d(x: u64, y: u64) -> Origin2d {
    Origin2d { x, y }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn allocator_report(
    allocations: u64,
    blocks: u64,
    total_allocated_bytes: u64,
    total_reserved_bytes: u64,
) -> AllocatorReport {
    AllocatorReport {
        allocations,
        blocks,
        total_allocated_bytes,
        total_reserved_bytes,
    }
}

/*
Default from: wgpu-types/src/lib.rs:7142
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn image_subresource_range(
    #[builder(default)] aspect: u64,
    base_mip_level: u64,
    mip_level_count: u64,
    base_array_layer: u64,
    array_layer_count: u64,
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
Default from: wgpu-types/src/lib.rs:5850
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn extent_3_d(width: u64, height: u64, depth_or_array_layers: u64) -> Extent3d {
    Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

/*
Default from: wgpu-types/src/lib.rs:1034
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn downlevel_capabilities(
    flags: u64,
    #[builder(default)] limits: u64,
    shader_model: u64,
) -> DownlevelCapabilities {
    DownlevelCapabilities {
        flags,
        limits,
        shader_model,
    }
}

/*
Default from: wgpu-types/src/counters.rs:145
#[derive(Clone, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn internal_counters(
    #[builder(default)] core: u64,
    #[builder(default)] hal: u64,
) -> InternalCounters {
    InternalCounters { core, hal }
}

/*
Default from: wgpu-types/src/lib.rs:6350
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn command_buffer_descriptor(label: u64) -> CommandBufferDescriptor {
    CommandBufferDescriptor { label }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn blend_state(#[builder(default)] color: u64, #[builder(default)] alpha: u64) -> BlendState {
    BlendState { color, alpha }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_transition(buffer: u64, state: u64) -> BufferTransition {
    BufferTransition { buffer, state }
}

/*
Default from: wgpu-types/src/features.rs:533
    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn features(
    #[builder(default)] features_wgpu: u64,
    #[builder(default)] features_webgpu: u64,
) -> Features {
    Features {
        features_wgpu,
        features_webgpu,
    }
}

/*
Default from: wgpu-types/src/instance.rs:235
#[derive(Default, Clone, Debug, Copy)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn memory_budget_thresholds(
    for_resource_creation: u64,
    for_device_loss: u64,
) -> MemoryBudgetThresholds {
    MemoryBudgetThresholds {
        for_resource_creation,
        for_device_loss,
    }
}

/*
Default from: wgpu-types/src/lib.rs:4469
#[derive(Clone, Copy, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn depth_bias_state(constant: u64, slope_scale: u64, clamp: u64) -> DepthBiasState {
    DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn push_constant_range(stages: u64, range: u64) -> PushConstantRange {
    PushConstantRange { stages, range }
}

/*
Default from: wgpu-types/src/lib.rs:1609
impl Default for BlendComponent {
    fn default() -> Self {
        Self::REPLACE
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn blend_component(
    src_factor: u64,
    dst_factor: u64,
    #[builder(default)] operation: u64,
) -> BlendComponent {
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

/*
Default from: wgpu-types/src/instance.rs:23
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn instance_descriptor(
    #[builder(default)] backends: u64,
    #[builder(default)] flags: u64,
    #[builder(default)] memory_budget_thresholds: u64,
    #[builder(default)] backend_options: u64,
) -> InstanceDescriptor {
    InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

/*
Default from: wgpu-types/src/lib.rs:4585
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn operations(#[builder(default)] load: u64, #[builder(default)] store: u64) -> Operations {
    Operations { load, store }
}

/*
Default from: wgpu-types/src/lib.rs:1784
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn primitive_state(
    #[builder(default)] topology: u64,
    strip_index_format: u64,
    #[builder(default)] front_face: u64,
    cull_mode: u64,
    unclipped_depth: u64,
    #[builder(default)] polygon_mode: u64,
    conservative: u64,
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

/*
Default from: wgpu-types/src/lib.rs:6436
#[derive(Clone, Copy, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn texel_copy_buffer_layout(
    offset: u64,
    bytes_per_row: u64,
    rows_per_image: u64,
) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset,
        bytes_per_row,
        rows_per_image,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn vertex_buffer_layout(
    array_stride: u64,
    #[builder(default)] step_mode: u64,
    attributes: u64,
) -> VertexBufferLayout {
    VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn blas_triangle_geometry(
    size: u64,
    vertex_buffer: u64,
    first_vertex: u64,
    vertex_stride: u64,
    index_buffer: u64,
    first_index: u64,
    transform_buffer: u64,
    transform_buffer_offset: u64,
) -> BlasTriangleGeometry {
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
Default from: wgpu/src/api/pipeline_layout.rs:32
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn pipeline_layout_descriptor(
    label: u64,
    bind_group_layouts: u64,
    push_constant_ranges: u64,
) -> PipelineLayoutDescriptor {
    PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

/*
Default from: wgpu/src/api/common_pipeline.rs:25
impl Default for PipelineCompilationOptions<'_> {
    fn default() -> Self {
        Self {
            constants: Default::default(),
            zero_initialize_workgroup_memory: true,
        }
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn pipeline_compilation_options(
    constants: u64,
    zero_initialize_workgroup_memory: u64,
) -> PipelineCompilationOptions {
    PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn compilation_message(message: u64, message_type: u64, location: u64) -> CompilationMessage {
    CompilationMessage {
        message,
        message_type,
        location,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_pipeline_descriptor(
    label: u64,
    layout: u64,
    vertex: u64,
    #[builder(default)] primitive: u64,
    depth_stencil: u64,
    #[builder(default)] multisample: u64,
    fragment: u64,
    multiview: u64,
    cache: u64,
) -> RenderPipelineDescriptor {
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
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_depth_stencil_attachment(
    view: u64,
    depth_ops: u64,
    stencil_ops: u64,
) -> RenderPassDepthStencilAttachment {
    RenderPassDepthStencilAttachment {
        view,
        depth_ops,
        stencil_ops,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_binding(buffer: u64, offset: u64, size: u64) -> BufferBinding {
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

/*
Default from: wgpu/src/api/render_bundle_encoder.rs:34
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_bundle_encoder_descriptor(
    label: u64,
    color_formats: u64,
    depth_stencil: u64,
    sample_count: u64,
    multiview: u64,
) -> RenderBundleEncoderDescriptor {
    RenderBundleEncoderDescriptor {
        label,
        color_formats,
        depth_stencil,
        sample_count,
        multiview,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_entry(binding: u64, resource: u64) -> BindGroupEntry {
    BindGroupEntry { binding, resource }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn vertex_state(
    module: u64,
    entry_point: u64,
    #[builder(default)] compilation_options: u64,
    buffers: u64,
) -> VertexState {
    VertexState {
        module,
        entry_point,
        compilation_options,
        buffers,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_color_attachment(
    view: u64,
    depth_slice: u64,
    resolve_target: u64,
    #[builder(default)] ops: u64,
) -> RenderPassColorAttachment {
    RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn pipeline_cache_descriptor(label: u64, data: u64, fallback: u64) -> PipelineCacheDescriptor {
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

/*
Default from: wgpu/src/api/compute_pass.rs:174
#[derive(Clone, Default, Debug)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn compute_pass_descriptor(label: u64, timestamp_writes: u64) -> ComputePassDescriptor {
    ComputePassDescriptor {
        label,
        timestamp_writes,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn shader_module_descriptor(label: u64, source: u64) -> ShaderModuleDescriptor {
    ShaderModuleDescriptor { label, source }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn source_location(
    line_number: u64,
    line_position: u64,
    offset: u64,
    length: u64,
) -> SourceLocation {
    SourceLocation {
        line_number,
        line_position,
        offset,
        length,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_timestamp_writes(
    query_set: u64,
    beginning_of_pass_write_index: u64,
    end_of_pass_write_index: u64,
) -> RenderPassTimestampWrites {
    RenderPassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn compute_pipeline_descriptor(
    label: u64,
    layout: u64,
    module: u64,
    entry_point: u64,
    #[builder(default)] compilation_options: u64,
    cache: u64,
) -> ComputePipelineDescriptor {
    ComputePipelineDescriptor {
        label,
        layout,
        module,
        entry_point,
        compilation_options,
        cache,
    }
}

/*
Default from: wgpu/src/api/render_pass.rs:561
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn render_pass_descriptor(
    label: u64,
    color_attachments: u64,
    depth_stencil_attachment: u64,
    timestamp_writes: u64,
    occlusion_query_set: u64,
) -> RenderPassDescriptor {
    RenderPassDescriptor {
        label,
        color_attachments,
        depth_stencil_attachment,
        timestamp_writes,
        occlusion_query_set,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn fragment_state(
    module: u64,
    entry_point: u64,
    #[builder(default)] compilation_options: u64,
    targets: u64,
) -> FragmentState {
    FragmentState {
        module,
        entry_point,
        compilation_options,
        targets,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn blas_build_entry(blas: u64, geometry: u64) -> BlasBuildEntry {
    BlasBuildEntry { blas, geometry }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn compute_pass_timestamp_writes(
    query_set: u64,
    beginning_of_pass_write_index: u64,
    end_of_pass_write_index: u64,
) -> ComputePassTimestampWrites {
    ComputePassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn buffer_init_descriptor(label: u64, contents: u64, usage: u64) -> BufferInitDescriptor {
    BufferInitDescriptor {
        label,
        contents,
        usage,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_descriptor(label: u64, layout: u64, entries: u64) -> BindGroupDescriptor {
    BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn bind_group_layout_descriptor(label: u64, entries: u64) -> BindGroupLayoutDescriptor {
    BindGroupLayoutDescriptor { label, entries }
}

/*
Item not found

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
pub fn compilation_info(messages: u64) -> CompilationInfo {
    CompilationInfo { messages }
}
