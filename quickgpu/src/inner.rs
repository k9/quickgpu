use std::borrow::Cow;
use std::num::NonZeroU32;
use std::ops::Range;
use wgpu::util::*;
use wgpu::wgt::{Dx12SwapchainKind, Dx12UseFrameLatencyWaitableObject, TextureSelector};
use wgpu::*;

/*
Unhandled Some("PushConstantRange") Id(4938)

*/

#[doc = "\nReturns [`PushConstantRangeBuilder`] for building [`wgpu::PushConstantRange`]\n\n|Builder Field|Status|\n|-|-|\n|stages|Required|\n|range|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|stages|Required|
///|range|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`CommandBufferDescriptorBuilder`] for building [`wgpu::CommandBufferDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn command_buffer_descriptor<L>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CommandBufferDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: L,
) -> CommandBufferDescriptor<L> {
    CommandBufferDescriptor { label }
}

/*
Derived default
*/

#[doc = "\nReturns [`TexelCopyBufferLayoutBuilder`] for building [`wgpu::TexelCopyBufferLayout`]\n\n|Builder Field|Status|\n|-|-|\n|offset|Defaults to [wgpu::BufferAddress::default()]|\n|bytes_per_row|Defaults to [Option::None]|\n|rows_per_image|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|offset|Defaults to [wgpu::BufferAddress::default()]|
///|bytes_per_row|Defaults to [Option::None]|
///|rows_per_image|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`MultisampleStateBuilder`] for building [`wgpu::MultisampleState`]\n\n|Builder Field|Status|\n|-|-|\n|count|Defaults to [1u32]|\n|mask|Defaults to [!0u64]|\n|alpha_to_coverage_enabled|Defaults to [false]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|count|Defaults to [1u32]|
///|mask|Defaults to [!0u64]|
///|alpha_to_coverage_enabled|Defaults to [false]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TexelCopyTextureInfo") Id(4844)

*/

#[doc = "\nReturns [`TexelCopyTextureInfoBaseBuilder`] for building [`wgpu::TexelCopyTextureInfoBase`]\n\n|Builder Field|Status|\n|-|-|\n|texture|Required|\n|mip_level|Required|\n|origin|Defaults to [wgpu::Origin3d::default()]|\n|aspect|Defaults to [wgpu::TextureAspect::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|texture|Required|
///|mip_level|Required|
///|origin|Defaults to [wgpu::Origin3d::default()]|
///|aspect|Defaults to [wgpu::TextureAspect::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`MemoryBudgetThresholdsBuilder`] for building [`wgpu::MemoryBudgetThresholds`]\n\n|Builder Field|Status|\n|-|-|\n|for_resource_creation|Defaults to [Option::None]|\n|for_device_loss|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|for_resource_creation|Defaults to [Option::None]|
///|for_device_loss|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`OperationsBuilder`] for building [`wgpu::Operations`]\n\n|Builder Field|Status|\n|-|-|\n|load|Required|\n|store|Defaults to [StoreOp::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|load|Required|
///|store|Defaults to [StoreOp::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`GlBackendOptionsBuilder`] for building [`wgpu::GlBackendOptions`]\n\n|Builder Field|Status|\n|-|-|\n|gles_minor_version|Defaults to [wgpu::Gles3MinorVersion::default()]|\n|fence_behavior|Defaults to [wgpu::GlFenceBehavior::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|gles_minor_version|Defaults to [wgpu::Gles3MinorVersion::default()]|
///|fence_behavior|Defaults to [wgpu::GlFenceBehavior::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`BufferTransitionBuilder`] for building [`wgpu::BufferTransition`]\n\n|Builder Field|Status|\n|-|-|\n|buffer|Required|\n|state|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|buffer|Required|
///|state|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:1728
    pub const REPLACE: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::Zero,
        operation: BlendOperation::Add,
    };

*/

#[doc = "\nReturns [`BlendComponentBuilder`] for building [`wgpu::BlendComponent`]\n\n|Builder Field|Status|\n|-|-|\n|src_factor|Defaults to [BlendFactor::One]|\n|dst_factor|Defaults to [BlendFactor::Zero]|\n|operation|Defaults to [BlendOperation::Add]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|src_factor|Defaults to [BlendFactor::One]|
///|dst_factor|Defaults to [BlendFactor::Zero]|
///|operation|Defaults to [BlendOperation::Add]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6006
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

*/

#[doc = "\nReturns [`Origin3dBuilder`] for building [`wgpu::Origin3d`]\n\n|Builder Field|Status|\n|-|-|\n|x|Defaults to [0u32]|\n|y|Defaults to [0u32]|\n|z|Defaults to [0u32]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|x|Defaults to [0u32]|
///|y|Defaults to [0u32]|
///|z|Defaults to [0u32]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`DispatchIndirectArgsBuilder`] for building [`wgpu::util::DispatchIndirectArgs`]\n\n|Builder Field|Status|\n|-|-|\n|x|Defaults to [wgpu::u32::default()]|\n|y|Defaults to [wgpu::u32::default()]|\n|z|Defaults to [wgpu::u32::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|x|Defaults to [wgpu::u32::default()]|
///|y|Defaults to [wgpu::u32::default()]|
///|z|Defaults to [wgpu::u32::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn dispatch_indirect_args(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DispatchIndirectArgs::x`]"]
    #[builder(into, default)]
    x: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DispatchIndirectArgs::y`]"]
    #[builder(into, default)]
    y: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DispatchIndirectArgs::z`]"]
    #[builder(into, default)]
    z: u32,
) -> DispatchIndirectArgs {
    DispatchIndirectArgs { x, y, z }
}

/*
Derived default
*/

#[doc = "\nReturns [`DrawIndexedIndirectArgsBuilder`] for building [`wgpu::util::DrawIndexedIndirectArgs`]\n\n|Builder Field|Status|\n|-|-|\n|index_count|Defaults to [wgpu::u32::default()]|\n|instance_count|Defaults to [wgpu::u32::default()]|\n|first_index|Defaults to [wgpu::u32::default()]|\n|base_vertex|Defaults to [wgpu::i32::default()]|\n|first_instance|Defaults to [wgpu::u32::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|index_count|Defaults to [wgpu::u32::default()]|
///|instance_count|Defaults to [wgpu::u32::default()]|
///|first_index|Defaults to [wgpu::u32::default()]|
///|base_vertex|Defaults to [wgpu::i32::default()]|
///|first_instance|Defaults to [wgpu::u32::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn draw_indexed_indirect_args(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndexedIndirectArgs::index_count`]"]
    #[builder(into, default)]
    index_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndexedIndirectArgs::instance_count`]"]
    #[builder(into, default)]
    instance_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndexedIndirectArgs::first_index`]"]
    #[builder(into, default)]
    first_index: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndexedIndirectArgs::base_vertex`]"]
    #[builder(into, default)]
    base_vertex: i32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndexedIndirectArgs::first_instance`]"]
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
Unhandled Some("RenderBundleDepthStencil") Id(4998)

*/

#[doc = "\nReturns [`RenderBundleDepthStencilBuilder`] for building [`wgpu::RenderBundleDepthStencil`]\n\n|Builder Field|Status|\n|-|-|\n|format|Required|\n|depth_read_only|Required|\n|stencil_read_only|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|format|Required|
///|depth_read_only|Required|
///|stencil_read_only|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("VertexAttribute") Id(3577)

*/

#[doc = "\nReturns [`VertexAttributeBuilder`] for building [`wgpu::VertexAttribute`]\n\n|Builder Field|Status|\n|-|-|\n|format|Required|\n|offset|Required|\n|shader_location|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|format|Required|
///|offset|Required|
///|shader_location|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`StencilStateBuilder`] for building [`wgpu::StencilState`]\n\n|Builder Field|Status|\n|-|-|\n|front|Defaults to [wgpu::StencilFaceState::default()]|\n|back|Defaults to [wgpu::StencilFaceState::default()]|\n|read_mask|Defaults to [wgpu::u32::default()]|\n|write_mask|Defaults to [wgpu::u32::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|front|Defaults to [wgpu::StencilFaceState::default()]|
///|back|Defaults to [wgpu::StencilFaceState::default()]|
///|read_mask|Defaults to [wgpu::u32::default()]|
///|write_mask|Defaults to [wgpu::u32::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("ColorTargetState") Id(2409)

*/

#[doc = "\nReturns [`ColorTargetStateBuilder`] for building [`wgpu::ColorTargetState`]\n\n|Builder Field|Status|\n|-|-|\n|format|Required|\n|blend|Defaults to [Option::None]|\n|write_mask|Defaults to [wgpu::ColorWrites::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|format|Required|
///|blend|Defaults to [Option::None]|
///|write_mask|Defaults to [wgpu::ColorWrites::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:4927
    pub const IGNORE: Self = StencilFaceState {
        compare: CompareFunction::Always,
        fail_op: StencilOperation::Keep,
        depth_fail_op: StencilOperation::Keep,
        pass_op: StencilOperation::Keep,
    };

*/

#[doc = "\nReturns [`StencilFaceStateBuilder`] for building [`wgpu::StencilFaceState`]\n\n|Builder Field|Status|\n|-|-|\n|compare|Defaults to [CompareFunction::Always]|\n|fail_op|Defaults to [StencilOperation::Keep]|\n|depth_fail_op|Defaults to [StencilOperation::Keep]|\n|pass_op|Defaults to [StencilOperation::Keep]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|compare|Defaults to [CompareFunction::Always]|
///|fail_op|Defaults to [StencilOperation::Keep]|
///|depth_fail_op|Defaults to [StencilOperation::Keep]|
///|pass_op|Defaults to [StencilOperation::Keep]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`BackendOptionsBuilder`] for building [`wgpu::BackendOptions`]\n\n|Builder Field|Status|\n|-|-|\n|gl|Defaults to [wgpu::GlBackendOptions::default()]|\n|dx12|Defaults to [wgpu::Dx12BackendOptions::default()]|\n|noop|Defaults to [wgpu::NoopBackendOptions::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|gl|Defaults to [wgpu::GlBackendOptions::default()]|
///|dx12|Defaults to [wgpu::Dx12BackendOptions::default()]|
///|noop|Defaults to [wgpu::NoopBackendOptions::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BlendState") Id(2373)

*/

#[doc = "\nReturns [`BlendStateBuilder`] for building [`wgpu::BlendState`]\n\n|Builder Field|Status|\n|-|-|\n|color|Defaults to [wgpu::BlendComponent::default()]|\n|alpha|Defaults to [wgpu::BlendComponent::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|color|Defaults to [wgpu::BlendComponent::default()]|
///|alpha|Defaults to [wgpu::BlendComponent::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`NoopBackendOptionsBuilder`] for building [`wgpu::NoopBackendOptions`]\n\n|Builder Field|Status|\n|-|-|\n|enable|Defaults to [wgpu::bool::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|enable|Defaults to [wgpu::bool::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn noop_backend_options(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::NoopBackendOptions::enable`]"]
    #[builder(into, default)]
    enable: bool,
) -> NoopBackendOptions {
    NoopBackendOptions { enable }
}

/*
Unhandled Some("Origin2d") Id(4515)

*/

#[doc = "\nReturns [`Origin2dBuilder`] for building [`wgpu::Origin2d`]\n\n|Builder Field|Status|\n|-|-|\n|x|Required|\n|y|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|x|Required|
///|y|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`InstanceDescriptorBuilder`] for building [`wgpu::InstanceDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|backends|Defaults to [wgpu::Backends::default()]|\n|flags|Defaults to [wgpu::InstanceFlags::default()]|\n|memory_budget_thresholds|Defaults to [wgpu::MemoryBudgetThresholds::default()]|\n|backend_options|Defaults to [wgpu::BackendOptions::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|backends|Defaults to [wgpu::Backends::default()]|
///|flags|Defaults to [wgpu::InstanceFlags::default()]|
///|memory_budget_thresholds|Defaults to [wgpu::MemoryBudgetThresholds::default()]|
///|backend_options|Defaults to [wgpu::BackendOptions::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`BindGroupLayoutEntryBuilder`] for building [`wgpu::BindGroupLayoutEntry`]\n\n|Builder Field|Status|\n|-|-|\n|binding|Required|\n|visibility|Required|\n|ty|Required|\n|count|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|binding|Required|
///|visibility|Required|
///|ty|Required|
///|count|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TexelCopyBufferInfo") Id(5278)

*/

#[doc = "\nReturns [`TexelCopyBufferInfoBaseBuilder`] for building [`wgpu::TexelCopyBufferInfoBase`]\n\n|Builder Field|Status|\n|-|-|\n|buffer|Required|\n|layout|Defaults to [wgpu::TexelCopyBufferLayout::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|buffer|Required|
///|layout|Defaults to [wgpu::TexelCopyBufferLayout::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TextureTransition") Id(4291)

*/

#[doc = "\nReturns [`TextureTransitionBuilder`] for building [`wgpu::TextureTransition`]\n\n|Builder Field|Status|\n|-|-|\n|texture|Required|\n|selector|Defaults to [Option::None]|\n|state|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|texture|Required|
///|selector|Defaults to [Option::None]|
///|state|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`PrimitiveStateBuilder`] for building [`wgpu::PrimitiveState`]\n\n|Builder Field|Status|\n|-|-|\n|topology|Defaults to [wgpu::PrimitiveTopology::default()]|\n|strip_index_format|Defaults to [Option::None]|\n|front_face|Defaults to [wgpu::FrontFace::default()]|\n|cull_mode|Defaults to [Option::None]|\n|unclipped_depth|Defaults to [wgpu::bool::default()]|\n|polygon_mode|Defaults to [wgpu::PolygonMode::default()]|\n|conservative|Defaults to [wgpu::bool::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|topology|Defaults to [wgpu::PrimitiveTopology::default()]|
///|strip_index_format|Defaults to [Option::None]|
///|front_face|Defaults to [wgpu::FrontFace::default()]|
///|cull_mode|Defaults to [Option::None]|
///|unclipped_depth|Defaults to [wgpu::bool::default()]|
///|polygon_mode|Defaults to [wgpu::PolygonMode::default()]|
///|conservative|Defaults to [wgpu::bool::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`DrawIndirectArgsBuilder`] for building [`wgpu::util::DrawIndirectArgs`]\n\n|Builder Field|Status|\n|-|-|\n|vertex_count|Defaults to [wgpu::u32::default()]|\n|instance_count|Defaults to [wgpu::u32::default()]|\n|first_vertex|Defaults to [wgpu::u32::default()]|\n|first_instance|Defaults to [wgpu::u32::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|vertex_count|Defaults to [wgpu::u32::default()]|
///|instance_count|Defaults to [wgpu::u32::default()]|
///|first_vertex|Defaults to [wgpu::u32::default()]|
///|first_instance|Defaults to [wgpu::u32::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn draw_indirect_args(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndirectArgs::vertex_count`]"]
    #[builder(into, default)]
    vertex_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndirectArgs::instance_count`]"]
    #[builder(into, default)]
    instance_count: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndirectArgs::first_vertex`]"]
    #[builder(into, default)]
    first_vertex: u32,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::util::DrawIndirectArgs::first_instance`]"]
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

#[doc = "\nReturns [`RequestAdapterOptionsBaseBuilder`] for building [`wgpu::RequestAdapterOptionsBase`]\n\n|Builder Field|Status|\n|-|-|\n|power_preference|Defaults to [PowerPreference::default()]|\n|force_fallback_adapter|Defaults to [false]|\n|compatible_surface|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|power_preference|Defaults to [PowerPreference::default()]|
///|force_fallback_adapter|Defaults to [false]|
///|compatible_surface|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`Dx12BackendOptionsBuilder`] for building [`wgpu::Dx12BackendOptions`]\n\n|Builder Field|Status|\n|-|-|\n|shader_compiler|Defaults to [wgpu::Dx12Compiler::default()]|\n|presentation_system|Defaults to [wgpu::Dx12SwapchainKind::default()]|\n|latency_waitable_object|Defaults to [wgpu::Dx12UseFrameLatencyWaitableObject::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|shader_compiler|Defaults to [wgpu::Dx12Compiler::default()]|
///|presentation_system|Defaults to [wgpu::Dx12SwapchainKind::default()]|
///|latency_waitable_object|Defaults to [wgpu::Dx12UseFrameLatencyWaitableObject::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`DepthBiasStateBuilder`] for building [`wgpu::DepthBiasState`]\n\n|Builder Field|Status|\n|-|-|\n|constant|Defaults to [wgpu::i32::default()]|\n|slope_scale|Defaults to [wgpu::f32::default()]|\n|clamp|Defaults to [wgpu::f32::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|constant|Defaults to [wgpu::i32::default()]|
///|slope_scale|Defaults to [wgpu::f32::default()]|
///|clamp|Defaults to [wgpu::f32::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:1152
impl Default for DownlevelLimits {
    fn default() -> Self {
        DownlevelLimits {}
    }
}

*/

#[doc = "\nReturns [`DownlevelLimitsBuilder`] for building [`wgpu::DownlevelLimits`]\n\n|Builder Field|Status|\n|-|-|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn downlevel_limits() -> DownlevelLimits {
    DownlevelLimits {}
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

#[doc = "\nReturns [`ExternalTextureTransferFunctionBuilder`] for building [`wgpu::ExternalTextureTransferFunction`]\n\n|Builder Field|Status|\n|-|-|\n|a|Defaults to [1.0]|\n|b|Defaults to [1.0]|\n|g|Defaults to [1.0]|\n|k|Defaults to [1.0]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|a|Defaults to [1.0]|
///|b|Defaults to [1.0]|
///|g|Defaults to [1.0]|
///|k|Defaults to [1.0]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`Extent3dBuilder`] for building [`wgpu::Extent3d`]\n\n|Builder Field|Status|\n|-|-|\n|width|Defaults to [1u32]|\n|height|Defaults to [1u32]|\n|depth_or_array_layers|Defaults to [1u32]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|width|Defaults to [1u32]|
///|height|Defaults to [1u32]|
///|depth_or_array_layers|Defaults to [1u32]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:7869
impl Default for ShaderRuntimeChecks {
    fn default() -> Self {
        Self::checked()
    }
}

*/

#[doc = "\nReturns [`ShaderRuntimeChecksBuilder`] for building [`wgpu::ShaderRuntimeChecks`]\n\n|Builder Field|Status|\n|-|-|\n|bounds_checks|Required|\n|force_loop_bounding|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|bounds_checks|Required|
///|force_loop_bounding|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("CopyExternalImageDestInfo") Id(5305)

*/

#[doc = "\nReturns [`CopyExternalImageDestInfoBuilder`] for building [`wgpu::CopyExternalImageDestInfo`]\n\n|Builder Field|Status|\n|-|-|\n|texture|Required|\n|mip_level|Required|\n|origin|Defaults to [wgpu::Origin3d::default()]|\n|aspect|Defaults to [wgpu::TextureAspect::default()]|\n|color_space|Required|\n|premultiplied_alpha|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|texture|Required|
///|mip_level|Required|
///|origin|Defaults to [wgpu::Origin3d::default()]|
///|aspect|Defaults to [wgpu::TextureAspect::default()]|
///|color_space|Required|
///|premultiplied_alpha|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("DepthStencilState") Id(3204)

*/

#[doc = "\nReturns [`DepthStencilStateBuilder`] for building [`wgpu::DepthStencilState`]\n\n|Builder Field|Status|\n|-|-|\n|format|Required|\n|depth_write_enabled|Required|\n|depth_compare|Required|\n|stencil|Defaults to [wgpu::StencilState::default()]|\n|bias|Defaults to [wgpu::DepthBiasState::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|format|Required|
///|depth_write_enabled|Required|
///|depth_compare|Required|
///|stencil|Defaults to [wgpu::StencilState::default()]|
///|bias|Defaults to [wgpu::DepthBiasState::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`ColorBuilder`] for building [`wgpu::Color`]\n\n|Builder Field|Status|\n|-|-|\n|r|Defaults to [wgpu::f64::default()]|\n|g|Defaults to [wgpu::f64::default()]|\n|b|Defaults to [wgpu::f64::default()]|\n|a|Defaults to [wgpu::f64::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|r|Defaults to [wgpu::f64::default()]|
///|g|Defaults to [wgpu::f64::default()]|
///|b|Defaults to [wgpu::f64::default()]|
///|a|Defaults to [wgpu::f64::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`ImageSubresourceRangeBuilder`] for building [`wgpu::ImageSubresourceRange`]\n\n|Builder Field|Status|\n|-|-|\n|aspect|Defaults to [wgpu::TextureAspect::default()]|\n|base_mip_level|Defaults to [wgpu::u32::default()]|\n|mip_level_count|Defaults to [Option::None]|\n|base_array_layer|Defaults to [wgpu::u32::default()]|\n|array_layer_count|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|aspect|Defaults to [wgpu::TextureAspect::default()]|
///|base_mip_level|Defaults to [wgpu::u32::default()]|
///|mip_level_count|Defaults to [Option::None]|
///|base_array_layer|Defaults to [wgpu::u32::default()]|
///|array_layer_count|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`CoreCountersBuilder`] for building [`wgpu::CoreCounters`]\n\n|Builder Field|Status|\n|-|-|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn core_counters() -> CoreCounters {
    CoreCounters {}
}

/*
Unhandled Some("PipelineCacheDescriptor") Id(1042)

*/

#[doc = "\nReturns [`PipelineCacheDescriptorBuilder`] for building [`wgpu::PipelineCacheDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|data|Defaults to [Option::None]|\n|fallback|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|data|Defaults to [Option::None]|
///|fallback|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("VertexBufferLayout") Id(646)

*/

#[doc = "\nReturns [`VertexBufferLayoutBuilder`] for building [`wgpu::VertexBufferLayout`]\n\n|Builder Field|Status|\n|-|-|\n|array_stride|Required|\n|step_mode|Defaults to [wgpu::VertexStepMode::default()]|\n|attributes|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|array_stride|Required|
///|step_mode|Defaults to [wgpu::VertexStepMode::default()]|
///|attributes|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("MeshPipelineDescriptor") Id(1266)

*/

#[doc = "\nReturns [`MeshPipelineDescriptorBuilder`] for building [`wgpu::MeshPipelineDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|layout|Defaults to [Option::None]|\n|task|Defaults to [Option::None]|\n|mesh|Required|\n|primitive|Defaults to [wgpu::PrimitiveState::default()]|\n|depth_stencil|Defaults to [Option::None]|\n|multisample|Defaults to [wgpu::MultisampleState::default()]|\n|fragment|Defaults to [Option::None]|\n|multiview|Defaults to [Option::None]|\n|cache|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|layout|Defaults to [Option::None]|
///|task|Defaults to [Option::None]|
///|mesh|Required|
///|primitive|Defaults to [wgpu::PrimitiveState::default()]|
///|depth_stencil|Defaults to [Option::None]|
///|multisample|Defaults to [wgpu::MultisampleState::default()]|
///|fragment|Defaults to [Option::None]|
///|multiview|Defaults to [Option::None]|
///|cache|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("ComputePipelineDescriptor") Id(1222)

*/

#[doc = "\nReturns [`ComputePipelineDescriptorBuilder`] for building [`wgpu::ComputePipelineDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|layout|Defaults to [Option::None]|\n|module|Required|\n|entry_point|Defaults to [Option::None]|\n|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|\n|cache|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|layout|Defaults to [Option::None]|
///|module|Required|
///|entry_point|Defaults to [Option::None]|
///|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|
///|cache|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BindGroupLayoutDescriptor") Id(266)

*/

#[doc = "\nReturns [`BindGroupLayoutDescriptorBuilder`] for building [`wgpu::BindGroupLayoutDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|entries|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|entries|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("ShaderModuleDescriptor") Id(1253)

*/

#[doc = "\nReturns [`ShaderModuleDescriptorBuilder`] for building [`wgpu::ShaderModuleDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|source|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|source|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`PipelineLayoutDescriptorBuilder`] for building [`wgpu::PipelineLayoutDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|bind_group_layouts|Defaults to [wgpu::&'a[&'aBindGroupLayout]::default()]|\n|push_constant_ranges|Defaults to [wgpu::&'a[PushConstantRange]::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|bind_group_layouts|Defaults to [wgpu::&'a[&'aBindGroupLayout]::default()]|
///|push_constant_ranges|Defaults to [wgpu::&'a[PushConstantRange]::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TaskState") Id(2388)

*/

#[doc = "\nReturns [`TaskStateBuilder`] for building [`wgpu::TaskState`]\n\n|Builder Field|Status|\n|-|-|\n|module|Required|\n|entry_point|Defaults to [Option::None]|\n|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|module|Required|
///|entry_point|Defaults to [Option::None]|
///|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`VertexStateBuilder`] for building [`wgpu::VertexState`]\n\n|Builder Field|Status|\n|-|-|\n|module|Required|\n|entry_point|Defaults to [Option::None]|\n|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|\n|buffers|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|module|Required|
///|entry_point|Defaults to [Option::None]|
///|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|
///|buffers|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`RenderBundleEncoderDescriptorBuilder`] for building [`wgpu::RenderBundleEncoderDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|color_formats|Defaults to [wgpu::&'a[Option<TextureFormat>]::default()]|\n|depth_stencil|Defaults to [Option::None]|\n|sample_count|Defaults to [wgpu::u32::default()]|\n|multiview|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|color_formats|Defaults to [wgpu::&'a[Option<TextureFormat>]::default()]|
///|depth_stencil|Defaults to [Option::None]|
///|sample_count|Defaults to [wgpu::u32::default()]|
///|multiview|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`RenderPassDescriptorBuilder`] for building [`wgpu::RenderPassDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|color_attachments|Defaults to [wgpu::&'a[Option<RenderPassColorAttachment<'a>>]::default()]|\n|depth_stencil_attachment|Defaults to [Option::None]|\n|timestamp_writes|Defaults to [Option::None]|\n|occlusion_query_set|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|color_attachments|Defaults to [wgpu::&'a[Option<RenderPassColorAttachment<'a>>]::default()]|
///|depth_stencil_attachment|Defaults to [Option::None]|
///|timestamp_writes|Defaults to [Option::None]|
///|occlusion_query_set|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("CompilationInfo") Id(2541)

*/

#[doc = "\nReturns [`CompilationInfoBuilder`] for building [`wgpu::CompilationInfo`]\n\n|Builder Field|Status|\n|-|-|\n|messages|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|messages|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn compilation_info(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CompilationInfo::messages`]"]
    #[builder(into)]
    messages: Vec<CompilationMessage>,
) -> CompilationInfo {
    CompilationInfo { messages }
}

/*
Unhandled Some("BufferBinding") Id(173)

*/

#[doc = "\nReturns [`BufferBindingBuilder`] for building [`wgpu::BufferBinding`]\n\n|Builder Field|Status|\n|-|-|\n|buffer|Required|\n|offset|Required|\n|size|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|buffer|Required|
///|offset|Required|
///|size|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`BufferInitDescriptorBuilder`] for building [`wgpu::util::BufferInitDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|contents|Required|\n|usage|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|contents|Required|
///|usage|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("RenderPassTimestampWrites") Id(2151)

*/

#[doc = "\nReturns [`RenderPassTimestampWritesBuilder`] for building [`wgpu::RenderPassTimestampWrites`]\n\n|Builder Field|Status|\n|-|-|\n|query_set|Required|\n|beginning_of_pass_write_index|Defaults to [Option::None]|\n|end_of_pass_write_index|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|query_set|Required|
///|beginning_of_pass_write_index|Defaults to [Option::None]|
///|end_of_pass_write_index|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`ComputePassDescriptorBuilder`] for building [`wgpu::ComputePassDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|timestamp_writes|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|timestamp_writes|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("RenderPassColorAttachment") Id(2178)

*/

#[doc = "\nReturns [`RenderPassColorAttachmentBuilder`] for building [`wgpu::RenderPassColorAttachment`]\n\n|Builder Field|Status|\n|-|-|\n|view|Required|\n|depth_slice|Defaults to [Option::None]|\n|resolve_target|Defaults to [Option::None]|\n|ops|Defaults to [wgpu::Operations<Color>::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|view|Required|
///|depth_slice|Defaults to [Option::None]|
///|resolve_target|Defaults to [Option::None]|
///|ops|Defaults to [wgpu::Operations<Color>::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`PipelineCompilationOptionsBuilder`] for building [`wgpu::PipelineCompilationOptions`]\n\n|Builder Field|Status|\n|-|-|\n|constants|Defaults to [wgpu::&'a[(&'astr,f64)]::default()]|\n|zero_initialize_workgroup_memory|Defaults to [true]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|constants|Defaults to [wgpu::&'a[(&'astr,f64)]::default()]|
///|zero_initialize_workgroup_memory|Defaults to [true]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`MeshStateBuilder`] for building [`wgpu::MeshState`]\n\n|Builder Field|Status|\n|-|-|\n|module|Required|\n|entry_point|Defaults to [Option::None]|\n|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|module|Required|
///|entry_point|Defaults to [Option::None]|
///|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BindGroupDescriptor") Id(269)

*/

#[doc = "\nReturns [`BindGroupDescriptorBuilder`] for building [`wgpu::BindGroupDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|layout|Required|\n|entries|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|layout|Required|
///|entries|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BindGroupEntry") Id(270)

*/

#[doc = "\nReturns [`BindGroupEntryBuilder`] for building [`wgpu::BindGroupEntry`]\n\n|Builder Field|Status|\n|-|-|\n|binding|Required|\n|resource|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|binding|Required|
///|resource|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("RenderPipelineDescriptor") Id(1264)

*/

#[doc = "\nReturns [`RenderPipelineDescriptorBuilder`] for building [`wgpu::RenderPipelineDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|layout|Defaults to [Option::None]|\n|vertex|Required|\n|primitive|Defaults to [wgpu::PrimitiveState::default()]|\n|depth_stencil|Defaults to [Option::None]|\n|multisample|Defaults to [wgpu::MultisampleState::default()]|\n|fragment|Defaults to [Option::None]|\n|multiview|Defaults to [Option::None]|\n|cache|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|layout|Defaults to [Option::None]|
///|vertex|Required|
///|primitive|Defaults to [wgpu::PrimitiveState::default()]|
///|depth_stencil|Defaults to [Option::None]|
///|multisample|Defaults to [wgpu::MultisampleState::default()]|
///|fragment|Defaults to [Option::None]|
///|multiview|Defaults to [Option::None]|
///|cache|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BlasTriangleGeometry") Id(437)

*/

#[doc = "\nReturns [`BlasTriangleGeometryBuilder`] for building [`wgpu::BlasTriangleGeometry`]\n\n|Builder Field|Status|\n|-|-|\n|size|Required|\n|vertex_buffer|Required|\n|first_vertex|Required|\n|vertex_stride|Required|\n|index_buffer|Defaults to [Option::None]|\n|first_index|Defaults to [Option::None]|\n|transform_buffer|Defaults to [Option::None]|\n|transform_buffer_offset|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|size|Required|
///|vertex_buffer|Required|
///|first_vertex|Required|
///|vertex_stride|Required|
///|index_buffer|Defaults to [Option::None]|
///|first_index|Defaults to [Option::None]|
///|transform_buffer|Defaults to [Option::None]|
///|transform_buffer_offset|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("RenderPassDepthStencilAttachment") Id(2204)

*/

#[doc = "\nReturns [`RenderPassDepthStencilAttachmentBuilder`] for building [`wgpu::RenderPassDepthStencilAttachment`]\n\n|Builder Field|Status|\n|-|-|\n|view|Required|\n|depth_ops|Defaults to [Option::None]|\n|stencil_ops|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|view|Required|
///|depth_ops|Defaults to [Option::None]|
///|stencil_ops|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BlasBuildEntry") Id(478)

*/

#[doc = "\nReturns [`BlasBuildEntryBuilder`] for building [`wgpu::BlasBuildEntry`]\n\n|Builder Field|Status|\n|-|-|\n|blas|Required|\n|geometry|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|blas|Required|
///|geometry|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("FragmentState") Id(2362)

*/

#[doc = "\nReturns [`FragmentStateBuilder`] for building [`wgpu::FragmentState`]\n\n|Builder Field|Status|\n|-|-|\n|module|Required|\n|entry_point|Defaults to [Option::None]|\n|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|\n|targets|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|module|Required|
///|entry_point|Defaults to [Option::None]|
///|compilation_options|Defaults to [wgpu::PipelineCompilationOptions<'a>::default()]|
///|targets|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("ComputePassTimestampWrites") Id(1123)

*/

#[doc = "\nReturns [`ComputePassTimestampWritesBuilder`] for building [`wgpu::ComputePassTimestampWrites`]\n\n|Builder Field|Status|\n|-|-|\n|query_set|Required|\n|beginning_of_pass_write_index|Defaults to [Option::None]|\n|end_of_pass_write_index|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|query_set|Required|
///|beginning_of_pass_write_index|Defaults to [Option::None]|
///|end_of_pass_write_index|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BufferDescriptor") Id(3932)

*/

#[doc = "\nReturns [`BufferDescriptorBuilder`] for building [`wgpu::BufferDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|size|Required|\n|usage|Required|\n|mapped_at_creation|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|size|Required|
///|usage|Required|
///|mapped_at_creation|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("CreateTlasDescriptor") Id(5845)

*/

#[doc = "\nReturns [`CreateTlasDescriptorBuilder`] for building [`wgpu::CreateTlasDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|max_instances|Required|\n|flags|Required|\n|update_mode|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|max_instances|Required|
///|flags|Required|
///|update_mode|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TexelCopyBufferInfo") Id(5278)

*/

#[doc = "\nReturns [`TexelCopyBufferInfoBuilder`] for building [`wgpu::TexelCopyBufferInfo`]\n\n|Builder Field|Status|\n|-|-|\n|buffer|Required|\n|layout|Defaults to [wgpu::TexelCopyBufferLayout::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|buffer|Required|
///|layout|Defaults to [wgpu::TexelCopyBufferLayout::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("ExternalTextureDescriptor") Id(4773)

*/

#[doc = "\nReturns [`ExternalTextureDescriptorBuilder`] for building [`wgpu::ExternalTextureDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|width|Required|\n|height|Required|\n|format|Required|\n|yuv_conversion_matrix|Required|\n|gamut_conversion_matrix|Required|\n|src_transfer_function|Defaults to [wgpu::ExternalTextureTransferFunction::default()]|\n|dst_transfer_function|Defaults to [wgpu::ExternalTextureTransferFunction::default()]|\n|sample_transform|Required|\n|load_transform|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|width|Required|
///|height|Required|
///|format|Required|
///|yuv_conversion_matrix|Required|
///|gamut_conversion_matrix|Required|
///|src_transfer_function|Defaults to [wgpu::ExternalTextureTransferFunction::default()]|
///|dst_transfer_function|Defaults to [wgpu::ExternalTextureTransferFunction::default()]|
///|sample_transform|Required|
///|load_transform|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`DeviceDescriptorBuilder`] for building [`wgpu::DeviceDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|required_features|Defaults to [wgpu::Features::default()]|\n|required_limits|Defaults to [wgpu::Limits::default()]|\n|experimental_features|Defaults to [wgpu::ExperimentalFeatures::default()]|\n|memory_hints|Defaults to [wgpu::MemoryHints::default()]|\n|trace|Defaults to [wgpu::Trace::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|required_features|Defaults to [wgpu::Features::default()]|
///|required_limits|Defaults to [wgpu::Limits::default()]|
///|experimental_features|Defaults to [wgpu::ExperimentalFeatures::default()]|
///|memory_hints|Defaults to [wgpu::MemoryHints::default()]|
///|trace|Defaults to [wgpu::Trace::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`RequestAdapterOptionsBuilder`] for building [`wgpu::RequestAdapterOptions`]\n\n|Builder Field|Status|\n|-|-|\n|power_preference|Defaults to [PowerPreference::default()]|\n|force_fallback_adapter|Defaults to [false]|\n|compatible_surface|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|power_preference|Defaults to [PowerPreference::default()]|
///|force_fallback_adapter|Defaults to [false]|
///|compatible_surface|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`ShaderModuleDescriptorPassthroughBuilder`] for building [`wgpu::ShaderModuleDescriptorPassthrough`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|entry_point|Defaults to [Into::<String>::into(\"\")]|\n|num_workgroups|Defaults to [(0,0,0)]|\n|runtime_checks|Defaults to [ShaderRuntimeChecks::unchecked()]|\n|spirv|Defaults to [Option::None]|\n|dxil|Defaults to [Option::None]|\n|msl|Defaults to [Option::None]|\n|hlsl|Defaults to [Option::None]|\n|glsl|Defaults to [Option::None]|\n|wgsl|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|entry_point|Defaults to [Into::<String>::into("")]|
///|num_workgroups|Defaults to [(0,0,0)]|
///|runtime_checks|Defaults to [ShaderRuntimeChecks::unchecked()]|
///|spirv|Defaults to [Option::None]|
///|dxil|Defaults to [Option::None]|
///|msl|Defaults to [Option::None]|
///|hlsl|Defaults to [Option::None]|
///|glsl|Defaults to [Option::None]|
///|wgsl|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn shader_module_descriptor_passthrough<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::entry_point`]"]
    # [builder (into , default = Into :: < String > :: into (""))]
    entry_point: String,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::num_workgroups`]"]
    # [builder (into , default = (0 , 0 , 0))]
    num_workgroups: (u32, u32, u32),
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::runtime_checks`]"]
    # [builder (into , default = ShaderRuntimeChecks :: unchecked ())]
    runtime_checks: ShaderRuntimeChecks,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::spirv`]"]
    #[builder(into)]
    spirv: Option<Cow<'a, [u32]>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::dxil`]"]
    #[builder(into)]
    dxil: Option<Cow<'a, [u8]>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::msl`]"]
    #[builder(into)]
    msl: Option<Cow<'a, str>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::hlsl`]"]
    #[builder(into)]
    hlsl: Option<Cow<'a, str>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::glsl`]"]
    #[builder(into)]
    glsl: Option<Cow<'a, str>>,
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::ShaderModuleDescriptorPassthrough::wgsl`]"]
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
Unhandled Some("SurfaceConfiguration") Id(3998)

*/

#[doc = "\nReturns [`SurfaceConfigurationBuilder`] for building [`wgpu::SurfaceConfiguration`]\n\n|Builder Field|Status|\n|-|-|\n|usage|Required|\n|format|Required|\n|width|Required|\n|height|Required|\n|present_mode|Defaults to [wgpu::PresentMode::default()]|\n|desired_maximum_frame_latency|Required|\n|alpha_mode|Defaults to [wgpu::CompositeAlphaMode::default()]|\n|view_formats|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|usage|Required|
///|format|Required|
///|width|Required|
///|height|Required|
///|present_mode|Defaults to [wgpu::PresentMode::default()]|
///|desired_maximum_frame_latency|Required|
///|alpha_mode|Defaults to [wgpu::CompositeAlphaMode::default()]|
///|view_formats|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TextureDescriptor") Id(4659)

*/

#[doc = "\nReturns [`TextureDescriptorBuilder`] for building [`wgpu::TextureDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|size|Defaults to [wgpu::Extent3d::default()]|\n|mip_level_count|Required|\n|sample_count|Required|\n|dimension|Required|\n|format|Required|\n|usage|Required|\n|view_formats|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|size|Defaults to [wgpu::Extent3d::default()]|
///|mip_level_count|Required|
///|sample_count|Required|
///|dimension|Required|
///|format|Required|
///|usage|Required|
///|view_formats|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

/*
Unhandled Some("QuerySetDescriptor") Id(1383)

*/

#[doc = "\nReturns [`QuerySetDescriptorBuilder`] for building [`wgpu::QuerySetDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|ty|Required|\n|count|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|ty|Required|
///|count|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Derived default
*/

#[doc = "\nReturns [`TextureViewDescriptorBuilder`] for building [`wgpu::TextureViewDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|format|Defaults to [Option::None]|\n|dimension|Defaults to [Option::None]|\n|usage|Defaults to [Option::None]|\n|aspect|Defaults to [wgpu::TextureAspect::default()]|\n|base_mip_level|Defaults to [wgpu::u32::default()]|\n|mip_level_count|Defaults to [Option::None]|\n|base_array_layer|Defaults to [wgpu::u32::default()]|\n|array_layer_count|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|format|Defaults to [Option::None]|
///|dimension|Defaults to [Option::None]|
///|usage|Defaults to [Option::None]|
///|aspect|Defaults to [wgpu::TextureAspect::default()]|
///|base_mip_level|Defaults to [wgpu::u32::default()]|
///|mip_level_count|Defaults to [Option::None]|
///|base_array_layer|Defaults to [wgpu::u32::default()]|
///|array_layer_count|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("TexelCopyTextureInfo") Id(4844)

*/

#[doc = "\nReturns [`TexelCopyTextureInfoBuilder`] for building [`wgpu::TexelCopyTextureInfo`]\n\n|Builder Field|Status|\n|-|-|\n|texture|Required|\n|mip_level|Required|\n|origin|Defaults to [wgpu::Origin3d::default()]|\n|aspect|Defaults to [wgpu::TextureAspect::default()]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|texture|Required|
///|mip_level|Required|
///|origin|Defaults to [wgpu::Origin3d::default()]|
///|aspect|Defaults to [wgpu::TextureAspect::default()]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:6787
impl<T> Default for RenderBundleDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

*/

#[doc = "\nReturns [`RenderBundleDescriptorBuilder`] for building [`wgpu::RenderBundleDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn render_bundle_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::RenderBundleDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
) -> RenderBundleDescriptor<'a> {
    RenderBundleDescriptor { label }
}

/*
Unhandled Some("CreateBlasDescriptor") Id(5813)

*/

#[doc = "\nReturns [`CreateBlasDescriptorBuilder`] for building [`wgpu::CreateBlasDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|flags|Required|\n|update_mode|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|flags|Required|
///|update_mode|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Unhandled Some("BlasTriangleGeometrySizeDescriptor") Id(329)

*/

#[doc = "\nReturns [`BlasTriangleGeometrySizeDescriptorBuilder`] for building [`wgpu::BlasTriangleGeometrySizeDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|vertex_format|Required|\n|vertex_count|Required|\n|index_format|Defaults to [Option::None]|\n|index_count|Defaults to [Option::None]|\n|flags|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|vertex_format|Required|
///|vertex_count|Required|
///|index_format|Defaults to [Option::None]|
///|index_count|Defaults to [Option::None]|
///|flags|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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

#[doc = "\nReturns [`SamplerDescriptorBuilder`] for building [`wgpu::SamplerDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n|address_mode_u|Defaults to [wgpu::AddressMode::default()]|\n|address_mode_v|Defaults to [wgpu::AddressMode::default()]|\n|address_mode_w|Defaults to [wgpu::AddressMode::default()]|\n|mag_filter|Defaults to [wgpu::FilterMode::default()]|\n|min_filter|Defaults to [wgpu::FilterMode::default()]|\n|mipmap_filter|Defaults to [wgpu::FilterMode::default()]|\n|lod_min_clamp|Defaults to [0.0]|\n|lod_max_clamp|Defaults to [32.0]|\n|compare|Defaults to [Option::None]|\n|anisotropy_clamp|Defaults to [1u16]|\n|border_color|Defaults to [Option::None]|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///|address_mode_u|Defaults to [wgpu::AddressMode::default()]|
///|address_mode_v|Defaults to [wgpu::AddressMode::default()]|
///|address_mode_w|Defaults to [wgpu::AddressMode::default()]|
///|mag_filter|Defaults to [wgpu::FilterMode::default()]|
///|min_filter|Defaults to [wgpu::FilterMode::default()]|
///|mipmap_filter|Defaults to [wgpu::FilterMode::default()]|
///|lod_min_clamp|Defaults to [0.0]|
///|lod_max_clamp|Defaults to [32.0]|
///|compare|Defaults to [Option::None]|
///|anisotropy_clamp|Defaults to [1u16]|
///|border_color|Defaults to [Option::None]|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
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
Default from: /Users/work/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-types-27.0.1/src/lib.rs:5449
impl<T> Default for CommandEncoderDescriptor<Option<T>> {
    fn default() -> Self {
        Self { label: None }
    }
}

*/

#[doc = "\nReturns [`CommandEncoderDescriptorBuilder`] for building [`wgpu::CommandEncoderDescriptor`]\n\n|Builder Field|Status|\n|-|-|\n|label|Required|\n"]
# [bon :: builder (builder_type (doc { 
///|Builder Field|Status|
///|-|-|
///|label|Required|
///
 }) , state_mod (vis = "pub(crate)") , finish_fn = build , derive (Into))]
pub fn command_encoder_descriptor<'a>(
    #[rustfmt::skip]
    #[doc = "Sets [`wgpu::CommandEncoderDescriptor::label`]"]
    #[builder(start_fn, into)]
    label: Label<'a>,
) -> CommandEncoderDescriptor<'a> {
    CommandEncoderDescriptor { label }
}

pub mod initializers {
    pub use super::{
        backend_options, bind_group_descriptor, bind_group_entry, bind_group_layout_descriptor,
        bind_group_layout_entry, blas_build_entry, blas_triangle_geometry,
        blas_triangle_geometry_size_descriptor, blend_component, blend_state, buffer_binding,
        buffer_descriptor, buffer_init_descriptor, buffer_transition, color, color_target_state,
        command_buffer_descriptor, command_encoder_descriptor, compilation_info,
        compute_pass_descriptor, compute_pass_timestamp_writes, compute_pipeline_descriptor,
        copy_external_image_dest_info, core_counters, create_blas_descriptor,
        create_tlas_descriptor, depth_bias_state, depth_stencil_state, device_descriptor,
        dispatch_indirect_args, downlevel_limits, draw_indexed_indirect_args, draw_indirect_args,
        dx_12_backend_options, extent3d, external_texture_descriptor,
        external_texture_transfer_function, fragment_state, gl_backend_options,
        image_subresource_range, instance_descriptor, memory_budget_thresholds,
        mesh_pipeline_descriptor, mesh_state, multisample_state, noop_backend_options, operations,
        origin2d, origin3d, pipeline_cache_descriptor, pipeline_compilation_options,
        pipeline_layout_descriptor, primitive_state, push_constant_range, query_set_descriptor,
        render_bundle_depth_stencil, render_bundle_descriptor, render_bundle_encoder_descriptor,
        render_pass_color_attachment, render_pass_depth_stencil_attachment, render_pass_descriptor,
        render_pass_timestamp_writes, render_pipeline_descriptor, request_adapter_options,
        request_adapter_options_base, sampler_descriptor, shader_module_descriptor,
        shader_module_descriptor_passthrough, shader_runtime_checks, stencil_face_state,
        stencil_state, surface_configuration, task_state, texel_copy_buffer_info,
        texel_copy_buffer_info_base, texel_copy_buffer_layout, texel_copy_texture_info,
        texel_copy_texture_info_base, texture_descriptor, texture_transition,
        texture_view_descriptor, vertex_attribute, vertex_buffer_layout, vertex_state,
    };
}
pub mod builders {
    pub use super::{
        BackendOptionsBuilder, BindGroupDescriptorBuilder, BindGroupEntryBuilder,
        BindGroupLayoutDescriptorBuilder, BindGroupLayoutEntryBuilder, BlasBuildEntryBuilder,
        BlasTriangleGeometryBuilder, BlasTriangleGeometrySizeDescriptorBuilder,
        BlendComponentBuilder, BlendStateBuilder, BufferBindingBuilder, BufferDescriptorBuilder,
        BufferInitDescriptorBuilder, BufferTransitionBuilder, ColorBuilder,
        ColorTargetStateBuilder, CommandBufferDescriptorBuilder, CommandEncoderDescriptorBuilder,
        CompilationInfoBuilder, ComputePassDescriptorBuilder, ComputePassTimestampWritesBuilder,
        ComputePipelineDescriptorBuilder, CopyExternalImageDestInfoBuilder, CoreCountersBuilder,
        CreateBlasDescriptorBuilder, CreateTlasDescriptorBuilder, DepthBiasStateBuilder,
        DepthStencilStateBuilder, DeviceDescriptorBuilder, DispatchIndirectArgsBuilder,
        DownlevelLimitsBuilder, DrawIndexedIndirectArgsBuilder, DrawIndirectArgsBuilder,
        Dx12BackendOptionsBuilder, Extent3dBuilder, ExternalTextureDescriptorBuilder,
        ExternalTextureTransferFunctionBuilder, FragmentStateBuilder, GlBackendOptionsBuilder,
        ImageSubresourceRangeBuilder, InstanceDescriptorBuilder, MemoryBudgetThresholdsBuilder,
        MeshPipelineDescriptorBuilder, MeshStateBuilder, MultisampleStateBuilder,
        NoopBackendOptionsBuilder, OperationsBuilder, Origin2dBuilder, Origin3dBuilder,
        PipelineCacheDescriptorBuilder, PipelineCompilationOptionsBuilder,
        PipelineLayoutDescriptorBuilder, PrimitiveStateBuilder, PushConstantRangeBuilder,
        QuerySetDescriptorBuilder, RenderBundleDepthStencilBuilder, RenderBundleDescriptorBuilder,
        RenderBundleEncoderDescriptorBuilder, RenderPassColorAttachmentBuilder,
        RenderPassDepthStencilAttachmentBuilder, RenderPassDescriptorBuilder,
        RenderPassTimestampWritesBuilder, RenderPipelineDescriptorBuilder,
        RequestAdapterOptionsBaseBuilder, RequestAdapterOptionsBuilder, SamplerDescriptorBuilder,
        ShaderModuleDescriptorBuilder, ShaderModuleDescriptorPassthroughBuilder,
        ShaderRuntimeChecksBuilder, StencilFaceStateBuilder, StencilStateBuilder,
        SurfaceConfigurationBuilder, TaskStateBuilder, TexelCopyBufferInfoBaseBuilder,
        TexelCopyBufferInfoBuilder, TexelCopyBufferLayoutBuilder, TexelCopyTextureInfoBaseBuilder,
        TexelCopyTextureInfoBuilder, TextureDescriptorBuilder, TextureTransitionBuilder,
        TextureViewDescriptorBuilder, VertexAttributeBuilder, VertexBufferLayoutBuilder,
        VertexStateBuilder,
    };
}
