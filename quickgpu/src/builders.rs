use std::num::{NonZero, NonZeroU32};
use wgpu::{wgt::TextureSelector, *};

#[bon::builder]
pub fn adapter_info(
    name: String,
    vendor: u32,
    device: u32,
    device_type: DeviceType,
    driver: String,
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

#[bon::builder]
pub fn backend_options(
    gl: GlBackendOptions,
    dx12: Dx12BackendOptions,
    noop: NoopBackendOptions,
) -> BackendOptions {
    BackendOptions { gl, dx12, noop }
}

#[bon::builder]
pub fn bind_group_descriptor<'a>(
    label: Label<'a>,
    layout: &'a BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
) -> BindGroupDescriptor<'a> {
    BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

#[bon::builder]
pub fn bind_group_layout_entry(
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

#[bon::builder]
pub fn blas_triangle_geometry<'a>(
    size: &'a BlasTriangleGeometrySizeDescriptor,
    vertex_buffer: &'a Buffer,
    first_vertex: u32,
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

#[bon::builder]
pub fn blend_component(
    src_factor: BlendFactor,
    dst_factor: BlendFactor,
    operation: BlendOperation,
) -> BlendComponent {
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

#[bon::builder]
pub fn buffer_binding<'a>(
    buffer: &'a Buffer,
    offset: BufferAddress,
    size: Option<BufferSize>,
) -> BufferBinding<'a> {
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

#[bon::builder]
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

#[bon::builder]
pub fn color(r: f64, g: f64, b: f64, a: f64) -> Color {
    Color { r, g, b, a }
}

#[bon::builder]
pub fn color_target_state(
    format: TextureFormat,
    blend: Option<BlendState>,
    write_mask: ColorWrites,
) -> ColorTargetState {
    ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

#[bon::builder]
pub fn compilation_message(
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

#[bon::builder]
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

#[bon::builder]
pub fn compute_pipeline_descriptor<'a>(
    label: Label<'a>,
    layout: Option<&'a PipelineLayout>,
    module: &'a ShaderModule,
    entry_point: Option<&'a str>,
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

#[bon::builder]
pub fn copy_external_image_dest_info<T>(
    texture: T,
    mip_level: u32,
    origin: Origin3d,
    aspect: TextureAspect,
    color_space: PredefinedColorSpace,
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

#[bon::builder]
pub fn depth_bias_state(constant: i32, slope_scale: f32, clamp: f32) -> DepthBiasState {
    DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

#[bon::builder]
pub fn depth_stencil_state(
    format: TextureFormat,
    depth_write_enabled: bool,
    depth_compare: CompareFunction,
    stencil: StencilState,
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

#[bon::builder]
pub fn downlevel_capabilities(
    flags: DownlevelFlags,
    limits: DownlevelLimits,
    shader_model: ShaderModel,
) -> DownlevelCapabilities {
    DownlevelCapabilities {
        flags,
        limits,
        shader_model,
    }
}

#[bon::builder]
pub fn extent_3d(width: u32, height: u32, depth_or_array_layers: u32) -> Extent3d {
    Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

#[bon::builder]
pub fn fragment_state<'a>(
    module: &'a ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: PipelineCompilationOptions<'a>,
    targets: &'a [Option<ColorTargetState>],
) -> FragmentState<'a> {
    FragmentState {
        module,
        entry_point,
        compilation_options,
        targets,
    }
}

#[bon::builder]
pub fn image_subresource_range(
    aspect: TextureAspect,
    base_mip_level: u32,
    mip_level_count: Option<u32>,
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

#[bon::builder]
pub fn instance_descriptor(
    backends: Backends,
    flags: InstanceFlags,
    memory_budget_thresholds: MemoryBudgetThresholds,
    backend_options: BackendOptions,
) -> InstanceDescriptor {
    InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

#[bon::builder]
pub fn limits(
    max_texture_dimension_1d: u32,
    max_texture_dimension_2d: u32,
    max_texture_dimension_3d: u32,
    max_texture_array_layers: u32,
    max_bind_groups: u32,
    max_bindings_per_bind_group: u32,
    max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    max_dynamic_storage_buffers_per_pipeline_layout: u32,
    max_sampled_textures_per_shader_stage: u32,
    max_samplers_per_shader_stage: u32,
    max_storage_buffers_per_shader_stage: u32,
    max_storage_textures_per_shader_stage: u32,
    max_uniform_buffers_per_shader_stage: u32,
    max_binding_array_elements_per_shader_stage: u32,
    max_binding_array_sampler_elements_per_shader_stage: u32,
    max_uniform_buffer_binding_size: u32,
    max_storage_buffer_binding_size: u32,
    max_vertex_buffers: u32,
    max_buffer_size: u64,
    max_vertex_attributes: u32,
    max_vertex_buffer_array_stride: u32,
    min_uniform_buffer_offset_alignment: u32,
    min_storage_buffer_offset_alignment: u32,
    max_inter_stage_shader_components: u32,
    max_color_attachments: u32,
    max_color_attachment_bytes_per_sample: u32,
    max_compute_workgroup_storage_size: u32,
    max_compute_invocations_per_workgroup: u32,
    max_compute_workgroup_size_x: u32,
    max_compute_workgroup_size_y: u32,
    max_compute_workgroup_size_z: u32,
    max_compute_workgroups_per_dimension: u32,
    min_subgroup_size: u32,
    max_subgroup_size: u32,
    max_push_constant_size: u32,
    max_non_sampler_bindings: u32,
    max_blas_primitive_count: u32,
    max_blas_geometry_count: u32,
    max_tlas_instance_count: u32,
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

#[bon::builder]
pub fn multisample_state(
    count: u32,
    mask: u64,
    alpha_to_coverage_enabled: bool,
) -> MultisampleState {
    MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
    }
}

#[bon::builder]
pub fn origin_3d(x: u32, y: u32, z: u32) -> Origin3d {
    Origin3d { x, y, z }
}

#[bon::builder]
pub fn pipeline_cache_descriptor<'a>(
    label: Label<'a>,
    data: Option<&'a [u8]>,
    fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

#[bon::builder]
pub fn pipeline_layout_descriptor<'a>(
    label: Label<'a>,
    bind_group_layouts: &'a [&'a BindGroupLayout],
    push_constant_ranges: &'a [PushConstantRange],
) -> PipelineLayoutDescriptor<'a> {
    PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

#[bon::builder]
pub fn primitive_state(
    topology: PrimitiveTopology,
    strip_index_format: Option<IndexFormat>,
    front_face: FrontFace,
    cull_mode: Option<Face>,
    unclipped_depth: bool,
    polygon_mode: PolygonMode,
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

#[bon::builder]
pub fn render_bundle_depth_stencil(
    format: TextureFormat,
    depth_read_only: bool,
    stencil_read_only: bool,
) -> RenderBundleDepthStencil {
    RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

#[bon::builder]
pub fn render_bundle_encoder_descriptor<'a>(
    label: Label<'a>,
    color_formats: &'a [Option<TextureFormat>],
    depth_stencil: Option<RenderBundleDepthStencil>,
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

#[bon::builder]
pub fn render_pass_color_attachment<'tex>(
    view: &'tex TextureView,
    depth_slice: Option<u32>,
    resolve_target: Option<&'tex TextureView>,
    ops: Operations<Color>,
) -> RenderPassColorAttachment<'tex> {
    RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
    }
}

#[bon::builder]
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

#[bon::builder]
pub fn render_pass_descriptor<'a>(
    label: Label<'a>,
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

#[bon::builder]
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

#[bon::builder]
pub fn render_pipeline_descriptor<'a>(
    label: Label<'a>,
    layout: Option<&'a PipelineLayout>,
    vertex: VertexState<'a>,
    primitive: PrimitiveState,
    depth_stencil: Option<DepthStencilState>,
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

#[bon::builder]
pub fn request_adapter_options_base<S>(
    power_preference: PowerPreference,
    force_fallback_adapter: bool,
    compatible_surface: Option<S>,
) -> RequestAdapterOptionsBase<S> {
    RequestAdapterOptionsBase {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

#[bon::builder]
pub fn source_location(
    line_number: u32,
    line_position: u32,
    offset: u32,
    length: u32,
) -> SourceLocation {
    SourceLocation {
        line_number,
        line_position,
        offset,
        length,
    }
}

#[bon::builder]
pub fn stencil_face_state(
    compare: CompareFunction,
    fail_op: StencilOperation,
    depth_fail_op: StencilOperation,
    pass_op: StencilOperation,
) -> StencilFaceState {
    StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

#[bon::builder]
pub fn stencil_state(
    front: StencilFaceState,
    back: StencilFaceState,
    read_mask: u32,
    write_mask: u32,
) -> StencilState {
    StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

#[bon::builder]
pub fn surface_capabilities(
    formats: Vec<TextureFormat>,
    present_modes: Vec<PresentMode>,
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

#[bon::builder]
pub fn texel_copy_buffer_layout(
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

#[bon::builder]
pub fn texel_copy_texture_info_base<T>(
    texture: T,
    mip_level: u32,
    origin: Origin3d,
    aspect: TextureAspect,
) -> TexelCopyTextureInfoBase<T> {
    TexelCopyTextureInfoBase {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

#[bon::builder]
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

#[bon::builder]
pub fn vertex_attribute(
    format: VertexFormat,
    offset: u64,
    shader_location: u32,
) -> VertexAttribute {
    VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

#[bon::builder]
pub fn vertex_buffer_layout<'a>(
    array_stride: BufferAddress,
    step_mode: VertexStepMode,
    attributes: &'a [VertexAttribute],
) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

#[bon::builder]
pub fn vertex_state<'a>(
    module: &'a ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: PipelineCompilationOptions<'a>,
    buffers: &'a [VertexBufferLayout<'a>],
) -> VertexState<'a> {
    VertexState {
        module,
        entry_point,
        compilation_options,
        buffers,
    }
}
