use std::borrow::Cow;
use std::num::NonZeroU32;
use std::ops::Range;

use wgpu::util::*;
use wgpu::wgt::{Dx12SwapchainKind, Dx12UseFrameLatencyWaitableObject, TextureSelector};
use wgpu::*;

/*
Unhandled Some("TextureTransition") Id(4291)

*/
#[doc = "\n        Returns [`TextureTransitionBuilder`] for building [`wgpu::TextureTransition`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texture_transition<T>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureTransition::texture`]"]
    #[builder(into)]
    texture: T,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureTransition::selector`]"]
    #[builder(into)]
    selector: Option<TextureSelector>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureTransition::state`]"]
    #[builder(into)]
    state: TextureUses,
) -> TextureTransition<T> {
    TextureTransition {
        texture,
        selector,
        state,
    }
}

/*
Unhandled Some("RenderBundleDepthStencil") Id(4998)

*/
#[doc = "\n        Returns [`RenderBundleDepthStencilBuilder`] for building [`wgpu::RenderBundleDepthStencil`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_bundle_depth_stencil(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleDepthStencil::format`]"]
    #[builder(into)]
    format: TextureFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleDepthStencil::depth_read_only`]"]
    #[builder(into)]
    depth_read_only: bool,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleDepthStencil::stencil_read_only`]"]
    #[builder(into)]
    stencil_read_only: bool,
) -> RenderBundleDepthStencil {
    RenderBundleDepthStencil {
        format,
        depth_read_only,
        stencil_read_only,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`ImageSubresourceRangeBuilder`] for building [`wgpu::ImageSubresourceRange`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn image_subresource_range(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ImageSubresourceRange::aspect`]"]
    #[builder(into, default)]
    aspect: TextureAspect,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ImageSubresourceRange::base_mip_level`]"]
    #[builder(into, default)]
    base_mip_level: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ImageSubresourceRange::mip_level_count`]"]
    #[builder(into)]
    mip_level_count: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ImageSubresourceRange::base_array_layer`]"]
    #[builder(into, default)]
    base_array_layer: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ImageSubresourceRange::array_layer_count`]"]
    #[builder(into)]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:1728
    pub const REPLACE: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::Zero,
        operation: BlendOperation::Add,
    };

*/
#[doc = "\n        Returns [`BlendComponentBuilder`] for building [`wgpu::BlendComponent`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn blend_component(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlendComponent::src_factor`]"]
    # [builder (into , default = BlendFactor :: One)]
    src_factor: BlendFactor,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlendComponent::dst_factor`]"]
    # [builder (into , default = BlendFactor :: Zero)]
    dst_factor: BlendFactor,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlendComponent::operation`]"]
    # [builder (into , default = BlendOperation :: Add)]
    operation: BlendOperation,
) -> BlendComponent {
    BlendComponent {
        src_factor,
        dst_factor,
        operation,
    }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:7869
impl Default for ShaderRuntimeChecks {
    fn default() -> Self {
        Self::checked()
    }
}

*/
#[doc = "\n        Returns [`ShaderRuntimeChecksBuilder`] for building [`wgpu::ShaderRuntimeChecks`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn shader_runtime_checks(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderRuntimeChecks::bounds_checks`]"]
    #[builder(into)]
    bounds_checks: bool,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderRuntimeChecks::force_loop_bounding`]"]
    #[builder(into)]
    force_loop_bounding: bool,
) -> ShaderRuntimeChecks {
    ShaderRuntimeChecks {
        bounds_checks,
        force_loop_bounding,
    }
}

/*
Unhandled Some("DepthStencilState") Id(3204)

*/
#[doc = "\n        Returns [`DepthStencilStateBuilder`] for building [`wgpu::DepthStencilState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn depth_stencil_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthStencilState::format`]"]
    #[builder(into)]
    format: TextureFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthStencilState::depth_write_enabled`]"]
    #[builder(into)]
    depth_write_enabled: bool,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthStencilState::depth_compare`]"]
    #[builder(into)]
    depth_compare: CompareFunction,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthStencilState::stencil`]"]
    #[builder(into, default)]
    stencil: StencilState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthStencilState::bias`]"]
    #[builder(into, default)]
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

/*
Derived default
*/
#[doc = "\n        Returns [`DrawIndirectArgsBuilder`] for building [`wgpu::DrawIndirectArgs`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn draw_indirect_args(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndirectArgs::vertex_count`]"]
    #[builder(into, default)]
    vertex_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndirectArgs::instance_count`]"]
    #[builder(into, default)]
    instance_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndirectArgs::first_vertex`]"]
    #[builder(into, default)]
    first_vertex: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndirectArgs::first_instance`]"]
    #[builder(into, default)]
    first_instance: u32,
) -> DrawIndirectArgs {
    DrawIndirectArgs {
        vertex_count,
        instance_count,
        first_vertex,
        first_instance,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`TexelCopyBufferLayoutBuilder`] for building [`wgpu::TexelCopyBufferLayout`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texel_copy_buffer_layout(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferLayout::offset`]"]
    #[builder(into, default)]
    offset: BufferAddress,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferLayout::bytes_per_row`]"]
    #[builder(into)]
    bytes_per_row: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferLayout::rows_per_image`]"]
    #[builder(into)]
    rows_per_image: Option<u32>,
) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset,
        bytes_per_row,
        rows_per_image,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`BackendOptionsBuilder`] for building [`wgpu::BackendOptions`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn backend_options(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BackendOptions::gl`]"]
    #[builder(into, default)]
    gl: GlBackendOptions,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BackendOptions::dx12`]"]
    #[builder(into, default)]
    dx12: Dx12BackendOptions,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BackendOptions::noop`]"]
    #[builder(into, default)]
    noop: NoopBackendOptions,
) -> BackendOptions {
    BackendOptions { gl, dx12, noop }
}

/*
Derived default
*/
#[doc = "\n        Returns [`DrawIndexedIndirectArgsBuilder`] for building [`wgpu::DrawIndexedIndirectArgs`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn draw_indexed_indirect_args(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndexedIndirectArgs::index_count`]"]
    #[builder(into, default)]
    index_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndexedIndirectArgs::instance_count`]"]
    #[builder(into, default)]
    instance_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndexedIndirectArgs::first_index`]"]
    #[builder(into, default)]
    first_index: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndexedIndirectArgs::base_vertex`]"]
    #[builder(into, default)]
    base_vertex: i32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DrawIndexedIndirectArgs::first_instance`]"]
    #[builder(into, default)]
    first_instance: u32,
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
Unhandled Some("PushConstantRange") Id(4938)

*/
#[doc = "\n        Returns [`PushConstantRangeBuilder`] for building [`wgpu::PushConstantRange`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn push_constant_range(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PushConstantRange::stages`]"]
    #[builder(into)]
    stages: ShaderStages,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PushConstantRange::range`]"]
    #[builder(into)]
    range: Range<u32>,
) -> PushConstantRange {
    PushConstantRange { stages, range }
}

/*
Derived default
*/
#[doc = "\n        Returns [`Dx12BackendOptionsBuilder`] for building [`wgpu::Dx12BackendOptions`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn dx_12_backend_options(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Dx12BackendOptions::shader_compiler`]"]
    #[builder(into, default)]
    shader_compiler: Dx12Compiler,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Dx12BackendOptions::presentation_system`]"]
    #[builder(into, default)]
    presentation_system: Dx12SwapchainKind,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Dx12BackendOptions::latency_waitable_object`]"]
    #[builder(into, default)]
    latency_waitable_object: Dx12UseFrameLatencyWaitableObject,
) -> Dx12BackendOptions {
    Dx12BackendOptions {
        shader_compiler,
        presentation_system,
        latency_waitable_object,
    }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:1152
impl Default for DownlevelLimits {
    fn default() -> Self {
        DownlevelLimits {}
    }
}

*/
#[doc = "\n        Returns [`DownlevelLimitsBuilder`] for building [`wgpu::DownlevelLimits`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn downlevel_limits() -> DownlevelLimits {
    DownlevelLimits {}
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:374
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
#[doc = "\n        Returns [`RequestAdapterOptionsBaseBuilder`] for building [`wgpu::RequestAdapterOptionsBase`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn request_adapter_options_base<S>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RequestAdapterOptionsBase::power_preference`]"]
    # [builder (into , default = PowerPreference :: default ())]
    power_preference: PowerPreference,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RequestAdapterOptionsBase::force_fallback_adapter`]"]
    #[builder(into, default = false)]
    force_fallback_adapter: bool,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RequestAdapterOptionsBase::compatible_surface`]"]
    #[builder(into)]
    compatible_surface: Option<S>,
) -> RequestAdapterOptionsBase<S> {
    RequestAdapterOptionsBase {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

/*
Unhandled Some("BlendState") Id(2373)

*/
#[doc = "\n        Returns [`BlendStateBuilder`] for building [`wgpu::BlendState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn blend_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlendState::color`]"]
    #[builder(into, default)]
    color: BlendComponent,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlendState::alpha`]"]
    #[builder(into, default)]
    alpha: BlendComponent,
) -> BlendState {
    BlendState { color, alpha }
}

/*
Unhandled Some("CopyExternalImageDestInfo") Id(5305)

*/
#[doc = "\n        Returns [`CopyExternalImageDestInfoBuilder`] for building [`wgpu::CopyExternalImageDestInfo`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn copy_external_image_dest_info<T>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CopyExternalImageDestInfo::texture`]"]
    #[builder(into)]
    texture: T,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CopyExternalImageDestInfo::mip_level`]"]
    #[builder(into)]
    mip_level: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CopyExternalImageDestInfo::origin`]"]
    #[builder(into, default)]
    origin: Origin3d,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CopyExternalImageDestInfo::aspect`]"]
    #[builder(into, default)]
    aspect: TextureAspect,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CopyExternalImageDestInfo::color_space`]"]
    #[builder(into)]
    color_space: PredefinedColorSpace,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CopyExternalImageDestInfo::premultiplied_alpha`]"]
    #[builder(into)]
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

/*
Unhandled Some("Origin2d") Id(4515)

*/
#[doc = "\n        Returns [`Origin2dBuilder`] for building [`wgpu::Origin2d`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn origin2d(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Origin2d::x`]"]
    #[builder(into)]
    x: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Origin2d::y`]"]
    #[builder(into)]
    y: u32,
) -> Origin2d {
    Origin2d { x, y }
}

/*
Derived default
*/
#[doc = "\n        Returns [`MemoryBudgetThresholdsBuilder`] for building [`wgpu::MemoryBudgetThresholds`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn memory_budget_thresholds(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MemoryBudgetThresholds::for_resource_creation`]"]
    #[builder(into)]
    for_resource_creation: Option<u8>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MemoryBudgetThresholds::for_device_loss`]"]
    #[builder(into)]
    for_device_loss: Option<u8>,
) -> MemoryBudgetThresholds {
    MemoryBudgetThresholds {
        for_resource_creation,
        for_device_loss,
    }
}

/*
Unhandled Some("TexelCopyTextureInfo") Id(4844)

*/
#[doc = "\n        Returns [`TexelCopyTextureInfoBaseBuilder`] for building [`wgpu::TexelCopyTextureInfoBase`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texel_copy_texture_info_base<T>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfoBase::texture`]"]
    #[builder(into)]
    texture: T,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfoBase::mip_level`]"]
    #[builder(into)]
    mip_level: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfoBase::origin`]"]
    #[builder(into, default)]
    origin: Origin3d,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfoBase::aspect`]"]
    #[builder(into, default)]
    aspect: TextureAspect,
) -> TexelCopyTextureInfoBase<T> {
    TexelCopyTextureInfoBase {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`DepthBiasStateBuilder`] for building [`wgpu::DepthBiasState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn depth_bias_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthBiasState::constant`]"]
    #[builder(into, default)]
    constant: i32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthBiasState::slope_scale`]"]
    #[builder(into, default)]
    slope_scale: f32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DepthBiasState::clamp`]"]
    #[builder(into, default)]
    clamp: f32,
) -> DepthBiasState {
    DepthBiasState {
        constant,
        slope_scale,
        clamp,
    }
}

/*
Unhandled Some("VertexAttribute") Id(3577)

*/
#[doc = "\n        Returns [`VertexAttributeBuilder`] for building [`wgpu::VertexAttribute`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn vertex_attribute(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexAttribute::format`]"]
    #[builder(into)]
    format: VertexFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexAttribute::offset`]"]
    #[builder(into)]
    offset: BufferAddress,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexAttribute::shader_location`]"]
    #[builder(into)]
    shader_location: ShaderLocation,
) -> VertexAttribute {
    VertexAttribute {
        format,
        offset,
        shader_location,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`InstanceDescriptorBuilder`] for building [`wgpu::InstanceDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn instance_descriptor(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::InstanceDescriptor::backends`]"]
    #[builder(into, default)]
    backends: Backends,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::InstanceDescriptor::flags`]"]
    #[builder(into, default)]
    flags: InstanceFlags,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::InstanceDescriptor::memory_budget_thresholds`]"]
    #[builder(into, default)]
    memory_budget_thresholds: MemoryBudgetThresholds,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::InstanceDescriptor::backend_options`]"]
    #[builder(into, default)]
    backend_options: BackendOptions,
) -> InstanceDescriptor {
    InstanceDescriptor {
        backends,
        flags,
        memory_budget_thresholds,
        backend_options,
    }
}

/*
Unhandled Some("BindGroupLayoutEntry") Id(5215)

*/
#[doc = "\n        Returns [`BindGroupLayoutEntryBuilder`] for building [`wgpu::BindGroupLayoutEntry`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn bind_group_layout_entry(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupLayoutEntry::binding`]"]
    #[builder(into)]
    binding: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupLayoutEntry::visibility`]"]
    #[builder(into)]
    visibility: ShaderStages,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupLayoutEntry::ty`]"]
    #[builder(into)]
    ty: BindingType,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupLayoutEntry::count`]"]
    #[builder(into)]
    count: Option<NonZeroU32>,
) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`StencilStateBuilder`] for building [`wgpu::StencilState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn stencil_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilState::front`]"]
    #[builder(into, default)]
    front: StencilFaceState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilState::back`]"]
    #[builder(into, default)]
    back: StencilFaceState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilState::read_mask`]"]
    #[builder(into, default)]
    read_mask: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilState::write_mask`]"]
    #[builder(into, default)]
    write_mask: u32,
) -> StencilState {
    StencilState {
        front,
        back,
        read_mask,
        write_mask,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`ColorBuilder`] for building [`wgpu::Color`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn color(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Color::r`]"]
    #[builder(into, default)]
    r: f64,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Color::g`]"]
    #[builder(into, default)]
    g: f64,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Color::b`]"]
    #[builder(into, default)]
    b: f64,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Color::a`]"]
    #[builder(into, default)]
    a: f64,
) -> Color {
    Color { r, g, b, a }
}

/*
Derived default
*/
#[doc = "\n        Returns [`NoopBackendOptionsBuilder`] for building [`wgpu::NoopBackendOptions`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn noop_backend_options(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::NoopBackendOptions::enable`]"]
    #[builder(into, default)]
    enable: bool,
) -> NoopBackendOptions {
    NoopBackendOptions { enable }
}

/*
Derived default
*/
#[doc = "\n        Returns [`PrimitiveStateBuilder`] for building [`wgpu::PrimitiveState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn primitive_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::topology`]"]
    #[builder(into, default)]
    topology: PrimitiveTopology,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::strip_index_format`]"]
    #[builder(into)]
    strip_index_format: Option<IndexFormat>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::front_face`]"]
    #[builder(into, default)]
    front_face: FrontFace,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::cull_mode`]"]
    #[builder(into)]
    cull_mode: Option<Face>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::unclipped_depth`]"]
    #[builder(into, default)]
    unclipped_depth: bool,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::polygon_mode`]"]
    #[builder(into, default)]
    polygon_mode: PolygonMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PrimitiveState::conservative`]"]
    #[builder(into, default)]
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

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6059
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
#[doc = "\n        Returns [`Extent3dBuilder`] for building [`wgpu::Extent3d`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn extent3d(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Extent3d::width`]"]
    #[builder(into, default = 1u32)]
    width: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Extent3d::height`]"]
    #[builder(into, default = 1u32)]
    height: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Extent3d::depth_or_array_layers`]"]
    #[builder(into, default = 1u32)]
    depth_or_array_layers: u32,
) -> Extent3d {
    Extent3d {
        width,
        height,
        depth_or_array_layers,
    }
}

/*
Unhandled Some("TexelCopyBufferInfo") Id(5278)

*/
#[doc = "\n        Returns [`TexelCopyBufferInfoBaseBuilder`] for building [`wgpu::TexelCopyBufferInfoBase`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texel_copy_buffer_info_base<B>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferInfoBase::buffer`]"]
    #[builder(into)]
    buffer: B,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferInfoBase::layout`]"]
    #[builder(into, default)]
    layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfoBase<B> {
    TexelCopyBufferInfoBase { buffer, layout }
}

/*
Derived default
*/
#[doc = "\n        Returns [`CoreCountersBuilder`] for building [`wgpu::CoreCounters`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn core_counters() -> CoreCounters {
    CoreCounters {}
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:1992
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
#[doc = "\n        Returns [`MultisampleStateBuilder`] for building [`wgpu::MultisampleState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn multisample_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MultisampleState::count`]"]
    #[builder(into, default = 1u32)]
    count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MultisampleState::mask`]"]
    # [builder (into , default = ! 0u64)]
    mask: u64,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MultisampleState::alpha_to_coverage_enabled`]"]
    #[builder(into, default = false)]
    alpha_to_coverage_enabled: bool,
) -> MultisampleState {
    MultisampleState {
        count,
        mask,
        alpha_to_coverage_enabled,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`CommandBufferDescriptorBuilder`] for building [`wgpu::CommandBufferDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn command_buffer_descriptor<L>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CommandBufferDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: L,
) -> CommandBufferDescriptor<L> {
    CommandBufferDescriptor { label }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6435
impl Default for ExternalTextureTransferFunction {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 1.0,
            g: 1.0,
            k: 1.0,
        }
    }
}

*/
#[doc = "\n        Returns [`ExternalTextureTransferFunctionBuilder`] for building [`wgpu::ExternalTextureTransferFunction`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn external_texture_transfer_function(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureTransferFunction::a`]"]
    #[builder(into, default = 1.0)]
    a: f32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureTransferFunction::b`]"]
    #[builder(into, default = 1.0)]
    b: f32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureTransferFunction::g`]"]
    #[builder(into, default = 1.0)]
    g: f32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureTransferFunction::k`]"]
    #[builder(into, default = 1.0)]
    k: f32,
) -> ExternalTextureTransferFunction {
    ExternalTextureTransferFunction { a, b, g, k }
}

/*
Unhandled Some("ColorTargetState") Id(2409)

*/
#[doc = "\n        Returns [`ColorTargetStateBuilder`] for building [`wgpu::ColorTargetState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn color_target_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ColorTargetState::format`]"]
    #[builder(into)]
    format: TextureFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ColorTargetState::blend`]"]
    #[builder(into)]
    blend: Option<BlendState>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ColorTargetState::write_mask`]"]
    #[builder(into, default)]
    write_mask: ColorWrites,
) -> ColorTargetState {
    ColorTargetState {
        format,
        blend,
        write_mask,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`GlBackendOptionsBuilder`] for building [`wgpu::GlBackendOptions`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn gl_backend_options(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::GlBackendOptions::gles_minor_version`]"]
    #[builder(into, default)]
    gles_minor_version: Gles3MinorVersion,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::GlBackendOptions::fence_behavior`]"]
    #[builder(into, default)]
    fence_behavior: GlFenceBehavior,
) -> GlBackendOptions {
    GlBackendOptions {
        gles_minor_version,
        fence_behavior,
    }
}

/*
Unhandled Some("BufferTransition") Id(3909)

*/
#[doc = "\n        Returns [`BufferTransitionBuilder`] for building [`wgpu::BufferTransition`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn buffer_transition<T>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferTransition::buffer`]"]
    #[builder(into)]
    buffer: T,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferTransition::state`]"]
    #[builder(into)]
    state: BufferUses,
) -> BufferTransition<T> {
    BufferTransition { buffer, state }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:4927
    pub const IGNORE: Self = StencilFaceState {
        compare: CompareFunction::Always,
        fail_op: StencilOperation::Keep,
        depth_fail_op: StencilOperation::Keep,
        pass_op: StencilOperation::Keep,
    };

*/
#[doc = "\n        Returns [`StencilFaceStateBuilder`] for building [`wgpu::StencilFaceState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn stencil_face_state(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilFaceState::compare`]"]
    # [builder (into , default = CompareFunction :: Always)]
    compare: CompareFunction,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilFaceState::fail_op`]"]
    # [builder (into , default = StencilOperation :: Keep)]
    fail_op: StencilOperation,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilFaceState::depth_fail_op`]"]
    # [builder (into , default = StencilOperation :: Keep)]
    depth_fail_op: StencilOperation,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::StencilFaceState::pass_op`]"]
    # [builder (into , default = StencilOperation :: Keep)]
    pass_op: StencilOperation,
) -> StencilFaceState {
    StencilFaceState {
        compare,
        fail_op,
        depth_fail_op,
        pass_op,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`DispatchIndirectArgsBuilder`] for building [`wgpu::DispatchIndirectArgs`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn dispatch_indirect_args(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DispatchIndirectArgs::x`]"]
    #[builder(into, default)]
    x: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DispatchIndirectArgs::y`]"]
    #[builder(into, default)]
    y: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DispatchIndirectArgs::z`]"]
    #[builder(into, default)]
    z: u32,
) -> DispatchIndirectArgs {
    DispatchIndirectArgs { x, y, z }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6006
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

*/
#[doc = "\n        Returns [`Origin3dBuilder`] for building [`wgpu::Origin3d`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn origin3d(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Origin3d::x`]"]
    #[builder(into, default = 0u32)]
    x: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Origin3d::y`]"]
    #[builder(into, default = 0u32)]
    y: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Origin3d::z`]"]
    #[builder(into, default = 0u32)]
    z: u32,
) -> Origin3d {
    Origin3d { x, y, z }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:4786
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
#[doc = "\n        Returns [`OperationsBuilder`] for building [`wgpu::Operations`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn operations<V>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Operations::load`]"]
    #[builder(into)]
    load: LoadOp<V>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::Operations::store`]"]
    # [builder (into , default = StoreOp :: default ())]
    store: StoreOp,
) -> Operations<V> {
    Operations { load, store }
}

/*
Unhandled Some("CompilationInfo") Id(2541)

*/
#[doc = "\n        Returns [`CompilationInfoBuilder`] for building [`wgpu::CompilationInfo`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn compilation_info(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CompilationInfo::messages`]"]
    #[builder(into)]
    messages: Vec<CompilationMessage>,
) -> CompilationInfo {
    CompilationInfo { messages }
}

/*
Unhandled Some("VertexBufferLayout") Id(646)

*/
#[doc = "\n        Returns [`VertexBufferLayoutBuilder`] for building [`wgpu::VertexBufferLayout`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn vertex_buffer_layout<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexBufferLayout::array_stride`]"]
    #[builder(into)]
    array_stride: BufferAddress,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexBufferLayout::step_mode`]"]
    #[builder(into, default)]
    step_mode: VertexStepMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexBufferLayout::attributes`]"]
    attributes: &'a [VertexAttribute],
) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
        array_stride,
        step_mode,
        attributes,
    }
}

/*
Unhandled Some("BindGroupEntry") Id(270)

*/
#[doc = "\n        Returns [`BindGroupEntryBuilder`] for building [`wgpu::BindGroupEntry`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn bind_group_entry<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupEntry::binding`]"]
    #[builder(into)]
    binding: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupEntry::resource`]"]
    #[builder(into)]
    resource: BindingResource<'a>,
) -> BindGroupEntry<'a> {
    BindGroupEntry { binding, resource }
}

/*
Unhandled Some("PipelineCacheDescriptor") Id(1042)

*/
#[doc = "\n        Returns [`PipelineCacheDescriptorBuilder`] for building [`wgpu::PipelineCacheDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn pipeline_cache_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineCacheDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineCacheDescriptor::data`]"]
    #[builder(into)]
    data: Option<&'a [u8]>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineCacheDescriptor::fallback`]"]
    #[builder(into)]
    fallback: bool,
) -> PipelineCacheDescriptor<'a> {
    PipelineCacheDescriptor {
        label,
        data,
        fallback,
    }
}

/*
Unhandled Some("ShaderModuleDescriptor") Id(1253)

*/
#[doc = "\n        Returns [`ShaderModuleDescriptorBuilder`] for building [`wgpu::ShaderModuleDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn shader_module_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptor::source`]"]
    #[builder(into)]
    source: ShaderSource<'a>,
) -> ShaderModuleDescriptor<'a> {
    ShaderModuleDescriptor { label, source }
}

/*
Derived default
*/
#[doc = "\n        Returns [`RenderBundleEncoderDescriptorBuilder`] for building [`wgpu::RenderBundleEncoderDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_bundle_encoder_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleEncoderDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleEncoderDescriptor::color_formats`]"]
    #[builder(default)]
    color_formats: &'a [Option<TextureFormat>],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleEncoderDescriptor::depth_stencil`]"]
    #[builder(into)]
    depth_stencil: Option<RenderBundleDepthStencil>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleEncoderDescriptor::sample_count`]"]
    #[builder(into, default)]
    sample_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleEncoderDescriptor::multiview`]"]
    #[builder(into)]
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

/*
Unhandled Some("ComputePassTimestampWrites") Id(1123)

*/
#[doc = "\n        Returns [`ComputePassTimestampWritesBuilder`] for building [`wgpu::ComputePassTimestampWrites`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn compute_pass_timestamp_writes<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePassTimestampWrites::query_set`]"]
    #[builder(into)]
    query_set: &'a QuerySet,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePassTimestampWrites::beginning_of_pass_write_index`]"]
    #[builder(into)]
    beginning_of_pass_write_index: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePassTimestampWrites::end_of_pass_write_index`]"]
    #[builder(into)]
    end_of_pass_write_index: Option<u32>,
) -> ComputePassTimestampWrites<'a> {
    ComputePassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

/*
Unhandled Some("RenderPassTimestampWrites") Id(2151)

*/
#[doc = "\n        Returns [`RenderPassTimestampWritesBuilder`] for building [`wgpu::RenderPassTimestampWrites`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_pass_timestamp_writes<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassTimestampWrites::query_set`]"]
    #[builder(into)]
    query_set: &'a QuerySet,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassTimestampWrites::beginning_of_pass_write_index`]"]
    #[builder(into)]
    beginning_of_pass_write_index: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassTimestampWrites::end_of_pass_write_index`]"]
    #[builder(into)]
    end_of_pass_write_index: Option<u32>,
) -> RenderPassTimestampWrites<'a> {
    RenderPassTimestampWrites {
        query_set,
        beginning_of_pass_write_index,
        end_of_pass_write_index,
    }
}

/*
Derived default
*/
#[doc = "\n        Returns [`RenderPassDescriptorBuilder`] for building [`wgpu::RenderPassDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_pass_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDescriptor::color_attachments`]"]
    #[builder(default)]
    color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDescriptor::depth_stencil_attachment`]"]
    #[builder(into)]
    depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDescriptor::timestamp_writes`]"]
    #[builder(into)]
    timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDescriptor::occlusion_query_set`]"]
    #[builder(into)]
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

/*
Derived default
*/
#[doc = "\n        Returns [`PipelineLayoutDescriptorBuilder`] for building [`wgpu::PipelineLayoutDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn pipeline_layout_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineLayoutDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineLayoutDescriptor::bind_group_layouts`]"]
    #[builder(default)]
    bind_group_layouts: &'a [&'a BindGroupLayout],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineLayoutDescriptor::push_constant_ranges`]"]
    #[builder(default)]
    push_constant_ranges: &'a [PushConstantRange],
) -> PipelineLayoutDescriptor<'a> {
    PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges,
    }
}

/*
Unhandled Some("BufferBinding") Id(173)

*/
#[doc = "\n        Returns [`BufferBindingBuilder`] for building [`wgpu::BufferBinding`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn buffer_binding<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferBinding::buffer`]"]
    #[builder(into)]
    buffer: &'a Buffer,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferBinding::offset`]"]
    #[builder(into)]
    offset: BufferAddress,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferBinding::size`]"]
    #[builder(into)]
    size: Option<BufferSize>,
) -> BufferBinding<'a> {
    BufferBinding {
        buffer,
        offset,
        size,
    }
}

/*
Unhandled Some("BufferInitDescriptor") Id(1341)

*/
#[doc = "\n        Returns [`BufferInitDescriptorBuilder`] for building [`wgpu::util::BufferInitDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn buffer_init_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::BufferInitDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::BufferInitDescriptor::contents`]"]
    contents: &'a [u8],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::BufferInitDescriptor::usage`]"]
    #[builder(into)]
    usage: BufferUsages,
) -> BufferInitDescriptor<'a> {
    BufferInitDescriptor {
        label,
        contents,
        usage,
    }
}

/*
Unhandled Some("MeshPipelineDescriptor") Id(1266)

*/
#[doc = "\n        Returns [`MeshPipelineDescriptorBuilder`] for building [`wgpu::MeshPipelineDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn mesh_pipeline_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::layout`]"]
    #[builder(into)]
    layout: Option<&'a PipelineLayout>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::task`]"]
    #[builder(into)]
    task: Option<TaskState<'a>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::mesh`]"]
    #[builder(into)]
    mesh: MeshState<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::primitive`]"]
    #[builder(into, default)]
    primitive: PrimitiveState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::depth_stencil`]"]
    #[builder(into)]
    depth_stencil: Option<DepthStencilState>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::multisample`]"]
    #[builder(into, default)]
    multisample: MultisampleState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::fragment`]"]
    #[builder(into)]
    fragment: Option<FragmentState<'a>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::multiview`]"]
    #[builder(into)]
    multiview: Option<NonZeroU32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshPipelineDescriptor::cache`]"]
    #[builder(into)]
    cache: Option<&'a PipelineCache>,
) -> MeshPipelineDescriptor<'a> {
    MeshPipelineDescriptor {
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

/*
Derived default
*/
#[doc = "\n        Returns [`ComputePassDescriptorBuilder`] for building [`wgpu::ComputePassDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn compute_pass_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePassDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePassDescriptor::timestamp_writes`]"]
    #[builder(into)]
    timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
) -> ComputePassDescriptor<'a> {
    ComputePassDescriptor {
        label,
        timestamp_writes,
    }
}

/*
Unhandled Some("BindGroupDescriptor") Id(269)

*/
#[doc = "\n        Returns [`BindGroupDescriptorBuilder`] for building [`wgpu::BindGroupDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn bind_group_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupDescriptor::layout`]"]
    #[builder(into)]
    layout: &'a BindGroupLayout,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupDescriptor::entries`]"]
    entries: &'a [BindGroupEntry<'a>],
) -> BindGroupDescriptor<'a> {
    BindGroupDescriptor {
        label,
        layout,
        entries,
    }
}

/*
Unhandled Some("TaskState") Id(2388)

*/
#[doc = "\n        Returns [`TaskStateBuilder`] for building [`wgpu::TaskState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn task_state<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TaskState::module`]"]
    #[builder(into)]
    module: &'a ShaderModule,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TaskState::entry_point`]"]
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TaskState::compilation_options`]"]
    #[builder(into, default)]
    compilation_options: PipelineCompilationOptions<'a>,
) -> TaskState<'a> {
    TaskState {
        module,
        entry_point,
        compilation_options,
    }
}

/*
Unhandled Some("VertexState") Id(2076)

*/
#[doc = "\n        Returns [`VertexStateBuilder`] for building [`wgpu::VertexState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn vertex_state<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexState::module`]"]
    #[builder(into)]
    module: &'a ShaderModule,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexState::entry_point`]"]
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexState::compilation_options`]"]
    #[builder(into, default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::VertexState::buffers`]"]
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
Unhandled Some("RenderPassDepthStencilAttachment") Id(2204)

*/
#[doc = "\n        Returns [`RenderPassDepthStencilAttachmentBuilder`] for building [`wgpu::RenderPassDepthStencilAttachment`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_pass_depth_stencil_attachment<'tex>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDepthStencilAttachment::view`]"]
    #[builder(into)]
    view: &'tex TextureView,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDepthStencilAttachment::depth_ops`]"]
    #[builder(into)]
    depth_ops: Option<Operations<f32>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassDepthStencilAttachment::stencil_ops`]"]
    #[builder(into)]
    stencil_ops: Option<Operations<u32>>,
) -> RenderPassDepthStencilAttachment<'tex> {
    RenderPassDepthStencilAttachment {
        view,
        depth_ops,
        stencil_ops,
    }
}

/*
Unhandled Some("FragmentState") Id(2362)

*/
#[doc = "\n        Returns [`FragmentStateBuilder`] for building [`wgpu::FragmentState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn fragment_state<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::FragmentState::module`]"]
    #[builder(into)]
    module: &'a ShaderModule,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::FragmentState::entry_point`]"]
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::FragmentState::compilation_options`]"]
    #[builder(into, default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::FragmentState::targets`]"]
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
Unhandled Some("ComputePipelineDescriptor") Id(1222)

*/
#[doc = "\n        Returns [`ComputePipelineDescriptorBuilder`] for building [`wgpu::ComputePipelineDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn compute_pipeline_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePipelineDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePipelineDescriptor::layout`]"]
    #[builder(into)]
    layout: Option<&'a PipelineLayout>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePipelineDescriptor::module`]"]
    #[builder(into)]
    module: &'a ShaderModule,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePipelineDescriptor::entry_point`]"]
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePipelineDescriptor::compilation_options`]"]
    #[builder(into, default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ComputePipelineDescriptor::cache`]"]
    #[builder(into)]
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

/*
Unhandled Some("BlasBuildEntry") Id(478)

*/
#[doc = "\n        Returns [`BlasBuildEntryBuilder`] for building [`wgpu::BlasBuildEntry`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn blas_build_entry<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasBuildEntry::blas`]"]
    #[builder(into)]
    blas: &'a Blas,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasBuildEntry::geometry`]"]
    #[builder(into)]
    geometry: BlasGeometries<'a>,
) -> BlasBuildEntry<'a> {
    BlasBuildEntry { blas, geometry }
}

/*
Unhandled Some("BlasTriangleGeometry") Id(437)

*/
#[doc = "\n        Returns [`BlasTriangleGeometryBuilder`] for building [`wgpu::BlasTriangleGeometry`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn blas_triangle_geometry<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::size`]"]
    #[builder(into)]
    size: &'a BlasTriangleGeometrySizeDescriptor,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::vertex_buffer`]"]
    #[builder(into)]
    vertex_buffer: &'a Buffer,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::first_vertex`]"]
    #[builder(into)]
    first_vertex: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::vertex_stride`]"]
    #[builder(into)]
    vertex_stride: BufferAddress,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::index_buffer`]"]
    #[builder(into)]
    index_buffer: Option<&'a Buffer>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::first_index`]"]
    #[builder(into)]
    first_index: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::transform_buffer`]"]
    #[builder(into)]
    transform_buffer: Option<&'a Buffer>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometry::transform_buffer_offset`]"]
    #[builder(into)]
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

/*
Unhandled Some("RenderPipelineDescriptor") Id(1264)

*/
#[doc = "\n        Returns [`RenderPipelineDescriptorBuilder`] for building [`wgpu::RenderPipelineDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_pipeline_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::layout`]"]
    #[builder(into)]
    layout: Option<&'a PipelineLayout>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::vertex`]"]
    #[builder(into)]
    vertex: VertexState<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::primitive`]"]
    #[builder(into, default)]
    primitive: PrimitiveState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::depth_stencil`]"]
    #[builder(into)]
    depth_stencil: Option<DepthStencilState>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::multisample`]"]
    #[builder(into, default)]
    multisample: MultisampleState,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::fragment`]"]
    #[builder(into)]
    fragment: Option<FragmentState<'a>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::multiview`]"]
    #[builder(into)]
    multiview: Option<NonZeroU32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPipelineDescriptor::cache`]"]
    #[builder(into)]
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
Unhandled Some("RenderPassColorAttachment") Id(2178)

*/
#[doc = "\n        Returns [`RenderPassColorAttachmentBuilder`] for building [`wgpu::RenderPassColorAttachment`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_pass_color_attachment<'tex>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassColorAttachment::view`]"]
    #[builder(into)]
    view: &'tex TextureView,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassColorAttachment::depth_slice`]"]
    #[builder(into)]
    depth_slice: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassColorAttachment::resolve_target`]"]
    #[builder(into)]
    resolve_target: Option<&'tex TextureView>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderPassColorAttachment::ops`]"]
    #[builder(into, default)]
    ops: Operations<Color>,
) -> RenderPassColorAttachment<'tex> {
    RenderPassColorAttachment {
        view,
        depth_slice,
        resolve_target,
        ops,
    }
}

/*
Unhandled Some("BindGroupLayoutDescriptor") Id(266)

*/
#[doc = "\n        Returns [`BindGroupLayoutDescriptorBuilder`] for building [`wgpu::BindGroupLayoutDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn bind_group_layout_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupLayoutDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BindGroupLayoutDescriptor::entries`]"]
    entries: &'a [BindGroupLayoutEntry],
) -> BindGroupLayoutDescriptor<'a> {
    BindGroupLayoutDescriptor { label, entries }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-27.0.1/src/api/common_pipeline.rs:25
impl Default for PipelineCompilationOptions<'_> {
    fn default() -> Self {
        Self {
            constants: Default::default(),
            zero_initialize_workgroup_memory: true,
        }
    }
}

*/
#[doc = "\n        Returns [`PipelineCompilationOptionsBuilder`] for building [`wgpu::PipelineCompilationOptions`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn pipeline_compilation_options<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineCompilationOptions::constants`]"]
    #[builder(default)]
    constants: &'a [(&'a str, f64)],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::PipelineCompilationOptions::zero_initialize_workgroup_memory`]"]
    #[builder(into, default = true)]
    zero_initialize_workgroup_memory: bool,
) -> PipelineCompilationOptions<'a> {
    PipelineCompilationOptions {
        constants,
        zero_initialize_workgroup_memory,
    }
}

/*
Unhandled Some("MeshState") Id(2414)

*/
#[doc = "\n        Returns [`MeshStateBuilder`] for building [`wgpu::MeshState`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn mesh_state<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshState::module`]"]
    #[builder(into)]
    module: &'a ShaderModule,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshState::entry_point`]"]
    #[builder(into)]
    entry_point: Option<&'a str>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::MeshState::compilation_options`]"]
    #[builder(into, default)]
    compilation_options: PipelineCompilationOptions<'a>,
) -> MeshState<'a> {
    MeshState {
        module,
        entry_point,
        compilation_options,
    }
}

/*
Unhandled Some("CreateTlasDescriptor") Id(5845)

*/
#[doc = "\n        Returns [`CreateTlasDescriptorBuilder`] for building [`wgpu::CreateTlasDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn create_tlas_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateTlasDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateTlasDescriptor::max_instances`]"]
    #[builder(into)]
    max_instances: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateTlasDescriptor::flags`]"]
    #[builder(into)]
    flags: AccelerationStructureFlags,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateTlasDescriptor::update_mode`]"]
    #[builder(into)]
    update_mode: AccelerationStructureUpdateMode,
) -> CreateTlasDescriptor<'a> {
    CreateTlasDescriptor {
        label,
        max_instances,
        flags,
        update_mode,
    }
}

/*
Unhandled Some("BufferDescriptor") Id(3932)

*/
#[doc = "\n        Returns [`BufferDescriptorBuilder`] for building [`wgpu::BufferDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn buffer_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferDescriptor::size`]"]
    #[builder(into)]
    size: BufferAddress,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferDescriptor::usage`]"]
    #[builder(into)]
    usage: BufferUsages,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BufferDescriptor::mapped_at_creation`]"]
    #[builder(into)]
    mapped_at_creation: bool,
) -> BufferDescriptor<'a> {
    BufferDescriptor {
        label,
        size,
        usage,
        mapped_at_creation,
    }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6787
impl<T> Default for RenderBundleDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

*/
#[doc = "\n        Returns [`RenderBundleDescriptorBuilder`] for building [`wgpu::RenderBundleDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_bundle_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
) -> RenderBundleDescriptor<'a> {
    RenderBundleDescriptor { label }
}

/*
Unhandled Some("QuerySetDescriptor") Id(1383)

*/
#[doc = "\n        Returns [`QuerySetDescriptorBuilder`] for building [`wgpu::QuerySetDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn query_set_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::QuerySetDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::QuerySetDescriptor::ty`]"]
    #[builder(into)]
    ty: QueryType,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::QuerySetDescriptor::count`]"]
    #[builder(into)]
    count: u32,
) -> QuerySetDescriptor<'a> {
    QuerySetDescriptor { label, ty, count }
}

/*
Unhandled Some("BlasTriangleGeometrySizeDescriptor") Id(329)

*/
#[doc = "\n        Returns [`BlasTriangleGeometrySizeDescriptorBuilder`] for building [`wgpu::BlasTriangleGeometrySizeDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn blas_triangle_geometry_size_descriptor(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometrySizeDescriptor::vertex_format`]"]
    #[builder(into)]
    vertex_format: VertexFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometrySizeDescriptor::vertex_count`]"]
    #[builder(into)]
    vertex_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometrySizeDescriptor::index_format`]"]
    #[builder(into)]
    index_format: Option<IndexFormat>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometrySizeDescriptor::index_count`]"]
    #[builder(into)]
    index_count: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::BlasTriangleGeometrySizeDescriptor::flags`]"]
    #[builder(into)]
    flags: AccelerationStructureGeometryFlags,
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
Derived default
*/
#[doc = "\n        Returns [`DeviceDescriptorBuilder`] for building [`wgpu::DeviceDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn device_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DeviceDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DeviceDescriptor::required_features`]"]
    #[builder(into, default)]
    required_features: Features,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DeviceDescriptor::required_limits`]"]
    #[builder(into, default)]
    required_limits: Limits,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DeviceDescriptor::experimental_features`]"]
    #[builder(into, default)]
    experimental_features: ExperimentalFeatures,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DeviceDescriptor::memory_hints`]"]
    #[builder(into, default)]
    memory_hints: MemoryHints,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::DeviceDescriptor::trace`]"]
    #[builder(into, default)]
    trace: Trace,
) -> DeviceDescriptor<'a> {
    DeviceDescriptor {
        label,
        required_features,
        required_limits,
        experimental_features,
        memory_hints,
        trace,
    }
}

/*
Unhandled Some("SurfaceConfiguration") Id(3998)

*/
#[doc = "\n        Returns [`SurfaceConfigurationBuilder`] for building [`wgpu::SurfaceConfiguration`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn surface_configuration(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::usage`]"]
    #[builder(into)]
    usage: TextureUsages,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::format`]"]
    #[builder(into)]
    format: TextureFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::width`]"]
    #[builder(into)]
    width: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::height`]"]
    #[builder(into)]
    height: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::present_mode`]"]
    #[builder(into, default)]
    present_mode: PresentMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::desired_maximum_frame_latency`]"]
    #[builder(into)]
    desired_maximum_frame_latency: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::alpha_mode`]"]
    #[builder(into, default)]
    alpha_mode: CompositeAlphaMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SurfaceConfiguration::view_formats`]"]
    #[builder(into)]
    view_formats: Vec<TextureFormat>,
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
Unhandled Some("TexelCopyBufferInfo") Id(5278)

*/
#[doc = "\n        Returns [`TexelCopyBufferInfoBuilder`] for building [`wgpu::TexelCopyBufferInfo`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texel_copy_buffer_info<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferInfo::buffer`]"]
    #[builder(into)]
    buffer: &'a Buffer,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyBufferInfo::layout`]"]
    #[builder(into, default)]
    layout: TexelCopyBufferLayout,
) -> TexelCopyBufferInfo<'a> {
    TexelCopyBufferInfo { buffer, layout }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:5449
impl<T> Default for CommandEncoderDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

*/
#[doc = "\n        Returns [`CommandEncoderDescriptorBuilder`] for building [`wgpu::CommandEncoderDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn command_encoder_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CommandEncoderDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
) -> CommandEncoderDescriptor<'a> {
    CommandEncoderDescriptor { label }
}

/*
Unhandled Some("CreateBlasDescriptor") Id(5813)

*/
#[doc = "\n        Returns [`CreateBlasDescriptorBuilder`] for building [`wgpu::CreateBlasDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn create_blas_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateBlasDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateBlasDescriptor::flags`]"]
    #[builder(into)]
    flags: AccelerationStructureFlags,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateBlasDescriptor::update_mode`]"]
    #[builder(into)]
    update_mode: AccelerationStructureUpdateMode,
) -> CreateBlasDescriptor<'a> {
    CreateBlasDescriptor {
        label,
        flags,
        update_mode,
    }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6603
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
#[doc = "\n        Returns [`SamplerDescriptorBuilder`] for building [`wgpu::SamplerDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn sampler_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::address_mode_u`]"]
    #[builder(into, default)]
    address_mode_u: AddressMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::address_mode_v`]"]
    #[builder(into, default)]
    address_mode_v: AddressMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::address_mode_w`]"]
    #[builder(into, default)]
    address_mode_w: AddressMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::mag_filter`]"]
    #[builder(into, default)]
    mag_filter: FilterMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::min_filter`]"]
    #[builder(into, default)]
    min_filter: FilterMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::mipmap_filter`]"]
    #[builder(into, default)]
    mipmap_filter: FilterMode,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::lod_min_clamp`]"]
    #[builder(into, default = 0.0)]
    lod_min_clamp: f32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::lod_max_clamp`]"]
    #[builder(into, default = 32.0)]
    lod_max_clamp: f32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::compare`]"]
    #[builder(into)]
    compare: Option<CompareFunction>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::anisotropy_clamp`]"]
    #[builder(into, default = 1u16)]
    anisotropy_clamp: u16,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::SamplerDescriptor::border_color`]"]
    #[builder(into)]
    border_color: Option<SamplerBorderColor>,
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
Derived default
*/
#[doc = "\n        Returns [`TextureViewDescriptorBuilder`] for building [`wgpu::TextureViewDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texture_view_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::format`]"]
    #[builder(into)]
    format: Option<TextureFormat>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::dimension`]"]
    #[builder(into)]
    dimension: Option<TextureViewDimension>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::usage`]"]
    #[builder(into)]
    usage: Option<TextureUsages>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::aspect`]"]
    #[builder(into, default)]
    aspect: TextureAspect,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::base_mip_level`]"]
    #[builder(into, default)]
    base_mip_level: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::mip_level_count`]"]
    #[builder(into)]
    mip_level_count: Option<u32>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::base_array_layer`]"]
    #[builder(into, default)]
    base_array_layer: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureViewDescriptor::array_layer_count`]"]
    #[builder(into)]
    array_layer_count: Option<u32>,
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

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:374
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
#[doc = "\n        Returns [`RequestAdapterOptionsBuilder`] for building [`wgpu::RequestAdapterOptions`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn request_adapter_options<'a, 'b>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RequestAdapterOptions::power_preference`]"]
    # [builder (into , default = PowerPreference :: default ())]
    power_preference: PowerPreference,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RequestAdapterOptions::force_fallback_adapter`]"]
    #[builder(into, default = false)]
    force_fallback_adapter: bool,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RequestAdapterOptions::compatible_surface`]"]
    #[builder(into)]
    compatible_surface: Option<&'a Surface<'b>>,
) -> RequestAdapterOptions<'a, 'b> {
    RequestAdapterOptions {
        power_preference,
        force_fallback_adapter,
        compatible_surface,
    }
}

/*
Unhandled Some("ExternalTextureDescriptor") Id(4773)

*/
#[doc = "\n        Returns [`ExternalTextureDescriptorBuilder`] for building [`wgpu::ExternalTextureDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn external_texture_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::width`]"]
    #[builder(into)]
    width: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::height`]"]
    #[builder(into)]
    height: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::format`]"]
    #[builder(into)]
    format: ExternalTextureFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::yuv_conversion_matrix`]"]
    #[builder(into)]
    yuv_conversion_matrix: [f32; 16usize],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::gamut_conversion_matrix`]"]
    #[builder(into)]
    gamut_conversion_matrix: [f32; 9usize],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::src_transfer_function`]"]
    #[builder(into, default)]
    src_transfer_function: ExternalTextureTransferFunction,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::dst_transfer_function`]"]
    #[builder(into, default)]
    dst_transfer_function: ExternalTextureTransferFunction,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::sample_transform`]"]
    #[builder(into)]
    sample_transform: [f32; 6usize],
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ExternalTextureDescriptor::load_transform`]"]
    #[builder(into)]
    load_transform: [f32; 6usize],
) -> ExternalTextureDescriptor<'a> {
    ExternalTextureDescriptor {
        label,
        width,
        height,
        format,
        yuv_conversion_matrix,
        gamut_conversion_matrix,
        src_transfer_function,
        dst_transfer_function,
        sample_transform,
        load_transform,
    }
}

/*
Unhandled Some("TexelCopyTextureInfo") Id(4844)

*/
#[doc = "\n        Returns [`TexelCopyTextureInfoBuilder`] for building [`wgpu::TexelCopyTextureInfo`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texel_copy_texture_info<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfo::texture`]"]
    #[builder(into)]
    texture: &'a Texture,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfo::mip_level`]"]
    #[builder(into)]
    mip_level: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfo::origin`]"]
    #[builder(into, default)]
    origin: Origin3d,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TexelCopyTextureInfo::aspect`]"]
    #[builder(into, default)]
    aspect: TextureAspect,
) -> TexelCopyTextureInfo<'a> {
    TexelCopyTextureInfo {
        texture,
        mip_level,
        origin,
        aspect,
    }
}

/*
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:8162
impl<'a, L: Default> Default for CreateShaderModuleDescriptorPassthrough<'a, L> {
    fn default() -> Self {
        Self {
            entry_point: "".into(),
            label: Default::default(),
            num_workgroups: (0, 0, 0),
            runtime_checks: ShaderRuntimeChecks::unchecked(),
            spirv: None,
            dxil: None,
            msl: None,
            hlsl: None,
            glsl: None,
            wgsl: None,
        }
    }
}

*/
#[doc = "\n        Returns [`ShaderModuleDescriptorPassthroughBuilder`] for building [`wgpu::CreateShaderModuleDescriptorPassthrough`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn shader_module_descriptor_passthrough<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::entry_point`]"]
    # [builder (into , default = Into :: < String > :: into (""))]
    entry_point: String,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::num_workgroups`]"]
    # [builder (into , default = (0 , 0 , 0))]
    num_workgroups: (u32, u32, u32),
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::runtime_checks`]"]
    # [builder (into , default = ShaderRuntimeChecks :: unchecked ())]
    runtime_checks: ShaderRuntimeChecks,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::spirv`]"]
    #[builder(into)]
    spirv: Option<Cow<'a, [u32]>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::dxil`]"]
    #[builder(into)]
    dxil: Option<Cow<'a, [u8]>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::msl`]"]
    #[builder(into)]
    msl: Option<Cow<'a, str>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::hlsl`]"]
    #[builder(into)]
    hlsl: Option<Cow<'a, str>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::glsl`]"]
    #[builder(into)]
    glsl: Option<Cow<'a, str>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CreateShaderModuleDescriptorPassthrough::wgsl`]"]
    #[builder(into)]
    wgsl: Option<Cow<'a, str>>,
) -> ShaderModuleDescriptorPassthrough<'a> {
    ShaderModuleDescriptorPassthrough {
        entry_point,
        label,
        num_workgroups,
        runtime_checks,
        spirv,
        dxil,
        msl,
        hlsl,
        glsl,
        wgsl,
    }
}

/*
Unhandled Some("TextureDescriptor") Id(4659)

*/
#[doc = "\n        Returns [`TextureDescriptorBuilder`] for building [`wgpu::TextureDescriptor`]\n\n        |Setter|Status|\n        |-|-|\n        |width|Required|\n        |height|Required|\n        |fill|Optional - default 0u32|\n    "]
# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn texture_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::size`]"]
    #[builder(into, default)]
    size: Extent3d,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::mip_level_count`]"]
    #[builder(into)]
    mip_level_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::sample_count`]"]
    #[builder(into)]
    sample_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::dimension`]"]
    #[builder(into)]
    dimension: TextureDimension,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::format`]"]
    #[builder(into)]
    format: TextureFormat,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::usage`]"]
    #[builder(into)]
    usage: TextureUsages,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::TextureDescriptor::view_formats`]"]
    #[builder(into)]
    view_formats: &'a [TextureFormat],
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
