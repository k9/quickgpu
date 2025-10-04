use std::borrow::Cow;
use std::num::NonZeroU32;
use std::ops::Range;

use wgpu::util::*;
use wgpu::wgt::TextureSelector;
use wgpu::*;

/*
Default from: wgpu-types/src/counters.rs:136
#[derive(Clone, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn core_counters() -> CoreCounters {
    CoreCounters {}
}

/*
Default from: wgpu-types/src/lib.rs:7389
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn draw_indexed_indirect_args(
    #[builder(into)] index_count: u32,
    #[builder(into)] instance_count: u32,
    #[builder(into)] first_index: u32,
    #[builder(into)] base_vertex: i32,
    #[builder(into)] first_instance: u32,
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
Unhandled Some("BlendState") Id(2277)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn blend_state(
    #[builder(default, into)] color: BlendComponent,
    #[builder(default, into)] alpha: BlendComponent,
) -> BlendState {
    BlendState { color, alpha }
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
#[builder(derive(Into))]
pub fn blend_component(
    #[builder(into)] src_factor: BlendFactor,
    #[builder(into)] dst_factor: BlendFactor,
    #[builder(default, into)] operation: BlendOperation,
) -> BlendComponent {
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

/*
Default from: wgpu-types/src/instance.rs:292
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn gl_backend_options(
    #[builder(default, into)] gles_minor_version: Gles3MinorVersion,
    #[builder(default, into)] fence_behavior: GlFenceBehavior,
) -> GlBackendOptions {
    GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

/*
Default from: wgpu-types/src/lib.rs:7142
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn image_subresource_range(
    #[builder(default, into)] aspect: TextureAspect,
    #[builder(into)] base_mip_level: u32,
    #[builder(into)] mip_level_count: Option<u32>,
    #[builder(into)] base_array_layer: u32,
    #[builder(into)] array_layer_count: Option<u32>,
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
Unhandled Some("TexelCopyTextureInfo") Id(4629)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texel_copy_texture_info_base<T>(
    #[builder(into)] texture: T,
    #[builder(into)] mip_level: u32,
    #[builder(default, into)] origin: Origin3d,
    #[builder(default, into)] aspect: TextureAspect,
) -> TexelCopyTextureInfoBase<T> {
    TexelCopyTextureInfoBase {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

/*
Default from: wgpu-types/src/lib.rs:4419
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn stencil_state(
    #[builder(default, into)] front: StencilFaceState,
    #[builder(default, into)] back: StencilFaceState,
    #[builder(into)] read_mask: u32,
    #[builder(into)] write_mask: u32,
) -> StencilState {
    StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

/*
Default from: wgpu-types/src/instance.rs:253
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn backend_options(
    #[builder(default, into)] gl: GlBackendOptions,
    #[builder(default, into)] dx12: Dx12BackendOptions,
    #[builder(default, into)] noop: NoopBackendOptions,
) -> BackendOptions {
    BackendOptions { gl, dx12, noop }
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
#[builder(derive(Into))]
pub fn instance_descriptor(
    #[builder(default, into)] backends: Backends,
    #[builder(default, into)] flags: InstanceFlags,
    #[builder(default, into)] memory_budget_thresholds: MemoryBudgetThresholds,
    #[builder(default, into)] backend_options: BackendOptions,
) -> InstanceDescriptor {
    InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

/*
Unhandled Some("TexelCopyBufferInfo") Id(5063)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texel_copy_buffer_info_base<B>(
    #[builder(into)] buffer: B,
    #[builder(default, into)] layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfoBase<B> {
    TexelCopyBufferInfoBase { buffer, layout }
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
#[builder(derive(Into))]
pub fn downlevel_limits() -> DownlevelLimits {
    DownlevelLimits {}
}

/*
Default from: wgpu-types/src/lib.rs:5669
#[derive(Clone, Copy, Debug, Default, PartialEq)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn color(
    #[builder(into)] r: f64,
    #[builder(into)] g: f64,
    #[builder(into)] b: f64,
    #[builder(into)] a: f64,
) -> Color {
    Color { r, g, b, a }
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
#[builder(derive(Into))]
pub fn operations<V>(
    #[builder(into)] load: LoadOp<V>,
    #[builder(default, into)] store: StoreOp,
) -> Operations<V> {
    Operations { load, store }
}

/*
Unhandled Some("DepthStencilState") Id(3105)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn depth_stencil_state(
    #[builder(into)] format: TextureFormat,
    #[builder(into)] depth_write_enabled: bool,
    #[builder(into)] depth_compare: CompareFunction,
    #[builder(default, into)] stencil: StencilState,
    #[builder(default, into)] bias: DepthBiasState,
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
Default from: wgpu-types/src/instance.rs:361
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn noop_backend_options(#[builder(into)] enable: bool) -> NoopBackendOptions {
    NoopBackendOptions { enable }
}

/*
Unhandled Some("BindGroupLayoutEntry") Id(5000)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn bind_group_layout_entry(
    #[builder(into)] binding: u32,
    #[builder(into)] visibility: ShaderStages,
    #[builder(into)] ty: BindingType,
    #[builder(into)] count: Option<NonZeroU32>,
) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

/*
Unhandled Some("TextureTransition") Id(4188)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texture_transition<T>(
    #[builder(into)] texture: T,
    #[builder(into)] selector: Option<TextureSelector>,
    #[builder(into)] state: TextureUses,
) -> TextureTransition<T> {
    TextureTransition {
        texture,
        selector,
        state,
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
#[builder(derive(Into))]
pub fn multisample_state(
    #[builder(into)] count: u32,
    #[builder(into)] mask: u64,
    #[builder(into)] alpha_to_coverage_enabled: bool,
) -> MultisampleState {
    MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
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
#[builder(derive(Into))]
pub fn stencil_face_state(
    #[builder(into)] compare: CompareFunction,
    #[builder(default, into)] fail_op: StencilOperation,
    #[builder(default, into)] depth_fail_op: StencilOperation,
    #[builder(default, into)] pass_op: StencilOperation,
) -> StencilFaceState {
    StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

/*
Default from: wgpu-types/src/instance.rs:330
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn dx_12_backend_options(
    #[builder(default, into)] shader_compiler: Dx12Compiler,
) -> Dx12BackendOptions {
    Dx12BackendOptions { shader_compiler }
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
#[builder(derive(Into))]
pub fn extent_3_d(
    #[builder(into)] width: u32,
    #[builder(into)] height: u32,
    #[builder(into)] depth_or_array_layers: u32,
) -> Extent3d {
    Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

/*
Default from: wgpu-types/src/instance.rs:235
#[derive(Default, Clone, Debug, Copy)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn memory_budget_thresholds(
    #[builder(into)] for_resource_creation: Option<u8>,
    #[builder(into)] for_device_loss: Option<u8>,
) -> MemoryBudgetThresholds {
    MemoryBudgetThresholds {
        for_resource_creation,
        for_device_loss,
    }
}

/*
Unhandled Some("ColorTargetState") Id(2313)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn color_target_state(
    #[builder(into)] format: TextureFormat,
    #[builder(into)] blend: Option<BlendState>,
    #[builder(default, into)] write_mask: ColorWrites,
) -> ColorTargetState {
    ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

/*
Unhandled Some("RenderBundleDepthStencil") Id(4783)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_bundle_depth_stencil(
    #[builder(into)] format: TextureFormat,
    #[builder(into)] depth_read_only: bool,
    #[builder(into)] stencil_read_only: bool,
) -> RenderBundleDepthStencil {
    RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

/*
Default from: wgpu-types/src/lib.rs:4469
#[derive(Clone, Copy, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn depth_bias_state(
    #[builder(into)] constant: i32,
    #[builder(into)] slope_scale: f32,
    #[builder(into)] clamp: f32,
) -> DepthBiasState {
    DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

/*
Default from: wgpu-types/src/lib.rs:6350
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn command_buffer_descriptor<L>(#[builder(into)] label: L) -> CommandBufferDescriptor<L> {
    CommandBufferDescriptor { label }
}

/*
Default from: wgpu-types/src/lib.rs:360
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn request_adapter_options_base<S>(
    #[builder(default, into)] power_preference: PowerPreference,
    #[builder(into)] force_fallback_adapter: bool,
    #[builder(into)] compatible_surface: Option<S>,
) -> RequestAdapterOptionsBase<S> {
    RequestAdapterOptionsBase {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

/*
Unhandled Some("Origin2d") Id(4408)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn origin_2_d(#[builder(into)] x: u32, #[builder(into)] y: u32) -> Origin2d {
    Origin2d { x, y }
}

/*
Unhandled Some("CopyExternalImageDestInfo") Id(5090)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn copy_external_image_dest_info<T>(
    #[builder(into)] texture: T,
    #[builder(into)] mip_level: u32,
    #[builder(default, into)] origin: Origin3d,
    #[builder(default, into)] aspect: TextureAspect,
    #[builder(into)] color_space: PredefinedColorSpace,
    #[builder(into)] premultiplied_alpha: bool,
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
Default from: wgpu-types/src/lib.rs:7498
impl Default for ShaderRuntimeChecks {
    fn default() -> Self {
        Self::checked()
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn shader_runtime_checks(
    #[builder(into)] bounds_checks: bool,
    #[builder(into)] force_loop_bounding: bool,
) -> ShaderRuntimeChecks {
    ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

/*
Default from: wgpu-types/src/lib.rs:7365
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn draw_indirect_args(
    #[builder(into)] vertex_count: u32,
    #[builder(into)] instance_count: u32,
    #[builder(into)] first_vertex: u32,
    #[builder(into)] first_instance: u32,
) -> DrawIndirectArgs {
    DrawIndirectArgs {
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    }
}

/*
Unhandled Some("PushConstantRange") Id(4723)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn push_constant_range(
    #[builder(into)] stages: ShaderStages,
    #[builder(into)] range: Range<u32>,
) -> PushConstantRange {
    PushConstantRange { stages, range }
}

/*
Default from: wgpu-types/src/lib.rs:1784
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn primitive_state(
    #[builder(default, into)] topology: PrimitiveTopology,
    #[builder(into)] strip_index_format: Option<IndexFormat>,
    #[builder(default, into)] front_face: FrontFace,
    #[builder(into)] cull_mode: Option<Face>,
    #[builder(into)] unclipped_depth: bool,
    #[builder(default, into)] polygon_mode: PolygonMode,
    #[builder(into)] conservative: bool,
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
Unhandled Some("VertexAttribute") Id(3478)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn vertex_attribute(
    #[builder(into)] format: VertexFormat,
    #[builder(into)] offset: BufferAddress,
    #[builder(into)] shader_location: ShaderLocation,
) -> VertexAttribute {
    VertexAttribute {
        format,
        offset,
        shader_location,
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
#[builder(derive(Into))]
pub fn origin_3_d(
    #[builder(into)] x: u32,
    #[builder(into)] y: u32,
    #[builder(into)] z: u32,
) -> Origin3d {
    Origin3d { x, y, z }
}

/*
Default from: wgpu-types/src/lib.rs:7415
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn dispatch_indirect_args(
    #[builder(into)] x: u32,
    #[builder(into)] y: u32,
    #[builder(into)] z: u32,
) -> DispatchIndirectArgs {
    DispatchIndirectArgs { x, y, z }
}

/*
Default from: wgpu-types/src/lib.rs:6436
#[derive(Clone, Copy, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texel_copy_buffer_layout(
    #[builder(into)] offset: BufferAddress,
    #[builder(into)] bytes_per_row: Option<u32>,
    #[builder(into)] rows_per_image: Option<u32>,
) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset,
        bytes_per_row,
        rows_per_image,
    }
}

/*
Unhandled Some("BufferTransition") Id(3810)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn buffer_transition<T>(
    #[builder(into)] buffer: T,
    #[builder(into)] state: BufferUses,
) -> BufferTransition<T> {
    BufferTransition { buffer, state }
}

/*
Unhandled Some("BindGroupEntry") Id(264)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn bind_group_entry<'a>(
    #[builder(into)] binding: u32,
    #[builder(into)] resource: BindingResource<'a>,
) -> BindGroupEntry<'a> {
    BindGroupEntry { binding, resource }
}

/*
Default from: wgpu/src/api/render_pass.rs:561
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_pass_descriptor<'a>(
    #[builder(into, start_fn)] label: Label<'a>,
    color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    #[builder(into)] depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
    #[builder(into)] timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
    #[builder(into)] occlusion_query_set: Option<&'a QuerySet>,
) -> RenderPassDescriptor<'a> {
    RenderPassDescriptor {
        label,
        color_attachments,
        depth_stencil_attachment,
        timestamp_writes,
        occlusion_query_set,
    }
}

/*
Unhandled Some("BindGroupLayoutDescriptor") Id(260)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn bind_group_layout_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    entries: &'a [BindGroupLayoutEntry],
) -> BindGroupLayoutDescriptor<'a> {
    BindGroupLayoutDescriptor { label, entries }
}

/*
Unhandled Some("ShaderModuleDescriptor") Id(1224)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn shader_module_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] source: ShaderSource<'a>,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor { label, source }
}

/*
Unhandled Some("RenderPassColorAttachment") Id(2101)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_pass_color_attachment<'tex>(
    #[builder(into)] view: &'tex TextureView,
    #[builder(into)] depth_slice: Option<u32>,
    #[builder(into)] resolve_target: Option<&'tex TextureView>,
    #[builder(default, into)] ops: Operations<Color>,
) -> RenderPassColorAttachment<'tex> {
    RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
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
#[builder(derive(Into))]
pub fn pipeline_compilation_options<'a>(
    constants: &'a [(&'a str, f64)],
    #[builder(into)] zero_initialize_workgroup_memory: bool,
) -> PipelineCompilationOptions<'a> {
    PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

/*
Unhandled Some("BufferInitDescriptor") Id(1306)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn buffer_init_descriptor<'a>(
    #[builder(start_fn, into)] label: Label<'a>,
    contents: &'a [u8],
    #[builder(into)] usage: BufferUsages,
) -> BufferInitDescriptor<'a> {
    BufferInitDescriptor {
        label,
        contents,
        usage,
    }
}

/*
Unhandled Some("RenderPipelineDescriptor") Id(1235)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_pipeline_descriptor<'a>(
    #[builder(start_fn, into)] label: Label<'a>,
    #[builder(into)] layout: Option<&'a PipelineLayout>,
    #[builder(into)] vertex: VertexState<'a>,
    #[builder(default, into)] primitive: PrimitiveState,
    #[builder(into)] depth_stencil: Option<DepthStencilState>,
    #[builder(default, into)] multisample: MultisampleState,
    #[builder(into)] fragment: Option<FragmentState<'a>>,
    #[builder(into)] multiview: Option<NonZeroU32>,
    #[builder(into)] cache: Option<&'a PipelineCache>,
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
Unhandled Some("PipelineCacheDescriptor") Id(1014)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn pipeline_cache_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] data: Option<&'a [u8]>,
    #[builder(into)] fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

/*
Unhandled Some("BufferBinding") Id(172)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn buffer_binding<'a>(
    #[builder(into)] buffer: &'a Buffer,
    #[builder(into)] offset: BufferAddress,
    #[builder(into)] size: Option<BufferSize>,
) -> BufferBinding<'a> {
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

/*
Unhandled Some("BlasTriangleGeometry") Id(431)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn blas_triangle_geometry<'a>(
    #[builder(into)] size: &'a BlasTriangleGeometrySizeDescriptor,
    #[builder(into)] vertex_buffer: &'a Buffer,
    #[builder(into)] first_vertex: u32,
    #[builder(into)] vertex_stride: BufferAddress,
    #[builder(into)] index_buffer: Option<&'a Buffer>,
    #[builder(into)] first_index: Option<u32>,
    #[builder(into)] transform_buffer: Option<&'a Buffer>,
    #[builder(into)] transform_buffer_offset: Option<BufferAddress>,
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
Default from: wgpu/src/api/compute_pass.rs:174
#[derive(Clone, Default, Debug)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn compute_pass_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
) -> ComputePassDescriptor<'a> {
    ComputePassDescriptor {
        label,
        timestamp_writes,
    }
}

/*
Unhandled Some("ComputePipelineDescriptor") Id(1193)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn compute_pipeline_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] layout: Option<&'a PipelineLayout>,
    #[builder(into)] module: &'a ShaderModule,
    #[builder(into)] entry_point: Option<&'a str>,
    #[builder(default, into)] compilation_options: PipelineCompilationOptions<'a>,
    #[builder(into)] cache: Option<&'a PipelineCache>,
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

/*
Unhandled Some("BlasBuildEntry") Id(472)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn blas_build_entry<'a>(
    #[builder(into)] blas: &'a Blas,
    #[builder(into)] geometry: BlasGeometries<'a>,
) -> BlasBuildEntry<'a> {
    BlasBuildEntry { blas, geometry }
}

/*
Unhandled Some("VertexState") Id(2003)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn vertex_state<'a>(
    #[builder(into)] module: &'a ShaderModule,
    #[builder(into)] entry_point: Option<&'a str>,
    #[builder(default, into)] compilation_options: PipelineCompilationOptions<'a>,
    buffers: &'a [VertexBufferLayout<'a>],
) -> VertexState<'a> {
    VertexState {
        module,
        entry_point,
        compilation_options,
        buffers,
    }
}

/*
Unhandled Some("CompilationInfo") Id(2380)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn compilation_info(#[builder(into)] messages: Vec<CompilationMessage>) -> CompilationInfo {
    CompilationInfo { messages }
}

/*
Default from: wgpu/src/api/render_bundle_encoder.rs:34
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_bundle_encoder_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    color_formats: &'a [Option<TextureFormat>],
    #[builder(into)] depth_stencil: Option<RenderBundleDepthStencil>,
    #[builder(into)] sample_count: u32,
    #[builder(into)] multiview: Option<NonZeroU32>,
) -> RenderBundleEncoderDescriptor<'a> {
    RenderBundleEncoderDescriptor {
        label,
        color_formats,
        depth_stencil,
        sample_count,
        multiview,
    }
}

/*
Unhandled Some("RenderPassDepthStencilAttachment") Id(2127)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_pass_depth_stencil_attachment<'tex>(
    #[builder(into)] view: &'tex TextureView,
    #[builder(into)] depth_ops: Option<Operations<f32>>,
    #[builder(into)] stencil_ops: Option<Operations<u32>>,
) -> RenderPassDepthStencilAttachment<'tex> {
    RenderPassDepthStencilAttachment {
        view,
        depth_ops,
        stencil_ops,
    }
}

/*
Unhandled Some("VertexBufferLayout") Id(2002)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn vertex_buffer_layout<'a>(
    #[builder(into)] array_stride: BufferAddress,
    #[builder(default, into)] step_mode: VertexStepMode,
    attributes: &'a [VertexAttribute],
) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

/*
Unhandled Some("RenderPassTimestampWrites") Id(2074)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_pass_timestamp_writes<'a>(
    #[builder(into)] query_set: &'a QuerySet,
    #[builder(into)] beginning_of_pass_write_index: Option<u32>,
    #[builder(into)] end_of_pass_write_index: Option<u32>,
) -> RenderPassTimestampWrites<'a> {
    RenderPassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

/*
Unhandled Some("FragmentState") Id(2285)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn fragment_state<'a>(
    #[builder(into)] module: &'a ShaderModule,
    #[builder(into)] entry_point: Option<&'a str>,
    #[builder(default, into)] compilation_options: PipelineCompilationOptions<'a>,
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
Unhandled Some("ComputePassTimestampWrites") Id(1094)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn compute_pass_timestamp_writes<'a>(
    #[builder(into)] query_set: &'a QuerySet,
    #[builder(into)] beginning_of_pass_write_index: Option<u32>,
    #[builder(into)] end_of_pass_write_index: Option<u32>,
) -> ComputePassTimestampWrites<'a> {
    ComputePassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

/*
Default from: wgpu/src/api/pipeline_layout.rs:32
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn pipeline_layout_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
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
Unhandled Some("BindGroupDescriptor") Id(263)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn bind_group_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] layout: &'a BindGroupLayout,
    entries: &'a [BindGroupEntry<'a>],
) -> BindGroupDescriptor<'a> {
    BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

/*
Default from: wgpu-types/src/lib.rs:1315
#[derive(Clone, Debug, Default)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn device_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(default, into)] required_features: Features,
    #[builder(default, into)] required_limits: Limits,
    #[builder(default, into)] memory_hints: MemoryHints,
    #[builder(default, into)] trace: Trace,
) -> DeviceDescriptor<'a> {
    DeviceDescriptor {
        label,
        required_features,
        required_limits,
        memory_hints,
        trace,
    }
}

/*
Default from: wgpu-types/src/lib.rs:6232
impl<L: Default> Default for SamplerDescriptor<L> {
    fn default() -> Self {
        Self {
            label: Default::default(),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn sampler_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(default, into)] address_mode_u: AddressMode,
    #[builder(default, into)] address_mode_v: AddressMode,
    #[builder(default, into)] address_mode_w: AddressMode,
    #[builder(default, into)] mag_filter: FilterMode,
    #[builder(default, into)] min_filter: FilterMode,
    #[builder(default, into)] mipmap_filter: FilterMode,
    #[builder(into)] lod_min_clamp: f32,
    #[builder(into)] lod_max_clamp: f32,
    #[builder(into)] compare: Option<CompareFunction>,
    #[builder(into)] anisotropy_clamp: u16,
    #[builder(into)] border_color: Option<SamplerBorderColor>,
) -> SamplerDescriptor<'a> {
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

/*
Unhandled Some("QuerySetDescriptor") Id(1294)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn query_set_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] ty: QueryType,
    #[builder(into)] count: u32,
) -> QuerySetDescriptor<'a> {
    QuerySetDescriptor { label, ty, count }
}

/*
Default from: wgpu-types/src/lib.rs:6416
impl<T> Default for RenderBundleDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn render_bundle_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
) -> RenderBundleDescriptor<'a> {
    RenderBundleDescriptor { label }
}

/*
Unhandled Some("ShaderModuleDescriptorDxil") Id(5968)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn shader_module_descriptor_dxil<'a>(
    #[builder(into)] entry_point: String,
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] num_workgroups: (u32, u32, u32),
    source: &'a [u8],
) -> ShaderModuleDescriptorDxil<'a> {
    ShaderModuleDescriptorDxil {
        entry_point,
        label,
        num_workgroups,
        source,
    }
}

/*
Unhandled Some("TextureDescriptor") Id(4552)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texture_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(default, into)] size: Extent3d,
    #[builder(into)] mip_level_count: u32,
    #[builder(into)] sample_count: u32,
    #[builder(into)] dimension: TextureDimension,
    #[builder(into)] format: TextureFormat,
    #[builder(into)] usage: TextureUsages,
    #[builder(into)] view_formats: &'a [TextureFormat],
) -> TextureDescriptor<'a> {
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

/*
Unhandled Some("BlasTriangleGeometrySizeDescriptor") Id(331)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn blas_triangle_geometry_size_descriptor(
    #[builder(into)] vertex_format: VertexFormat,
    #[builder(into)] vertex_count: u32,
    #[builder(into)] index_format: Option<IndexFormat>,
    #[builder(into)] index_count: Option<u32>,
    #[builder(into)] flags: AccelerationStructureGeometryFlags,
) -> BlasTriangleGeometrySizeDescriptor {
    BlasTriangleGeometrySizeDescriptor {
        vertex_format,
        vertex_count,
        index_format,
        index_count,
        flags,
    }
}

/*
Unhandled Some("TexelCopyBufferInfo") Id(5063)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texel_copy_buffer_info<'a>(
    #[builder(into)] buffer: &'a Buffer,
    #[builder(default, into)] layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfo<'a> {
    TexelCopyBufferInfo { buffer, layout }
}

/*
Unhandled Some("SurfaceConfiguration") Id(3895)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn surface_configuration(
    #[builder(into)] usage: TextureUsages,
    #[builder(into)] format: TextureFormat,
    #[builder(into)] width: u32,
    #[builder(into)] height: u32,
    #[builder(default, into)] present_mode: PresentMode,
    #[builder(into)] desired_maximum_frame_latency: u32,
    #[builder(default, into)] alpha_mode: CompositeAlphaMode,
    #[builder(into)] view_formats: Vec<TextureFormat>,
) -> SurfaceConfiguration {
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

/*
Default from: wgpu-types/src/lib.rs:5240
impl<T> Default for CommandEncoderDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn command_encoder_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
) -> CommandEncoderDescriptor<'a> {
    CommandEncoderDescriptor { label }
}

/*
Unhandled Some("TexelCopyTextureInfo") Id(4629)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texel_copy_texture_info<'a>(
    #[builder(into)] texture: &'a Texture,
    #[builder(into)] mip_level: u32,
    #[builder(default, into)] origin: Origin3d,
    #[builder(default, into)] aspect: TextureAspect,
) -> TexelCopyTextureInfo<'a> {
    TexelCopyTextureInfo {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

/*
Unhandled Some("ShaderModuleDescriptorHlsl") Id(5971)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn shader_module_descriptor_hlsl<'a>(
    #[builder(into)] entry_point: String,
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] num_workgroups: (u32, u32, u32),
    #[builder(into)] source: &'a str,
) -> ShaderModuleDescriptorHlsl<'a> {
    ShaderModuleDescriptorHlsl {
        entry_point,
        label,
        num_workgroups,
        source,
    }
}

/*
Unhandled Some("CreateTlasDescriptor") Id(5637)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn create_tlas_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] max_instances: u32,
    #[builder(into)] flags: AccelerationStructureFlags,
    #[builder(into)] update_mode: AccelerationStructureUpdateMode,
) -> CreateTlasDescriptor<'a> {
    CreateTlasDescriptor {
        label,
        max_instances,
        flags,
        update_mode,
    }
}

/*
Unhandled Some("ShaderModuleDescriptorMsl") Id(5965)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn shader_module_descriptor_msl<'a>(
    #[builder(into)] entry_point: String,
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] num_workgroups: (u32, u32, u32),
    #[builder(into)] source: Cow<'a, str>,
) -> ShaderModuleDescriptorMsl<'a> {
    ShaderModuleDescriptorMsl {
        entry_point,
        label,
        num_workgroups,
        source,
    }
}

/*
Unhandled Some("CreateBlasDescriptor") Id(5605)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn create_blas_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] flags: AccelerationStructureFlags,
    #[builder(into)] update_mode: AccelerationStructureUpdateMode,
) -> CreateBlasDescriptor<'a> {
    CreateBlasDescriptor {
        label,
        flags,
        update_mode,
    }
}

/*
Default from: wgpu-types/src/lib.rs:360
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
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn request_adapter_options<'a, 'b>(
    #[builder(default, into)] power_preference: PowerPreference,
    #[builder(into)] force_fallback_adapter: bool,
    #[builder(into)] compatible_surface: Option<&'a Surface<'b>>,
) -> RequestAdapterOptions<'a, 'b> {
    RequestAdapterOptions {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

/*
Unhandled Some("ShaderModuleDescriptorSpirV") Id(5962)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn shader_module_descriptor_spir_v<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] source: Cow<'a, [u32]>,
) -> ShaderModuleDescriptorSpirV<'a> {
    ShaderModuleDescriptorSpirV { label, source }
}

/*
Unhandled Some("BufferDescriptor") Id(3829)

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn buffer_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] size: BufferAddress,
    #[builder(into)] usage: BufferUsages,
    #[builder(into)] mapped_at_creation: bool,
) -> BufferDescriptor<'a> {
    BufferDescriptor {
        label,
        size,
        usage,
        mapped_at_creation,
    }
}

/*
Default from: wgpu-types/src/lib.rs:6032
#[derive(Clone, Debug, Default, Eq, PartialEq)]

*/
#[bon::builder(state_mod(vis = "pub(crate)"))]
#[builder(derive(Into))]
pub fn texture_view_descriptor<'a>(
    #[builder(default, into)] label: Label<'a>,
    #[builder(into)] format: Option<TextureFormat>,
    #[builder(into)] dimension: Option<TextureViewDimension>,
    #[builder(into)] usage: Option<TextureUsages>,
    #[builder(default, into)] aspect: TextureAspect,
    #[builder(into)] base_mip_level: u32,
    #[builder(into)] mip_level_count: Option<u32>,
    #[builder(into)] base_array_layer: u32,
    #[builder(into)] array_layer_count: Option<u32>,
) -> TextureViewDescriptor<'a> {
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
