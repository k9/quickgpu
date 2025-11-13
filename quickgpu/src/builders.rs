use crate::Nested;
use std::borrow::Cow;
use std::num::NonZeroU32;
use std::ops::Range;

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn vertex_attribute(
    format: wgpu::VertexFormat,
    offset: wgpu::BufferAddress,
    shader_location: wgpu::ShaderLocation,
) -> wgpu::VertexAttribute {
    wgpu::VertexAttribute {
        format: format,
        offset: offset,
        shader_location: shader_location,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blend_component(
    # [builder (default = wgpu :: BlendFactor :: One)] src_factor: wgpu::BlendFactor,
    # [builder (default = wgpu :: BlendFactor :: Zero)] dst_factor: wgpu::BlendFactor,
    # [builder (default = wgpu :: BlendOperation :: Add)] operation: wgpu::BlendOperation,
) -> wgpu::BlendComponent {
    wgpu::BlendComponent {
        src_factor: src_factor,
        dst_factor: dst_factor,
        operation: operation,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn external_texture_transfer_function(
    #[builder(default = 1.0)] a: f32,
    #[builder(default = 1.0)] b: f32,
    #[builder(default = 1.0)] g: f32,
    #[builder(default = 1.0)] k: f32,
) -> wgpu::ExternalTextureTransferFunction {
    wgpu::ExternalTextureTransferFunction {
        a: a,
        b: b,
        g: g,
        k: k,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_buffer_info_base<B>(
    buffer: B,
    layout: impl Nested<wgpu::TexelCopyBufferLayout>,
) -> wgpu::TexelCopyBufferInfoBase<B> {
    wgpu::TexelCopyBufferInfoBase {
        buffer: buffer,
        layout: layout.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn pipeline_layout_descriptor<'a>(
    #[builder(default)] label: wgpu::Label<'a>,
    #[builder(default)] bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    #[builder(default)] push_constant_ranges: &'a [wgpu::PushConstantRange],
) -> wgpu::PipelineLayoutDescriptor<'a> {
    wgpu::PipelineLayoutDescriptor {
        label: label,
        bind_group_layouts: bind_group_layouts,
        push_constant_ranges: push_constant_ranges,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_bundle_encoder_descriptor<'a>(
    #[builder(default)] label: wgpu::Label<'a>,
    #[builder(default)] color_formats: &'a [Option<wgpu::TextureFormat>],
    depth_stencil: Option<impl Nested<wgpu::RenderBundleDepthStencil>>,
    #[builder(default)] sample_count: u32,
    multiview: Option<NonZeroU32>,
) -> wgpu::RenderBundleEncoderDescriptor<'a> {
    wgpu::RenderBundleEncoderDescriptor {
        label: label,
        color_formats: color_formats,
        depth_stencil: depth_stencil.unnest(),
        sample_count: sample_count,
        multiview: multiview,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn create_tlas_descriptor<'a>(
    label: wgpu::Label<'a>,
    max_instances: u32,
    flags: wgpu::wgt::AccelerationStructureFlags,
    update_mode: wgpu::wgt::AccelerationStructureUpdateMode,
) -> wgpu::CreateTlasDescriptor<'a> {
    wgpu::CreateTlasDescriptor {
        label: label,
        max_instances: max_instances,
        flags: flags,
        update_mode: update_mode,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn core_counters() -> wgpu::CoreCounters {
    wgpu::CoreCounters {}
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn draw_indexed_indirect_args(
    #[builder(default)] index_count: u32,
    #[builder(default)] instance_count: u32,
    #[builder(default)] first_index: u32,
    #[builder(default)] base_vertex: i32,
    #[builder(default)] first_instance: u32,
) -> wgpu::util::DrawIndexedIndirectArgs {
    wgpu::util::DrawIndexedIndirectArgs {
        index_count: index_count,
        instance_count: instance_count,
        first_index: first_index,
        base_vertex: base_vertex,
        first_instance: first_instance,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_transition<T>(buffer: T, state: wgpu::BufferUses) -> wgpu::BufferTransition<T> {
    wgpu::BufferTransition {
        buffer: buffer,
        state: state,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn sampler_descriptor<'a>(
    # [builder (default = Default :: default ())] label: wgpu::Label<'a>,
    # [builder (default = Default :: default ())] address_mode_u: wgpu::AddressMode,
    # [builder (default = Default :: default ())] address_mode_v: wgpu::AddressMode,
    # [builder (default = Default :: default ())] address_mode_w: wgpu::AddressMode,
    # [builder (default = Default :: default ())] mag_filter: wgpu::FilterMode,
    # [builder (default = Default :: default ())] min_filter: wgpu::FilterMode,
    # [builder (default = Default :: default ())] mipmap_filter: wgpu::FilterMode,
    #[builder(default = 0.0)] lod_min_clamp: f32,
    #[builder(default = 32.0)] lod_max_clamp: f32,
    compare: Option<wgpu::CompareFunction>,
    #[builder(default = 1)] anisotropy_clamp: u16,
    border_color: Option<wgpu::SamplerBorderColor>,
) -> wgpu::SamplerDescriptor<'a> {
    wgpu::SamplerDescriptor {
        label: label,
        address_mode_u: address_mode_u,
        address_mode_v: address_mode_v,
        address_mode_w: address_mode_w,
        mag_filter: mag_filter,
        min_filter: min_filter,
        mipmap_filter: mipmap_filter,
        lod_min_clamp: lod_min_clamp,
        lod_max_clamp: lod_max_clamp,
        compare: compare,
        anisotropy_clamp: anisotropy_clamp,
        border_color: border_color,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_descriptor<'a>(
    label: wgpu::Label<'a>,
    size: wgpu::BufferAddress,
    usage: wgpu::BufferUsages,
    mapped_at_creation: bool,
) -> wgpu::BufferDescriptor<'a> {
    wgpu::BufferDescriptor {
        label: label,
        size: size,
        usage: usage,
        mapped_at_creation: mapped_at_creation,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_bundle_descriptor<'a>(
    # [builder (default = None)] label: wgpu::Label<'a>,
) -> wgpu::RenderBundleDescriptor<'a> {
    wgpu::RenderBundleDescriptor { label: label }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn multisample_state(
    #[builder(default = 1)] count: u32,
    # [builder (default = ! 0)] mask: u64,
    #[builder(default = false)] alpha_to_coverage_enabled: bool,
) -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count: count,
        mask: mask,
        alpha_to_coverage_enabled: alpha_to_coverage_enabled,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texture_transition<T>(
    texture: T,
    selector: Option<wgpu::wgt::TextureSelector>,
    state: wgpu::TextureUses,
) -> wgpu::TextureTransition<T> {
    wgpu::TextureTransition {
        texture: texture,
        selector: selector,
        state: state,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blas_build_entry<'a>(
    blas: &'a wgpu::Blas,
    geometry: wgpu::BlasGeometries<'a>,
) -> wgpu::BlasBuildEntry<'a> {
    wgpu::BlasBuildEntry {
        blas: blas,
        geometry: geometry,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texture_descriptor<'a>(
    label: wgpu::Label<'a>,
    size: impl Nested<wgpu::Extent3d>,
    mip_level_count: u32,
    sample_count: u32,
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
    view_formats: &'a [wgpu::TextureFormat],
) -> wgpu::TextureDescriptor<'a> {
    wgpu::TextureDescriptor {
        label: label,
        size: size.unnest(),
        mip_level_count: mip_level_count,
        sample_count: sample_count,
        dimension: dimension,
        format: format,
        usage: usage,
        view_formats: view_formats,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn stencil_face_state(
    # [builder (default = wgpu :: CompareFunction :: Always)] compare: wgpu::CompareFunction,
    # [builder (default = wgpu :: StencilOperation :: Keep)] fail_op: wgpu::StencilOperation,
    # [builder (default = wgpu :: StencilOperation :: Keep)] depth_fail_op: wgpu::StencilOperation,
    # [builder (default = wgpu :: StencilOperation :: Keep)] pass_op: wgpu::StencilOperation,
) -> wgpu::StencilFaceState {
    wgpu::StencilFaceState {
        compare: compare,
        fail_op: fail_op,
        depth_fail_op: depth_fail_op,
        pass_op: pass_op,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn depth_stencil_state(
    format: wgpu::TextureFormat,
    depth_write_enabled: bool,
    depth_compare: wgpu::CompareFunction,
    stencil: impl Nested<wgpu::StencilState>,
    bias: impl Nested<wgpu::DepthBiasState>,
) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: format,
        depth_write_enabled: depth_write_enabled,
        depth_compare: depth_compare,
        stencil: stencil.unnest(),
        bias: bias.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn dispatch_indirect_args(
    #[builder(default)] x: u32,
    #[builder(default)] y: u32,
    #[builder(default)] z: u32,
) -> wgpu::util::DispatchIndirectArgs {
    wgpu::util::DispatchIndirectArgs { x: x, y: y, z: z }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn push_constant_range(
    stages: wgpu::ShaderStages,
    range: Range<u32>,
) -> wgpu::PushConstantRange {
    wgpu::PushConstantRange {
        stages: stages,
        range: range,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn shader_module_descriptor_passthrough<'a>(
    # [builder (default = "" . into ())] entry_point: String,
    # [builder (default = Default :: default ())] label: wgpu::Label<'a>,
    # [builder (default = (0 , 0 , 0))] num_workgroups: (u32, u32, u32),
    # [builder (default = wgpu :: ShaderRuntimeChecks :: unchecked ())] runtime_checks: impl Nested<
        wgpu::ShaderRuntimeChecks,
    >,
    spirv: Option<Cow<'a, [u32]>>,
    dxil: Option<Cow<'a, [u8]>>,
    msl: Option<Cow<'a, str>>,
    hlsl: Option<Cow<'a, str>>,
    glsl: Option<Cow<'a, str>>,
    wgsl: Option<Cow<'a, str>>,
) -> wgpu::ShaderModuleDescriptorPassthrough<'a> {
    wgpu::ShaderModuleDescriptorPassthrough {
        entry_point: entry_point,
        label: label,
        num_workgroups: num_workgroups,
        runtime_checks: runtime_checks.unnest(),
        spirv: spirv,
        dxil: dxil,
        msl: msl,
        hlsl: hlsl,
        glsl: glsl,
        wgsl: wgsl,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn task_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
) -> wgpu::TaskState<'a> {
    wgpu::TaskState {
        module: module,
        entry_point: entry_point,
        compilation_options: compilation_options.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_color_attachment<'tex>(
    view: &'tex wgpu::TextureView,
    depth_slice: Option<u32>,
    resolve_target: Option<&'tex wgpu::TextureView>,
    ops: impl Nested<wgpu::Operations<wgpu::Color>>,
) -> wgpu::RenderPassColorAttachment<'tex> {
    wgpu::RenderPassColorAttachment {
        view: view,
        depth_slice: depth_slice,
        resolve_target: resolve_target,
        ops: ops.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn external_texture_descriptor<'a>(
    label: wgpu::Label<'a>,
    width: u32,
    height: u32,
    format: wgpu::ExternalTextureFormat,
    yuv_conversion_matrix: [f32; 16],
    gamut_conversion_matrix: [f32; 9],
    src_transfer_function: impl Nested<wgpu::ExternalTextureTransferFunction>,
    dst_transfer_function: impl Nested<wgpu::ExternalTextureTransferFunction>,
    sample_transform: [f32; 6],
    load_transform: [f32; 6],
) -> wgpu::ExternalTextureDescriptor<'a> {
    wgpu::ExternalTextureDescriptor {
        label: label,
        width: width,
        height: height,
        format: format,
        yuv_conversion_matrix: yuv_conversion_matrix,
        gamut_conversion_matrix: gamut_conversion_matrix,
        src_transfer_function: src_transfer_function.unnest(),
        dst_transfer_function: dst_transfer_function.unnest(),
        sample_transform: sample_transform,
        load_transform: load_transform,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: &'a wgpu::BindGroupLayout,
    entries: &'a [wgpu::BindGroupEntry<'a>],
) -> wgpu::BindGroupDescriptor<'a> {
    wgpu::BindGroupDescriptor {
        label: label,
        layout: layout,
        entries: entries,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn shader_runtime_checks(
    bounds_checks: bool,
    force_loop_bounding: bool,
) -> wgpu::ShaderRuntimeChecks {
    wgpu::ShaderRuntimeChecks {
        bounds_checks: bounds_checks,
        force_loop_bounding: force_loop_bounding,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_texture_info<'a>(
    texture: &'a wgpu::Texture,
    mip_level: u32,
    origin: impl Nested<wgpu::Origin3d>,
    aspect: wgpu::TextureAspect,
) -> wgpu::TexelCopyTextureInfo<'a> {
    wgpu::TexelCopyTextureInfo {
        texture: texture,
        mip_level: mip_level,
        origin: origin.unnest(),
        aspect: aspect,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_init_descriptor<'a>(
    label: wgpu::Label<'a>,
    contents: &'a [u8],
    usage: wgpu::BufferUsages,
) -> wgpu::util::BufferInitDescriptor<'a> {
    wgpu::util::BufferInitDescriptor {
        label: label,
        contents: contents,
        usage: usage,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn fragment_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    targets: &'a [Option<wgpu::ColorTargetState>],
) -> wgpu::FragmentState<'a> {
    wgpu::FragmentState {
        module: module,
        entry_point: entry_point,
        compilation_options: compilation_options.unnest(),
        targets: targets,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn memory_budget_thresholds(
    for_resource_creation: Option<u8>,
    for_device_loss: Option<u8>,
) -> wgpu::MemoryBudgetThresholds {
    wgpu::MemoryBudgetThresholds {
        for_resource_creation: for_resource_creation,
        for_device_loss: for_device_loss,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn command_encoder_descriptor<'a>(
    # [builder (default = None)] label: wgpu::Label<'a>,
) -> wgpu::CommandEncoderDescriptor<'a> {
    wgpu::CommandEncoderDescriptor { label: label }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn device_descriptor<'a>(
    #[builder(default)] label: wgpu::Label<'a>,
    #[builder(default)] required_features: wgpu::Features,
    #[builder(default)] required_limits: wgpu::Limits,
    #[builder(default)] experimental_features: wgpu::ExperimentalFeatures,
    #[builder(default)] memory_hints: wgpu::MemoryHints,
    #[builder(default)] trace: wgpu::Trace,
) -> wgpu::DeviceDescriptor<'a> {
    wgpu::DeviceDescriptor {
        label: label,
        required_features: required_features,
        required_limits: required_limits,
        experimental_features: experimental_features,
        memory_hints: memory_hints,
        trace: trace,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_descriptor<'a>(
    #[builder(default)] label: wgpu::Label<'a>,
    #[builder(default)] color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
    depth_stencil_attachment: Option<impl Nested<wgpu::RenderPassDepthStencilAttachment<'a>>>,
    timestamp_writes: Option<impl Nested<wgpu::RenderPassTimestampWrites<'a>>>,
    occlusion_query_set: Option<&'a wgpu::QuerySet>,
) -> wgpu::RenderPassDescriptor<'a> {
    wgpu::RenderPassDescriptor {
        label: label,
        color_attachments: color_attachments,
        depth_stencil_attachment: depth_stencil_attachment.unnest(),
        timestamp_writes: timestamp_writes.unnest(),
        occlusion_query_set: occlusion_query_set,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn gl_backend_options(
    #[builder(default)] gles_minor_version: wgpu::Gles3MinorVersion,
    #[builder(default)] fence_behavior: wgpu::GlFenceBehavior,
) -> wgpu::GlBackendOptions {
    wgpu::GlBackendOptions {
        gles_minor_version: gles_minor_version,
        fence_behavior: fence_behavior,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn pipeline_cache_descriptor<'a>(
    label: wgpu::Label<'a>,
    data: Option<&'a [u8]>,
    fallback: bool,
) -> wgpu::PipelineCacheDescriptor<'a> {
    wgpu::PipelineCacheDescriptor {
        label: label,
        data: data,
        fallback: fallback,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn vertex_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    buffers: &'a [wgpu::VertexBufferLayout<'a>],
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module: module,
        entry_point: entry_point,
        compilation_options: compilation_options.unnest(),
        buffers: buffers,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn origin_2_d(x: u32, y: u32) -> wgpu::Origin2d {
    wgpu::Origin2d { x: x, y: y }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn backend_options(
    #[builder(default)] gl: impl Nested<wgpu::GlBackendOptions>,
    #[builder(default)] dx12: impl Nested<wgpu::Dx12BackendOptions>,
    #[builder(default)] noop: impl Nested<wgpu::NoopBackendOptions>,
) -> wgpu::BackendOptions {
    wgpu::BackendOptions {
        gl: gl.unnest(),
        dx12: dx12.unnest(),
        noop: noop.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn draw_indirect_args(
    #[builder(default)] vertex_count: u32,
    #[builder(default)] instance_count: u32,
    #[builder(default)] first_vertex: u32,
    #[builder(default)] first_instance: u32,
) -> wgpu::util::DrawIndirectArgs {
    wgpu::util::DrawIndirectArgs {
        vertex_count: vertex_count,
        instance_count: instance_count,
        first_vertex: first_vertex,
        first_instance: first_instance,
    }
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
        size: size,
        vertex_buffer: vertex_buffer,
        first_vertex: first_vertex,
        vertex_stride: vertex_stride,
        index_buffer: index_buffer,
        first_index: first_index,
        transform_buffer: transform_buffer,
        transform_buffer_offset: transform_buffer_offset,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn primitive_state(
    #[builder(default)] topology: wgpu::PrimitiveTopology,
    strip_index_format: Option<wgpu::IndexFormat>,
    #[builder(default)] front_face: wgpu::FrontFace,
    cull_mode: Option<wgpu::Face>,
    #[builder(default)] unclipped_depth: bool,
    #[builder(default)] polygon_mode: wgpu::PolygonMode,
    #[builder(default)] conservative: bool,
) -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology: topology,
        strip_index_format: strip_index_format,
        front_face: front_face,
        cull_mode: cull_mode,
        unclipped_depth: unclipped_depth,
        polygon_mode: polygon_mode,
        conservative: conservative,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compute_pipeline_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    cache: Option<&'a wgpu::PipelineCache>,
) -> wgpu::ComputePipelineDescriptor<'a> {
    wgpu::ComputePipelineDescriptor {
        label: label,
        layout: layout,
        module: module,
        entry_point: entry_point,
        compilation_options: compilation_options.unnest(),
        cache: cache,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_timestamp_writes<'a>(
    query_set: &'a wgpu::QuerySet,
    beginning_of_pass_write_index: Option<u32>,
    end_of_pass_write_index: Option<u32>,
) -> wgpu::RenderPassTimestampWrites<'a> {
    wgpu::RenderPassTimestampWrites {
        query_set: query_set,
        beginning_of_pass_write_index: beginning_of_pass_write_index,
        end_of_pass_write_index: end_of_pass_write_index,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn image_subresource_range(
    #[builder(default)] aspect: wgpu::TextureAspect,
    #[builder(default)] base_mip_level: u32,
    mip_level_count: Option<u32>,
    #[builder(default)] base_array_layer: u32,
    array_layer_count: Option<u32>,
) -> wgpu::ImageSubresourceRange {
    wgpu::ImageSubresourceRange {
        aspect: aspect,
        base_mip_level: base_mip_level,
        mip_level_count: mip_level_count,
        base_array_layer: base_array_layer,
        array_layer_count: array_layer_count,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn copy_external_image_dest_info<T>(
    texture: T,
    mip_level: u32,
    origin: impl Nested<wgpu::Origin3d>,
    aspect: wgpu::TextureAspect,
    color_space: wgpu::PredefinedColorSpace,
    premultiplied_alpha: bool,
) -> wgpu::CopyExternalImageDestInfo<T> {
    wgpu::CopyExternalImageDestInfo {
        texture: texture,
        mip_level: mip_level,
        origin: origin.unnest(),
        aspect: aspect,
        color_space: color_space,
        premultiplied_alpha: premultiplied_alpha,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn pipeline_compilation_options<'a>(
    # [builder (default = Default :: default ())] constants: &'a [(&'a str, f64)],
    #[builder(default = true)] zero_initialize_workgroup_memory: bool,
) -> wgpu::PipelineCompilationOptions<'a> {
    wgpu::PipelineCompilationOptions {
        constants: constants,
        zero_initialize_workgroup_memory: zero_initialize_workgroup_memory,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn query_set_descriptor<'a>(
    label: wgpu::Label<'a>,
    ty: wgpu::QueryType,
    count: u32,
) -> wgpu::QuerySetDescriptor<'a> {
    wgpu::QuerySetDescriptor {
        label: label,
        ty: ty,
        count: count,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn mesh_state<'a>(
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
) -> wgpu::MeshState<'a> {
    wgpu::MeshState {
        module: module,
        entry_point: entry_point,
        compilation_options: compilation_options.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn request_adapter_options_base<S>(
    # [builder (default = wgpu :: PowerPreference :: default ())]
    power_preference: wgpu::PowerPreference,
    #[builder(default = false)] force_fallback_adapter: bool,
    compatible_surface: Option<S>,
) -> wgpu::RequestAdapterOptionsBase<S> {
    wgpu::RequestAdapterOptionsBase {
        power_preference: power_preference,
        force_fallback_adapter: force_fallback_adapter,
        compatible_surface: compatible_surface,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pipeline_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex: impl Nested<wgpu::VertexState<'a>>,
    primitive: impl Nested<wgpu::PrimitiveState>,
    depth_stencil: Option<impl Nested<wgpu::DepthStencilState>>,
    multisample: impl Nested<wgpu::MultisampleState>,
    fragment: Option<impl Nested<wgpu::FragmentState<'a>>>,
    multiview: Option<NonZeroU32>,
    cache: Option<&'a wgpu::PipelineCache>,
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        label: label,
        layout: layout,
        vertex: vertex.unnest(),
        primitive: primitive.unnest(),
        depth_stencil: depth_stencil.unnest(),
        multisample: multisample.unnest(),
        fragment: fragment.unnest(),
        multiview: multiview,
        cache: cache,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn surface_configuration(
    usage: wgpu::TextureUsages,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
    present_mode: wgpu::PresentMode,
    desired_maximum_frame_latency: u32,
    alpha_mode: wgpu::CompositeAlphaMode,
    view_formats: Vec<wgpu::TextureFormat>,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: usage,
        format: format,
        width: width,
        height: height,
        present_mode: present_mode,
        desired_maximum_frame_latency: desired_maximum_frame_latency,
        alpha_mode: alpha_mode,
        view_formats: view_formats,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texture_view_descriptor<'a>(
    #[builder(default)] label: wgpu::Label<'a>,
    format: Option<wgpu::TextureFormat>,
    dimension: Option<wgpu::TextureViewDimension>,
    usage: Option<wgpu::TextureUsages>,
    #[builder(default)] aspect: wgpu::TextureAspect,
    #[builder(default)] base_mip_level: u32,
    mip_level_count: Option<u32>,
    #[builder(default)] base_array_layer: u32,
    array_layer_count: Option<u32>,
) -> wgpu::TextureViewDescriptor<'a> {
    wgpu::TextureViewDescriptor {
        label: label,
        format: format,
        dimension: dimension,
        usage: usage,
        aspect: aspect,
        base_mip_level: base_mip_level,
        mip_level_count: mip_level_count,
        base_array_layer: base_array_layer,
        array_layer_count: array_layer_count,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn extent_3_d(
    #[builder(default = 1)] width: u32,
    #[builder(default = 1)] height: u32,
    #[builder(default = 1)] depth_or_array_layers: u32,
) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width: width,
        height: height,
        depth_or_array_layers: depth_or_array_layers,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn dx_12_backend_options(
    #[builder(default)] shader_compiler: wgpu::Dx12Compiler,
    #[builder(default)] presentation_system: wgpu::wgt::Dx12SwapchainKind,
    #[builder(default)] latency_waitable_object: wgpu::wgt::Dx12UseFrameLatencyWaitableObject,
) -> wgpu::Dx12BackendOptions {
    wgpu::Dx12BackendOptions {
        shader_compiler: shader_compiler,
        presentation_system: presentation_system,
        latency_waitable_object: latency_waitable_object,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_buffer_layout(
    #[builder(default)] offset: wgpu::BufferAddress,
    bytes_per_row: Option<u32>,
    rows_per_image: Option<u32>,
) -> wgpu::TexelCopyBufferLayout {
    wgpu::TexelCopyBufferLayout {
        offset: offset,
        bytes_per_row: bytes_per_row,
        rows_per_image: rows_per_image,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_buffer_info<'a>(
    buffer: &'a wgpu::Buffer,
    layout: impl Nested<wgpu::TexelCopyBufferLayout>,
) -> wgpu::TexelCopyBufferInfo<'a> {
    wgpu::TexelCopyBufferInfo {
        buffer: buffer,
        layout: layout.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn depth_bias_state(
    #[builder(default)] constant: i32,
    #[builder(default)] slope_scale: f32,
    #[builder(default)] clamp: f32,
) -> wgpu::DepthBiasState {
    wgpu::DepthBiasState {
        constant: constant,
        slope_scale: slope_scale,
        clamp: clamp,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn command_buffer_descriptor<L: Default>(
    #[builder(default)] label: L,
) -> wgpu::CommandBufferDescriptor<L> {
    wgpu::CommandBufferDescriptor { label: label }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_entry<'a>(
    binding: u32,
    resource: wgpu::BindingResource<'a>,
) -> wgpu::BindGroupEntry<'a> {
    wgpu::BindGroupEntry {
        binding: binding,
        resource: resource,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn stencil_state(
    #[builder(default)] front: impl Nested<wgpu::StencilFaceState>,
    #[builder(default)] back: impl Nested<wgpu::StencilFaceState>,
    #[builder(default)] read_mask: u32,
    #[builder(default)] write_mask: u32,
) -> wgpu::StencilState {
    wgpu::StencilState {
        front: front.unnest(),
        back: back.unnest(),
        read_mask: read_mask,
        write_mask: write_mask,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn operations<V: Default>(
    # [builder (default = wgpu :: LoadOp :: default ())] load: wgpu::LoadOp<V>,
    # [builder (default = wgpu :: StoreOp :: default ())] store: wgpu::StoreOp,
) -> wgpu::Operations<V> {
    wgpu::Operations {
        load: load,
        store: store,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn color(
    #[builder(default)] r: f64,
    #[builder(default)] g: f64,
    #[builder(default)] b: f64,
    #[builder(default)] a: f64,
) -> wgpu::Color {
    wgpu::Color {
        r: r,
        g: g,
        b: b,
        a: a,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn color_target_state(
    format: wgpu::TextureFormat,
    blend: Option<impl Nested<wgpu::BlendState>>,
    write_mask: wgpu::ColorWrites,
) -> wgpu::ColorTargetState {
    wgpu::ColorTargetState {
        format: format,
        blend: blend.unnest(),
        write_mask: write_mask,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_layout_entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    ty: wgpu::BindingType,
    count: Option<NonZeroU32>,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: binding,
        visibility: visibility,
        ty: ty,
        count: count,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compute_pass_timestamp_writes<'a>(
    query_set: &'a wgpu::QuerySet,
    beginning_of_pass_write_index: Option<u32>,
    end_of_pass_write_index: Option<u32>,
) -> wgpu::ComputePassTimestampWrites<'a> {
    wgpu::ComputePassTimestampWrites {
        query_set: query_set,
        beginning_of_pass_write_index: beginning_of_pass_write_index,
        end_of_pass_write_index: end_of_pass_write_index,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn mesh_pipeline_descriptor<'a>(
    label: wgpu::Label<'a>,
    layout: Option<&'a wgpu::PipelineLayout>,
    task: Option<impl Nested<wgpu::TaskState<'a>>>,
    mesh: impl Nested<wgpu::MeshState<'a>>,
    primitive: impl Nested<wgpu::PrimitiveState>,
    depth_stencil: Option<impl Nested<wgpu::DepthStencilState>>,
    multisample: impl Nested<wgpu::MultisampleState>,
    fragment: Option<impl Nested<wgpu::FragmentState<'a>>>,
    multiview: Option<NonZeroU32>,
    cache: Option<&'a wgpu::PipelineCache>,
) -> wgpu::MeshPipelineDescriptor<'a> {
    wgpu::MeshPipelineDescriptor {
        label: label,
        layout: layout,
        task: task.unnest(),
        mesh: mesh.unnest(),
        primitive: primitive.unnest(),
        depth_stencil: depth_stencil.unnest(),
        multisample: multisample.unnest(),
        fragment: fragment.unnest(),
        multiview: multiview,
        cache: cache,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn vertex_buffer_layout<'a>(
    array_stride: wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode,
    attributes: &'a [wgpu::VertexAttribute],
) -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
        array_stride: array_stride,
        step_mode: step_mode,
        attributes: attributes,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn buffer_binding<'a>(
    buffer: &'a wgpu::Buffer,
    offset: wgpu::BufferAddress,
    size: Option<wgpu::BufferSize>,
) -> wgpu::BufferBinding<'a> {
    wgpu::BufferBinding {
        buffer: buffer,
        offset: offset,
        size: size,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn bind_group_layout_descriptor<'a>(
    label: wgpu::Label<'a>,
    entries: &'a [wgpu::BindGroupLayoutEntry],
) -> wgpu::BindGroupLayoutDescriptor<'a> {
    wgpu::BindGroupLayoutDescriptor {
        label: label,
        entries: entries,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn origin_3_d(
    #[builder(default = 0)] x: u32,
    #[builder(default = 0)] y: u32,
    #[builder(default = 0)] z: u32,
) -> wgpu::Origin3d {
    wgpu::Origin3d { x: x, y: y, z: z }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn create_blas_descriptor<'a>(
    label: wgpu::Label<'a>,
    flags: wgpu::wgt::AccelerationStructureFlags,
    update_mode: wgpu::wgt::AccelerationStructureUpdateMode,
) -> wgpu::CreateBlasDescriptor<'a> {
    wgpu::CreateBlasDescriptor {
        label: label,
        flags: flags,
        update_mode: update_mode,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn instance_descriptor(
    #[builder(default)] backends: wgpu::Backends,
    #[builder(default)] flags: wgpu::InstanceFlags,
    #[builder(default)] memory_budget_thresholds: impl Nested<wgpu::MemoryBudgetThresholds>,
    #[builder(default)] backend_options: impl Nested<wgpu::BackendOptions>,
) -> wgpu::InstanceDescriptor {
    wgpu::InstanceDescriptor {
        backends: backends,
        flags: flags,
        memory_budget_thresholds: memory_budget_thresholds.unnest(),
        backend_options: backend_options.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn shader_module_descriptor<'a>(
    label: wgpu::Label<'a>,
    source: wgpu::ShaderSource<'a>,
) -> wgpu::ShaderModuleDescriptor<'a> {
    wgpu::ShaderModuleDescriptor {
        label: label,
        source: source,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn request_adapter_options<'a, 'b>(
    # [builder (default = wgpu :: PowerPreference :: default ())]
    power_preference: wgpu::PowerPreference,
    #[builder(default = false)] force_fallback_adapter: bool,
    compatible_surface: Option<&'a wgpu::Surface<'b>>,
) -> wgpu::RequestAdapterOptions<'a, 'b> {
    wgpu::RequestAdapterOptions {
        power_preference: power_preference,
        force_fallback_adapter: force_fallback_adapter,
        compatible_surface: compatible_surface,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_bundle_depth_stencil(
    format: wgpu::TextureFormat,
    depth_read_only: bool,
    stencil_read_only: bool,
) -> wgpu::RenderBundleDepthStencil {
    wgpu::RenderBundleDepthStencil {
        format: format,
        depth_read_only: depth_read_only,
        stencil_read_only: stencil_read_only,
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn render_pass_depth_stencil_attachment<'tex>(
    view: &'tex wgpu::TextureView,
    depth_ops: Option<impl Nested<wgpu::Operations<f32>>>,
    stencil_ops: Option<impl Nested<wgpu::Operations<u32>>>,
) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
    wgpu::RenderPassDepthStencilAttachment {
        view: view,
        depth_ops: depth_ops.unnest(),
        stencil_ops: stencil_ops.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compute_pass_descriptor<'a>(
    #[builder(default)] label: wgpu::Label<'a>,
    timestamp_writes: Option<impl Nested<wgpu::ComputePassTimestampWrites<'a>>>,
) -> wgpu::ComputePassDescriptor<'a> {
    wgpu::ComputePassDescriptor {
        label: label,
        timestamp_writes: timestamp_writes.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn downlevel_limits() -> wgpu::DownlevelLimits {
    wgpu::DownlevelLimits {}
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn blend_state(
    color: impl Nested<wgpu::BlendComponent>,
    alpha: impl Nested<wgpu::BlendComponent>,
) -> wgpu::BlendState {
    wgpu::BlendState {
        color: color.unnest(),
        alpha: alpha.unnest(),
    }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn noop_backend_options(#[builder(default)] enable: bool) -> wgpu::NoopBackendOptions {
    wgpu::NoopBackendOptions { enable: enable }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn compilation_info(messages: Vec<wgpu::CompilationMessage>) -> wgpu::CompilationInfo {
    wgpu::CompilationInfo { messages: messages }
}

# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)]
pub fn texel_copy_texture_info_base<T>(
    texture: T,
    mip_level: u32,
    origin: impl Nested<wgpu::Origin3d>,
    aspect: wgpu::TextureAspect,
) -> wgpu::TexelCopyTextureInfoBase<T> {
    wgpu::TexelCopyTextureInfoBase {
        texture: texture,
        mip_level: mip_level,
        origin: origin.unnest(),
        aspect: aspect,
    }
}
