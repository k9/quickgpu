use std::num::{NonZero, NonZeroU32};
use std::ops::Range;
use wgpu::{wgt::TextureSelector, *};

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn adapter_info(
    #[builder(default)]
    name: String,
    #[builder(default)]
    vendor: u32,
    #[builder(default)]
    device: u32,
    device_type: DeviceType,
    #[builder(default)]
    driver: String,
    #[builder(default)]
    driver_info: String,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn backend_options(
    #[builder(default)]
    gl: GlBackendOptions,
    #[builder(default)]
    dx12: Dx12BackendOptions,
    #[builder(default)]
    noop: NoopBackendOptions,
) -> BackendOptions {
    BackendOptions { gl, dx12, noop }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    layout: &'a BindGroupLayout,
    #[builder(default)]
    entries: &'a [BindGroupEntry<'a>],
) -> BindGroupDescriptor<'a> {
    BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_entry<'a>(
    #[builder(default)]
    binding: u32,
    resource: BindingResource<'a>,
) -> BindGroupEntry<'a> {
    BindGroupEntry {
        binding,
        resource,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_layout_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    #[builder(default)]
    entries: &'a [BindGroupLayoutEntry],
) -> BindGroupLayoutDescriptor<'a> {
    BindGroupLayoutDescriptor {
        label,
        entries,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_layout_entry(
    #[builder(default)]
    binding: u32,
    visibility: ShaderStages,
    ty: BindingType,
    count: Option<NonZero<u32>>,
) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blas_build_entry<'a>(
    blas: &'a Blas,
    geometry: BlasGeometries<'a>,
) -> BlasBuildEntry<'a> {
    BlasBuildEntry { blas, geometry }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blas_triangle_geometry<'a>(
    size: &'a BlasTriangleGeometrySizeDescriptor,
    vertex_buffer: &'a Buffer,
    #[builder(default)]
    first_vertex: u32,
    #[builder(default)]
    vertex_stride: BufferAddress,
    index_buffer: Option<&'a Buffer>,
    first_index: Option<u32>,
    transform_buffer: Option<&'a Buffer>,
    transform_buffer_offset: Option<BufferAddress>,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blend_component(
    src_factor: BlendFactor,
    dst_factor: BlendFactor,
    #[builder(default)]
    operation: BlendOperation,
) -> BlendComponent {
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blend_state(
    #[builder(default)]
    color: BlendComponent,
    #[builder(default)]
    alpha: BlendComponent,
) -> BlendState {
    BlendState { color, alpha }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn buffer_binding<'a>(
    buffer: &'a Buffer,
    #[builder(default)]
    offset: BufferAddress,
    size: Option<BufferSize>,
) -> BufferBinding<'a> {
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn buffer_texture_copy_info(
    #[builder(default)]
    copy_width: u64,
    #[builder(default)]
    copy_height: u64,
    #[builder(default)]
    depth_or_array_layers: u64,
    #[builder(default)]
    offset: u64,
    #[builder(default)]
    block_size_bytes: u64,
    #[builder(default)]
    block_width_texels: u64,
    #[builder(default)]
    block_height_texels: u64,
    #[builder(default)]
    width_blocks: u64,
    #[builder(default)]
    height_blocks: u64,
    #[builder(default)]
    row_bytes_dense: u64,
    #[builder(default)]
    row_stride_bytes: u64,
    #[builder(default)]
    image_stride_rows: u64,
    #[builder(default)]
    image_stride_bytes: u64,
    #[builder(default)]
    image_rows_dense: u64,
    #[builder(default)]
    image_bytes_dense: u64,
    #[builder(default)]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn buffer_transition<T>(buffer: T, state: BufferUses) -> BufferTransition<T> {
    BufferTransition { buffer, state }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn color(
    #[builder(default)]
    r: f64,
    #[builder(default)]
    g: f64,
    #[builder(default)]
    b: f64,
    #[builder(default)]
    a: f64,
) -> Color {
    Color { r, g, b, a }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn color_target_state(
    format: TextureFormat,
    blend: Option<BlendState>,
    #[builder(default)]
    write_mask: ColorWrites,
) -> ColorTargetState {
    ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compilation_message(
    #[builder(default)]
    message: String,
    message_type: CompilationMessageType,
    location: Option<SourceLocation>,
) -> CompilationMessage {
    CompilationMessage {
        message,
        message_type,
        location,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compute_pass_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
) -> ComputePassDescriptor<'a> {
    ComputePassDescriptor {
        label,
        timestamp_writes,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compute_pipeline_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    layout: Option<&'a PipelineLayout>,
    module: &'a ShaderModule,
    entry_point: Option<&'a str>,
    #[builder(default)]
    compilation_options: PipelineCompilationOptions<'a>,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn copy_external_image_dest_info<T>(
    texture: T,
    #[builder(default)]
    mip_level: u32,
    #[builder(default)]
    origin: Origin3d,
    #[builder(default)]
    aspect: TextureAspect,
    color_space: PredefinedColorSpace,
    #[builder(default)]
    premultiplied_alpha: bool,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn depth_bias_state(
    #[builder(default)]
    constant: i32,
    #[builder(default)]
    slope_scale: f32,
    #[builder(default)]
    clamp: f32,
) -> DepthBiasState {
    DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn depth_stencil_state(
    format: TextureFormat,
    #[builder(default)]
    depth_write_enabled: bool,
    depth_compare: CompareFunction,
    #[builder(default)]
    stencil: StencilState,
    #[builder(default)]
    bias: DepthBiasState,
) -> DepthStencilState {
    DepthStencilState {
        format,
        depth_write_enabled,
        depth_compare,
        stencil,
        bias,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn downlevel_capabilities(
    flags: DownlevelFlags,
    #[builder(default)]
    limits: DownlevelLimits,
    shader_model: ShaderModel,
) -> DownlevelCapabilities {
    DownlevelCapabilities {
        flags,
        limits,
        shader_model,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn extent_3d(
    #[builder(default)]
    width: u32,
    #[builder(default)]
    height: u32,
    #[builder(default)]
    depth_or_array_layers: u32,
) -> Extent3d {
    Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn features(
    #[builder(default)]
    features_wgpu: FeaturesWGPU,
    #[builder(default)]
    features_webgpu: FeaturesWebGPU,
) -> Features {
    Features {
        features_wgpu,
        features_webgpu,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn fragment_state<'a>(
    module: &'a ShaderModule,
    entry_point: Option<&'a str>,
    #[builder(default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[builder(default)]
    targets: &'a [Option<ColorTargetState>],
) -> FragmentState<'a> {
    FragmentState {
        module,
        entry_point,
        compilation_options,
        targets,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn gl_backend_options(
    #[builder(default)]
    gles_minor_version: Gles3MinorVersion,
    #[builder(default)]
    fence_behavior: GlFenceBehavior,
) -> GlBackendOptions {
    GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn image_subresource_range(
    #[builder(default)]
    aspect: TextureAspect,
    #[builder(default)]
    base_mip_level: u32,
    mip_level_count: Option<u32>,
    #[builder(default)]
    base_array_layer: u32,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn instance_descriptor(
    #[builder(default)]
    backends: Backends,
    #[builder(default)]
    flags: InstanceFlags,
    #[builder(default)]
    memory_budget_thresholds: MemoryBudgetThresholds,
    #[builder(default)]
    backend_options: BackendOptions,
) -> InstanceDescriptor {
    InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn internal_counters(
    #[builder(default)]
    core: CoreCounters,
    #[builder(default)]
    hal: HalCounters,
) -> InternalCounters {
    InternalCounters { core, hal }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn limits(
    #[builder(default)]
    max_texture_dimension_1d: u32,
    #[builder(default)]
    max_texture_dimension_2d: u32,
    #[builder(default)]
    max_texture_dimension_3d: u32,
    #[builder(default)]
    max_texture_array_layers: u32,
    #[builder(default)]
    max_bind_groups: u32,
    #[builder(default)]
    max_bindings_per_bind_group: u32,
    #[builder(default)]
    max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    #[builder(default)]
    max_dynamic_storage_buffers_per_pipeline_layout: u32,
    #[builder(default)]
    max_sampled_textures_per_shader_stage: u32,
    #[builder(default)]
    max_samplers_per_shader_stage: u32,
    #[builder(default)]
    max_storage_buffers_per_shader_stage: u32,
    #[builder(default)]
    max_storage_textures_per_shader_stage: u32,
    #[builder(default)]
    max_uniform_buffers_per_shader_stage: u32,
    #[builder(default)]
    max_binding_array_elements_per_shader_stage: u32,
    #[builder(default)]
    max_binding_array_sampler_elements_per_shader_stage: u32,
    #[builder(default)]
    max_uniform_buffer_binding_size: u32,
    #[builder(default)]
    max_storage_buffer_binding_size: u32,
    #[builder(default)]
    max_vertex_buffers: u32,
    #[builder(default)]
    max_buffer_size: u64,
    #[builder(default)]
    max_vertex_attributes: u32,
    #[builder(default)]
    max_vertex_buffer_array_stride: u32,
    #[builder(default)]
    min_uniform_buffer_offset_alignment: u32,
    #[builder(default)]
    min_storage_buffer_offset_alignment: u32,
    #[builder(default)]
    max_inter_stage_shader_components: u32,
    #[builder(default)]
    max_color_attachments: u32,
    #[builder(default)]
    max_color_attachment_bytes_per_sample: u32,
    #[builder(default)]
    max_compute_workgroup_storage_size: u32,
    #[builder(default)]
    max_compute_invocations_per_workgroup: u32,
    #[builder(default)]
    max_compute_workgroup_size_x: u32,
    #[builder(default)]
    max_compute_workgroup_size_y: u32,
    #[builder(default)]
    max_compute_workgroup_size_z: u32,
    #[builder(default)]
    max_compute_workgroups_per_dimension: u32,
    #[builder(default)]
    min_subgroup_size: u32,
    #[builder(default)]
    max_subgroup_size: u32,
    #[builder(default)]
    max_push_constant_size: u32,
    #[builder(default)]
    max_non_sampler_bindings: u32,
    #[builder(default)]
    max_blas_primitive_count: u32,
    #[builder(default)]
    max_blas_geometry_count: u32,
    #[builder(default)]
    max_tlas_instance_count: u32,
    #[builder(default)]
    max_acceleration_structures_per_shader_stage: u32,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn memory_budget_thresholds(
    for_resource_creation: Option<u8>,
    for_device_loss: Option<u8>,
) -> MemoryBudgetThresholds {
    MemoryBudgetThresholds {
        for_resource_creation,
        for_device_loss,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn multisample_state(
    #[builder(default)]
    count: u32,
    #[builder(default)]
    mask: u64,
    #[builder(default)]
    alpha_to_coverage_enabled: bool,
) -> MultisampleState {
    MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn operations<V>(
    load: LoadOp<V>,
    #[builder(default)]
    store: StoreOp,
) -> Operations<V> {
    Operations { load, store }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn origin_2d(#[builder(default)] x: u32, #[builder(default)] y: u32) -> Origin2d {
    Origin2d { x, y }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn origin_3d(
    #[builder(default)]
    x: u32,
    #[builder(default)]
    y: u32,
    #[builder(default)]
    z: u32,
) -> Origin3d {
    Origin3d { x, y, z }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_cache_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    data: Option<&'a [u8]>,
    #[builder(default)]
    fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_compilation_options<'a>(
    #[builder(default)]
    constants: &'a [(&'a str, f64)],
    #[builder(default)]
    zero_initialize_workgroup_memory: bool,
) -> PipelineCompilationOptions<'a> {
    PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_layout_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    #[builder(default)]
    bind_group_layouts: &'a [&'a BindGroupLayout],
    #[builder(default)]
    push_constant_ranges: &'a [PushConstantRange],
) -> PipelineLayoutDescriptor<'a> {
    PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn primitive_state(
    #[builder(default)]
    topology: PrimitiveTopology,
    strip_index_format: Option<IndexFormat>,
    #[builder(default)]
    front_face: FrontFace,
    cull_mode: Option<Face>,
    #[builder(default)]
    unclipped_depth: bool,
    #[builder(default)]
    polygon_mode: PolygonMode,
    #[builder(default)]
    conservative: bool,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn push_constant_range(
    stages: ShaderStages,
    #[builder(default)]
    range: Range<u32>,
) -> PushConstantRange {
    PushConstantRange { stages, range }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_bundle_depth_stencil(
    format: TextureFormat,
    #[builder(default)]
    depth_read_only: bool,
    #[builder(default)]
    stencil_read_only: bool,
) -> RenderBundleDepthStencil {
    RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_bundle_encoder_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    #[builder(default)]
    color_formats: &'a [Option<TextureFormat>],
    depth_stencil: Option<RenderBundleDepthStencil>,
    #[builder(default)]
    sample_count: u32,
    multiview: Option<NonZeroU32>,
) -> RenderBundleEncoderDescriptor<'a> {
    RenderBundleEncoderDescriptor {
        label,
        color_formats,
        depth_stencil,
        sample_count,
        multiview,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_pass_color_attachment<'tex>(
    view: &'tex TextureView,
    depth_slice: Option<u32>,
    resolve_target: Option<&'tex TextureView>,
    #[builder(default)]
    ops: Operations<Color>,
) -> RenderPassColorAttachment<'tex> {
    RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_pass_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    #[builder(default)]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_pipeline_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    layout: Option<&'a PipelineLayout>,
    vertex: VertexState<'a>,
    #[builder(default)]
    primitive: PrimitiveState,
    depth_stencil: Option<DepthStencilState>,
    #[builder(default)]
    multisample: MultisampleState,
    fragment: Option<FragmentState<'a>>,
    multiview: Option<NonZeroU32>,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn request_adapter_options_base<S>(
    #[builder(default)]
    power_preference: PowerPreference,
    #[builder(default)]
    force_fallback_adapter: bool,
    compatible_surface: Option<S>,
) -> RequestAdapterOptionsBase<S> {
    RequestAdapterOptionsBase {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor<'a>(
    #[builder(default)]
    label: Label<'a>,
    source: ShaderSource<'a>,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor {
        label,
        source,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_runtime_checks(
    #[builder(default)]
    bounds_checks: bool,
    #[builder(default)]
    force_loop_bounding: bool,
) -> ShaderRuntimeChecks {
    ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn source_location(
    #[builder(default)]
    line_number: u32,
    #[builder(default)]
    line_position: u32,
    #[builder(default)]
    offset: u32,
    #[builder(default)]
    length: u32,
) -> SourceLocation {
    SourceLocation {
        line_number,
        line_position,
        offset,
        length,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn stencil_face_state(
    compare: CompareFunction,
    #[builder(default)]
    fail_op: StencilOperation,
    #[builder(default)]
    depth_fail_op: StencilOperation,
    #[builder(default)]
    pass_op: StencilOperation,
) -> StencilFaceState {
    StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn stencil_state(
    #[builder(default)]
    front: StencilFaceState,
    #[builder(default)]
    back: StencilFaceState,
    #[builder(default)]
    read_mask: u32,
    #[builder(default)]
    write_mask: u32,
) -> StencilState {
    StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn surface_capabilities(
    #[builder(default)]
    formats: Vec<TextureFormat>,
    #[builder(default)]
    present_modes: Vec<PresentMode>,
    #[builder(default)]
    alpha_modes: Vec<CompositeAlphaMode>,
    usages: TextureUsages,
) -> SurfaceCapabilities {
    SurfaceCapabilities {
        formats,
        present_modes,
        alpha_modes,
        usages,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_buffer_info_base<B>(
    buffer: B,
    #[builder(default)]
    layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfoBase<B> {
    TexelCopyBufferInfoBase {
        buffer,
        layout,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_buffer_layout(
    #[builder(default)]
    offset: u64,
    bytes_per_row: Option<u32>,
    rows_per_image: Option<u32>,
) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset,
        bytes_per_row,
        rows_per_image,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_texture_info_base<T>(
    texture: T,
    #[builder(default)]
    mip_level: u32,
    #[builder(default)]
    origin: Origin3d,
    #[builder(default)]
    aspect: TextureAspect,
) -> TexelCopyTextureInfoBase<T> {
    TexelCopyTextureInfoBase {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texture_format_features(
    allowed_usages: TextureUsages,
    flags: TextureFormatFeatureFlags,
) -> TextureFormatFeatures {
    TextureFormatFeatures {
        allowed_usages,
        flags,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texture_transition<T>(
    texture: T,
    selector: Option<TextureSelector>,
    state: TextureUses,
) -> TextureTransition<T> {
    TextureTransition {
        texture,
        selector,
        state,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_attribute(
    format: VertexFormat,
    #[builder(default)]
    offset: u64,
    #[builder(default)]
    shader_location: u32,
) -> VertexAttribute {
    VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_buffer_layout<'a>(
    #[builder(default)]
    array_stride: BufferAddress,
    #[builder(default)]
    step_mode: VertexStepMode,
    #[builder(default)]
    attributes: &'a [VertexAttribute],
) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_state<'a>(
    module: &'a ShaderModule,
    entry_point: Option<&'a str>,
    #[builder(default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[builder(default)]
    buffers: &'a [VertexBufferLayout<'a>],
) -> VertexState<'a> {
    VertexState {
        module,
        entry_point,
        compilation_options,
        buffers,
    }
}
