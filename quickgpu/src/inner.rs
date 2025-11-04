use std::num::NonZeroU32;
use std::ops::Range;

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn backend_options(
    gl: wgpu::GlBackendOptions,
    dx12: wgpu::Dx12BackendOptions,
    noop: wgpu::NoopBackendOptions,
) -> wgpu::BackendOptions {
    wgpu::BackendOptions { gl, dx12, noop }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: &'a wgpu::BindGroupLayout,
    entries: &'a [wgpu::BindGroupEntry<'a>],
) -> wgpu::BindGroupDescriptor<'a> {
    wgpu::BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_entry<'a>(
    binding: u32,
    resource: wgpu::BindingResource<'a>,
) -> wgpu::BindGroupEntry<'a> {
    wgpu::BindGroupEntry { binding, resource }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_layout_descriptor<'a>(
    label: wgpu::Label<'a>,
    entries: &'a [wgpu::BindGroupLayoutEntry],
) -> wgpu::BindGroupLayoutDescriptor<'a> {
    wgpu::BindGroupLayoutDescriptor { label, entries }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_layout_entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    ty: wgpu::BindingType,
    count: Option<NonZeroU32>,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blas_build_entry<'a>(
    blas: &'a wgpu::Blas,
    geometry: wgpu::BlasGeometries<'a>,
) -> wgpu::BlasBuildEntry<'a> {
    wgpu::BlasBuildEntry { blas, geometry }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blas_triangle_geometry<'a>(
    size: &'a wgpu::BlasTriangleGeometrySizeDescriptor,
    vertex_buffer: &'a wgpu::Buffer,
    first_vertex: u32,
    vertex_stride: wgpu::BufferAddress,
    index_buffer: Option<&'a wgpu::Buffer>,
    first_index: Option<u32>,
    transform_buffer: Option<&'a wgpu::Buffer>,
    transform_buffer_offset: Option<wgpu::BufferAddress>,
) -> wgpu::BlasTriangleGeometry<'a> {
    wgpu::BlasTriangleGeometry {
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

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blend_component(
    src_factor: wgpu::BlendFactor,
    dst_factor: wgpu::BlendFactor,
    operation: wgpu::BlendOperation,
) -> wgpu::BlendComponent {
    wgpu::BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blend_state(color: wgpu::BlendComponent, alpha: wgpu::BlendComponent) -> wgpu::BlendState {
    wgpu::BlendState { color, alpha }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_binding<'a>(
    buffer: &'a wgpu::Buffer,
    offset: wgpu::BufferAddress,
    size: Option<wgpu::BufferSize>,
) -> wgpu::BufferBinding<'a> {
    wgpu::BufferBinding {
        buffer,
        offset,
        size,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_transition<T>(buffer: T, state: wgpu::BufferUses) -> wgpu::BufferTransition<T> {
    wgpu::BufferTransition { buffer, state }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn color(r: f64, g: f64, b: f64, a: f64) -> wgpu::Color {
    wgpu::Color { r, g, b, a }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn color_target_state(
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
    write_mask: wgpu::ColorWrites,
) -> wgpu::ColorTargetState {
    wgpu::ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn command_buffer_descriptor<L>(label: L) -> wgpu::CommandBufferDescriptor<L> {
    wgpu::CommandBufferDescriptor { label }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compilation_info(messages: Vec<wgpu::CompilationMessage>) -> wgpu::CompilationInfo {
    wgpu::CompilationInfo { messages }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compute_pass_descriptor<'a>(
    label: wgpu::Label<'a>,
    timestamp_writes: Option<wgpu::ComputePassTimestampWrites<'a>>,
) -> wgpu::ComputePassDescriptor<'a> {
    wgpu::ComputePassDescriptor {
        label,
        timestamp_writes,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compute_pass_timestamp_writes<'a>(
    query_set: &'a wgpu::QuerySet,
    beginning_of_pass_write_index: Option<u32>,
    end_of_pass_write_index: Option<u32>,
) -> wgpu::ComputePassTimestampWrites<'a> {
    wgpu::ComputePassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compute_pipeline_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
    cache: Option<&'a wgpu::PipelineCache>,
) -> wgpu::ComputePipelineDescriptor<'a> {
    wgpu::ComputePipelineDescriptor {
        label,
        layout,
        module,
        entry_point,
        compilation_options,
        cache,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn copy_external_image_dest_info<T>(
    texture: T,
    mip_level: u32,
    origin: wgpu::Origin3d,
    aspect: wgpu::TextureAspect,
    color_space: wgpu::PredefinedColorSpace,
    premultiplied_alpha: bool,
) -> wgpu::CopyExternalImageDestInfo<T> {
    wgpu::CopyExternalImageDestInfo {
        texture,
        mip_level,
        origin,
        aspect,
        color_space,
        premultiplied_alpha,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn core_counters() -> wgpu::CoreCounters {
    wgpu::CoreCounters {}
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn depth_bias_state(constant: i32, slope_scale: f32, clamp: f32) -> wgpu::DepthBiasState {
    wgpu::DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn depth_stencil_state(
    format: wgpu::TextureFormat,
    depth_write_enabled: bool,
    depth_compare: wgpu::CompareFunction,
    stencil: wgpu::StencilState,
    bias: wgpu::DepthBiasState,
) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled,
        depth_compare,
        stencil,
        bias,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn downlevel_limits() -> wgpu::DownlevelLimits {
    wgpu::DownlevelLimits {}
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn dx_12_backend_options(
    shader_compiler: wgpu::Dx12Compiler,
    presentation_system: wgpu::wgt::Dx12SwapchainKind,
    latency_waitable_object: wgpu::wgt::Dx12UseFrameLatencyWaitableObject,
) -> wgpu::Dx12BackendOptions {
    wgpu::Dx12BackendOptions {
        shader_compiler,
        presentation_system,
        latency_waitable_object,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn extent_3_d(width: u32, height: u32, depth_or_array_layers: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn external_texture_transfer_function(
    a: f32,
    b: f32,
    g: f32,
    k: f32,
) -> wgpu::ExternalTextureTransferFunction {
    wgpu::ExternalTextureTransferFunction { a, b, g, k }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn fragment_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
    targets: &'a [Option<wgpu::ColorTargetState>],
) -> wgpu::FragmentState<'a> {
    wgpu::FragmentState {
        module,
        entry_point,
        compilation_options,
        targets,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn gl_backend_options(
    gles_minor_version: wgpu::Gles3MinorVersion,
    fence_behavior: wgpu::GlFenceBehavior,
) -> wgpu::GlBackendOptions {
    wgpu::GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn image_subresource_range(
    aspect: wgpu::TextureAspect,
    base_mip_level: u32,
    mip_level_count: Option<u32>,
    base_array_layer: u32,
    array_layer_count: Option<u32>,
) -> wgpu::ImageSubresourceRange {
    wgpu::ImageSubresourceRange {
        aspect,
        base_mip_level,
        mip_level_count,
        base_array_layer,
        array_layer_count,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn instance_descriptor(
    backends: wgpu::Backends,
    flags: wgpu::InstanceFlags,
    memory_budget_thresholds: wgpu::MemoryBudgetThresholds,
    backend_options: wgpu::BackendOptions,
) -> wgpu::InstanceDescriptor {
    wgpu::InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn memory_budget_thresholds(
    for_resource_creation: Option<u8>,
    for_device_loss: Option<u8>,
) -> wgpu::MemoryBudgetThresholds {
    wgpu::MemoryBudgetThresholds {
        for_resource_creation,
        for_device_loss,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn mesh_pipeline_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    task: Option<wgpu::TaskState<'a>>,
    mesh: wgpu::MeshState<'a>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    fragment: Option<wgpu::FragmentState<'a>>,
    multiview: Option<NonZeroU32>,
    cache: Option<&'a wgpu::PipelineCache>,
) -> wgpu::MeshPipelineDescriptor<'a> {
    wgpu::MeshPipelineDescriptor {
        label,
        layout,
        task,
        mesh,
        primitive,
        depth_stencil,
        multisample,
        fragment,
        multiview,
        cache,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn mesh_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
) -> wgpu::MeshState<'a> {
    wgpu::MeshState {
        module,
        entry_point,
        compilation_options,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn multisample_state(
    count: u32,
    mask: u64,
    alpha_to_coverage_enabled: bool,
) -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn noop_backend_options(enable: bool) -> wgpu::NoopBackendOptions {
    wgpu::NoopBackendOptions { enable }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn operations<V>(load: wgpu::LoadOp<V>, store: wgpu::StoreOp) -> wgpu::Operations<V> {
    wgpu::Operations { load, store }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn origin_2_d(x: u32, y: u32) -> wgpu::Origin2d {
    wgpu::Origin2d { x, y }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn origin_3_d(x: u32, y: u32, z: u32) -> wgpu::Origin3d {
    wgpu::Origin3d { x, y, z }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn pipeline_cache_descriptor<'a>(
    label: wgpu::Label<'a>,
    data: Option<&'a [u8]>,
    fallback: bool,
) -> wgpu::PipelineCacheDescriptor<'a> {
    wgpu::PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn pipeline_compilation_options<'a>(
    constants: &'a [(&'a str, f64)],
    zero_initialize_workgroup_memory: bool,
) -> wgpu::PipelineCompilationOptions<'a> {
    wgpu::PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn pipeline_layout_descriptor<'a>(
    label: wgpu::Label<'a>,
    bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    push_constant_ranges: &'a [wgpu::PushConstantRange],
) -> wgpu::PipelineLayoutDescriptor<'a> {
    wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn primitive_state(
    topology: wgpu::PrimitiveTopology,
    strip_index_format: Option<wgpu::IndexFormat>,
    front_face: wgpu::FrontFace,
    cull_mode: Option<wgpu::Face>,
    unclipped_depth: bool,
    polygon_mode: wgpu::PolygonMode,
    conservative: bool,
) -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology,
        strip_index_format,
        front_face,
        cull_mode,
        unclipped_depth,
        polygon_mode,
        conservative,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn push_constant_range(
    stages: wgpu::ShaderStages,
    range: Range<u32>,
) -> wgpu::PushConstantRange {
    wgpu::PushConstantRange { stages, range }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_bundle_depth_stencil(
    format: wgpu::TextureFormat,
    depth_read_only: bool,
    stencil_read_only: bool,
) -> wgpu::RenderBundleDepthStencil {
    wgpu::RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_bundle_encoder_descriptor<'a>(
    label: wgpu::Label<'a>,
    color_formats: &'a [Option<wgpu::TextureFormat>],
    depth_stencil: Option<wgpu::RenderBundleDepthStencil>,
    sample_count: u32,
    multiview: Option<NonZeroU32>,
) -> wgpu::RenderBundleEncoderDescriptor<'a> {
    wgpu::RenderBundleEncoderDescriptor {
        label,
        color_formats,
        depth_stencil,
        sample_count,
        multiview,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_color_attachment<'tex>(
    view: &'tex wgpu::TextureView,
    depth_slice: Option<u32>,
    resolve_target: Option<&'tex wgpu::TextureView>,
    ops: wgpu::Operations<wgpu::Color>,
) -> wgpu::RenderPassColorAttachment<'tex> {
    wgpu::RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_depth_stencil_attachment<'tex>(
    view: &'tex wgpu::TextureView,
    depth_ops: Option<wgpu::Operations<f32>>,
    stencil_ops: Option<wgpu::Operations<u32>>,
) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
    wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops,
        stencil_ops,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_descriptor<'a>(
    label: wgpu::Label<'a>,
    color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    timestamp_writes: Option<wgpu::RenderPassTimestampWrites<'a>>,
    occlusion_query_set: Option<&'a wgpu::QuerySet>,
) -> wgpu::RenderPassDescriptor<'a> {
    wgpu::RenderPassDescriptor {
        label,
        color_attachments,
        depth_stencil_attachment,
        timestamp_writes,
        occlusion_query_set,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_timestamp_writes<'a>(
    query_set: &'a wgpu::QuerySet,
    beginning_of_pass_write_index: Option<u32>,
    end_of_pass_write_index: Option<u32>,
) -> wgpu::RenderPassTimestampWrites<'a> {
    wgpu::RenderPassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pipeline_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex: wgpu::VertexState<'a>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    fragment: Option<wgpu::FragmentState<'a>>,
    multiview: Option<NonZeroU32>,
    cache: Option<&'a wgpu::PipelineCache>,
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
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

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn request_adapter_options_base<S>(
    power_preference: wgpu::PowerPreference,
    force_fallback_adapter: bool,
    compatible_surface: Option<S>,
) -> wgpu::RequestAdapterOptionsBase<S> {
    wgpu::RequestAdapterOptionsBase {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn shader_module_descriptor<'a>(
    label: wgpu::Label<'a>,
    source: wgpu::ShaderSource<'a>,
) -> wgpu::ShaderModuleDescriptor<'a> {
    wgpu::ShaderModuleDescriptor { label, source }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn shader_runtime_checks(
    bounds_checks: bool,
    force_loop_bounding: bool,
) -> wgpu::ShaderRuntimeChecks {
    wgpu::ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn stencil_face_state(
    compare: wgpu::CompareFunction,
    fail_op: wgpu::StencilOperation,
    depth_fail_op: wgpu::StencilOperation,
    pass_op: wgpu::StencilOperation,
) -> wgpu::StencilFaceState {
    wgpu::StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn stencil_state(
    front: wgpu::StencilFaceState,
    back: wgpu::StencilFaceState,
    read_mask: u32,
    write_mask: u32,
) -> wgpu::StencilState {
    wgpu::StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn task_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
) -> wgpu::TaskState<'a> {
    wgpu::TaskState {
        module,
        entry_point,
        compilation_options,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_buffer_info_base<B>(
    buffer: B,
    layout: wgpu::TexelCopyBufferLayout,
) -> wgpu::TexelCopyBufferInfoBase<B> {
    wgpu::TexelCopyBufferInfoBase { buffer, layout }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_buffer_layout(
    offset: wgpu::BufferAddress,
    bytes_per_row: Option<u32>,
    rows_per_image: Option<u32>,
) -> wgpu::TexelCopyBufferLayout {
    wgpu::TexelCopyBufferLayout {
        offset,
        bytes_per_row,
        rows_per_image,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_texture_info_base<T>(
    texture: T,
    mip_level: u32,
    origin: wgpu::Origin3d,
    aspect: wgpu::TextureAspect,
) -> wgpu::TexelCopyTextureInfoBase<T> {
    wgpu::TexelCopyTextureInfoBase {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texture_transition<T>(
    texture: T,
    selector: Option<wgpu::wgt::TextureSelector>,
    state: wgpu::TextureUses,
) -> wgpu::TextureTransition<T> {
    wgpu::TextureTransition {
        texture,
        selector,
        state,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn vertex_attribute(
    format: wgpu::VertexFormat,
    offset: wgpu::BufferAddress,
    shader_location: wgpu::ShaderLocation,
) -> wgpu::VertexAttribute {
    wgpu::VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn vertex_buffer_layout<'a>(
    array_stride: wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode,
    attributes: &'a [wgpu::VertexAttribute],
) -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn vertex_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
    buffers: &'a [wgpu::VertexBufferLayout<'a>],
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module,
        entry_point,
        compilation_options,
        buffers,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_init_descriptor<'a>(
    label: wgpu::Label<'a>,
    contents: &'a [u8],
    usage: wgpu::BufferUsages,
) -> wgpu::util::BufferInitDescriptor<'a> {
    wgpu::util::BufferInitDescriptor {
        label,
        contents,
        usage,
    }
}
