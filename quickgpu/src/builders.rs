use std::num::{NonZero, NonZeroU32};
use std::ops::Range;
use wgpu::{wgt::TextureSelector, *};

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
    layout: &'a BindGroupLayout,
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
    #[builder(default)] binding: u32,
    resource: BindingResource<'a>,
) -> BindGroupEntry<'a> {
    BindGroupEntry { binding, resource }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_layout_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
    entries: &'a [BindGroupLayoutEntry],
) -> BindGroupLayoutDescriptor<'a> {
    BindGroupLayoutDescriptor { label, entries }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn bind_group_layout_entry(
    #[builder(default)] binding: u32,
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
pub fn blas_build_entry<'a>(blas: &'a Blas, geometry: BlasGeometries<'a>) -> BlasBuildEntry<'a> {
    BlasBuildEntry { blas, geometry }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blas_triangle_geometry<'a>(
    size: &'a BlasTriangleGeometrySizeDescriptor,
    vertex_buffer: &'a Buffer,
    #[builder(default)] first_vertex: u32,
    #[builder(default)] vertex_stride: BufferAddress,
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
    #[builder(default)] operation: BlendOperation,
) -> BlendComponent {
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn blend_state(
    #[builder(default)] color: BlendComponent,
    #[builder(default)] alpha: BlendComponent,
) -> BlendState {
    BlendState { color, alpha }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn buffer_binding<'a>(
    buffer: &'a Buffer,
    #[builder(default)] offset: BufferAddress,
    size: Option<BufferSize>,
) -> BufferBinding<'a> {
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn color(
    #[builder(default)] r: f64,
    #[builder(default)] g: f64,
    #[builder(default)] b: f64,
    #[builder(default)] a: f64,
) -> Color {
    Color { r, g, b, a }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn compute_pass_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
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
    #[builder(default = None)] label: Label<'a>,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn extent_3d(
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn gl_backend_options(
    #[builder(default)] gles_minor_version: Gles3MinorVersion,
    #[builder(default)] fence_behavior: GlFenceBehavior,
) -> GlBackendOptions {
    GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn operations<V>(
    #[builder(default = LoadOp::Load)] load: LoadOp<V>,
    #[builder(default)] store: StoreOp,
) -> Operations<V> {
    Operations { load, store }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn origin_2d(#[builder(default)] x: u32, #[builder(default)] y: u32) -> Origin2d {
    Origin2d { x, y }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn origin_3d(
    #[builder(default)] x: u32,
    #[builder(default)] y: u32,
    #[builder(default)] z: u32,
) -> Origin3d {
    Origin3d { x, y, z }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_cache_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
    data: Option<&'a [u8]>,
    #[builder(default)] fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_compilation_options<'a>(
    constants: &'a [(&'a str, f64)],
    #[builder(default)] zero_initialize_workgroup_memory: bool,
) -> PipelineCompilationOptions<'a> {
    PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn pipeline_layout_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
    bind_group_layouts: &'a [&'a BindGroupLayout],
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn push_constant_range(
    stages: ShaderStages,
    #[builder(default)] range: Range<u32>,
) -> PushConstantRange {
    PushConstantRange { stages, range }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn render_bundle_encoder_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
    color_formats: &'a [Option<TextureFormat>],
    depth_stencil: Option<RenderBundleDepthStencil>,
    #[builder(default)] sample_count: u32,
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
    #[builder(default)] ops: Operations<Color>,
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
    #[builder(default = None)] label: Label<'a>,
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
    #[builder(default = None)] label: Label<'a>,
    layout: Option<&'a PipelineLayout>,
    vertex: VertexState<'a>,
    #[builder(default)] primitive: PrimitiveState,
    depth_stencil: Option<DepthStencilState>,
    #[builder(default)] multisample: MultisampleState,
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_module_descriptor<'a>(
    #[builder(default = None)] label: Label<'a>,
    source: ShaderSource<'a>,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor { label, source }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn shader_runtime_checks(
    #[builder(default)] bounds_checks: bool,
    #[builder(default)] force_loop_bounding: bool,
) -> ShaderRuntimeChecks {
    ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn texel_copy_buffer_info_base<B>(
    buffer: B,
    #[builder(default)] layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfoBase<B> {
    TexelCopyBufferInfoBase { buffer, layout }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
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
    #[builder(default)] offset: u64,
    #[builder(default)] shader_location: u32,
) -> VertexAttribute {
    VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

#[bon::builder(state_mod(vis = "pub(crate)"), derive(Into))]
pub fn vertex_buffer_layout<'a>(
    #[builder(default)] array_stride: BufferAddress,
    #[builder(default)] step_mode: VertexStepMode,
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
