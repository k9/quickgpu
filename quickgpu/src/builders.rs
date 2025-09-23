use std::borrow::Cow;
use std::num::{NonZero, NonZeroU32};
use std::ops::Range;
use wgpu::{wgt::TextureSelector, *};

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    #[builder(into)]
    layout: &'a BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
) -> BindGroupDescriptor<'a> {
    let zzz = "";
    BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_entry<'a>(
    #[builder(into)]
    binding: u32,
    #[builder(into)]
    resource: BindingResource<'a>,
) -> BindGroupEntry<'a> {
    let zzz = "";
    BindGroupEntry {
        binding,
        resource,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_layout_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    entries: &'a [BindGroupLayoutEntry],
) -> BindGroupLayoutDescriptor<'a> {
    let zzz = "";
    BindGroupLayoutDescriptor {
        label,
        entries,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_layout_entry(
    #[builder(into)]
    binding: u32,
    #[builder(into)]
    visibility: ShaderStages,
    #[builder(into)]
    ty: BindingType,
    #[builder(into)]
    count: Option<NonZero<u32>>,
) -> BindGroupLayoutEntry {
    let zzz = "";
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blas_triangle_geometry<'a>(
    #[builder(into)]
    size: &'a BlasTriangleGeometrySizeDescriptor,
    #[builder(into)]
    vertex_buffer: &'a Buffer,
    #[builder(into)]
    first_vertex: u32,
    #[builder(into)]
    vertex_stride: BufferAddress,
    #[builder(into)]
    index_buffer: Option<&'a Buffer>,
    #[builder(into)]
    first_index: Option<u32>,
    #[builder(into)]
    transform_buffer: Option<&'a Buffer>,
    #[builder(into)]
    transform_buffer_offset: Option<BufferAddress>,
) -> BlasTriangleGeometry<'a> {
    let zzz = "";
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
    #[builder(default = BlendFactor::One, into)]
    src_factor: BlendFactor,
    #[builder(default = BlendFactor::Zero, into)]
    dst_factor: BlendFactor,
    #[builder(default = BlendOperation::Add, into)]
    operation: BlendOperation,
) -> BlendComponent {
    let zzz = "";
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blend_state(
    #[builder(into)]
    color: BlendComponent,
    #[builder(into)]
    alpha: BlendComponent,
) -> BlendState {
    let zzz = "";
    BlendState { color, alpha }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn buffer_binding<'a>(
    #[builder(into)]
    buffer: &'a Buffer,
    #[builder(default = 0u64, into)]
    offset: BufferAddress,
    #[builder(into)]
    size: Option<BufferSize>,
) -> BufferBinding<'a> {
    let zzz = "";
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn color(
    #[builder(into)]
    r: f64,
    #[builder(into)]
    g: f64,
    #[builder(into)]
    b: f64,
    #[builder(into)]
    a: f64,
) -> Color {
    let zzz = "";
    Color { r, g, b, a }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn color_target_state(
    #[builder(into)]
    format: TextureFormat,
    #[builder(into)]
    blend: Option<BlendState>,
    #[builder(default = ColorWrites::default(), into)]
    write_mask: ColorWrites,
) -> ColorTargetState {
    let zzz = "";
    ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn command_buffer_descriptor<L>(
    #[builder(into)]
    label: L,
) -> CommandBufferDescriptor<L> {
    let zzz = "";
    CommandBufferDescriptor { label }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compilation_info(
    #[builder(into)]
    messages: Vec<CompilationMessage>,
) -> CompilationInfo {
    let zzz = "";
    CompilationInfo { messages }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compute_pass_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    #[builder(into)]
    timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
) -> ComputePassDescriptor<'a> {
    let zzz = "";
    ComputePassDescriptor {
        label,
        timestamp_writes,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compute_pass_timestamp_writes<'a>(
    #[builder(into)]
    query_set: &'a QuerySet,
    #[builder(into)]
    beginning_of_pass_write_index: Option<u32>,
    #[builder(into)]
    end_of_pass_write_index: Option<u32>,
) -> ComputePassTimestampWrites<'a> {
    let zzz = "";
    ComputePassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compute_pipeline_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    #[builder(into)]
    layout: Option<&'a PipelineLayout>,
    #[builder(into)]
    module: &'a ShaderModule,
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[builder(into)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[builder(into)]
    cache: Option<&'a PipelineCache>,
) -> ComputePipelineDescriptor<'a> {
    let zzz = "";
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
    #[builder(into)]
    texture: T,
    #[builder(into)]
    mip_level: u32,
    #[builder(into)]
    origin: Origin3d,
    #[builder(into)]
    aspect: TextureAspect,
    #[builder(default = PredefinedColorSpace::Srgb, into)]
    color_space: PredefinedColorSpace,
    #[builder(default = false, into)]
    premultiplied_alpha: bool,
) -> CopyExternalImageDestInfo<T> {
    let zzz = "";
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
    #[builder(into)]
    constant: i32,
    #[builder(into)]
    slope_scale: f32,
    #[builder(into)]
    clamp: f32,
) -> DepthBiasState {
    let zzz = "";
    DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn depth_stencil_state(
    #[builder(into)]
    format: TextureFormat,
    #[builder(into)]
    depth_write_enabled: bool,
    #[builder(into)]
    depth_compare: CompareFunction,
    #[builder(into)]
    stencil: StencilState,
    #[builder(into)]
    bias: DepthBiasState,
) -> DepthStencilState {
    let zzz = "";
    DepthStencilState {
        format,
        depth_write_enabled,
        depth_compare,
        stencil,
        bias,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn dx_12_backend_options(
    #[builder(into)]
    shader_compiler: Dx12Compiler,
) -> Dx12BackendOptions {
    let zzz = "";
    Dx12BackendOptions {
        shader_compiler,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn extent_3d(
    #[builder(into)]
    width: u32,
    #[builder(into)]
    height: u32,
    #[builder(into)]
    depth_or_array_layers: u32,
) -> Extent3d {
    let zzz = "";
    Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn fragment_state<'a>(
    #[builder(into)]
    module: &'a ShaderModule,
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[builder(into)]
    compilation_options: PipelineCompilationOptions<'a>,
    targets: &'a [Option<ColorTargetState>],
) -> FragmentState<'a> {
    let zzz = "";
    FragmentState {
        module,
        entry_point,
        compilation_options,
        targets,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn gl_backend_options(
    #[builder(into)]
    gles_minor_version: Gles3MinorVersion,
    #[builder(into)]
    fence_behavior: GlFenceBehavior,
) -> GlBackendOptions {
    let zzz = "";
    GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn image_subresource_range(
    #[builder(into)]
    aspect: TextureAspect,
    #[builder(into)]
    base_mip_level: u32,
    #[builder(into)]
    mip_level_count: Option<u32>,
    #[builder(into)]
    base_array_layer: u32,
    #[builder(into)]
    array_layer_count: Option<u32>,
) -> ImageSubresourceRange {
    let zzz = "";
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
    #[builder(into)]
    backends: Backends,
    #[builder(into)]
    flags: InstanceFlags,
    #[builder(into)]
    memory_budget_thresholds: MemoryBudgetThresholds,
    #[builder(into)]
    backend_options: BackendOptions,
) -> InstanceDescriptor {
    let zzz = "";
    InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn memory_budget_thresholds(
    #[builder(into)]
    for_resource_creation: Option<u8>,
    #[builder(into)]
    for_device_loss: Option<u8>,
) -> MemoryBudgetThresholds {
    let zzz = "";
    MemoryBudgetThresholds {
        for_resource_creation,
        for_device_loss,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn multisample_state(
    #[builder(into)]
    count: u32,
    #[builder(into)]
    mask: u64,
    #[builder(into)]
    alpha_to_coverage_enabled: bool,
) -> MultisampleState {
    let zzz = "";
    MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn noop_backend_options(#[builder(into)] enable: bool) -> NoopBackendOptions {
    let zzz = "";
    NoopBackendOptions { enable }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn operations<V>(
    #[builder(default = LoadOp::Load, into)]
    load: LoadOp<V>,
    #[builder(into)]
    store: StoreOp,
) -> Operations<V> {
    let zzz = "";
    Operations { load, store }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn origin_2d(#[builder(into)] x: u32, #[builder(into)] y: u32) -> Origin2d {
    let zzz = "";
    Origin2d { x, y }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn origin_3d(
    #[builder(into)]
    x: u32,
    #[builder(into)]
    y: u32,
    #[builder(into)]
    z: u32,
) -> Origin3d {
    let zzz = "";
    Origin3d { x, y, z }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_cache_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    #[builder(into)]
    data: Option<&'a [u8]>,
    #[builder(into)]
    fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    let zzz = "";
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_compilation_options<'a>(
    constants: &'a [(&'a str, f64)],
    #[builder(into)]
    zero_initialize_workgroup_memory: bool,
) -> PipelineCompilationOptions<'a> {
    let zzz = "";
    PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_layout_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    bind_group_layouts: &'a [&'a BindGroupLayout],
    push_constant_ranges: &'a [PushConstantRange],
) -> PipelineLayoutDescriptor<'a> {
    let zzz = "";
    PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn primitive_state(
    #[builder(into)]
    topology: PrimitiveTopology,
    #[builder(into)]
    strip_index_format: Option<IndexFormat>,
    #[builder(into)]
    front_face: FrontFace,
    #[builder(into)]
    cull_mode: Option<Face>,
    #[builder(into)]
    unclipped_depth: bool,
    #[builder(into)]
    polygon_mode: PolygonMode,
    #[builder(into)]
    conservative: bool,
) -> PrimitiveState {
    let zzz = "";
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
    #[builder(into)]
    stages: ShaderStages,
    #[builder(into)]
    range: Range<u32>,
) -> PushConstantRange {
    let zzz = "";
    PushConstantRange { stages, range }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_bundle_depth_stencil(
    #[builder(into)]
    format: TextureFormat,
    #[builder(into)]
    depth_read_only: bool,
    #[builder(into)]
    stencil_read_only: bool,
) -> RenderBundleDepthStencil {
    let zzz = "";
    RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_bundle_encoder_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    color_formats: &'a [Option<TextureFormat>],
    #[builder(into)]
    depth_stencil: Option<RenderBundleDepthStencil>,
    #[builder(into)]
    sample_count: u32,
    #[builder(into)]
    multiview: Option<NonZeroU32>,
) -> RenderBundleEncoderDescriptor<'a> {
    let zzz = "";
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
    #[builder(into)]
    view: &'tex TextureView,
    #[builder(into)]
    depth_slice: Option<u32>,
    #[builder(into)]
    resolve_target: Option<&'tex TextureView>,
    #[builder(into)]
    ops: Operations<Color>,
) -> RenderPassColorAttachment<'tex> {
    let zzz = "";
    RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_pass_depth_stencil_attachment<'tex>(
    #[builder(into)]
    view: &'tex TextureView,
    #[builder(into)]
    depth_ops: Option<Operations<f32>>,
    #[builder(into)]
    stencil_ops: Option<Operations<u32>>,
) -> RenderPassDepthStencilAttachment<'tex> {
    let zzz = "";
    RenderPassDepthStencilAttachment {
        view,
        depth_ops,
        stencil_ops,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_pass_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    #[builder(into)]
    depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
    #[builder(into)]
    timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
    #[builder(into)]
    occlusion_query_set: Option<&'a QuerySet>,
) -> RenderPassDescriptor<'a> {
    let zzz = "";
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
    #[builder(into)]
    query_set: &'a QuerySet,
    #[builder(into)]
    beginning_of_pass_write_index: Option<u32>,
    #[builder(into)]
    end_of_pass_write_index: Option<u32>,
) -> RenderPassTimestampWrites<'a> {
    let zzz = "";
    RenderPassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_pipeline_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    #[builder(into)]
    layout: Option<&'a PipelineLayout>,
    #[builder(into)]
    vertex: VertexState<'a>,
    #[builder(into)]
    primitive: PrimitiveState,
    #[builder(into)]
    depth_stencil: Option<DepthStencilState>,
    #[builder(into)]
    multisample: MultisampleState,
    #[builder(into)]
    fragment: Option<FragmentState<'a>>,
    #[builder(into)]
    multiview: Option<NonZeroU32>,
    #[builder(into)]
    cache: Option<&'a PipelineCache>,
) -> RenderPipelineDescriptor<'a> {
    let zzz = "";
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
    #[builder(default, into)]
    power_preference: PowerPreference,
    #[builder(default = false, into)]
    force_fallback_adapter: bool,
    #[builder(into)]
    compatible_surface: Option<S>,
) -> RequestAdapterOptionsBase<S> {
    let zzz = "";
    RequestAdapterOptionsBase {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor<'a>(
    #[builder(default = None, into)]
    label: Label<'a>,
    #[builder(into)]
    source: ShaderSource<'a>,
) -> ShaderModuleDescriptor<'a> {
    let zzz = "";
    ShaderModuleDescriptor {
        label,
        source,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_runtime_checks(
    #[builder(into)]
    bounds_checks: bool,
    #[builder(into)]
    force_loop_bounding: bool,
) -> ShaderRuntimeChecks {
    let zzz = "";
    ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn stencil_face_state(
    #[builder(into)]
    compare: CompareFunction,
    #[builder(into)]
    fail_op: StencilOperation,
    #[builder(into)]
    depth_fail_op: StencilOperation,
    #[builder(into)]
    pass_op: StencilOperation,
) -> StencilFaceState {
    let zzz = "";
    StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn stencil_state(
    #[builder(into)]
    front: StencilFaceState,
    #[builder(into)]
    back: StencilFaceState,
    #[builder(into)]
    read_mask: u32,
    #[builder(into)]
    write_mask: u32,
) -> StencilState {
    let zzz = "";
    StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_buffer_info_base<B>(
    #[builder(into)]
    buffer: B,
    #[builder(into)]
    layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfoBase<B> {
    let zzz = "";
    TexelCopyBufferInfoBase {
        buffer,
        layout,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_buffer_layout(
    #[builder(into)]
    offset: u64,
    #[builder(into)]
    bytes_per_row: Option<u32>,
    #[builder(into)]
    rows_per_image: Option<u32>,
) -> TexelCopyBufferLayout {
    let zzz = "";
    TexelCopyBufferLayout {
        offset,
        bytes_per_row,
        rows_per_image,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_texture_info_base<T>(
    #[builder(into)]
    texture: T,
    #[builder(into)]
    mip_level: u32,
    #[builder(into)]
    origin: Origin3d,
    #[builder(into)]
    aspect: TextureAspect,
) -> TexelCopyTextureInfoBase<T> {
    let zzz = "";
    TexelCopyTextureInfoBase {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texture_transition<T>(
    #[builder(into)]
    texture: T,
    #[builder(into)]
    selector: Option<TextureSelector>,
    #[builder(into)]
    state: TextureUses,
) -> TextureTransition<T> {
    let zzz = "";
    TextureTransition {
        texture,
        selector,
        state,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_attribute(
    #[builder(into)]
    format: VertexFormat,
    #[builder(into)]
    offset: u64,
    #[builder(into)]
    shader_location: u32,
) -> VertexAttribute {
    let zzz = "";
    VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_buffer_layout<'a>(
    #[builder(into)]
    array_stride: BufferAddress,
    #[builder(into)]
    step_mode: VertexStepMode,
    attributes: &'a [VertexAttribute],
) -> VertexBufferLayout<'a> {
    let zzz = "";
    VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_state<'a>(
    #[builder(into)]
    module: &'a ShaderModule,
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[builder(into)]
    compilation_options: PipelineCompilationOptions<'a>,
    buffers: &'a [VertexBufferLayout<'a>],
) -> VertexState<'a> {
    let zzz = "";
    VertexState {
        module,
        entry_point,
        compilation_options,
        buffers,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blas_triangle_geometry_size_descriptor(
    #[builder(into)]
    vertex_format: VertexFormat,
    #[builder(into)]
    vertex_count: u32,
    #[builder(into)]
    index_format: Option<IndexFormat>,
    #[builder(into)]
    index_count: Option<u32>,
    #[builder(into)]
    flags: AccelerationStructureGeometryFlags,
) -> BlasTriangleGeometrySizeDescriptor {
    let zzz = "";
    BlasTriangleGeometrySizeDescriptor {
        vertex_format,
        vertex_count,
        index_format,
        index_count,
        flags,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn buffer_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    size: u64,
    #[builder(into)]
    usage: BufferUsages,
    #[builder(into)]
    mapped_at_creation: bool,
) -> BufferDescriptor<'a> {
    let zzz = "";
    BufferDescriptor {
        label,
        size,
        usage,
        mapped_at_creation,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn command_encoder_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
) -> CommandEncoderDescriptor<'a> {
    let zzz = "";
    CommandEncoderDescriptor { label }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn create_blas_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    flags: AccelerationStructureFlags,
    #[builder(into)]
    update_mode: AccelerationStructureUpdateMode,
) -> CreateBlasDescriptor<'a> {
    let zzz = "";
    CreateBlasDescriptor {
        label,
        flags,
        update_mode,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn create_tlas_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    max_instances: u32,
    #[builder(into)]
    flags: AccelerationStructureFlags,
    #[builder(into)]
    update_mode: AccelerationStructureUpdateMode,
) -> CreateTlasDescriptor<'a> {
    let zzz = "";
    CreateTlasDescriptor {
        label,
        max_instances,
        flags,
        update_mode,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn device_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    required_features: Features,
    #[builder(into)]
    required_limits: Limits,
    #[builder(into)]
    memory_hints: MemoryHints,
    #[builder(into)]
    trace: Trace,
) -> DeviceDescriptor<'a> {
    let zzz = "";
    DeviceDescriptor {
        label,
        required_features,
        required_limits,
        memory_hints,
        trace,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn query_set_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    ty: QueryType,
    #[builder(into)]
    count: u32,
) -> QuerySetDescriptor<'a> {
    let zzz = "";
    QuerySetDescriptor {
        label,
        ty,
        count,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_bundle_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
) -> RenderBundleDescriptor<'a> {
    let zzz = "";
    RenderBundleDescriptor { label }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn request_adapter_options<'a, 'b>(
    #[builder(default, into)]
    power_preference: PowerPreference,
    #[builder(into)]
    force_fallback_adapter: bool,
    #[builder(into)]
    compatible_surface: Option<&'a Surface<'b>>,
) -> RequestAdapterOptions<'a, 'b> {
    let zzz = "";
    RequestAdapterOptions {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn sampler_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    address_mode_u: AddressMode,
    #[builder(into)]
    address_mode_v: AddressMode,
    #[builder(into)]
    address_mode_w: AddressMode,
    #[builder(into)]
    mag_filter: FilterMode,
    #[builder(into)]
    min_filter: FilterMode,
    #[builder(into)]
    mipmap_filter: FilterMode,
    #[builder(into)]
    lod_min_clamp: f32,
    #[builder(into)]
    lod_max_clamp: f32,
    #[builder(into)]
    compare: Option<CompareFunction>,
    #[builder(into)]
    anisotropy_clamp: u16,
    #[builder(into)]
    border_color: Option<SamplerBorderColor>,
) -> SamplerDescriptor<'a> {
    let zzz = "";
    SamplerDescriptor {
        label,
        address_mode_u,
        address_mode_v,
        address_mode_w,
        mag_filter,
        min_filter,
        mipmap_filter,
        lod_min_clamp,
        lod_max_clamp,
        compare,
        anisotropy_clamp,
        border_color,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor_dxil<'a>(
    #[builder(into)]
    entry_point: String,
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    num_workgroups: (u32, u32, u32),
    source: &'a [u8],
) -> ShaderModuleDescriptorDxil<'a> {
    let zzz = "";
    ShaderModuleDescriptorDxil {
        entry_point,
        label,
        num_workgroups,
        source,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor_hlsl<'a>(
    #[builder(into)]
    entry_point: String,
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    num_workgroups: (u32, u32, u32),
    #[builder(into)]
    source: &'a str,
) -> ShaderModuleDescriptorHlsl<'a> {
    let zzz = "";
    ShaderModuleDescriptorHlsl {
        entry_point,
        label,
        num_workgroups,
        source,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor_msl<'a>(
    #[builder(into)]
    entry_point: String,
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    num_workgroups: (u32, u32, u32),
    #[builder(into)]
    source: Cow<'a, str>,
) -> ShaderModuleDescriptorMsl<'a> {
    let zzz = "";
    ShaderModuleDescriptorMsl {
        entry_point,
        label,
        num_workgroups,
        source,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor_spir_v<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    source: Cow<'a, [u32]>,
) -> ShaderModuleDescriptorSpirV<'a> {
    let zzz = "";
    ShaderModuleDescriptorSpirV {
        label,
        source,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn surface_configuration(
    #[builder(into)]
    usage: TextureUsages,
    #[builder(into)]
    format: TextureFormat,
    #[builder(into)]
    width: u32,
    #[builder(into)]
    height: u32,
    #[builder(into)]
    present_mode: PresentMode,
    #[builder(into)]
    desired_maximum_frame_latency: u32,
    #[builder(into)]
    alpha_mode: CompositeAlphaMode,
    #[builder(into)]
    view_formats: Vec<TextureFormat>,
) -> SurfaceConfiguration {
    let zzz = "";
    SurfaceConfiguration {
        usage,
        format,
        width,
        height,
        present_mode,
        desired_maximum_frame_latency,
        alpha_mode,
        view_formats,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_buffer_info<'a>(
    #[builder(into)]
    buffer: &'a Buffer,
    #[builder(into)]
    layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfo<'a> {
    let zzz = "";
    TexelCopyBufferInfo {
        buffer,
        layout,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_texture_info<'a>(
    #[builder(into)]
    texture: &'a Texture,
    #[builder(into)]
    mip_level: u32,
    #[builder(into)]
    origin: Origin3d,
    #[builder(into)]
    aspect: TextureAspect,
) -> TexelCopyTextureInfo<'a> {
    let zzz = "";
    TexelCopyTextureInfo {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texture_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    size: Extent3d,
    #[builder(into)]
    mip_level_count: u32,
    #[builder(into)]
    sample_count: u32,
    #[builder(into)]
    dimension: TextureDimension,
    #[builder(into)]
    format: TextureFormat,
    #[builder(into)]
    usage: TextureUsages,
    view_formats: &'a [TextureFormat],
) -> TextureDescriptor<'a> {
    let zzz = "";
    TextureDescriptor {
        label,
        size,
        mip_level_count,
        sample_count,
        dimension,
        format,
        usage,
        view_formats,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texture_view_descriptor<'a>(
    #[builder(into)]
    label: Option<&'a str>,
    #[builder(into)]
    format: Option<TextureFormat>,
    #[builder(into)]
    dimension: Option<TextureViewDimension>,
    #[builder(into)]
    usage: Option<TextureUsages>,
    #[builder(into)]
    aspect: TextureAspect,
    #[builder(into)]
    base_mip_level: u32,
    #[builder(into)]
    mip_level_count: Option<u32>,
    #[builder(into)]
    base_array_layer: u32,
    #[builder(into)]
    array_layer_count: Option<u32>,
) -> TextureViewDescriptor<'a> {
    let zzz = "";
    TextureViewDescriptor {
        label,
        format,
        dimension,
        usage,
        aspect,
        base_mip_level,
        mip_level_count,
        base_array_layer,
        array_layer_count,
    }
}
