pub mod external_texture_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct WidthSet(pub u32);
    pub struct WidthUnset(PhantomData<u32>);
    pub trait WidthRequired {}
    pub trait WidthIsUnset: WidthRequired {}
    impl WidthRequired for WidthUnset {}
    impl WidthIsUnset for WidthUnset {}
    impl WidthRequired for WidthSet {}
    impl WidthSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct HeightSet(pub u32);
    pub struct HeightUnset(PhantomData<u32>);
    pub trait HeightRequired {}
    pub trait HeightIsUnset: HeightRequired {}
    impl HeightRequired for HeightUnset {}
    impl HeightIsUnset for HeightUnset {}
    impl HeightRequired for HeightSet {}
    impl HeightSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct FormatSet(pub wgpu::ExternalTextureFormat);
    pub struct FormatUnset(PhantomData<wgpu::ExternalTextureFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::ExternalTextureFormat {
            self.0
        }
    }
    pub struct YuvConversionMatrixSet(pub [f32; 16]);
    pub struct YuvConversionMatrixUnset(PhantomData<[f32; 16]>);
    pub trait YuvConversionMatrixRequired {}
    pub trait YuvConversionMatrixIsUnset: YuvConversionMatrixRequired {}
    impl YuvConversionMatrixRequired for YuvConversionMatrixUnset {}
    impl YuvConversionMatrixIsUnset for YuvConversionMatrixUnset {}
    impl YuvConversionMatrixRequired for YuvConversionMatrixSet {}
    impl YuvConversionMatrixSet {
        fn get(self) -> [f32; 16] {
            self.0
        }
    }
    pub struct GamutConversionMatrixSet(pub [f32; 9]);
    pub struct GamutConversionMatrixUnset(PhantomData<[f32; 9]>);
    pub trait GamutConversionMatrixRequired {}
    pub trait GamutConversionMatrixIsUnset: GamutConversionMatrixRequired {}
    impl GamutConversionMatrixRequired for GamutConversionMatrixUnset {}
    impl GamutConversionMatrixIsUnset for GamutConversionMatrixUnset {}
    impl GamutConversionMatrixRequired for GamutConversionMatrixSet {}
    impl GamutConversionMatrixSet {
        fn get(self) -> [f32; 9] {
            self.0
        }
    }
    pub struct SrcTransferFunctionSet(pub wgpu::ExternalTextureTransferFunction);
    pub struct SrcTransferFunctionUnset(PhantomData<wgpu::ExternalTextureTransferFunction>);
    pub trait SrcTransferFunctionRequired {}
    pub trait SrcTransferFunctionIsUnset: SrcTransferFunctionRequired {}
    impl SrcTransferFunctionRequired for SrcTransferFunctionUnset {}
    impl SrcTransferFunctionIsUnset for SrcTransferFunctionUnset {}
    impl SrcTransferFunctionRequired for SrcTransferFunctionSet {}
    impl SrcTransferFunctionSet {
        fn get(self) -> wgpu::ExternalTextureTransferFunction {
            self.0
        }
    }
    pub struct DstTransferFunctionSet(pub wgpu::ExternalTextureTransferFunction);
    pub struct DstTransferFunctionUnset(PhantomData<wgpu::ExternalTextureTransferFunction>);
    pub trait DstTransferFunctionRequired {}
    pub trait DstTransferFunctionIsUnset: DstTransferFunctionRequired {}
    impl DstTransferFunctionRequired for DstTransferFunctionUnset {}
    impl DstTransferFunctionIsUnset for DstTransferFunctionUnset {}
    impl DstTransferFunctionRequired for DstTransferFunctionSet {}
    impl DstTransferFunctionSet {
        fn get(self) -> wgpu::ExternalTextureTransferFunction {
            self.0
        }
    }
    pub struct SampleTransformSet(pub [f32; 6]);
    pub struct SampleTransformUnset(PhantomData<[f32; 6]>);
    pub trait SampleTransformRequired {}
    pub trait SampleTransformIsUnset: SampleTransformRequired {}
    impl SampleTransformRequired for SampleTransformUnset {}
    impl SampleTransformIsUnset for SampleTransformUnset {}
    impl SampleTransformRequired for SampleTransformSet {}
    impl SampleTransformSet {
        fn get(self) -> [f32; 6] {
            self.0
        }
    }
    pub struct LoadTransformSet(pub [f32; 6]);
    pub struct LoadTransformUnset(PhantomData<[f32; 6]>);
    pub trait LoadTransformRequired {}
    pub trait LoadTransformIsUnset: LoadTransformRequired {}
    impl LoadTransformRequired for LoadTransformUnset {}
    impl LoadTransformIsUnset for LoadTransformUnset {}
    impl LoadTransformRequired for LoadTransformSet {}
    impl LoadTransformSet {
        fn get(self) -> [f32; 6] {
            self.0
        }
    }
}

pub mod fragment_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ModuleSet<'a>(pub &'a wgpu::ShaderModule);
    pub struct ModuleUnset<'a>(PhantomData<&'a wgpu::ShaderModule>);
    pub trait ModuleRequired {}
    pub trait ModuleIsUnset: ModuleRequired {}
    impl<'a> ModuleRequired for ModuleUnset<'a> {}
    impl<'a> ModuleIsUnset for ModuleUnset<'a> {}
    impl<'a> ModuleRequired for ModuleSet<'a> {}
    impl<'a> ModuleSet<'a> {
        fn get(self) -> &'a wgpu::ShaderModule {
            self.0
        }
    }
    pub struct EntryPointSet<'a>(pub Option<&'a str>);
    pub struct EntryPointUnset<'a>(PhantomData<Option<&'a str>>);
    pub trait EntryPointRequired {}
    pub trait EntryPointIsUnset: EntryPointRequired {}
    impl<'a> EntryPointRequired for EntryPointUnset<'a> {}
    impl<'a> EntryPointIsUnset for EntryPointUnset<'a> {}
    impl<'a> EntryPointRequired for EntryPointSet<'a> {}
    impl<'a> EntryPointSet<'a> {
        fn get(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct CompilationOptionsSet<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    pub struct CompilationOptionsUnset<'a>(PhantomData<wgpu::PipelineCompilationOptions<'a>>);
    pub trait CompilationOptionsRequired {}
    pub trait CompilationOptionsIsUnset: CompilationOptionsRequired {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsIsUnset for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsSet<'a> {}
    impl<'a> CompilationOptionsSet<'a> {
        fn get(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    pub struct TargetsSet<'a>(pub &'a [Option<wgpu::ColorTargetState>]);
    pub struct TargetsUnset<'a>(PhantomData<&'a [Option<wgpu::ColorTargetState>]>);
    pub trait TargetsRequired {}
    pub trait TargetsIsUnset: TargetsRequired {}
    impl<'a> TargetsRequired for TargetsUnset<'a> {}
    impl<'a> TargetsIsUnset for TargetsUnset<'a> {}
    impl<'a> TargetsRequired for TargetsSet<'a> {}
    impl<'a> TargetsSet<'a> {
        fn get(self) -> &'a [Option<wgpu::ColorTargetState>] {
            self.0
        }
    }
}

pub mod extent_3_d_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct WidthSet(pub u32);
    pub struct WidthUnset(PhantomData<u32>);
    pub trait WidthRequired {}
    pub trait WidthIsUnset: WidthRequired {}
    impl WidthRequired for WidthUnset {}
    impl WidthIsUnset for WidthUnset {}
    impl WidthRequired for WidthSet {}
    impl WidthSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct HeightSet(pub u32);
    pub struct HeightUnset(PhantomData<u32>);
    pub trait HeightRequired {}
    pub trait HeightIsUnset: HeightRequired {}
    impl HeightRequired for HeightUnset {}
    impl HeightIsUnset for HeightUnset {}
    impl HeightRequired for HeightSet {}
    impl HeightSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct DepthOrArrayLayersSet(pub u32);
    pub struct DepthOrArrayLayersUnset(PhantomData<u32>);
    pub trait DepthOrArrayLayersRequired {}
    pub trait DepthOrArrayLayersIsUnset: DepthOrArrayLayersRequired {}
    impl DepthOrArrayLayersRequired for DepthOrArrayLayersUnset {}
    impl DepthOrArrayLayersIsUnset for DepthOrArrayLayersUnset {}
    impl DepthOrArrayLayersRequired for DepthOrArrayLayersSet {}
    impl DepthOrArrayLayersSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod primitive_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct TopologySet(pub wgpu::PrimitiveTopology);
    pub struct TopologyUnset(PhantomData<wgpu::PrimitiveTopology>);
    pub trait TopologyRequired {}
    pub trait TopologyIsUnset: TopologyRequired {}
    impl TopologyRequired for TopologyUnset {}
    impl TopologyIsUnset for TopologyUnset {}
    impl TopologyRequired for TopologySet {}
    impl TopologySet {
        fn get(self) -> wgpu::PrimitiveTopology {
            self.0
        }
    }
    pub struct StripIndexFormatSet(pub Option<wgpu::IndexFormat>);
    pub struct StripIndexFormatUnset(PhantomData<Option<wgpu::IndexFormat>>);
    pub trait StripIndexFormatRequired {}
    pub trait StripIndexFormatIsUnset: StripIndexFormatRequired {}
    impl StripIndexFormatRequired for StripIndexFormatUnset {}
    impl StripIndexFormatIsUnset for StripIndexFormatUnset {}
    impl StripIndexFormatRequired for StripIndexFormatSet {}
    impl StripIndexFormatSet {
        fn get(self) -> Option<wgpu::IndexFormat> {
            self.0
        }
    }
    pub struct FrontFaceSet(pub wgpu::FrontFace);
    pub struct FrontFaceUnset(PhantomData<wgpu::FrontFace>);
    pub trait FrontFaceRequired {}
    pub trait FrontFaceIsUnset: FrontFaceRequired {}
    impl FrontFaceRequired for FrontFaceUnset {}
    impl FrontFaceIsUnset for FrontFaceUnset {}
    impl FrontFaceRequired for FrontFaceSet {}
    impl FrontFaceSet {
        fn get(self) -> wgpu::FrontFace {
            self.0
        }
    }
    pub struct CullModeSet(pub Option<wgpu::Face>);
    pub struct CullModeUnset(PhantomData<Option<wgpu::Face>>);
    pub trait CullModeRequired {}
    pub trait CullModeIsUnset: CullModeRequired {}
    impl CullModeRequired for CullModeUnset {}
    impl CullModeIsUnset for CullModeUnset {}
    impl CullModeRequired for CullModeSet {}
    impl CullModeSet {
        fn get(self) -> Option<wgpu::Face> {
            self.0
        }
    }
    pub struct UnclippedDepthSet(pub bool);
    pub struct UnclippedDepthUnset(PhantomData<bool>);
    pub trait UnclippedDepthRequired {}
    pub trait UnclippedDepthIsUnset: UnclippedDepthRequired {}
    impl UnclippedDepthRequired for UnclippedDepthUnset {}
    impl UnclippedDepthIsUnset for UnclippedDepthUnset {}
    impl UnclippedDepthRequired for UnclippedDepthSet {}
    impl UnclippedDepthSet {
        fn get(self) -> bool {
            self.0
        }
    }
    pub struct PolygonModeSet(pub wgpu::PolygonMode);
    pub struct PolygonModeUnset(PhantomData<wgpu::PolygonMode>);
    pub trait PolygonModeRequired {}
    pub trait PolygonModeIsUnset: PolygonModeRequired {}
    impl PolygonModeRequired for PolygonModeUnset {}
    impl PolygonModeIsUnset for PolygonModeUnset {}
    impl PolygonModeRequired for PolygonModeSet {}
    impl PolygonModeSet {
        fn get(self) -> wgpu::PolygonMode {
            self.0
        }
    }
    pub struct ConservativeSet(pub bool);
    pub struct ConservativeUnset(PhantomData<bool>);
    pub trait ConservativeRequired {}
    pub trait ConservativeIsUnset: ConservativeRequired {}
    impl ConservativeRequired for ConservativeUnset {}
    impl ConservativeIsUnset for ConservativeUnset {}
    impl ConservativeRequired for ConservativeSet {}
    impl ConservativeSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod gl_backend_options_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct GlesMinorVersionSet(pub wgpu::Gles3MinorVersion);
    pub struct GlesMinorVersionUnset(PhantomData<wgpu::Gles3MinorVersion>);
    pub trait GlesMinorVersionRequired {}
    pub trait GlesMinorVersionIsUnset: GlesMinorVersionRequired {}
    impl GlesMinorVersionRequired for GlesMinorVersionUnset {}
    impl GlesMinorVersionIsUnset for GlesMinorVersionUnset {}
    impl GlesMinorVersionRequired for GlesMinorVersionSet {}
    impl GlesMinorVersionSet {
        fn get(self) -> wgpu::Gles3MinorVersion {
            self.0
        }
    }
    pub struct FenceBehaviorSet(pub wgpu::GlFenceBehavior);
    pub struct FenceBehaviorUnset(PhantomData<wgpu::GlFenceBehavior>);
    pub trait FenceBehaviorRequired {}
    pub trait FenceBehaviorIsUnset: FenceBehaviorRequired {}
    impl FenceBehaviorRequired for FenceBehaviorUnset {}
    impl FenceBehaviorIsUnset for FenceBehaviorUnset {}
    impl FenceBehaviorRequired for FenceBehaviorSet {}
    impl FenceBehaviorSet {
        fn get(self) -> wgpu::GlFenceBehavior {
            self.0
        }
    }
}

pub mod shader_module_descriptor_passthrough_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct EntryPointSet(pub String);
    pub struct EntryPointUnset(PhantomData<String>);
    pub trait EntryPointRequired {}
    pub trait EntryPointIsUnset: EntryPointRequired {}
    impl EntryPointRequired for EntryPointUnset {}
    impl EntryPointIsUnset for EntryPointUnset {}
    impl EntryPointRequired for EntryPointSet {}
    impl EntryPointSet {
        fn get(self) -> String {
            self.0
        }
    }
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct NumWorkgroupsSet(pub (u32, u32, u32));
    pub struct NumWorkgroupsUnset(PhantomData<(u32, u32, u32)>);
    pub trait NumWorkgroupsRequired {}
    pub trait NumWorkgroupsIsUnset: NumWorkgroupsRequired {}
    impl NumWorkgroupsRequired for NumWorkgroupsUnset {}
    impl NumWorkgroupsIsUnset for NumWorkgroupsUnset {}
    impl NumWorkgroupsRequired for NumWorkgroupsSet {}
    impl NumWorkgroupsSet {
        fn get(self) -> (u32, u32, u32) {
            self.0
        }
    }
    pub struct RuntimeChecksSet(pub wgpu::ShaderRuntimeChecks);
    pub struct RuntimeChecksUnset(PhantomData<wgpu::ShaderRuntimeChecks>);
    pub trait RuntimeChecksRequired {}
    pub trait RuntimeChecksIsUnset: RuntimeChecksRequired {}
    impl RuntimeChecksRequired for RuntimeChecksUnset {}
    impl RuntimeChecksIsUnset for RuntimeChecksUnset {}
    impl RuntimeChecksRequired for RuntimeChecksSet {}
    impl RuntimeChecksSet {
        fn get(self) -> wgpu::ShaderRuntimeChecks {
            self.0
        }
    }
    pub struct SpirvSet<'a>(pub Option<Cow<'a, [u32]>>);
    pub struct SpirvUnset<'a>(PhantomData<Option<Cow<'a, [u32]>>>);
    pub trait SpirvRequired {}
    pub trait SpirvIsUnset: SpirvRequired {}
    impl<'a> SpirvRequired for SpirvUnset<'a> {}
    impl<'a> SpirvIsUnset for SpirvUnset<'a> {}
    impl<'a> SpirvRequired for SpirvSet<'a> {}
    impl<'a> SpirvSet<'a> {
        fn get(self) -> Option<Cow<'a, [u32]>> {
            self.0
        }
    }
    pub struct DxilSet<'a>(pub Option<Cow<'a, [u8]>>);
    pub struct DxilUnset<'a>(PhantomData<Option<Cow<'a, [u8]>>>);
    pub trait DxilRequired {}
    pub trait DxilIsUnset: DxilRequired {}
    impl<'a> DxilRequired for DxilUnset<'a> {}
    impl<'a> DxilIsUnset for DxilUnset<'a> {}
    impl<'a> DxilRequired for DxilSet<'a> {}
    impl<'a> DxilSet<'a> {
        fn get(self) -> Option<Cow<'a, [u8]>> {
            self.0
        }
    }
    pub struct MslSet<'a>(pub Option<Cow<'a, str>>);
    pub struct MslUnset<'a>(PhantomData<Option<Cow<'a, str>>>);
    pub trait MslRequired {}
    pub trait MslIsUnset: MslRequired {}
    impl<'a> MslRequired for MslUnset<'a> {}
    impl<'a> MslIsUnset for MslUnset<'a> {}
    impl<'a> MslRequired for MslSet<'a> {}
    impl<'a> MslSet<'a> {
        fn get(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    pub struct HlslSet<'a>(pub Option<Cow<'a, str>>);
    pub struct HlslUnset<'a>(PhantomData<Option<Cow<'a, str>>>);
    pub trait HlslRequired {}
    pub trait HlslIsUnset: HlslRequired {}
    impl<'a> HlslRequired for HlslUnset<'a> {}
    impl<'a> HlslIsUnset for HlslUnset<'a> {}
    impl<'a> HlslRequired for HlslSet<'a> {}
    impl<'a> HlslSet<'a> {
        fn get(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    pub struct GlslSet<'a>(pub Option<Cow<'a, str>>);
    pub struct GlslUnset<'a>(PhantomData<Option<Cow<'a, str>>>);
    pub trait GlslRequired {}
    pub trait GlslIsUnset: GlslRequired {}
    impl<'a> GlslRequired for GlslUnset<'a> {}
    impl<'a> GlslIsUnset for GlslUnset<'a> {}
    impl<'a> GlslRequired for GlslSet<'a> {}
    impl<'a> GlslSet<'a> {
        fn get(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    pub struct WgslSet<'a>(pub Option<Cow<'a, str>>);
    pub struct WgslUnset<'a>(PhantomData<Option<Cow<'a, str>>>);
    pub trait WgslRequired {}
    pub trait WgslIsUnset: WgslRequired {}
    impl<'a> WgslRequired for WgslUnset<'a> {}
    impl<'a> WgslIsUnset for WgslUnset<'a> {}
    impl<'a> WgslRequired for WgslSet<'a> {}
    impl<'a> WgslSet<'a> {
        fn get(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
}

pub mod surface_configuration_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct UsageSet(pub wgpu::TextureUsages);
    pub struct UsageUnset(PhantomData<wgpu::TextureUsages>);
    pub trait UsageRequired {}
    pub trait UsageIsUnset: UsageRequired {}
    impl UsageRequired for UsageUnset {}
    impl UsageIsUnset for UsageUnset {}
    impl UsageRequired for UsageSet {}
    impl UsageSet {
        fn get(self) -> wgpu::TextureUsages {
            self.0
        }
    }
    pub struct FormatSet(pub wgpu::TextureFormat);
    pub struct FormatUnset(PhantomData<wgpu::TextureFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::TextureFormat {
            self.0
        }
    }
    pub struct WidthSet(pub u32);
    pub struct WidthUnset(PhantomData<u32>);
    pub trait WidthRequired {}
    pub trait WidthIsUnset: WidthRequired {}
    impl WidthRequired for WidthUnset {}
    impl WidthIsUnset for WidthUnset {}
    impl WidthRequired for WidthSet {}
    impl WidthSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct HeightSet(pub u32);
    pub struct HeightUnset(PhantomData<u32>);
    pub trait HeightRequired {}
    pub trait HeightIsUnset: HeightRequired {}
    impl HeightRequired for HeightUnset {}
    impl HeightIsUnset for HeightUnset {}
    impl HeightRequired for HeightSet {}
    impl HeightSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct PresentModeSet(pub wgpu::PresentMode);
    pub struct PresentModeUnset(PhantomData<wgpu::PresentMode>);
    pub trait PresentModeRequired {}
    pub trait PresentModeIsUnset: PresentModeRequired {}
    impl PresentModeRequired for PresentModeUnset {}
    impl PresentModeIsUnset for PresentModeUnset {}
    impl PresentModeRequired for PresentModeSet {}
    impl PresentModeSet {
        fn get(self) -> wgpu::PresentMode {
            self.0
        }
    }
    pub struct DesiredMaximumFrameLatencySet(pub u32);
    pub struct DesiredMaximumFrameLatencyUnset(PhantomData<u32>);
    pub trait DesiredMaximumFrameLatencyRequired {}
    pub trait DesiredMaximumFrameLatencyIsUnset: DesiredMaximumFrameLatencyRequired {}
    impl DesiredMaximumFrameLatencyRequired for DesiredMaximumFrameLatencyUnset {}
    impl DesiredMaximumFrameLatencyIsUnset for DesiredMaximumFrameLatencyUnset {}
    impl DesiredMaximumFrameLatencyRequired for DesiredMaximumFrameLatencySet {}
    impl DesiredMaximumFrameLatencySet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct AlphaModeSet(pub wgpu::CompositeAlphaMode);
    pub struct AlphaModeUnset(PhantomData<wgpu::CompositeAlphaMode>);
    pub trait AlphaModeRequired {}
    pub trait AlphaModeIsUnset: AlphaModeRequired {}
    impl AlphaModeRequired for AlphaModeUnset {}
    impl AlphaModeIsUnset for AlphaModeUnset {}
    impl AlphaModeRequired for AlphaModeSet {}
    impl AlphaModeSet {
        fn get(self) -> wgpu::CompositeAlphaMode {
            self.0
        }
    }
    pub struct ViewFormatsSet(pub Vec<wgpu::TextureFormat>);
    pub struct ViewFormatsUnset(PhantomData<Vec<wgpu::TextureFormat>>);
    pub trait ViewFormatsRequired {}
    pub trait ViewFormatsIsUnset: ViewFormatsRequired {}
    impl ViewFormatsRequired for ViewFormatsUnset {}
    impl ViewFormatsIsUnset for ViewFormatsUnset {}
    impl ViewFormatsRequired for ViewFormatsSet {}
    impl ViewFormatsSet {
        fn get(self) -> Vec<wgpu::TextureFormat> {
            self.0
        }
    }
}

pub mod texel_copy_texture_info_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct TextureSet<'a>(pub &'a wgpu::Texture);
    pub struct TextureUnset<'a>(PhantomData<&'a wgpu::Texture>);
    pub trait TextureRequired {}
    pub trait TextureIsUnset: TextureRequired {}
    impl<'a> TextureRequired for TextureUnset<'a> {}
    impl<'a> TextureIsUnset for TextureUnset<'a> {}
    impl<'a> TextureRequired for TextureSet<'a> {}
    impl<'a> TextureSet<'a> {
        fn get(self) -> &'a wgpu::Texture {
            self.0
        }
    }
    pub struct MipLevelSet(pub u32);
    pub struct MipLevelUnset(PhantomData<u32>);
    pub trait MipLevelRequired {}
    pub trait MipLevelIsUnset: MipLevelRequired {}
    impl MipLevelRequired for MipLevelUnset {}
    impl MipLevelIsUnset for MipLevelUnset {}
    impl MipLevelRequired for MipLevelSet {}
    impl MipLevelSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct OriginSet(pub wgpu::Origin3d);
    pub struct OriginUnset(PhantomData<wgpu::Origin3d>);
    pub trait OriginRequired {}
    pub trait OriginIsUnset: OriginRequired {}
    impl OriginRequired for OriginUnset {}
    impl OriginIsUnset for OriginUnset {}
    impl OriginRequired for OriginSet {}
    impl OriginSet {
        fn get(self) -> wgpu::Origin3d {
            self.0
        }
    }
    pub struct AspectSet(pub wgpu::TextureAspect);
    pub struct AspectUnset(PhantomData<wgpu::TextureAspect>);
    pub trait AspectRequired {}
    pub trait AspectIsUnset: AspectRequired {}
    impl AspectRequired for AspectUnset {}
    impl AspectIsUnset for AspectUnset {}
    impl AspectRequired for AspectSet {}
    impl AspectSet {
        fn get(self) -> wgpu::TextureAspect {
            self.0
        }
    }
}

pub mod render_pipeline_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct LayoutSet<'a>(pub Option<&'a wgpu::PipelineLayout>);
    pub struct LayoutUnset<'a>(PhantomData<Option<&'a wgpu::PipelineLayout>>);
    pub trait LayoutRequired {}
    pub trait LayoutIsUnset: LayoutRequired {}
    impl<'a> LayoutRequired for LayoutUnset<'a> {}
    impl<'a> LayoutIsUnset for LayoutUnset<'a> {}
    impl<'a> LayoutRequired for LayoutSet<'a> {}
    impl<'a> LayoutSet<'a> {
        fn get(self) -> Option<&'a wgpu::PipelineLayout> {
            self.0
        }
    }
    pub struct VertexSet<'a>(pub wgpu::VertexState<'a>);
    pub struct VertexUnset<'a>(PhantomData<wgpu::VertexState<'a>>);
    pub trait VertexRequired {}
    pub trait VertexIsUnset: VertexRequired {}
    impl<'a> VertexRequired for VertexUnset<'a> {}
    impl<'a> VertexIsUnset for VertexUnset<'a> {}
    impl<'a> VertexRequired for VertexSet<'a> {}
    impl<'a> VertexSet<'a> {
        fn get(self) -> wgpu::VertexState<'a> {
            self.0
        }
    }
    pub struct PrimitiveSet(pub wgpu::PrimitiveState);
    pub struct PrimitiveUnset(PhantomData<wgpu::PrimitiveState>);
    pub trait PrimitiveRequired {}
    pub trait PrimitiveIsUnset: PrimitiveRequired {}
    impl PrimitiveRequired for PrimitiveUnset {}
    impl PrimitiveIsUnset for PrimitiveUnset {}
    impl PrimitiveRequired for PrimitiveSet {}
    impl PrimitiveSet {
        fn get(self) -> wgpu::PrimitiveState {
            self.0
        }
    }
    pub struct DepthStencilSet(pub Option<wgpu::DepthStencilState>);
    pub struct DepthStencilUnset(PhantomData<Option<wgpu::DepthStencilState>>);
    pub trait DepthStencilRequired {}
    pub trait DepthStencilIsUnset: DepthStencilRequired {}
    impl DepthStencilRequired for DepthStencilUnset {}
    impl DepthStencilIsUnset for DepthStencilUnset {}
    impl DepthStencilRequired for DepthStencilSet {}
    impl DepthStencilSet {
        fn get(self) -> Option<wgpu::DepthStencilState> {
            self.0
        }
    }
    pub struct MultisampleSet(pub wgpu::MultisampleState);
    pub struct MultisampleUnset(PhantomData<wgpu::MultisampleState>);
    pub trait MultisampleRequired {}
    pub trait MultisampleIsUnset: MultisampleRequired {}
    impl MultisampleRequired for MultisampleUnset {}
    impl MultisampleIsUnset for MultisampleUnset {}
    impl MultisampleRequired for MultisampleSet {}
    impl MultisampleSet {
        fn get(self) -> wgpu::MultisampleState {
            self.0
        }
    }
    pub struct FragmentSet<'a>(pub Option<wgpu::FragmentState<'a>>);
    pub struct FragmentUnset<'a>(PhantomData<Option<wgpu::FragmentState<'a>>>);
    pub trait FragmentRequired {}
    pub trait FragmentIsUnset: FragmentRequired {}
    impl<'a> FragmentRequired for FragmentUnset<'a> {}
    impl<'a> FragmentIsUnset for FragmentUnset<'a> {}
    impl<'a> FragmentRequired for FragmentSet<'a> {}
    impl<'a> FragmentSet<'a> {
        fn get(self) -> Option<wgpu::FragmentState<'a>> {
            self.0
        }
    }
    pub struct MultiviewSet(pub Option<NonZeroU32>);
    pub struct MultiviewUnset(PhantomData<Option<NonZeroU32>>);
    pub trait MultiviewRequired {}
    pub trait MultiviewIsUnset: MultiviewRequired {}
    impl MultiviewRequired for MultiviewUnset {}
    impl MultiviewIsUnset for MultiviewUnset {}
    impl MultiviewRequired for MultiviewSet {}
    impl MultiviewSet {
        fn get(self) -> Option<NonZeroU32> {
            self.0
        }
    }
    pub struct CacheSet<'a>(pub Option<&'a wgpu::PipelineCache>);
    pub struct CacheUnset<'a>(PhantomData<Option<&'a wgpu::PipelineCache>>);
    pub trait CacheRequired {}
    pub trait CacheIsUnset: CacheRequired {}
    impl<'a> CacheRequired for CacheUnset<'a> {}
    impl<'a> CacheIsUnset for CacheUnset<'a> {}
    impl<'a> CacheRequired for CacheSet<'a> {}
    impl<'a> CacheSet<'a> {
        fn get(self) -> Option<&'a wgpu::PipelineCache> {
            self.0
        }
    }
}

pub mod render_bundle_depth_stencil_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct FormatSet(pub wgpu::TextureFormat);
    pub struct FormatUnset(PhantomData<wgpu::TextureFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::TextureFormat {
            self.0
        }
    }
    pub struct DepthReadOnlySet(pub bool);
    pub struct DepthReadOnlyUnset(PhantomData<bool>);
    pub trait DepthReadOnlyRequired {}
    pub trait DepthReadOnlyIsUnset: DepthReadOnlyRequired {}
    impl DepthReadOnlyRequired for DepthReadOnlyUnset {}
    impl DepthReadOnlyIsUnset for DepthReadOnlyUnset {}
    impl DepthReadOnlyRequired for DepthReadOnlySet {}
    impl DepthReadOnlySet {
        fn get(self) -> bool {
            self.0
        }
    }
    pub struct StencilReadOnlySet(pub bool);
    pub struct StencilReadOnlyUnset(PhantomData<bool>);
    pub trait StencilReadOnlyRequired {}
    pub trait StencilReadOnlyIsUnset: StencilReadOnlyRequired {}
    impl StencilReadOnlyRequired for StencilReadOnlyUnset {}
    impl StencilReadOnlyIsUnset for StencilReadOnlyUnset {}
    impl StencilReadOnlyRequired for StencilReadOnlySet {}
    impl StencilReadOnlySet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod push_constant_range_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct StagesSet(pub wgpu::ShaderStages);
    pub struct StagesUnset(PhantomData<wgpu::ShaderStages>);
    pub trait StagesRequired {}
    pub trait StagesIsUnset: StagesRequired {}
    impl StagesRequired for StagesUnset {}
    impl StagesIsUnset for StagesUnset {}
    impl StagesRequired for StagesSet {}
    impl StagesSet {
        fn get(self) -> wgpu::ShaderStages {
            self.0
        }
    }
    pub struct RangeSet(pub Range<u32>);
    pub struct RangeUnset(PhantomData<Range<u32>>);
    pub trait RangeRequired {}
    pub trait RangeIsUnset: RangeRequired {}
    impl RangeRequired for RangeUnset {}
    impl RangeIsUnset for RangeUnset {}
    impl RangeRequired for RangeSet {}
    impl RangeSet {
        fn get(self) -> Range<u32> {
            self.0
        }
    }
}

pub mod origin_3_d_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct XSet(pub u32);
    pub struct XUnset(PhantomData<u32>);
    pub trait XRequired {}
    pub trait XIsUnset: XRequired {}
    impl XRequired for XUnset {}
    impl XIsUnset for XUnset {}
    impl XRequired for XSet {}
    impl XSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct YSet(pub u32);
    pub struct YUnset(PhantomData<u32>);
    pub trait YRequired {}
    pub trait YIsUnset: YRequired {}
    impl YRequired for YUnset {}
    impl YIsUnset for YUnset {}
    impl YRequired for YSet {}
    impl YSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct ZSet(pub u32);
    pub struct ZUnset(PhantomData<u32>);
    pub trait ZRequired {}
    pub trait ZIsUnset: ZRequired {}
    impl ZRequired for ZUnset {}
    impl ZIsUnset for ZUnset {}
    impl ZRequired for ZSet {}
    impl ZSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod draw_indexed_indirect_args_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct IndexCountSet(pub u32);
    pub struct IndexCountUnset(PhantomData<u32>);
    pub trait IndexCountRequired {}
    pub trait IndexCountIsUnset: IndexCountRequired {}
    impl IndexCountRequired for IndexCountUnset {}
    impl IndexCountIsUnset for IndexCountUnset {}
    impl IndexCountRequired for IndexCountSet {}
    impl IndexCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct InstanceCountSet(pub u32);
    pub struct InstanceCountUnset(PhantomData<u32>);
    pub trait InstanceCountRequired {}
    pub trait InstanceCountIsUnset: InstanceCountRequired {}
    impl InstanceCountRequired for InstanceCountUnset {}
    impl InstanceCountIsUnset for InstanceCountUnset {}
    impl InstanceCountRequired for InstanceCountSet {}
    impl InstanceCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct FirstIndexSet(pub u32);
    pub struct FirstIndexUnset(PhantomData<u32>);
    pub trait FirstIndexRequired {}
    pub trait FirstIndexIsUnset: FirstIndexRequired {}
    impl FirstIndexRequired for FirstIndexUnset {}
    impl FirstIndexIsUnset for FirstIndexUnset {}
    impl FirstIndexRequired for FirstIndexSet {}
    impl FirstIndexSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct BaseVertexSet(pub i32);
    pub struct BaseVertexUnset(PhantomData<i32>);
    pub trait BaseVertexRequired {}
    pub trait BaseVertexIsUnset: BaseVertexRequired {}
    impl BaseVertexRequired for BaseVertexUnset {}
    impl BaseVertexIsUnset for BaseVertexUnset {}
    impl BaseVertexRequired for BaseVertexSet {}
    impl BaseVertexSet {
        fn get(self) -> i32 {
            self.0
        }
    }
    pub struct FirstInstanceSet(pub u32);
    pub struct FirstInstanceUnset(PhantomData<u32>);
    pub trait FirstInstanceRequired {}
    pub trait FirstInstanceIsUnset: FirstInstanceRequired {}
    impl FirstInstanceRequired for FirstInstanceUnset {}
    impl FirstInstanceIsUnset for FirstInstanceUnset {}
    impl FirstInstanceRequired for FirstInstanceSet {}
    impl FirstInstanceSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod texture_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct SizeSet(pub wgpu::Extent3d);
    pub struct SizeUnset(PhantomData<wgpu::Extent3d>);
    pub trait SizeRequired {}
    pub trait SizeIsUnset: SizeRequired {}
    impl SizeRequired for SizeUnset {}
    impl SizeIsUnset for SizeUnset {}
    impl SizeRequired for SizeSet {}
    impl SizeSet {
        fn get(self) -> wgpu::Extent3d {
            self.0
        }
    }
    pub struct MipLevelCountSet(pub u32);
    pub struct MipLevelCountUnset(PhantomData<u32>);
    pub trait MipLevelCountRequired {}
    pub trait MipLevelCountIsUnset: MipLevelCountRequired {}
    impl MipLevelCountRequired for MipLevelCountUnset {}
    impl MipLevelCountIsUnset for MipLevelCountUnset {}
    impl MipLevelCountRequired for MipLevelCountSet {}
    impl MipLevelCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct SampleCountSet(pub u32);
    pub struct SampleCountUnset(PhantomData<u32>);
    pub trait SampleCountRequired {}
    pub trait SampleCountIsUnset: SampleCountRequired {}
    impl SampleCountRequired for SampleCountUnset {}
    impl SampleCountIsUnset for SampleCountUnset {}
    impl SampleCountRequired for SampleCountSet {}
    impl SampleCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct DimensionSet(pub wgpu::TextureDimension);
    pub struct DimensionUnset(PhantomData<wgpu::TextureDimension>);
    pub trait DimensionRequired {}
    pub trait DimensionIsUnset: DimensionRequired {}
    impl DimensionRequired for DimensionUnset {}
    impl DimensionIsUnset for DimensionUnset {}
    impl DimensionRequired for DimensionSet {}
    impl DimensionSet {
        fn get(self) -> wgpu::TextureDimension {
            self.0
        }
    }
    pub struct FormatSet(pub wgpu::TextureFormat);
    pub struct FormatUnset(PhantomData<wgpu::TextureFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::TextureFormat {
            self.0
        }
    }
    pub struct UsageSet(pub wgpu::TextureUsages);
    pub struct UsageUnset(PhantomData<wgpu::TextureUsages>);
    pub trait UsageRequired {}
    pub trait UsageIsUnset: UsageRequired {}
    impl UsageRequired for UsageUnset {}
    impl UsageIsUnset for UsageUnset {}
    impl UsageRequired for UsageSet {}
    impl UsageSet {
        fn get(self) -> wgpu::TextureUsages {
            self.0
        }
    }
    pub struct ViewFormatsSet<'a>(pub &'a [wgpu::TextureFormat]);
    pub struct ViewFormatsUnset<'a>(PhantomData<&'a [wgpu::TextureFormat]>);
    pub trait ViewFormatsRequired {}
    pub trait ViewFormatsIsUnset: ViewFormatsRequired {}
    impl<'a> ViewFormatsRequired for ViewFormatsUnset<'a> {}
    impl<'a> ViewFormatsIsUnset for ViewFormatsUnset<'a> {}
    impl<'a> ViewFormatsRequired for ViewFormatsSet<'a> {}
    impl<'a> ViewFormatsSet<'a> {
        fn get(self) -> &'a [wgpu::TextureFormat] {
            self.0
        }
    }
}

pub mod task_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ModuleSet<'a>(pub &'a wgpu::ShaderModule);
    pub struct ModuleUnset<'a>(PhantomData<&'a wgpu::ShaderModule>);
    pub trait ModuleRequired {}
    pub trait ModuleIsUnset: ModuleRequired {}
    impl<'a> ModuleRequired for ModuleUnset<'a> {}
    impl<'a> ModuleIsUnset for ModuleUnset<'a> {}
    impl<'a> ModuleRequired for ModuleSet<'a> {}
    impl<'a> ModuleSet<'a> {
        fn get(self) -> &'a wgpu::ShaderModule {
            self.0
        }
    }
    pub struct EntryPointSet<'a>(pub Option<&'a str>);
    pub struct EntryPointUnset<'a>(PhantomData<Option<&'a str>>);
    pub trait EntryPointRequired {}
    pub trait EntryPointIsUnset: EntryPointRequired {}
    impl<'a> EntryPointRequired for EntryPointUnset<'a> {}
    impl<'a> EntryPointIsUnset for EntryPointUnset<'a> {}
    impl<'a> EntryPointRequired for EntryPointSet<'a> {}
    impl<'a> EntryPointSet<'a> {
        fn get(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct CompilationOptionsSet<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    pub struct CompilationOptionsUnset<'a>(PhantomData<wgpu::PipelineCompilationOptions<'a>>);
    pub trait CompilationOptionsRequired {}
    pub trait CompilationOptionsIsUnset: CompilationOptionsRequired {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsIsUnset for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsSet<'a> {}
    impl<'a> CompilationOptionsSet<'a> {
        fn get(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
}

pub mod operations_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LoadSet<V>(pub wgpu::LoadOp<V>);
    pub struct LoadUnset<V>(PhantomData<wgpu::LoadOp<V>>);
    pub trait LoadRequired {}
    pub trait LoadIsUnset: LoadRequired {}
    impl<V> LoadRequired for LoadUnset<V> {}
    impl<V> LoadIsUnset for LoadUnset<V> {}
    impl<V> LoadRequired for LoadSet<V> {}
    impl<V> LoadSet<V> {
        fn get(self) -> wgpu::LoadOp<V> {
            self.0
        }
    }
    pub struct StoreSet(pub wgpu::StoreOp);
    pub struct StoreUnset(PhantomData<wgpu::StoreOp>);
    pub trait StoreRequired {}
    pub trait StoreIsUnset: StoreRequired {}
    impl StoreRequired for StoreUnset {}
    impl StoreIsUnset for StoreUnset {}
    impl StoreRequired for StoreSet {}
    impl StoreSet {
        fn get(self) -> wgpu::StoreOp {
            self.0
        }
    }
}

pub mod stencil_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct FrontSet(pub wgpu::StencilFaceState);
    pub struct FrontUnset(PhantomData<wgpu::StencilFaceState>);
    pub trait FrontRequired {}
    pub trait FrontIsUnset: FrontRequired {}
    impl FrontRequired for FrontUnset {}
    impl FrontIsUnset for FrontUnset {}
    impl FrontRequired for FrontSet {}
    impl FrontSet {
        fn get(self) -> wgpu::StencilFaceState {
            self.0
        }
    }
    pub struct BackSet(pub wgpu::StencilFaceState);
    pub struct BackUnset(PhantomData<wgpu::StencilFaceState>);
    pub trait BackRequired {}
    pub trait BackIsUnset: BackRequired {}
    impl BackRequired for BackUnset {}
    impl BackIsUnset for BackUnset {}
    impl BackRequired for BackSet {}
    impl BackSet {
        fn get(self) -> wgpu::StencilFaceState {
            self.0
        }
    }
    pub struct ReadMaskSet(pub u32);
    pub struct ReadMaskUnset(PhantomData<u32>);
    pub trait ReadMaskRequired {}
    pub trait ReadMaskIsUnset: ReadMaskRequired {}
    impl ReadMaskRequired for ReadMaskUnset {}
    impl ReadMaskIsUnset for ReadMaskUnset {}
    impl ReadMaskRequired for ReadMaskSet {}
    impl ReadMaskSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct WriteMaskSet(pub u32);
    pub struct WriteMaskUnset(PhantomData<u32>);
    pub trait WriteMaskRequired {}
    pub trait WriteMaskIsUnset: WriteMaskRequired {}
    impl WriteMaskRequired for WriteMaskUnset {}
    impl WriteMaskIsUnset for WriteMaskUnset {}
    impl WriteMaskRequired for WriteMaskSet {}
    impl WriteMaskSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod image_subresource_range_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct AspectSet(pub wgpu::TextureAspect);
    pub struct AspectUnset(PhantomData<wgpu::TextureAspect>);
    pub trait AspectRequired {}
    pub trait AspectIsUnset: AspectRequired {}
    impl AspectRequired for AspectUnset {}
    impl AspectIsUnset for AspectUnset {}
    impl AspectRequired for AspectSet {}
    impl AspectSet {
        fn get(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    pub struct BaseMipLevelSet(pub u32);
    pub struct BaseMipLevelUnset(PhantomData<u32>);
    pub trait BaseMipLevelRequired {}
    pub trait BaseMipLevelIsUnset: BaseMipLevelRequired {}
    impl BaseMipLevelRequired for BaseMipLevelUnset {}
    impl BaseMipLevelIsUnset for BaseMipLevelUnset {}
    impl BaseMipLevelRequired for BaseMipLevelSet {}
    impl BaseMipLevelSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct MipLevelCountSet(pub Option<u32>);
    pub struct MipLevelCountUnset(PhantomData<Option<u32>>);
    pub trait MipLevelCountRequired {}
    pub trait MipLevelCountIsUnset: MipLevelCountRequired {}
    impl MipLevelCountRequired for MipLevelCountUnset {}
    impl MipLevelCountIsUnset for MipLevelCountUnset {}
    impl MipLevelCountRequired for MipLevelCountSet {}
    impl MipLevelCountSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct BaseArrayLayerSet(pub u32);
    pub struct BaseArrayLayerUnset(PhantomData<u32>);
    pub trait BaseArrayLayerRequired {}
    pub trait BaseArrayLayerIsUnset: BaseArrayLayerRequired {}
    impl BaseArrayLayerRequired for BaseArrayLayerUnset {}
    impl BaseArrayLayerIsUnset for BaseArrayLayerUnset {}
    impl BaseArrayLayerRequired for BaseArrayLayerSet {}
    impl BaseArrayLayerSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct ArrayLayerCountSet(pub Option<u32>);
    pub struct ArrayLayerCountUnset(PhantomData<Option<u32>>);
    pub trait ArrayLayerCountRequired {}
    pub trait ArrayLayerCountIsUnset: ArrayLayerCountRequired {}
    impl ArrayLayerCountRequired for ArrayLayerCountUnset {}
    impl ArrayLayerCountIsUnset for ArrayLayerCountUnset {}
    impl ArrayLayerCountRequired for ArrayLayerCountSet {}
    impl ArrayLayerCountSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
}

pub mod render_pass_color_attachment_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ViewSet<'tex>(pub &'tex wgpu::TextureView);
    pub struct ViewUnset<'tex>(PhantomData<&'tex wgpu::TextureView>);
    pub trait ViewRequired {}
    pub trait ViewIsUnset: ViewRequired {}
    impl<'tex> ViewRequired for ViewUnset<'tex> {}
    impl<'tex> ViewIsUnset for ViewUnset<'tex> {}
    impl<'tex> ViewRequired for ViewSet<'tex> {}
    impl<'tex> ViewSet<'tex> {
        fn get(self) -> &'tex wgpu::TextureView {
            self.0
        }
    }
    pub struct DepthSliceSet(pub Option<u32>);
    pub struct DepthSliceUnset(PhantomData<Option<u32>>);
    pub trait DepthSliceRequired {}
    pub trait DepthSliceIsUnset: DepthSliceRequired {}
    impl DepthSliceRequired for DepthSliceUnset {}
    impl DepthSliceIsUnset for DepthSliceUnset {}
    impl DepthSliceRequired for DepthSliceSet {}
    impl DepthSliceSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct ResolveTargetSet<'tex>(pub Option<&'tex wgpu::TextureView>);
    pub struct ResolveTargetUnset<'tex>(PhantomData<Option<&'tex wgpu::TextureView>>);
    pub trait ResolveTargetRequired {}
    pub trait ResolveTargetIsUnset: ResolveTargetRequired {}
    impl<'tex> ResolveTargetRequired for ResolveTargetUnset<'tex> {}
    impl<'tex> ResolveTargetIsUnset for ResolveTargetUnset<'tex> {}
    impl<'tex> ResolveTargetRequired for ResolveTargetSet<'tex> {}
    impl<'tex> ResolveTargetSet<'tex> {
        fn get(self) -> Option<&'tex wgpu::TextureView> {
            self.0
        }
    }
    pub struct OpsSet(pub wgpu::Operations<wgpu::Color>);
    pub struct OpsUnset(PhantomData<wgpu::Operations<wgpu::Color>>);
    pub trait OpsRequired {}
    pub trait OpsIsUnset: OpsRequired {}
    impl OpsRequired for OpsUnset {}
    impl OpsIsUnset for OpsUnset {}
    impl OpsRequired for OpsSet {}
    impl OpsSet {
        fn get(self) -> wgpu::Operations<wgpu::Color> {
            self.0
        }
    }
}

pub mod render_pass_timestamp_writes_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct QuerySetSet<'a>(pub &'a wgpu::QuerySet);
    pub struct QuerySetUnset<'a>(PhantomData<&'a wgpu::QuerySet>);
    pub trait QuerySetRequired {}
    pub trait QuerySetIsUnset: QuerySetRequired {}
    impl<'a> QuerySetRequired for QuerySetUnset<'a> {}
    impl<'a> QuerySetIsUnset for QuerySetUnset<'a> {}
    impl<'a> QuerySetRequired for QuerySetSet<'a> {}
    impl<'a> QuerySetSet<'a> {
        fn get(self) -> &'a wgpu::QuerySet {
            self.0
        }
    }
    pub struct BeginningOfPassWriteIndexSet(pub Option<u32>);
    pub struct BeginningOfPassWriteIndexUnset(PhantomData<Option<u32>>);
    pub trait BeginningOfPassWriteIndexRequired {}
    pub trait BeginningOfPassWriteIndexIsUnset: BeginningOfPassWriteIndexRequired {}
    impl BeginningOfPassWriteIndexRequired for BeginningOfPassWriteIndexUnset {}
    impl BeginningOfPassWriteIndexIsUnset for BeginningOfPassWriteIndexUnset {}
    impl BeginningOfPassWriteIndexRequired for BeginningOfPassWriteIndexSet {}
    impl BeginningOfPassWriteIndexSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct EndOfPassWriteIndexSet(pub Option<u32>);
    pub struct EndOfPassWriteIndexUnset(PhantomData<Option<u32>>);
    pub trait EndOfPassWriteIndexRequired {}
    pub trait EndOfPassWriteIndexIsUnset: EndOfPassWriteIndexRequired {}
    impl EndOfPassWriteIndexRequired for EndOfPassWriteIndexUnset {}
    impl EndOfPassWriteIndexIsUnset for EndOfPassWriteIndexUnset {}
    impl EndOfPassWriteIndexRequired for EndOfPassWriteIndexSet {}
    impl EndOfPassWriteIndexSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
}

pub mod instance_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BackendsSet(pub wgpu::Backends);
    pub struct BackendsUnset(PhantomData<wgpu::Backends>);
    pub trait BackendsRequired {}
    pub trait BackendsIsUnset: BackendsRequired {}
    impl BackendsRequired for BackendsUnset {}
    impl BackendsIsUnset for BackendsUnset {}
    impl BackendsRequired for BackendsSet {}
    impl BackendsSet {
        fn get(self) -> wgpu::Backends {
            self.0
        }
    }
    pub struct FlagsSet(pub wgpu::InstanceFlags);
    pub struct FlagsUnset(PhantomData<wgpu::InstanceFlags>);
    pub trait FlagsRequired {}
    pub trait FlagsIsUnset: FlagsRequired {}
    impl FlagsRequired for FlagsUnset {}
    impl FlagsIsUnset for FlagsUnset {}
    impl FlagsRequired for FlagsSet {}
    impl FlagsSet {
        fn get(self) -> wgpu::InstanceFlags {
            self.0
        }
    }
    pub struct MemoryBudgetThresholdsSet(pub wgpu::MemoryBudgetThresholds);
    pub struct MemoryBudgetThresholdsUnset(PhantomData<wgpu::MemoryBudgetThresholds>);
    pub trait MemoryBudgetThresholdsRequired {}
    pub trait MemoryBudgetThresholdsIsUnset: MemoryBudgetThresholdsRequired {}
    impl MemoryBudgetThresholdsRequired for MemoryBudgetThresholdsUnset {}
    impl MemoryBudgetThresholdsIsUnset for MemoryBudgetThresholdsUnset {}
    impl MemoryBudgetThresholdsRequired for MemoryBudgetThresholdsSet {}
    impl MemoryBudgetThresholdsSet {
        fn get(self) -> wgpu::MemoryBudgetThresholds {
            self.0
        }
    }
    pub struct BackendOptionsSet(pub wgpu::BackendOptions);
    pub struct BackendOptionsUnset(PhantomData<wgpu::BackendOptions>);
    pub trait BackendOptionsRequired {}
    pub trait BackendOptionsIsUnset: BackendOptionsRequired {}
    impl BackendOptionsRequired for BackendOptionsUnset {}
    impl BackendOptionsIsUnset for BackendOptionsUnset {}
    impl BackendOptionsRequired for BackendOptionsSet {}
    impl BackendOptionsSet {
        fn get(self) -> wgpu::BackendOptions {
            self.0
        }
    }
}

pub mod memory_budget_thresholds_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ForResourceCreationSet(pub Option<u8>);
    pub struct ForResourceCreationUnset(PhantomData<Option<u8>>);
    pub trait ForResourceCreationRequired {}
    pub trait ForResourceCreationIsUnset: ForResourceCreationRequired {}
    impl ForResourceCreationRequired for ForResourceCreationUnset {}
    impl ForResourceCreationIsUnset for ForResourceCreationUnset {}
    impl ForResourceCreationRequired for ForResourceCreationSet {}
    impl ForResourceCreationSet {
        fn get(self) -> Option<u8> {
            self.0
        }
    }
    pub struct ForDeviceLossSet(pub Option<u8>);
    pub struct ForDeviceLossUnset(PhantomData<Option<u8>>);
    pub trait ForDeviceLossRequired {}
    pub trait ForDeviceLossIsUnset: ForDeviceLossRequired {}
    impl ForDeviceLossRequired for ForDeviceLossUnset {}
    impl ForDeviceLossIsUnset for ForDeviceLossUnset {}
    impl ForDeviceLossRequired for ForDeviceLossSet {}
    impl ForDeviceLossSet {
        fn get(self) -> Option<u8> {
            self.0
        }
    }
}

pub mod multisample_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct CountSet(pub u32);
    pub struct CountUnset(PhantomData<u32>);
    pub trait CountRequired {}
    pub trait CountIsUnset: CountRequired {}
    impl CountRequired for CountUnset {}
    impl CountIsUnset for CountUnset {}
    impl CountRequired for CountSet {}
    impl CountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct MaskSet(pub u64);
    pub struct MaskUnset(PhantomData<u64>);
    pub trait MaskRequired {}
    pub trait MaskIsUnset: MaskRequired {}
    impl MaskRequired for MaskUnset {}
    impl MaskIsUnset for MaskUnset {}
    impl MaskRequired for MaskSet {}
    impl MaskSet {
        fn get(self) -> u64 {
            self.0
        }
    }
    pub struct AlphaToCoverageEnabledSet(pub bool);
    pub struct AlphaToCoverageEnabledUnset(PhantomData<bool>);
    pub trait AlphaToCoverageEnabledRequired {}
    pub trait AlphaToCoverageEnabledIsUnset: AlphaToCoverageEnabledRequired {}
    impl AlphaToCoverageEnabledRequired for AlphaToCoverageEnabledUnset {}
    impl AlphaToCoverageEnabledIsUnset for AlphaToCoverageEnabledUnset {}
    impl AlphaToCoverageEnabledRequired for AlphaToCoverageEnabledSet {}
    impl AlphaToCoverageEnabledSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod backend_options_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct GlSet(pub wgpu::GlBackendOptions);
    pub struct GlUnset(PhantomData<wgpu::GlBackendOptions>);
    pub trait GlRequired {}
    pub trait GlIsUnset: GlRequired {}
    impl GlRequired for GlUnset {}
    impl GlIsUnset for GlUnset {}
    impl GlRequired for GlSet {}
    impl GlSet {
        fn get(self) -> wgpu::GlBackendOptions {
            self.0
        }
    }
    pub struct Dx12Set(pub wgpu::Dx12BackendOptions);
    pub struct Dx12Unset(PhantomData<wgpu::Dx12BackendOptions>);
    pub trait Dx12Required {}
    pub trait Dx12IsUnset: Dx12Required {}
    impl Dx12Required for Dx12Unset {}
    impl Dx12IsUnset for Dx12Unset {}
    impl Dx12Required for Dx12Set {}
    impl Dx12Set {
        fn get(self) -> wgpu::Dx12BackendOptions {
            self.0
        }
    }
    pub struct NoopSet(pub wgpu::NoopBackendOptions);
    pub struct NoopUnset(PhantomData<wgpu::NoopBackendOptions>);
    pub trait NoopRequired {}
    pub trait NoopIsUnset: NoopRequired {}
    impl NoopRequired for NoopUnset {}
    impl NoopIsUnset for NoopUnset {}
    impl NoopRequired for NoopSet {}
    impl NoopSet {
        fn get(self) -> wgpu::NoopBackendOptions {
            self.0
        }
    }
}

pub mod texel_copy_buffer_layout_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct OffsetSet(pub wgpu::BufferAddress);
    pub struct OffsetUnset(PhantomData<wgpu::BufferAddress>);
    pub trait OffsetRequired {}
    pub trait OffsetIsUnset: OffsetRequired {}
    impl OffsetRequired for OffsetUnset {}
    impl OffsetIsUnset for OffsetUnset {}
    impl OffsetRequired for OffsetSet {}
    impl OffsetSet {
        fn get(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct BytesPerRowSet(pub Option<u32>);
    pub struct BytesPerRowUnset(PhantomData<Option<u32>>);
    pub trait BytesPerRowRequired {}
    pub trait BytesPerRowIsUnset: BytesPerRowRequired {}
    impl BytesPerRowRequired for BytesPerRowUnset {}
    impl BytesPerRowIsUnset for BytesPerRowUnset {}
    impl BytesPerRowRequired for BytesPerRowSet {}
    impl BytesPerRowSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct RowsPerImageSet(pub Option<u32>);
    pub struct RowsPerImageUnset(PhantomData<Option<u32>>);
    pub trait RowsPerImageRequired {}
    pub trait RowsPerImageIsUnset: RowsPerImageRequired {}
    impl RowsPerImageRequired for RowsPerImageUnset {}
    impl RowsPerImageIsUnset for RowsPerImageUnset {}
    impl RowsPerImageRequired for RowsPerImageSet {}
    impl RowsPerImageSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
}

pub mod draw_indirect_args_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct VertexCountSet(pub u32);
    pub struct VertexCountUnset(PhantomData<u32>);
    pub trait VertexCountRequired {}
    pub trait VertexCountIsUnset: VertexCountRequired {}
    impl VertexCountRequired for VertexCountUnset {}
    impl VertexCountIsUnset for VertexCountUnset {}
    impl VertexCountRequired for VertexCountSet {}
    impl VertexCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct InstanceCountSet(pub u32);
    pub struct InstanceCountUnset(PhantomData<u32>);
    pub trait InstanceCountRequired {}
    pub trait InstanceCountIsUnset: InstanceCountRequired {}
    impl InstanceCountRequired for InstanceCountUnset {}
    impl InstanceCountIsUnset for InstanceCountUnset {}
    impl InstanceCountRequired for InstanceCountSet {}
    impl InstanceCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct FirstVertexSet(pub u32);
    pub struct FirstVertexUnset(PhantomData<u32>);
    pub trait FirstVertexRequired {}
    pub trait FirstVertexIsUnset: FirstVertexRequired {}
    impl FirstVertexRequired for FirstVertexUnset {}
    impl FirstVertexIsUnset for FirstVertexUnset {}
    impl FirstVertexRequired for FirstVertexSet {}
    impl FirstVertexSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct FirstInstanceSet(pub u32);
    pub struct FirstInstanceUnset(PhantomData<u32>);
    pub trait FirstInstanceRequired {}
    pub trait FirstInstanceIsUnset: FirstInstanceRequired {}
    impl FirstInstanceRequired for FirstInstanceUnset {}
    impl FirstInstanceIsUnset for FirstInstanceUnset {}
    impl FirstInstanceRequired for FirstInstanceSet {}
    impl FirstInstanceSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod render_bundle_encoder_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct ColorFormatsSet<'a>(pub &'a [Option<wgpu::TextureFormat>]);
    pub struct ColorFormatsUnset<'a>(PhantomData<&'a [Option<wgpu::TextureFormat>]>);
    pub trait ColorFormatsRequired {}
    pub trait ColorFormatsIsUnset: ColorFormatsRequired {}
    impl<'a> ColorFormatsRequired for ColorFormatsUnset<'a> {}
    impl<'a> ColorFormatsIsUnset for ColorFormatsUnset<'a> {}
    impl<'a> ColorFormatsRequired for ColorFormatsSet<'a> {}
    impl<'a> ColorFormatsSet<'a> {
        fn get(self) -> &'a [Option<wgpu::TextureFormat>] {
            self.0
        }
    }
    pub struct DepthStencilSet(pub Option<wgpu::RenderBundleDepthStencil>);
    pub struct DepthStencilUnset(PhantomData<Option<wgpu::RenderBundleDepthStencil>>);
    pub trait DepthStencilRequired {}
    pub trait DepthStencilIsUnset: DepthStencilRequired {}
    impl DepthStencilRequired for DepthStencilUnset {}
    impl DepthStencilIsUnset for DepthStencilUnset {}
    impl DepthStencilRequired for DepthStencilSet {}
    impl DepthStencilSet {
        fn get(self) -> Option<wgpu::RenderBundleDepthStencil> {
            self.0
        }
    }
    pub struct SampleCountSet(pub u32);
    pub struct SampleCountUnset(PhantomData<u32>);
    pub trait SampleCountRequired {}
    pub trait SampleCountIsUnset: SampleCountRequired {}
    impl SampleCountRequired for SampleCountUnset {}
    impl SampleCountIsUnset for SampleCountUnset {}
    impl SampleCountRequired for SampleCountSet {}
    impl SampleCountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct MultiviewSet(pub Option<NonZeroU32>);
    pub struct MultiviewUnset(PhantomData<Option<NonZeroU32>>);
    pub trait MultiviewRequired {}
    pub trait MultiviewIsUnset: MultiviewRequired {}
    impl MultiviewRequired for MultiviewUnset {}
    impl MultiviewIsUnset for MultiviewUnset {}
    impl MultiviewRequired for MultiviewSet {}
    impl MultiviewSet {
        fn get(self) -> Option<NonZeroU32> {
            self.0
        }
    }
}

pub mod vertex_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ModuleSet<'a>(pub &'a wgpu::ShaderModule);
    pub struct ModuleUnset<'a>(PhantomData<&'a wgpu::ShaderModule>);
    pub trait ModuleRequired {}
    pub trait ModuleIsUnset: ModuleRequired {}
    impl<'a> ModuleRequired for ModuleUnset<'a> {}
    impl<'a> ModuleIsUnset for ModuleUnset<'a> {}
    impl<'a> ModuleRequired for ModuleSet<'a> {}
    impl<'a> ModuleSet<'a> {
        fn get(self) -> &'a wgpu::ShaderModule {
            self.0
        }
    }
    pub struct EntryPointSet<'a>(pub Option<&'a str>);
    pub struct EntryPointUnset<'a>(PhantomData<Option<&'a str>>);
    pub trait EntryPointRequired {}
    pub trait EntryPointIsUnset: EntryPointRequired {}
    impl<'a> EntryPointRequired for EntryPointUnset<'a> {}
    impl<'a> EntryPointIsUnset for EntryPointUnset<'a> {}
    impl<'a> EntryPointRequired for EntryPointSet<'a> {}
    impl<'a> EntryPointSet<'a> {
        fn get(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct CompilationOptionsSet<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    pub struct CompilationOptionsUnset<'a>(PhantomData<wgpu::PipelineCompilationOptions<'a>>);
    pub trait CompilationOptionsRequired {}
    pub trait CompilationOptionsIsUnset: CompilationOptionsRequired {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsIsUnset for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsSet<'a> {}
    impl<'a> CompilationOptionsSet<'a> {
        fn get(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    pub struct BuffersSet<'a>(pub &'a [wgpu::VertexBufferLayout<'a>]);
    pub struct BuffersUnset<'a>(PhantomData<&'a [wgpu::VertexBufferLayout<'a>]>);
    pub trait BuffersRequired {}
    pub trait BuffersIsUnset: BuffersRequired {}
    impl<'a> BuffersRequired for BuffersUnset<'a> {}
    impl<'a> BuffersIsUnset for BuffersUnset<'a> {}
    impl<'a> BuffersRequired for BuffersSet<'a> {}
    impl<'a> BuffersSet<'a> {
        fn get(self) -> &'a [wgpu::VertexBufferLayout<'a>] {
            self.0
        }
    }
}

pub mod render_pass_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct ColorAttachmentsSet<'a>(pub &'a [Option<wgpu::RenderPassColorAttachment<'a>>]);
    pub struct ColorAttachmentsUnset<'a>(
        PhantomData<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>,
    );
    pub trait ColorAttachmentsRequired {}
    pub trait ColorAttachmentsIsUnset: ColorAttachmentsRequired {}
    impl<'a> ColorAttachmentsRequired for ColorAttachmentsUnset<'a> {}
    impl<'a> ColorAttachmentsIsUnset for ColorAttachmentsUnset<'a> {}
    impl<'a> ColorAttachmentsRequired for ColorAttachmentsSet<'a> {}
    impl<'a> ColorAttachmentsSet<'a> {
        fn get(self) -> &'a [Option<wgpu::RenderPassColorAttachment<'a>>] {
            self.0
        }
    }
    pub struct DepthStencilAttachmentSet<'a>(
        pub Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    );
    pub struct DepthStencilAttachmentUnset<'a>(
        PhantomData<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>,
    );
    pub trait DepthStencilAttachmentRequired {}
    pub trait DepthStencilAttachmentIsUnset: DepthStencilAttachmentRequired {}
    impl<'a> DepthStencilAttachmentRequired for DepthStencilAttachmentUnset<'a> {}
    impl<'a> DepthStencilAttachmentIsUnset for DepthStencilAttachmentUnset<'a> {}
    impl<'a> DepthStencilAttachmentRequired for DepthStencilAttachmentSet<'a> {}
    impl<'a> DepthStencilAttachmentSet<'a> {
        fn get(self) -> Option<wgpu::RenderPassDepthStencilAttachment<'a>> {
            self.0
        }
    }
    pub struct TimestampWritesSet<'a>(pub Option<wgpu::RenderPassTimestampWrites<'a>>);
    pub struct TimestampWritesUnset<'a>(PhantomData<Option<wgpu::RenderPassTimestampWrites<'a>>>);
    pub trait TimestampWritesRequired {}
    pub trait TimestampWritesIsUnset: TimestampWritesRequired {}
    impl<'a> TimestampWritesRequired for TimestampWritesUnset<'a> {}
    impl<'a> TimestampWritesIsUnset for TimestampWritesUnset<'a> {}
    impl<'a> TimestampWritesRequired for TimestampWritesSet<'a> {}
    impl<'a> TimestampWritesSet<'a> {
        fn get(self) -> Option<wgpu::RenderPassTimestampWrites<'a>> {
            self.0
        }
    }
    pub struct OcclusionQuerySetSet<'a>(pub Option<&'a wgpu::QuerySet>);
    pub struct OcclusionQuerySetUnset<'a>(PhantomData<Option<&'a wgpu::QuerySet>>);
    pub trait OcclusionQuerySetRequired {}
    pub trait OcclusionQuerySetIsUnset: OcclusionQuerySetRequired {}
    impl<'a> OcclusionQuerySetRequired for OcclusionQuerySetUnset<'a> {}
    impl<'a> OcclusionQuerySetIsUnset for OcclusionQuerySetUnset<'a> {}
    impl<'a> OcclusionQuerySetRequired for OcclusionQuerySetSet<'a> {}
    impl<'a> OcclusionQuerySetSet<'a> {
        fn get(self) -> Option<&'a wgpu::QuerySet> {
            self.0
        }
    }
}

pub mod pipeline_layout_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct BindGroupLayoutsSet<'a>(pub &'a [&'a wgpu::BindGroupLayout]);
    pub struct BindGroupLayoutsUnset<'a>(PhantomData<&'a [&'a wgpu::BindGroupLayout]>);
    pub trait BindGroupLayoutsRequired {}
    pub trait BindGroupLayoutsIsUnset: BindGroupLayoutsRequired {}
    impl<'a> BindGroupLayoutsRequired for BindGroupLayoutsUnset<'a> {}
    impl<'a> BindGroupLayoutsIsUnset for BindGroupLayoutsUnset<'a> {}
    impl<'a> BindGroupLayoutsRequired for BindGroupLayoutsSet<'a> {}
    impl<'a> BindGroupLayoutsSet<'a> {
        fn get(self) -> &'a [&'a wgpu::BindGroupLayout] {
            self.0
        }
    }
    pub struct PushConstantRangesSet<'a>(pub &'a [wgpu::PushConstantRange]);
    pub struct PushConstantRangesUnset<'a>(PhantomData<&'a [wgpu::PushConstantRange]>);
    pub trait PushConstantRangesRequired {}
    pub trait PushConstantRangesIsUnset: PushConstantRangesRequired {}
    impl<'a> PushConstantRangesRequired for PushConstantRangesUnset<'a> {}
    impl<'a> PushConstantRangesIsUnset for PushConstantRangesUnset<'a> {}
    impl<'a> PushConstantRangesRequired for PushConstantRangesSet<'a> {}
    impl<'a> PushConstantRangesSet<'a> {
        fn get(self) -> &'a [wgpu::PushConstantRange] {
            self.0
        }
    }
}

pub mod bind_group_entry_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BindingSet(pub u32);
    pub struct BindingUnset(PhantomData<u32>);
    pub trait BindingRequired {}
    pub trait BindingIsUnset: BindingRequired {}
    impl BindingRequired for BindingUnset {}
    impl BindingIsUnset for BindingUnset {}
    impl BindingRequired for BindingSet {}
    impl BindingSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct ResourceSet<'a>(pub wgpu::BindingResource<'a>);
    pub struct ResourceUnset<'a>(PhantomData<wgpu::BindingResource<'a>>);
    pub trait ResourceRequired {}
    pub trait ResourceIsUnset: ResourceRequired {}
    impl<'a> ResourceRequired for ResourceUnset<'a> {}
    impl<'a> ResourceIsUnset for ResourceUnset<'a> {}
    impl<'a> ResourceRequired for ResourceSet<'a> {}
    impl<'a> ResourceSet<'a> {
        fn get(self) -> wgpu::BindingResource<'a> {
            self.0
        }
    }
}

pub mod compute_pass_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct TimestampWritesSet<'a>(pub Option<wgpu::ComputePassTimestampWrites<'a>>);
    pub struct TimestampWritesUnset<'a>(PhantomData<Option<wgpu::ComputePassTimestampWrites<'a>>>);
    pub trait TimestampWritesRequired {}
    pub trait TimestampWritesIsUnset: TimestampWritesRequired {}
    impl<'a> TimestampWritesRequired for TimestampWritesUnset<'a> {}
    impl<'a> TimestampWritesIsUnset for TimestampWritesUnset<'a> {}
    impl<'a> TimestampWritesRequired for TimestampWritesSet<'a> {}
    impl<'a> TimestampWritesSet<'a> {
        fn get(self) -> Option<wgpu::ComputePassTimestampWrites<'a>> {
            self.0
        }
    }
}

pub mod depth_stencil_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct FormatSet(pub wgpu::TextureFormat);
    pub struct FormatUnset(PhantomData<wgpu::TextureFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::TextureFormat {
            self.0
        }
    }
    pub struct DepthWriteEnabledSet(pub bool);
    pub struct DepthWriteEnabledUnset(PhantomData<bool>);
    pub trait DepthWriteEnabledRequired {}
    pub trait DepthWriteEnabledIsUnset: DepthWriteEnabledRequired {}
    impl DepthWriteEnabledRequired for DepthWriteEnabledUnset {}
    impl DepthWriteEnabledIsUnset for DepthWriteEnabledUnset {}
    impl DepthWriteEnabledRequired for DepthWriteEnabledSet {}
    impl DepthWriteEnabledSet {
        fn get(self) -> bool {
            self.0
        }
    }
    pub struct DepthCompareSet(pub wgpu::CompareFunction);
    pub struct DepthCompareUnset(PhantomData<wgpu::CompareFunction>);
    pub trait DepthCompareRequired {}
    pub trait DepthCompareIsUnset: DepthCompareRequired {}
    impl DepthCompareRequired for DepthCompareUnset {}
    impl DepthCompareIsUnset for DepthCompareUnset {}
    impl DepthCompareRequired for DepthCompareSet {}
    impl DepthCompareSet {
        fn get(self) -> wgpu::CompareFunction {
            self.0
        }
    }
    pub struct StencilSet(pub wgpu::StencilState);
    pub struct StencilUnset(PhantomData<wgpu::StencilState>);
    pub trait StencilRequired {}
    pub trait StencilIsUnset: StencilRequired {}
    impl StencilRequired for StencilUnset {}
    impl StencilIsUnset for StencilUnset {}
    impl StencilRequired for StencilSet {}
    impl StencilSet {
        fn get(self) -> wgpu::StencilState {
            self.0
        }
    }
    pub struct BiasSet(pub wgpu::DepthBiasState);
    pub struct BiasUnset(PhantomData<wgpu::DepthBiasState>);
    pub trait BiasRequired {}
    pub trait BiasIsUnset: BiasRequired {}
    impl BiasRequired for BiasUnset {}
    impl BiasIsUnset for BiasUnset {}
    impl BiasRequired for BiasSet {}
    impl BiasSet {
        fn get(self) -> wgpu::DepthBiasState {
            self.0
        }
    }
}

pub mod mesh_pipeline_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct LayoutSet<'a>(pub Option<&'a wgpu::PipelineLayout>);
    pub struct LayoutUnset<'a>(PhantomData<Option<&'a wgpu::PipelineLayout>>);
    pub trait LayoutRequired {}
    pub trait LayoutIsUnset: LayoutRequired {}
    impl<'a> LayoutRequired for LayoutUnset<'a> {}
    impl<'a> LayoutIsUnset for LayoutUnset<'a> {}
    impl<'a> LayoutRequired for LayoutSet<'a> {}
    impl<'a> LayoutSet<'a> {
        fn get(self) -> Option<&'a wgpu::PipelineLayout> {
            self.0
        }
    }
    pub struct TaskSet<'a>(pub Option<wgpu::TaskState<'a>>);
    pub struct TaskUnset<'a>(PhantomData<Option<wgpu::TaskState<'a>>>);
    pub trait TaskRequired {}
    pub trait TaskIsUnset: TaskRequired {}
    impl<'a> TaskRequired for TaskUnset<'a> {}
    impl<'a> TaskIsUnset for TaskUnset<'a> {}
    impl<'a> TaskRequired for TaskSet<'a> {}
    impl<'a> TaskSet<'a> {
        fn get(self) -> Option<wgpu::TaskState<'a>> {
            self.0
        }
    }
    pub struct MeshSet<'a>(pub wgpu::MeshState<'a>);
    pub struct MeshUnset<'a>(PhantomData<wgpu::MeshState<'a>>);
    pub trait MeshRequired {}
    pub trait MeshIsUnset: MeshRequired {}
    impl<'a> MeshRequired for MeshUnset<'a> {}
    impl<'a> MeshIsUnset for MeshUnset<'a> {}
    impl<'a> MeshRequired for MeshSet<'a> {}
    impl<'a> MeshSet<'a> {
        fn get(self) -> wgpu::MeshState<'a> {
            self.0
        }
    }
    pub struct PrimitiveSet(pub wgpu::PrimitiveState);
    pub struct PrimitiveUnset(PhantomData<wgpu::PrimitiveState>);
    pub trait PrimitiveRequired {}
    pub trait PrimitiveIsUnset: PrimitiveRequired {}
    impl PrimitiveRequired for PrimitiveUnset {}
    impl PrimitiveIsUnset for PrimitiveUnset {}
    impl PrimitiveRequired for PrimitiveSet {}
    impl PrimitiveSet {
        fn get(self) -> wgpu::PrimitiveState {
            self.0
        }
    }
    pub struct DepthStencilSet(pub Option<wgpu::DepthStencilState>);
    pub struct DepthStencilUnset(PhantomData<Option<wgpu::DepthStencilState>>);
    pub trait DepthStencilRequired {}
    pub trait DepthStencilIsUnset: DepthStencilRequired {}
    impl DepthStencilRequired for DepthStencilUnset {}
    impl DepthStencilIsUnset for DepthStencilUnset {}
    impl DepthStencilRequired for DepthStencilSet {}
    impl DepthStencilSet {
        fn get(self) -> Option<wgpu::DepthStencilState> {
            self.0
        }
    }
    pub struct MultisampleSet(pub wgpu::MultisampleState);
    pub struct MultisampleUnset(PhantomData<wgpu::MultisampleState>);
    pub trait MultisampleRequired {}
    pub trait MultisampleIsUnset: MultisampleRequired {}
    impl MultisampleRequired for MultisampleUnset {}
    impl MultisampleIsUnset for MultisampleUnset {}
    impl MultisampleRequired for MultisampleSet {}
    impl MultisampleSet {
        fn get(self) -> wgpu::MultisampleState {
            self.0
        }
    }
    pub struct FragmentSet<'a>(pub Option<wgpu::FragmentState<'a>>);
    pub struct FragmentUnset<'a>(PhantomData<Option<wgpu::FragmentState<'a>>>);
    pub trait FragmentRequired {}
    pub trait FragmentIsUnset: FragmentRequired {}
    impl<'a> FragmentRequired for FragmentUnset<'a> {}
    impl<'a> FragmentIsUnset for FragmentUnset<'a> {}
    impl<'a> FragmentRequired for FragmentSet<'a> {}
    impl<'a> FragmentSet<'a> {
        fn get(self) -> Option<wgpu::FragmentState<'a>> {
            self.0
        }
    }
    pub struct MultiviewSet(pub Option<NonZeroU32>);
    pub struct MultiviewUnset(PhantomData<Option<NonZeroU32>>);
    pub trait MultiviewRequired {}
    pub trait MultiviewIsUnset: MultiviewRequired {}
    impl MultiviewRequired for MultiviewUnset {}
    impl MultiviewIsUnset for MultiviewUnset {}
    impl MultiviewRequired for MultiviewSet {}
    impl MultiviewSet {
        fn get(self) -> Option<NonZeroU32> {
            self.0
        }
    }
    pub struct CacheSet<'a>(pub Option<&'a wgpu::PipelineCache>);
    pub struct CacheUnset<'a>(PhantomData<Option<&'a wgpu::PipelineCache>>);
    pub trait CacheRequired {}
    pub trait CacheIsUnset: CacheRequired {}
    impl<'a> CacheRequired for CacheUnset<'a> {}
    impl<'a> CacheIsUnset for CacheUnset<'a> {}
    impl<'a> CacheRequired for CacheSet<'a> {}
    impl<'a> CacheSet<'a> {
        fn get(self) -> Option<&'a wgpu::PipelineCache> {
            self.0
        }
    }
}

pub mod texel_copy_texture_info_base_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct TextureSet<T>(pub T);
    pub struct TextureUnset<T>(PhantomData<T>);
    pub trait TextureRequired {}
    pub trait TextureIsUnset: TextureRequired {}
    impl<T> TextureRequired for TextureUnset<T> {}
    impl<T> TextureIsUnset for TextureUnset<T> {}
    impl<T> TextureRequired for TextureSet<T> {}
    impl<T> TextureSet<T> {
        fn get(self) -> T {
            self.0
        }
    }
    pub struct MipLevelSet(pub u32);
    pub struct MipLevelUnset(PhantomData<u32>);
    pub trait MipLevelRequired {}
    pub trait MipLevelIsUnset: MipLevelRequired {}
    impl MipLevelRequired for MipLevelUnset {}
    impl MipLevelIsUnset for MipLevelUnset {}
    impl MipLevelRequired for MipLevelSet {}
    impl MipLevelSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct OriginSet(pub wgpu::Origin3d);
    pub struct OriginUnset(PhantomData<wgpu::Origin3d>);
    pub trait OriginRequired {}
    pub trait OriginIsUnset: OriginRequired {}
    impl OriginRequired for OriginUnset {}
    impl OriginIsUnset for OriginUnset {}
    impl OriginRequired for OriginSet {}
    impl OriginSet {
        fn get(self) -> wgpu::Origin3d {
            self.0
        }
    }
    pub struct AspectSet(pub wgpu::TextureAspect);
    pub struct AspectUnset(PhantomData<wgpu::TextureAspect>);
    pub trait AspectRequired {}
    pub trait AspectIsUnset: AspectRequired {}
    impl AspectRequired for AspectUnset {}
    impl AspectIsUnset for AspectUnset {}
    impl AspectRequired for AspectSet {}
    impl AspectSet {
        fn get(self) -> wgpu::TextureAspect {
            self.0
        }
    }
}

pub mod blend_component_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct SrcFactorSet(pub wgpu::BlendFactor);
    pub struct SrcFactorUnset(PhantomData<wgpu::BlendFactor>);
    pub trait SrcFactorRequired {}
    pub trait SrcFactorIsUnset: SrcFactorRequired {}
    impl SrcFactorRequired for SrcFactorUnset {}
    impl SrcFactorIsUnset for SrcFactorUnset {}
    impl SrcFactorRequired for SrcFactorSet {}
    impl SrcFactorSet {
        fn get(self) -> wgpu::BlendFactor {
            self.0
        }
    }
    pub struct DstFactorSet(pub wgpu::BlendFactor);
    pub struct DstFactorUnset(PhantomData<wgpu::BlendFactor>);
    pub trait DstFactorRequired {}
    pub trait DstFactorIsUnset: DstFactorRequired {}
    impl DstFactorRequired for DstFactorUnset {}
    impl DstFactorIsUnset for DstFactorUnset {}
    impl DstFactorRequired for DstFactorSet {}
    impl DstFactorSet {
        fn get(self) -> wgpu::BlendFactor {
            self.0
        }
    }
    pub struct OperationSet(pub wgpu::BlendOperation);
    pub struct OperationUnset(PhantomData<wgpu::BlendOperation>);
    pub trait OperationRequired {}
    pub trait OperationIsUnset: OperationRequired {}
    impl OperationRequired for OperationUnset {}
    impl OperationIsUnset for OperationUnset {}
    impl OperationRequired for OperationSet {}
    impl OperationSet {
        fn get(self) -> wgpu::BlendOperation {
            self.0
        }
    }
}

pub mod dispatch_indirect_args_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct XSet(pub u32);
    pub struct XUnset(PhantomData<u32>);
    pub trait XRequired {}
    pub trait XIsUnset: XRequired {}
    impl XRequired for XUnset {}
    impl XIsUnset for XUnset {}
    impl XRequired for XSet {}
    impl XSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct YSet(pub u32);
    pub struct YUnset(PhantomData<u32>);
    pub trait YRequired {}
    pub trait YIsUnset: YRequired {}
    impl YRequired for YUnset {}
    impl YIsUnset for YUnset {}
    impl YRequired for YSet {}
    impl YSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct ZSet(pub u32);
    pub struct ZUnset(PhantomData<u32>);
    pub trait ZRequired {}
    pub trait ZIsUnset: ZRequired {}
    impl ZRequired for ZUnset {}
    impl ZIsUnset for ZUnset {}
    impl ZRequired for ZSet {}
    impl ZSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod render_bundle_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
}

pub mod sampler_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct AddressModeUSet(pub wgpu::AddressMode);
    pub struct AddressModeUUnset(PhantomData<wgpu::AddressMode>);
    pub trait AddressModeURequired {}
    pub trait AddressModeUIsUnset: AddressModeURequired {}
    impl AddressModeURequired for AddressModeUUnset {}
    impl AddressModeUIsUnset for AddressModeUUnset {}
    impl AddressModeURequired for AddressModeUSet {}
    impl AddressModeUSet {
        fn get(self) -> wgpu::AddressMode {
            self.0
        }
    }
    pub struct AddressModeVSet(pub wgpu::AddressMode);
    pub struct AddressModeVUnset(PhantomData<wgpu::AddressMode>);
    pub trait AddressModeVRequired {}
    pub trait AddressModeVIsUnset: AddressModeVRequired {}
    impl AddressModeVRequired for AddressModeVUnset {}
    impl AddressModeVIsUnset for AddressModeVUnset {}
    impl AddressModeVRequired for AddressModeVSet {}
    impl AddressModeVSet {
        fn get(self) -> wgpu::AddressMode {
            self.0
        }
    }
    pub struct AddressModeWSet(pub wgpu::AddressMode);
    pub struct AddressModeWUnset(PhantomData<wgpu::AddressMode>);
    pub trait AddressModeWRequired {}
    pub trait AddressModeWIsUnset: AddressModeWRequired {}
    impl AddressModeWRequired for AddressModeWUnset {}
    impl AddressModeWIsUnset for AddressModeWUnset {}
    impl AddressModeWRequired for AddressModeWSet {}
    impl AddressModeWSet {
        fn get(self) -> wgpu::AddressMode {
            self.0
        }
    }
    pub struct MagFilterSet(pub wgpu::FilterMode);
    pub struct MagFilterUnset(PhantomData<wgpu::FilterMode>);
    pub trait MagFilterRequired {}
    pub trait MagFilterIsUnset: MagFilterRequired {}
    impl MagFilterRequired for MagFilterUnset {}
    impl MagFilterIsUnset for MagFilterUnset {}
    impl MagFilterRequired for MagFilterSet {}
    impl MagFilterSet {
        fn get(self) -> wgpu::FilterMode {
            self.0
        }
    }
    pub struct MinFilterSet(pub wgpu::FilterMode);
    pub struct MinFilterUnset(PhantomData<wgpu::FilterMode>);
    pub trait MinFilterRequired {}
    pub trait MinFilterIsUnset: MinFilterRequired {}
    impl MinFilterRequired for MinFilterUnset {}
    impl MinFilterIsUnset for MinFilterUnset {}
    impl MinFilterRequired for MinFilterSet {}
    impl MinFilterSet {
        fn get(self) -> wgpu::FilterMode {
            self.0
        }
    }
    pub struct MipmapFilterSet(pub wgpu::FilterMode);
    pub struct MipmapFilterUnset(PhantomData<wgpu::FilterMode>);
    pub trait MipmapFilterRequired {}
    pub trait MipmapFilterIsUnset: MipmapFilterRequired {}
    impl MipmapFilterRequired for MipmapFilterUnset {}
    impl MipmapFilterIsUnset for MipmapFilterUnset {}
    impl MipmapFilterRequired for MipmapFilterSet {}
    impl MipmapFilterSet {
        fn get(self) -> wgpu::FilterMode {
            self.0
        }
    }
    pub struct LodMinClampSet(pub f32);
    pub struct LodMinClampUnset(PhantomData<f32>);
    pub trait LodMinClampRequired {}
    pub trait LodMinClampIsUnset: LodMinClampRequired {}
    impl LodMinClampRequired for LodMinClampUnset {}
    impl LodMinClampIsUnset for LodMinClampUnset {}
    impl LodMinClampRequired for LodMinClampSet {}
    impl LodMinClampSet {
        fn get(self) -> f32 {
            self.0
        }
    }
    pub struct LodMaxClampSet(pub f32);
    pub struct LodMaxClampUnset(PhantomData<f32>);
    pub trait LodMaxClampRequired {}
    pub trait LodMaxClampIsUnset: LodMaxClampRequired {}
    impl LodMaxClampRequired for LodMaxClampUnset {}
    impl LodMaxClampIsUnset for LodMaxClampUnset {}
    impl LodMaxClampRequired for LodMaxClampSet {}
    impl LodMaxClampSet {
        fn get(self) -> f32 {
            self.0
        }
    }
    pub struct CompareSet(pub Option<wgpu::CompareFunction>);
    pub struct CompareUnset(PhantomData<Option<wgpu::CompareFunction>>);
    pub trait CompareRequired {}
    pub trait CompareIsUnset: CompareRequired {}
    impl CompareRequired for CompareUnset {}
    impl CompareIsUnset for CompareUnset {}
    impl CompareRequired for CompareSet {}
    impl CompareSet {
        fn get(self) -> Option<wgpu::CompareFunction> {
            self.0
        }
    }
    pub struct AnisotropyClampSet(pub u16);
    pub struct AnisotropyClampUnset(PhantomData<u16>);
    pub trait AnisotropyClampRequired {}
    pub trait AnisotropyClampIsUnset: AnisotropyClampRequired {}
    impl AnisotropyClampRequired for AnisotropyClampUnset {}
    impl AnisotropyClampIsUnset for AnisotropyClampUnset {}
    impl AnisotropyClampRequired for AnisotropyClampSet {}
    impl AnisotropyClampSet {
        fn get(self) -> u16 {
            self.0
        }
    }
    pub struct BorderColorSet(pub Option<wgpu::SamplerBorderColor>);
    pub struct BorderColorUnset(PhantomData<Option<wgpu::SamplerBorderColor>>);
    pub trait BorderColorRequired {}
    pub trait BorderColorIsUnset: BorderColorRequired {}
    impl BorderColorRequired for BorderColorUnset {}
    impl BorderColorIsUnset for BorderColorUnset {}
    impl BorderColorRequired for BorderColorSet {}
    impl BorderColorSet {
        fn get(self) -> Option<wgpu::SamplerBorderColor> {
            self.0
        }
    }
}

pub mod buffer_init_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct ContentsSet<'a>(pub &'a [u8]);
    pub struct ContentsUnset<'a>(PhantomData<&'a [u8]>);
    pub trait ContentsRequired {}
    pub trait ContentsIsUnset: ContentsRequired {}
    impl<'a> ContentsRequired for ContentsUnset<'a> {}
    impl<'a> ContentsIsUnset for ContentsUnset<'a> {}
    impl<'a> ContentsRequired for ContentsSet<'a> {}
    impl<'a> ContentsSet<'a> {
        fn get(self) -> &'a [u8] {
            self.0
        }
    }
    pub struct UsageSet(pub wgpu::BufferUsages);
    pub struct UsageUnset(PhantomData<wgpu::BufferUsages>);
    pub trait UsageRequired {}
    pub trait UsageIsUnset: UsageRequired {}
    impl UsageRequired for UsageUnset {}
    impl UsageIsUnset for UsageUnset {}
    impl UsageRequired for UsageSet {}
    impl UsageSet {
        fn get(self) -> wgpu::BufferUsages {
            self.0
        }
    }
}

pub mod downlevel_limits_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
}

pub mod bind_group_layout_entry_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BindingSet(pub u32);
    pub struct BindingUnset(PhantomData<u32>);
    pub trait BindingRequired {}
    pub trait BindingIsUnset: BindingRequired {}
    impl BindingRequired for BindingUnset {}
    impl BindingIsUnset for BindingUnset {}
    impl BindingRequired for BindingSet {}
    impl BindingSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct VisibilitySet(pub wgpu::ShaderStages);
    pub struct VisibilityUnset(PhantomData<wgpu::ShaderStages>);
    pub trait VisibilityRequired {}
    pub trait VisibilityIsUnset: VisibilityRequired {}
    impl VisibilityRequired for VisibilityUnset {}
    impl VisibilityIsUnset for VisibilityUnset {}
    impl VisibilityRequired for VisibilitySet {}
    impl VisibilitySet {
        fn get(self) -> wgpu::ShaderStages {
            self.0
        }
    }
    pub struct TySet(pub wgpu::BindingType);
    pub struct TyUnset(PhantomData<wgpu::BindingType>);
    pub trait TyRequired {}
    pub trait TyIsUnset: TyRequired {}
    impl TyRequired for TyUnset {}
    impl TyIsUnset for TyUnset {}
    impl TyRequired for TySet {}
    impl TySet {
        fn get(self) -> wgpu::BindingType {
            self.0
        }
    }
    pub struct CountSet(pub Option<NonZeroU32>);
    pub struct CountUnset(PhantomData<Option<NonZeroU32>>);
    pub trait CountRequired {}
    pub trait CountIsUnset: CountRequired {}
    impl CountRequired for CountUnset {}
    impl CountIsUnset for CountUnset {}
    impl CountRequired for CountSet {}
    impl CountSet {
        fn get(self) -> Option<NonZeroU32> {
            self.0
        }
    }
}

pub mod create_tlas_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct MaxInstancesSet(pub u32);
    pub struct MaxInstancesUnset(PhantomData<u32>);
    pub trait MaxInstancesRequired {}
    pub trait MaxInstancesIsUnset: MaxInstancesRequired {}
    impl MaxInstancesRequired for MaxInstancesUnset {}
    impl MaxInstancesIsUnset for MaxInstancesUnset {}
    impl MaxInstancesRequired for MaxInstancesSet {}
    impl MaxInstancesSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct FlagsSet(pub wgpu::wgt::AccelerationStructureFlags);
    pub struct FlagsUnset(PhantomData<wgpu::wgt::AccelerationStructureFlags>);
    pub trait FlagsRequired {}
    pub trait FlagsIsUnset: FlagsRequired {}
    impl FlagsRequired for FlagsUnset {}
    impl FlagsIsUnset for FlagsUnset {}
    impl FlagsRequired for FlagsSet {}
    impl FlagsSet {
        fn get(self) -> wgpu::wgt::AccelerationStructureFlags {
            self.0
        }
    }
    pub struct UpdateModeSet(pub wgpu::wgt::AccelerationStructureUpdateMode);
    pub struct UpdateModeUnset(PhantomData<wgpu::wgt::AccelerationStructureUpdateMode>);
    pub trait UpdateModeRequired {}
    pub trait UpdateModeIsUnset: UpdateModeRequired {}
    impl UpdateModeRequired for UpdateModeUnset {}
    impl UpdateModeIsUnset for UpdateModeUnset {}
    impl UpdateModeRequired for UpdateModeSet {}
    impl UpdateModeSet {
        fn get(self) -> wgpu::wgt::AccelerationStructureUpdateMode {
            self.0
        }
    }
}

pub mod texel_copy_buffer_info_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BufferSet<'a>(pub &'a wgpu::Buffer);
    pub struct BufferUnset<'a>(PhantomData<&'a wgpu::Buffer>);
    pub trait BufferRequired {}
    pub trait BufferIsUnset: BufferRequired {}
    impl<'a> BufferRequired for BufferUnset<'a> {}
    impl<'a> BufferIsUnset for BufferUnset<'a> {}
    impl<'a> BufferRequired for BufferSet<'a> {}
    impl<'a> BufferSet<'a> {
        fn get(self) -> &'a wgpu::Buffer {
            self.0
        }
    }
    pub struct LayoutSet(pub wgpu::TexelCopyBufferLayout);
    pub struct LayoutUnset(PhantomData<wgpu::TexelCopyBufferLayout>);
    pub trait LayoutRequired {}
    pub trait LayoutIsUnset: LayoutRequired {}
    impl LayoutRequired for LayoutUnset {}
    impl LayoutIsUnset for LayoutUnset {}
    impl LayoutRequired for LayoutSet {}
    impl LayoutSet {
        fn get(self) -> wgpu::TexelCopyBufferLayout {
            self.0
        }
    }
}

pub mod compute_pass_timestamp_writes_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct QuerySetSet<'a>(pub &'a wgpu::QuerySet);
    pub struct QuerySetUnset<'a>(PhantomData<&'a wgpu::QuerySet>);
    pub trait QuerySetRequired {}
    pub trait QuerySetIsUnset: QuerySetRequired {}
    impl<'a> QuerySetRequired for QuerySetUnset<'a> {}
    impl<'a> QuerySetIsUnset for QuerySetUnset<'a> {}
    impl<'a> QuerySetRequired for QuerySetSet<'a> {}
    impl<'a> QuerySetSet<'a> {
        fn get(self) -> &'a wgpu::QuerySet {
            self.0
        }
    }
    pub struct BeginningOfPassWriteIndexSet(pub Option<u32>);
    pub struct BeginningOfPassWriteIndexUnset(PhantomData<Option<u32>>);
    pub trait BeginningOfPassWriteIndexRequired {}
    pub trait BeginningOfPassWriteIndexIsUnset: BeginningOfPassWriteIndexRequired {}
    impl BeginningOfPassWriteIndexRequired for BeginningOfPassWriteIndexUnset {}
    impl BeginningOfPassWriteIndexIsUnset for BeginningOfPassWriteIndexUnset {}
    impl BeginningOfPassWriteIndexRequired for BeginningOfPassWriteIndexSet {}
    impl BeginningOfPassWriteIndexSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct EndOfPassWriteIndexSet(pub Option<u32>);
    pub struct EndOfPassWriteIndexUnset(PhantomData<Option<u32>>);
    pub trait EndOfPassWriteIndexRequired {}
    pub trait EndOfPassWriteIndexIsUnset: EndOfPassWriteIndexRequired {}
    impl EndOfPassWriteIndexRequired for EndOfPassWriteIndexUnset {}
    impl EndOfPassWriteIndexIsUnset for EndOfPassWriteIndexUnset {}
    impl EndOfPassWriteIndexRequired for EndOfPassWriteIndexSet {}
    impl EndOfPassWriteIndexSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
}

pub mod stencil_face_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct CompareSet(pub wgpu::CompareFunction);
    pub struct CompareUnset(PhantomData<wgpu::CompareFunction>);
    pub trait CompareRequired {}
    pub trait CompareIsUnset: CompareRequired {}
    impl CompareRequired for CompareUnset {}
    impl CompareIsUnset for CompareUnset {}
    impl CompareRequired for CompareSet {}
    impl CompareSet {
        fn get(self) -> wgpu::CompareFunction {
            self.0
        }
    }
    pub struct FailOpSet(pub wgpu::StencilOperation);
    pub struct FailOpUnset(PhantomData<wgpu::StencilOperation>);
    pub trait FailOpRequired {}
    pub trait FailOpIsUnset: FailOpRequired {}
    impl FailOpRequired for FailOpUnset {}
    impl FailOpIsUnset for FailOpUnset {}
    impl FailOpRequired for FailOpSet {}
    impl FailOpSet {
        fn get(self) -> wgpu::StencilOperation {
            self.0
        }
    }
    pub struct DepthFailOpSet(pub wgpu::StencilOperation);
    pub struct DepthFailOpUnset(PhantomData<wgpu::StencilOperation>);
    pub trait DepthFailOpRequired {}
    pub trait DepthFailOpIsUnset: DepthFailOpRequired {}
    impl DepthFailOpRequired for DepthFailOpUnset {}
    impl DepthFailOpIsUnset for DepthFailOpUnset {}
    impl DepthFailOpRequired for DepthFailOpSet {}
    impl DepthFailOpSet {
        fn get(self) -> wgpu::StencilOperation {
            self.0
        }
    }
    pub struct PassOpSet(pub wgpu::StencilOperation);
    pub struct PassOpUnset(PhantomData<wgpu::StencilOperation>);
    pub trait PassOpRequired {}
    pub trait PassOpIsUnset: PassOpRequired {}
    impl PassOpRequired for PassOpUnset {}
    impl PassOpIsUnset for PassOpUnset {}
    impl PassOpRequired for PassOpSet {}
    impl PassOpSet {
        fn get(self) -> wgpu::StencilOperation {
            self.0
        }
    }
}

pub mod origin_2_d_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct XSet(pub u32);
    pub struct XUnset(PhantomData<u32>);
    pub trait XRequired {}
    pub trait XIsUnset: XRequired {}
    impl XRequired for XUnset {}
    impl XIsUnset for XUnset {}
    impl XRequired for XSet {}
    impl XSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct YSet(pub u32);
    pub struct YUnset(PhantomData<u32>);
    pub trait YRequired {}
    pub trait YIsUnset: YRequired {}
    impl YRequired for YUnset {}
    impl YIsUnset for YUnset {}
    impl YRequired for YSet {}
    impl YSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod core_counters_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
}

pub mod blend_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ColorSet(pub wgpu::BlendComponent);
    pub struct ColorUnset(PhantomData<wgpu::BlendComponent>);
    pub trait ColorRequired {}
    pub trait ColorIsUnset: ColorRequired {}
    impl ColorRequired for ColorUnset {}
    impl ColorIsUnset for ColorUnset {}
    impl ColorRequired for ColorSet {}
    impl ColorSet {
        fn get(self) -> wgpu::BlendComponent {
            self.0
        }
    }
    pub struct AlphaSet(pub wgpu::BlendComponent);
    pub struct AlphaUnset(PhantomData<wgpu::BlendComponent>);
    pub trait AlphaRequired {}
    pub trait AlphaIsUnset: AlphaRequired {}
    impl AlphaRequired for AlphaUnset {}
    impl AlphaIsUnset for AlphaUnset {}
    impl AlphaRequired for AlphaSet {}
    impl AlphaSet {
        fn get(self) -> wgpu::BlendComponent {
            self.0
        }
    }
}

pub mod bind_group_layout_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct EntriesSet<'a>(pub &'a [wgpu::BindGroupLayoutEntry]);
    pub struct EntriesUnset<'a>(PhantomData<&'a [wgpu::BindGroupLayoutEntry]>);
    pub trait EntriesRequired {}
    pub trait EntriesIsUnset: EntriesRequired {}
    impl<'a> EntriesRequired for EntriesUnset<'a> {}
    impl<'a> EntriesIsUnset for EntriesUnset<'a> {}
    impl<'a> EntriesRequired for EntriesSet<'a> {}
    impl<'a> EntriesSet<'a> {
        fn get(self) -> &'a [wgpu::BindGroupLayoutEntry] {
            self.0
        }
    }
}

pub mod blas_triangle_geometry_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct SizeSet<'a>(pub &'a wgpu::BlasTriangleGeometrySizeDescriptor);
    pub struct SizeUnset<'a>(PhantomData<&'a wgpu::BlasTriangleGeometrySizeDescriptor>);
    pub trait SizeRequired {}
    pub trait SizeIsUnset: SizeRequired {}
    impl<'a> SizeRequired for SizeUnset<'a> {}
    impl<'a> SizeIsUnset for SizeUnset<'a> {}
    impl<'a> SizeRequired for SizeSet<'a> {}
    impl<'a> SizeSet<'a> {
        fn get(self) -> &'a wgpu::BlasTriangleGeometrySizeDescriptor {
            self.0
        }
    }
    pub struct VertexBufferSet<'a>(pub &'a wgpu::Buffer);
    pub struct VertexBufferUnset<'a>(PhantomData<&'a wgpu::Buffer>);
    pub trait VertexBufferRequired {}
    pub trait VertexBufferIsUnset: VertexBufferRequired {}
    impl<'a> VertexBufferRequired for VertexBufferUnset<'a> {}
    impl<'a> VertexBufferIsUnset for VertexBufferUnset<'a> {}
    impl<'a> VertexBufferRequired for VertexBufferSet<'a> {}
    impl<'a> VertexBufferSet<'a> {
        fn get(self) -> &'a wgpu::Buffer {
            self.0
        }
    }
    pub struct FirstVertexSet(pub u32);
    pub struct FirstVertexUnset(PhantomData<u32>);
    pub trait FirstVertexRequired {}
    pub trait FirstVertexIsUnset: FirstVertexRequired {}
    impl FirstVertexRequired for FirstVertexUnset {}
    impl FirstVertexIsUnset for FirstVertexUnset {}
    impl FirstVertexRequired for FirstVertexSet {}
    impl FirstVertexSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct VertexStrideSet(pub wgpu::BufferAddress);
    pub struct VertexStrideUnset(PhantomData<wgpu::BufferAddress>);
    pub trait VertexStrideRequired {}
    pub trait VertexStrideIsUnset: VertexStrideRequired {}
    impl VertexStrideRequired for VertexStrideUnset {}
    impl VertexStrideIsUnset for VertexStrideUnset {}
    impl VertexStrideRequired for VertexStrideSet {}
    impl VertexStrideSet {
        fn get(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct IndexBufferSet<'a>(pub Option<&'a wgpu::Buffer>);
    pub struct IndexBufferUnset<'a>(PhantomData<Option<&'a wgpu::Buffer>>);
    pub trait IndexBufferRequired {}
    pub trait IndexBufferIsUnset: IndexBufferRequired {}
    impl<'a> IndexBufferRequired for IndexBufferUnset<'a> {}
    impl<'a> IndexBufferIsUnset for IndexBufferUnset<'a> {}
    impl<'a> IndexBufferRequired for IndexBufferSet<'a> {}
    impl<'a> IndexBufferSet<'a> {
        fn get(self) -> Option<&'a wgpu::Buffer> {
            self.0
        }
    }
    pub struct FirstIndexSet(pub Option<u32>);
    pub struct FirstIndexUnset(PhantomData<Option<u32>>);
    pub trait FirstIndexRequired {}
    pub trait FirstIndexIsUnset: FirstIndexRequired {}
    impl FirstIndexRequired for FirstIndexUnset {}
    impl FirstIndexIsUnset for FirstIndexUnset {}
    impl FirstIndexRequired for FirstIndexSet {}
    impl FirstIndexSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct TransformBufferSet<'a>(pub Option<&'a wgpu::Buffer>);
    pub struct TransformBufferUnset<'a>(PhantomData<Option<&'a wgpu::Buffer>>);
    pub trait TransformBufferRequired {}
    pub trait TransformBufferIsUnset: TransformBufferRequired {}
    impl<'a> TransformBufferRequired for TransformBufferUnset<'a> {}
    impl<'a> TransformBufferIsUnset for TransformBufferUnset<'a> {}
    impl<'a> TransformBufferRequired for TransformBufferSet<'a> {}
    impl<'a> TransformBufferSet<'a> {
        fn get(self) -> Option<&'a wgpu::Buffer> {
            self.0
        }
    }
    pub struct TransformBufferOffsetSet(pub Option<wgpu::BufferAddress>);
    pub struct TransformBufferOffsetUnset(PhantomData<Option<wgpu::BufferAddress>>);
    pub trait TransformBufferOffsetRequired {}
    pub trait TransformBufferOffsetIsUnset: TransformBufferOffsetRequired {}
    impl TransformBufferOffsetRequired for TransformBufferOffsetUnset {}
    impl TransformBufferOffsetIsUnset for TransformBufferOffsetUnset {}
    impl TransformBufferOffsetRequired for TransformBufferOffsetSet {}
    impl TransformBufferOffsetSet {
        fn get(self) -> Option<wgpu::BufferAddress> {
            self.0
        }
    }
}

pub mod create_blas_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct FlagsSet(pub wgpu::wgt::AccelerationStructureFlags);
    pub struct FlagsUnset(PhantomData<wgpu::wgt::AccelerationStructureFlags>);
    pub trait FlagsRequired {}
    pub trait FlagsIsUnset: FlagsRequired {}
    impl FlagsRequired for FlagsUnset {}
    impl FlagsIsUnset for FlagsUnset {}
    impl FlagsRequired for FlagsSet {}
    impl FlagsSet {
        fn get(self) -> wgpu::wgt::AccelerationStructureFlags {
            self.0
        }
    }
    pub struct UpdateModeSet(pub wgpu::wgt::AccelerationStructureUpdateMode);
    pub struct UpdateModeUnset(PhantomData<wgpu::wgt::AccelerationStructureUpdateMode>);
    pub trait UpdateModeRequired {}
    pub trait UpdateModeIsUnset: UpdateModeRequired {}
    impl UpdateModeRequired for UpdateModeUnset {}
    impl UpdateModeIsUnset for UpdateModeUnset {}
    impl UpdateModeRequired for UpdateModeSet {}
    impl UpdateModeSet {
        fn get(self) -> wgpu::wgt::AccelerationStructureUpdateMode {
            self.0
        }
    }
}

pub mod texel_copy_buffer_info_base_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BufferSet<B>(pub B);
    pub struct BufferUnset<B>(PhantomData<B>);
    pub trait BufferRequired {}
    pub trait BufferIsUnset: BufferRequired {}
    impl<B> BufferRequired for BufferUnset<B> {}
    impl<B> BufferIsUnset for BufferUnset<B> {}
    impl<B> BufferRequired for BufferSet<B> {}
    impl<B> BufferSet<B> {
        fn get(self) -> B {
            self.0
        }
    }
    pub struct LayoutSet(pub wgpu::TexelCopyBufferLayout);
    pub struct LayoutUnset(PhantomData<wgpu::TexelCopyBufferLayout>);
    pub trait LayoutRequired {}
    pub trait LayoutIsUnset: LayoutRequired {}
    impl LayoutRequired for LayoutUnset {}
    impl LayoutIsUnset for LayoutUnset {}
    impl LayoutRequired for LayoutSet {}
    impl LayoutSet {
        fn get(self) -> wgpu::TexelCopyBufferLayout {
            self.0
        }
    }
}

pub mod pipeline_compilation_options_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ConstantsSet<'a>(pub &'a [(&'a str, f64)]);
    pub struct ConstantsUnset<'a>(PhantomData<&'a [(&'a str, f64)]>);
    pub trait ConstantsRequired {}
    pub trait ConstantsIsUnset: ConstantsRequired {}
    impl<'a> ConstantsRequired for ConstantsUnset<'a> {}
    impl<'a> ConstantsIsUnset for ConstantsUnset<'a> {}
    impl<'a> ConstantsRequired for ConstantsSet<'a> {}
    impl<'a> ConstantsSet<'a> {
        fn get(self) -> &'a [(&'a str, f64)] {
            self.0
        }
    }
    pub struct ZeroInitializeWorkgroupMemorySet(pub bool);
    pub struct ZeroInitializeWorkgroupMemoryUnset(PhantomData<bool>);
    pub trait ZeroInitializeWorkgroupMemoryRequired {}
    pub trait ZeroInitializeWorkgroupMemoryIsUnset: ZeroInitializeWorkgroupMemoryRequired {}
    impl ZeroInitializeWorkgroupMemoryRequired for ZeroInitializeWorkgroupMemoryUnset {}
    impl ZeroInitializeWorkgroupMemoryIsUnset for ZeroInitializeWorkgroupMemoryUnset {}
    impl ZeroInitializeWorkgroupMemoryRequired for ZeroInitializeWorkgroupMemorySet {}
    impl ZeroInitializeWorkgroupMemorySet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod device_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct RequiredFeaturesSet(pub wgpu::Features);
    pub struct RequiredFeaturesUnset(PhantomData<wgpu::Features>);
    pub trait RequiredFeaturesRequired {}
    pub trait RequiredFeaturesIsUnset: RequiredFeaturesRequired {}
    impl RequiredFeaturesRequired for RequiredFeaturesUnset {}
    impl RequiredFeaturesIsUnset for RequiredFeaturesUnset {}
    impl RequiredFeaturesRequired for RequiredFeaturesSet {}
    impl RequiredFeaturesSet {
        fn get(self) -> wgpu::Features {
            self.0
        }
    }
    pub struct RequiredLimitsSet(pub wgpu::Limits);
    pub struct RequiredLimitsUnset(PhantomData<wgpu::Limits>);
    pub trait RequiredLimitsRequired {}
    pub trait RequiredLimitsIsUnset: RequiredLimitsRequired {}
    impl RequiredLimitsRequired for RequiredLimitsUnset {}
    impl RequiredLimitsIsUnset for RequiredLimitsUnset {}
    impl RequiredLimitsRequired for RequiredLimitsSet {}
    impl RequiredLimitsSet {
        fn get(self) -> wgpu::Limits {
            self.0
        }
    }
    pub struct ExperimentalFeaturesSet(pub wgpu::ExperimentalFeatures);
    pub struct ExperimentalFeaturesUnset(PhantomData<wgpu::ExperimentalFeatures>);
    pub trait ExperimentalFeaturesRequired {}
    pub trait ExperimentalFeaturesIsUnset: ExperimentalFeaturesRequired {}
    impl ExperimentalFeaturesRequired for ExperimentalFeaturesUnset {}
    impl ExperimentalFeaturesIsUnset for ExperimentalFeaturesUnset {}
    impl ExperimentalFeaturesRequired for ExperimentalFeaturesSet {}
    impl ExperimentalFeaturesSet {
        fn get(self) -> wgpu::ExperimentalFeatures {
            self.0
        }
    }
    pub struct MemoryHintsSet(pub wgpu::MemoryHints);
    pub struct MemoryHintsUnset(PhantomData<wgpu::MemoryHints>);
    pub trait MemoryHintsRequired {}
    pub trait MemoryHintsIsUnset: MemoryHintsRequired {}
    impl MemoryHintsRequired for MemoryHintsUnset {}
    impl MemoryHintsIsUnset for MemoryHintsUnset {}
    impl MemoryHintsRequired for MemoryHintsSet {}
    impl MemoryHintsSet {
        fn get(self) -> wgpu::MemoryHints {
            self.0
        }
    }
    pub struct TraceSet(pub wgpu::Trace);
    pub struct TraceUnset(PhantomData<wgpu::Trace>);
    pub trait TraceRequired {}
    pub trait TraceIsUnset: TraceRequired {}
    impl TraceRequired for TraceUnset {}
    impl TraceIsUnset for TraceUnset {}
    impl TraceRequired for TraceSet {}
    impl TraceSet {
        fn get(self) -> wgpu::Trace {
            self.0
        }
    }
}

pub mod request_adapter_options_base_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct PowerPreferenceSet(pub wgpu::PowerPreference);
    pub struct PowerPreferenceUnset(PhantomData<wgpu::PowerPreference>);
    pub trait PowerPreferenceRequired {}
    pub trait PowerPreferenceIsUnset: PowerPreferenceRequired {}
    impl PowerPreferenceRequired for PowerPreferenceUnset {}
    impl PowerPreferenceIsUnset for PowerPreferenceUnset {}
    impl PowerPreferenceRequired for PowerPreferenceSet {}
    impl PowerPreferenceSet {
        fn get(self) -> wgpu::PowerPreference {
            self.0
        }
    }
    pub struct ForceFallbackAdapterSet(pub bool);
    pub struct ForceFallbackAdapterUnset(PhantomData<bool>);
    pub trait ForceFallbackAdapterRequired {}
    pub trait ForceFallbackAdapterIsUnset: ForceFallbackAdapterRequired {}
    impl ForceFallbackAdapterRequired for ForceFallbackAdapterUnset {}
    impl ForceFallbackAdapterIsUnset for ForceFallbackAdapterUnset {}
    impl ForceFallbackAdapterRequired for ForceFallbackAdapterSet {}
    impl ForceFallbackAdapterSet {
        fn get(self) -> bool {
            self.0
        }
    }
    pub struct CompatibleSurfaceSet<S>(pub Option<S>);
    pub struct CompatibleSurfaceUnset<S>(PhantomData<Option<S>>);
    pub trait CompatibleSurfaceRequired {}
    pub trait CompatibleSurfaceIsUnset: CompatibleSurfaceRequired {}
    impl<S> CompatibleSurfaceRequired for CompatibleSurfaceUnset<S> {}
    impl<S> CompatibleSurfaceIsUnset for CompatibleSurfaceUnset<S> {}
    impl<S> CompatibleSurfaceRequired for CompatibleSurfaceSet<S> {}
    impl<S> CompatibleSurfaceSet<S> {
        fn get(self) -> Option<S> {
            self.0
        }
    }
}

pub mod shader_module_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct SourceSet<'a>(pub wgpu::ShaderSource<'a>);
    pub struct SourceUnset<'a>(PhantomData<wgpu::ShaderSource<'a>>);
    pub trait SourceRequired {}
    pub trait SourceIsUnset: SourceRequired {}
    impl<'a> SourceRequired for SourceUnset<'a> {}
    impl<'a> SourceIsUnset for SourceUnset<'a> {}
    impl<'a> SourceRequired for SourceSet<'a> {}
    impl<'a> SourceSet<'a> {
        fn get(self) -> wgpu::ShaderSource<'a> {
            self.0
        }
    }
}

pub mod bind_group_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct LayoutSet<'a>(pub &'a wgpu::BindGroupLayout);
    pub struct LayoutUnset<'a>(PhantomData<&'a wgpu::BindGroupLayout>);
    pub trait LayoutRequired {}
    pub trait LayoutIsUnset: LayoutRequired {}
    impl<'a> LayoutRequired for LayoutUnset<'a> {}
    impl<'a> LayoutIsUnset for LayoutUnset<'a> {}
    impl<'a> LayoutRequired for LayoutSet<'a> {}
    impl<'a> LayoutSet<'a> {
        fn get(self) -> &'a wgpu::BindGroupLayout {
            self.0
        }
    }
    pub struct EntriesSet<'a>(pub &'a [wgpu::BindGroupEntry<'a>]);
    pub struct EntriesUnset<'a>(PhantomData<&'a [wgpu::BindGroupEntry<'a>]>);
    pub trait EntriesRequired {}
    pub trait EntriesIsUnset: EntriesRequired {}
    impl<'a> EntriesRequired for EntriesUnset<'a> {}
    impl<'a> EntriesIsUnset for EntriesUnset<'a> {}
    impl<'a> EntriesRequired for EntriesSet<'a> {}
    impl<'a> EntriesSet<'a> {
        fn get(self) -> &'a [wgpu::BindGroupEntry<'a>] {
            self.0
        }
    }
}

pub mod render_pass_depth_stencil_attachment_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ViewSet<'tex>(pub &'tex wgpu::TextureView);
    pub struct ViewUnset<'tex>(PhantomData<&'tex wgpu::TextureView>);
    pub trait ViewRequired {}
    pub trait ViewIsUnset: ViewRequired {}
    impl<'tex> ViewRequired for ViewUnset<'tex> {}
    impl<'tex> ViewIsUnset for ViewUnset<'tex> {}
    impl<'tex> ViewRequired for ViewSet<'tex> {}
    impl<'tex> ViewSet<'tex> {
        fn get(self) -> &'tex wgpu::TextureView {
            self.0
        }
    }
    pub struct DepthOpsSet(pub Option<wgpu::Operations<f32>>);
    pub struct DepthOpsUnset(PhantomData<Option<wgpu::Operations<f32>>>);
    pub trait DepthOpsRequired {}
    pub trait DepthOpsIsUnset: DepthOpsRequired {}
    impl DepthOpsRequired for DepthOpsUnset {}
    impl DepthOpsIsUnset for DepthOpsUnset {}
    impl DepthOpsRequired for DepthOpsSet {}
    impl DepthOpsSet {
        fn get(self) -> Option<wgpu::Operations<f32>> {
            self.0
        }
    }
    pub struct StencilOpsSet(pub Option<wgpu::Operations<u32>>);
    pub struct StencilOpsUnset(PhantomData<Option<wgpu::Operations<u32>>>);
    pub trait StencilOpsRequired {}
    pub trait StencilOpsIsUnset: StencilOpsRequired {}
    impl StencilOpsRequired for StencilOpsUnset {}
    impl StencilOpsIsUnset for StencilOpsUnset {}
    impl StencilOpsRequired for StencilOpsSet {}
    impl StencilOpsSet {
        fn get(self) -> Option<wgpu::Operations<u32>> {
            self.0
        }
    }
}

pub mod pipeline_cache_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct DataSet<'a>(pub Option<&'a [u8]>);
    pub struct DataUnset<'a>(PhantomData<Option<&'a [u8]>>);
    pub trait DataRequired {}
    pub trait DataIsUnset: DataRequired {}
    impl<'a> DataRequired for DataUnset<'a> {}
    impl<'a> DataIsUnset for DataUnset<'a> {}
    impl<'a> DataRequired for DataSet<'a> {}
    impl<'a> DataSet<'a> {
        fn get(self) -> Option<&'a [u8]> {
            self.0
        }
    }
    pub struct FallbackSet(pub bool);
    pub struct FallbackUnset(PhantomData<bool>);
    pub trait FallbackRequired {}
    pub trait FallbackIsUnset: FallbackRequired {}
    impl FallbackRequired for FallbackUnset {}
    impl FallbackIsUnset for FallbackUnset {}
    impl FallbackRequired for FallbackSet {}
    impl FallbackSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod mesh_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ModuleSet<'a>(pub &'a wgpu::ShaderModule);
    pub struct ModuleUnset<'a>(PhantomData<&'a wgpu::ShaderModule>);
    pub trait ModuleRequired {}
    pub trait ModuleIsUnset: ModuleRequired {}
    impl<'a> ModuleRequired for ModuleUnset<'a> {}
    impl<'a> ModuleIsUnset for ModuleUnset<'a> {}
    impl<'a> ModuleRequired for ModuleSet<'a> {}
    impl<'a> ModuleSet<'a> {
        fn get(self) -> &'a wgpu::ShaderModule {
            self.0
        }
    }
    pub struct EntryPointSet<'a>(pub Option<&'a str>);
    pub struct EntryPointUnset<'a>(PhantomData<Option<&'a str>>);
    pub trait EntryPointRequired {}
    pub trait EntryPointIsUnset: EntryPointRequired {}
    impl<'a> EntryPointRequired for EntryPointUnset<'a> {}
    impl<'a> EntryPointIsUnset for EntryPointUnset<'a> {}
    impl<'a> EntryPointRequired for EntryPointSet<'a> {}
    impl<'a> EntryPointSet<'a> {
        fn get(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct CompilationOptionsSet<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    pub struct CompilationOptionsUnset<'a>(PhantomData<wgpu::PipelineCompilationOptions<'a>>);
    pub trait CompilationOptionsRequired {}
    pub trait CompilationOptionsIsUnset: CompilationOptionsRequired {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsIsUnset for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsSet<'a> {}
    impl<'a> CompilationOptionsSet<'a> {
        fn get(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
}

pub mod vertex_attribute_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct FormatSet(pub wgpu::VertexFormat);
    pub struct FormatUnset(PhantomData<wgpu::VertexFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::VertexFormat {
            self.0
        }
    }
    pub struct OffsetSet(pub wgpu::BufferAddress);
    pub struct OffsetUnset(PhantomData<wgpu::BufferAddress>);
    pub trait OffsetRequired {}
    pub trait OffsetIsUnset: OffsetRequired {}
    impl OffsetRequired for OffsetUnset {}
    impl OffsetIsUnset for OffsetUnset {}
    impl OffsetRequired for OffsetSet {}
    impl OffsetSet {
        fn get(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct ShaderLocationSet(pub wgpu::ShaderLocation);
    pub struct ShaderLocationUnset(PhantomData<wgpu::ShaderLocation>);
    pub trait ShaderLocationRequired {}
    pub trait ShaderLocationIsUnset: ShaderLocationRequired {}
    impl ShaderLocationRequired for ShaderLocationUnset {}
    impl ShaderLocationIsUnset for ShaderLocationUnset {}
    impl ShaderLocationRequired for ShaderLocationSet {}
    impl ShaderLocationSet {
        fn get(self) -> wgpu::ShaderLocation {
            self.0
        }
    }
}

pub mod buffer_binding_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BufferSet<'a>(pub &'a wgpu::Buffer);
    pub struct BufferUnset<'a>(PhantomData<&'a wgpu::Buffer>);
    pub trait BufferRequired {}
    pub trait BufferIsUnset: BufferRequired {}
    impl<'a> BufferRequired for BufferUnset<'a> {}
    impl<'a> BufferIsUnset for BufferUnset<'a> {}
    impl<'a> BufferRequired for BufferSet<'a> {}
    impl<'a> BufferSet<'a> {
        fn get(self) -> &'a wgpu::Buffer {
            self.0
        }
    }
    pub struct OffsetSet(pub wgpu::BufferAddress);
    pub struct OffsetUnset(PhantomData<wgpu::BufferAddress>);
    pub trait OffsetRequired {}
    pub trait OffsetIsUnset: OffsetRequired {}
    impl OffsetRequired for OffsetUnset {}
    impl OffsetIsUnset for OffsetUnset {}
    impl OffsetRequired for OffsetSet {}
    impl OffsetSet {
        fn get(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct SizeSet(pub Option<wgpu::BufferSize>);
    pub struct SizeUnset(PhantomData<Option<wgpu::BufferSize>>);
    pub trait SizeRequired {}
    pub trait SizeIsUnset: SizeRequired {}
    impl SizeRequired for SizeUnset {}
    impl SizeIsUnset for SizeUnset {}
    impl SizeRequired for SizeSet {}
    impl SizeSet {
        fn get(self) -> Option<wgpu::BufferSize> {
            self.0
        }
    }
}

pub mod compilation_info_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct MessagesSet(pub Vec<wgpu::CompilationMessage>);
    pub struct MessagesUnset(PhantomData<Vec<wgpu::CompilationMessage>>);
    pub trait MessagesRequired {}
    pub trait MessagesIsUnset: MessagesRequired {}
    impl MessagesRequired for MessagesUnset {}
    impl MessagesIsUnset for MessagesUnset {}
    impl MessagesRequired for MessagesSet {}
    impl MessagesSet {
        fn get(self) -> Vec<wgpu::CompilationMessage> {
            self.0
        }
    }
}

pub mod shader_runtime_checks_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BoundsChecksSet(pub bool);
    pub struct BoundsChecksUnset(PhantomData<bool>);
    pub trait BoundsChecksRequired {}
    pub trait BoundsChecksIsUnset: BoundsChecksRequired {}
    impl BoundsChecksRequired for BoundsChecksUnset {}
    impl BoundsChecksIsUnset for BoundsChecksUnset {}
    impl BoundsChecksRequired for BoundsChecksSet {}
    impl BoundsChecksSet {
        fn get(self) -> bool {
            self.0
        }
    }
    pub struct ForceLoopBoundingSet(pub bool);
    pub struct ForceLoopBoundingUnset(PhantomData<bool>);
    pub trait ForceLoopBoundingRequired {}
    pub trait ForceLoopBoundingIsUnset: ForceLoopBoundingRequired {}
    impl ForceLoopBoundingRequired for ForceLoopBoundingUnset {}
    impl ForceLoopBoundingIsUnset for ForceLoopBoundingUnset {}
    impl ForceLoopBoundingRequired for ForceLoopBoundingSet {}
    impl ForceLoopBoundingSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod request_adapter_options_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct PowerPreferenceSet(pub wgpu::PowerPreference);
    pub struct PowerPreferenceUnset(PhantomData<wgpu::PowerPreference>);
    pub trait PowerPreferenceRequired {}
    pub trait PowerPreferenceIsUnset: PowerPreferenceRequired {}
    impl PowerPreferenceRequired for PowerPreferenceUnset {}
    impl PowerPreferenceIsUnset for PowerPreferenceUnset {}
    impl PowerPreferenceRequired for PowerPreferenceSet {}
    impl PowerPreferenceSet {
        fn get(self) -> wgpu::PowerPreference {
            self.0
        }
    }
    pub struct ForceFallbackAdapterSet(pub bool);
    pub struct ForceFallbackAdapterUnset(PhantomData<bool>);
    pub trait ForceFallbackAdapterRequired {}
    pub trait ForceFallbackAdapterIsUnset: ForceFallbackAdapterRequired {}
    impl ForceFallbackAdapterRequired for ForceFallbackAdapterUnset {}
    impl ForceFallbackAdapterIsUnset for ForceFallbackAdapterUnset {}
    impl ForceFallbackAdapterRequired for ForceFallbackAdapterSet {}
    impl ForceFallbackAdapterSet {
        fn get(self) -> bool {
            self.0
        }
    }
    pub struct CompatibleSurfaceSet<'a, 'b>(pub Option<&'a wgpu::Surface<'b>>);
    pub struct CompatibleSurfaceUnset<'a, 'b>(PhantomData<Option<&'a wgpu::Surface<'b>>>);
    pub trait CompatibleSurfaceRequired {}
    pub trait CompatibleSurfaceIsUnset: CompatibleSurfaceRequired {}
    impl<'a, 'b> CompatibleSurfaceRequired for CompatibleSurfaceUnset<'a, 'b> {}
    impl<'a, 'b> CompatibleSurfaceIsUnset for CompatibleSurfaceUnset<'a, 'b> {}
    impl<'a, 'b> CompatibleSurfaceRequired for CompatibleSurfaceSet<'a, 'b> {}
    impl<'a, 'b> CompatibleSurfaceSet<'a, 'b> {
        fn get(self) -> Option<&'a wgpu::Surface<'b>> {
            self.0
        }
    }
}

pub mod external_texture_transfer_function_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ASet(pub f32);
    pub struct AUnset(PhantomData<f32>);
    pub trait ARequired {}
    pub trait AIsUnset: ARequired {}
    impl ARequired for AUnset {}
    impl AIsUnset for AUnset {}
    impl ARequired for ASet {}
    impl ASet {
        fn get(self) -> f32 {
            self.0
        }
    }
    pub struct BSet(pub f32);
    pub struct BUnset(PhantomData<f32>);
    pub trait BRequired {}
    pub trait BIsUnset: BRequired {}
    impl BRequired for BUnset {}
    impl BIsUnset for BUnset {}
    impl BRequired for BSet {}
    impl BSet {
        fn get(self) -> f32 {
            self.0
        }
    }
    pub struct GSet(pub f32);
    pub struct GUnset(PhantomData<f32>);
    pub trait GRequired {}
    pub trait GIsUnset: GRequired {}
    impl GRequired for GUnset {}
    impl GIsUnset for GUnset {}
    impl GRequired for GSet {}
    impl GSet {
        fn get(self) -> f32 {
            self.0
        }
    }
    pub struct KSet(pub f32);
    pub struct KUnset(PhantomData<f32>);
    pub trait KRequired {}
    pub trait KIsUnset: KRequired {}
    impl KRequired for KUnset {}
    impl KIsUnset for KUnset {}
    impl KRequired for KSet {}
    impl KSet {
        fn get(self) -> f32 {
            self.0
        }
    }
}

pub mod color_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct RSet(pub f64);
    pub struct RUnset(PhantomData<f64>);
    pub trait RRequired {}
    pub trait RIsUnset: RRequired {}
    impl RRequired for RUnset {}
    impl RIsUnset for RUnset {}
    impl RRequired for RSet {}
    impl RSet {
        fn get(self) -> f64 {
            self.0
        }
    }
    pub struct GSet(pub f64);
    pub struct GUnset(PhantomData<f64>);
    pub trait GRequired {}
    pub trait GIsUnset: GRequired {}
    impl GRequired for GUnset {}
    impl GIsUnset for GUnset {}
    impl GRequired for GSet {}
    impl GSet {
        fn get(self) -> f64 {
            self.0
        }
    }
    pub struct BSet(pub f64);
    pub struct BUnset(PhantomData<f64>);
    pub trait BRequired {}
    pub trait BIsUnset: BRequired {}
    impl BRequired for BUnset {}
    impl BIsUnset for BUnset {}
    impl BRequired for BSet {}
    impl BSet {
        fn get(self) -> f64 {
            self.0
        }
    }
    pub struct ASet(pub f64);
    pub struct AUnset(PhantomData<f64>);
    pub trait ARequired {}
    pub trait AIsUnset: ARequired {}
    impl ARequired for AUnset {}
    impl AIsUnset for AUnset {}
    impl ARequired for ASet {}
    impl ASet {
        fn get(self) -> f64 {
            self.0
        }
    }
}

pub mod command_encoder_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
}

pub mod query_set_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct TySet(pub wgpu::QueryType);
    pub struct TyUnset(PhantomData<wgpu::QueryType>);
    pub trait TyRequired {}
    pub trait TyIsUnset: TyRequired {}
    impl TyRequired for TyUnset {}
    impl TyIsUnset for TyUnset {}
    impl TyRequired for TySet {}
    impl TySet {
        fn get(self) -> wgpu::QueryType {
            self.0
        }
    }
    pub struct CountSet(pub u32);
    pub struct CountUnset(PhantomData<u32>);
    pub trait CountRequired {}
    pub trait CountIsUnset: CountRequired {}
    impl CountRequired for CountUnset {}
    impl CountIsUnset for CountUnset {}
    impl CountRequired for CountSet {}
    impl CountSet {
        fn get(self) -> u32 {
            self.0
        }
    }
}

pub mod texture_view_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct FormatSet(pub Option<wgpu::TextureFormat>);
    pub struct FormatUnset(PhantomData<Option<wgpu::TextureFormat>>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> Option<wgpu::TextureFormat> {
            self.0
        }
    }
    pub struct DimensionSet(pub Option<wgpu::TextureViewDimension>);
    pub struct DimensionUnset(PhantomData<Option<wgpu::TextureViewDimension>>);
    pub trait DimensionRequired {}
    pub trait DimensionIsUnset: DimensionRequired {}
    impl DimensionRequired for DimensionUnset {}
    impl DimensionIsUnset for DimensionUnset {}
    impl DimensionRequired for DimensionSet {}
    impl DimensionSet {
        fn get(self) -> Option<wgpu::TextureViewDimension> {
            self.0
        }
    }
    pub struct UsageSet(pub Option<wgpu::TextureUsages>);
    pub struct UsageUnset(PhantomData<Option<wgpu::TextureUsages>>);
    pub trait UsageRequired {}
    pub trait UsageIsUnset: UsageRequired {}
    impl UsageRequired for UsageUnset {}
    impl UsageIsUnset for UsageUnset {}
    impl UsageRequired for UsageSet {}
    impl UsageSet {
        fn get(self) -> Option<wgpu::TextureUsages> {
            self.0
        }
    }
    pub struct AspectSet(pub wgpu::TextureAspect);
    pub struct AspectUnset(PhantomData<wgpu::TextureAspect>);
    pub trait AspectRequired {}
    pub trait AspectIsUnset: AspectRequired {}
    impl AspectRequired for AspectUnset {}
    impl AspectIsUnset for AspectUnset {}
    impl AspectRequired for AspectSet {}
    impl AspectSet {
        fn get(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    pub struct BaseMipLevelSet(pub u32);
    pub struct BaseMipLevelUnset(PhantomData<u32>);
    pub trait BaseMipLevelRequired {}
    pub trait BaseMipLevelIsUnset: BaseMipLevelRequired {}
    impl BaseMipLevelRequired for BaseMipLevelUnset {}
    impl BaseMipLevelIsUnset for BaseMipLevelUnset {}
    impl BaseMipLevelRequired for BaseMipLevelSet {}
    impl BaseMipLevelSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct MipLevelCountSet(pub Option<u32>);
    pub struct MipLevelCountUnset(PhantomData<Option<u32>>);
    pub trait MipLevelCountRequired {}
    pub trait MipLevelCountIsUnset: MipLevelCountRequired {}
    impl MipLevelCountRequired for MipLevelCountUnset {}
    impl MipLevelCountIsUnset for MipLevelCountUnset {}
    impl MipLevelCountRequired for MipLevelCountSet {}
    impl MipLevelCountSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
    pub struct BaseArrayLayerSet(pub u32);
    pub struct BaseArrayLayerUnset(PhantomData<u32>);
    pub trait BaseArrayLayerRequired {}
    pub trait BaseArrayLayerIsUnset: BaseArrayLayerRequired {}
    impl BaseArrayLayerRequired for BaseArrayLayerUnset {}
    impl BaseArrayLayerIsUnset for BaseArrayLayerUnset {}
    impl BaseArrayLayerRequired for BaseArrayLayerSet {}
    impl BaseArrayLayerSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct ArrayLayerCountSet(pub Option<u32>);
    pub struct ArrayLayerCountUnset(PhantomData<Option<u32>>);
    pub trait ArrayLayerCountRequired {}
    pub trait ArrayLayerCountIsUnset: ArrayLayerCountRequired {}
    impl ArrayLayerCountRequired for ArrayLayerCountUnset {}
    impl ArrayLayerCountIsUnset for ArrayLayerCountUnset {}
    impl ArrayLayerCountRequired for ArrayLayerCountSet {}
    impl ArrayLayerCountSet {
        fn get(self) -> Option<u32> {
            self.0
        }
    }
}

pub mod buffer_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct SizeSet(pub wgpu::BufferAddress);
    pub struct SizeUnset(PhantomData<wgpu::BufferAddress>);
    pub trait SizeRequired {}
    pub trait SizeIsUnset: SizeRequired {}
    impl SizeRequired for SizeUnset {}
    impl SizeIsUnset for SizeUnset {}
    impl SizeRequired for SizeSet {}
    impl SizeSet {
        fn get(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct UsageSet(pub wgpu::BufferUsages);
    pub struct UsageUnset(PhantomData<wgpu::BufferUsages>);
    pub trait UsageRequired {}
    pub trait UsageIsUnset: UsageRequired {}
    impl UsageRequired for UsageUnset {}
    impl UsageIsUnset for UsageUnset {}
    impl UsageRequired for UsageSet {}
    impl UsageSet {
        fn get(self) -> wgpu::BufferUsages {
            self.0
        }
    }
    pub struct MappedAtCreationSet(pub bool);
    pub struct MappedAtCreationUnset(PhantomData<bool>);
    pub trait MappedAtCreationRequired {}
    pub trait MappedAtCreationIsUnset: MappedAtCreationRequired {}
    impl MappedAtCreationRequired for MappedAtCreationUnset {}
    impl MappedAtCreationIsUnset for MappedAtCreationUnset {}
    impl MappedAtCreationRequired for MappedAtCreationSet {}
    impl MappedAtCreationSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod texture_transition_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct TextureSet<T>(pub T);
    pub struct TextureUnset<T>(PhantomData<T>);
    pub trait TextureRequired {}
    pub trait TextureIsUnset: TextureRequired {}
    impl<T> TextureRequired for TextureUnset<T> {}
    impl<T> TextureIsUnset for TextureUnset<T> {}
    impl<T> TextureRequired for TextureSet<T> {}
    impl<T> TextureSet<T> {
        fn get(self) -> T {
            self.0
        }
    }
    pub struct SelectorSet(pub Option<wgpu::wgt::TextureSelector>);
    pub struct SelectorUnset(PhantomData<Option<wgpu::wgt::TextureSelector>>);
    pub trait SelectorRequired {}
    pub trait SelectorIsUnset: SelectorRequired {}
    impl SelectorRequired for SelectorUnset {}
    impl SelectorIsUnset for SelectorUnset {}
    impl SelectorRequired for SelectorSet {}
    impl SelectorSet {
        fn get(self) -> Option<wgpu::wgt::TextureSelector> {
            self.0
        }
    }
    pub struct StateSet(pub wgpu::TextureUses);
    pub struct StateUnset(PhantomData<wgpu::TextureUses>);
    pub trait StateRequired {}
    pub trait StateIsUnset: StateRequired {}
    impl StateRequired for StateUnset {}
    impl StateIsUnset for StateUnset {}
    impl StateRequired for StateSet {}
    impl StateSet {
        fn get(self) -> wgpu::TextureUses {
            self.0
        }
    }
}

pub mod depth_bias_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ConstantSet(pub i32);
    pub struct ConstantUnset(PhantomData<i32>);
    pub trait ConstantRequired {}
    pub trait ConstantIsUnset: ConstantRequired {}
    impl ConstantRequired for ConstantUnset {}
    impl ConstantIsUnset for ConstantUnset {}
    impl ConstantRequired for ConstantSet {}
    impl ConstantSet {
        fn get(self) -> i32 {
            self.0
        }
    }
    pub struct SlopeScaleSet(pub f32);
    pub struct SlopeScaleUnset(PhantomData<f32>);
    pub trait SlopeScaleRequired {}
    pub trait SlopeScaleIsUnset: SlopeScaleRequired {}
    impl SlopeScaleRequired for SlopeScaleUnset {}
    impl SlopeScaleIsUnset for SlopeScaleUnset {}
    impl SlopeScaleRequired for SlopeScaleSet {}
    impl SlopeScaleSet {
        fn get(self) -> f32 {
            self.0
        }
    }
    pub struct ClampSet(pub f32);
    pub struct ClampUnset(PhantomData<f32>);
    pub trait ClampRequired {}
    pub trait ClampIsUnset: ClampRequired {}
    impl ClampRequired for ClampUnset {}
    impl ClampIsUnset for ClampUnset {}
    impl ClampRequired for ClampSet {}
    impl ClampSet {
        fn get(self) -> f32 {
            self.0
        }
    }
}

pub mod color_target_state_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct FormatSet(pub wgpu::TextureFormat);
    pub struct FormatUnset(PhantomData<wgpu::TextureFormat>);
    pub trait FormatRequired {}
    pub trait FormatIsUnset: FormatRequired {}
    impl FormatRequired for FormatUnset {}
    impl FormatIsUnset for FormatUnset {}
    impl FormatRequired for FormatSet {}
    impl FormatSet {
        fn get(self) -> wgpu::TextureFormat {
            self.0
        }
    }
    pub struct BlendSet(pub Option<wgpu::BlendState>);
    pub struct BlendUnset(PhantomData<Option<wgpu::BlendState>>);
    pub trait BlendRequired {}
    pub trait BlendIsUnset: BlendRequired {}
    impl BlendRequired for BlendUnset {}
    impl BlendIsUnset for BlendUnset {}
    impl BlendRequired for BlendSet {}
    impl BlendSet {
        fn get(self) -> Option<wgpu::BlendState> {
            self.0
        }
    }
    pub struct WriteMaskSet(pub wgpu::ColorWrites);
    pub struct WriteMaskUnset(PhantomData<wgpu::ColorWrites>);
    pub trait WriteMaskRequired {}
    pub trait WriteMaskIsUnset: WriteMaskRequired {}
    impl WriteMaskRequired for WriteMaskUnset {}
    impl WriteMaskIsUnset for WriteMaskUnset {}
    impl WriteMaskRequired for WriteMaskSet {}
    impl WriteMaskSet {
        fn get(self) -> wgpu::ColorWrites {
            self.0
        }
    }
}

pub mod dx_12_backend_options_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ShaderCompilerSet(pub wgpu::Dx12Compiler);
    pub struct ShaderCompilerUnset(PhantomData<wgpu::Dx12Compiler>);
    pub trait ShaderCompilerRequired {}
    pub trait ShaderCompilerIsUnset: ShaderCompilerRequired {}
    impl ShaderCompilerRequired for ShaderCompilerUnset {}
    impl ShaderCompilerIsUnset for ShaderCompilerUnset {}
    impl ShaderCompilerRequired for ShaderCompilerSet {}
    impl ShaderCompilerSet {
        fn get(self) -> wgpu::Dx12Compiler {
            self.0
        }
    }
    pub struct PresentationSystemSet(pub wgpu::wgt::Dx12SwapchainKind);
    pub struct PresentationSystemUnset(PhantomData<wgpu::wgt::Dx12SwapchainKind>);
    pub trait PresentationSystemRequired {}
    pub trait PresentationSystemIsUnset: PresentationSystemRequired {}
    impl PresentationSystemRequired for PresentationSystemUnset {}
    impl PresentationSystemIsUnset for PresentationSystemUnset {}
    impl PresentationSystemRequired for PresentationSystemSet {}
    impl PresentationSystemSet {
        fn get(self) -> wgpu::wgt::Dx12SwapchainKind {
            self.0
        }
    }
    pub struct LatencyWaitableObjectSet(pub wgpu::wgt::Dx12UseFrameLatencyWaitableObject);
    pub struct LatencyWaitableObjectUnset(
        PhantomData<wgpu::wgt::Dx12UseFrameLatencyWaitableObject>,
    );
    pub trait LatencyWaitableObjectRequired {}
    pub trait LatencyWaitableObjectIsUnset: LatencyWaitableObjectRequired {}
    impl LatencyWaitableObjectRequired for LatencyWaitableObjectUnset {}
    impl LatencyWaitableObjectIsUnset for LatencyWaitableObjectUnset {}
    impl LatencyWaitableObjectRequired for LatencyWaitableObjectSet {}
    impl LatencyWaitableObjectSet {
        fn get(self) -> wgpu::wgt::Dx12UseFrameLatencyWaitableObject {
            self.0
        }
    }
}

pub mod copy_external_image_dest_info_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct TextureSet<T>(pub T);
    pub struct TextureUnset<T>(PhantomData<T>);
    pub trait TextureRequired {}
    pub trait TextureIsUnset: TextureRequired {}
    impl<T> TextureRequired for TextureUnset<T> {}
    impl<T> TextureIsUnset for TextureUnset<T> {}
    impl<T> TextureRequired for TextureSet<T> {}
    impl<T> TextureSet<T> {
        fn get(self) -> T {
            self.0
        }
    }
    pub struct MipLevelSet(pub u32);
    pub struct MipLevelUnset(PhantomData<u32>);
    pub trait MipLevelRequired {}
    pub trait MipLevelIsUnset: MipLevelRequired {}
    impl MipLevelRequired for MipLevelUnset {}
    impl MipLevelIsUnset for MipLevelUnset {}
    impl MipLevelRequired for MipLevelSet {}
    impl MipLevelSet {
        fn get(self) -> u32 {
            self.0
        }
    }
    pub struct OriginSet(pub wgpu::Origin3d);
    pub struct OriginUnset(PhantomData<wgpu::Origin3d>);
    pub trait OriginRequired {}
    pub trait OriginIsUnset: OriginRequired {}
    impl OriginRequired for OriginUnset {}
    impl OriginIsUnset for OriginUnset {}
    impl OriginRequired for OriginSet {}
    impl OriginSet {
        fn get(self) -> wgpu::Origin3d {
            self.0
        }
    }
    pub struct AspectSet(pub wgpu::TextureAspect);
    pub struct AspectUnset(PhantomData<wgpu::TextureAspect>);
    pub trait AspectRequired {}
    pub trait AspectIsUnset: AspectRequired {}
    impl AspectRequired for AspectUnset {}
    impl AspectIsUnset for AspectUnset {}
    impl AspectRequired for AspectSet {}
    impl AspectSet {
        fn get(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    pub struct ColorSpaceSet(pub wgpu::PredefinedColorSpace);
    pub struct ColorSpaceUnset(PhantomData<wgpu::PredefinedColorSpace>);
    pub trait ColorSpaceRequired {}
    pub trait ColorSpaceIsUnset: ColorSpaceRequired {}
    impl ColorSpaceRequired for ColorSpaceUnset {}
    impl ColorSpaceIsUnset for ColorSpaceUnset {}
    impl ColorSpaceRequired for ColorSpaceSet {}
    impl ColorSpaceSet {
        fn get(self) -> wgpu::PredefinedColorSpace {
            self.0
        }
    }
    pub struct PremultipliedAlphaSet(pub bool);
    pub struct PremultipliedAlphaUnset(PhantomData<bool>);
    pub trait PremultipliedAlphaRequired {}
    pub trait PremultipliedAlphaIsUnset: PremultipliedAlphaRequired {}
    impl PremultipliedAlphaRequired for PremultipliedAlphaUnset {}
    impl PremultipliedAlphaIsUnset for PremultipliedAlphaUnset {}
    impl PremultipliedAlphaRequired for PremultipliedAlphaSet {}
    impl PremultipliedAlphaSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod blas_build_entry_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BlasSet<'a>(pub &'a wgpu::Blas);
    pub struct BlasUnset<'a>(PhantomData<&'a wgpu::Blas>);
    pub trait BlasRequired {}
    pub trait BlasIsUnset: BlasRequired {}
    impl<'a> BlasRequired for BlasUnset<'a> {}
    impl<'a> BlasIsUnset for BlasUnset<'a> {}
    impl<'a> BlasRequired for BlasSet<'a> {}
    impl<'a> BlasSet<'a> {
        fn get(self) -> &'a wgpu::Blas {
            self.0
        }
    }
    pub struct GeometrySet<'a>(pub wgpu::BlasGeometries<'a>);
    pub struct GeometryUnset<'a>(PhantomData<wgpu::BlasGeometries<'a>>);
    pub trait GeometryRequired {}
    pub trait GeometryIsUnset: GeometryRequired {}
    impl<'a> GeometryRequired for GeometryUnset<'a> {}
    impl<'a> GeometryIsUnset for GeometryUnset<'a> {}
    impl<'a> GeometryRequired for GeometrySet<'a> {}
    impl<'a> GeometrySet<'a> {
        fn get(self) -> wgpu::BlasGeometries<'a> {
            self.0
        }
    }
}

pub mod command_buffer_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<L>(pub L);
    pub struct LabelUnset<L>(PhantomData<L>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<L> LabelRequired for LabelUnset<L> {}
    impl<L> LabelIsUnset for LabelUnset<L> {}
    impl<L> LabelRequired for LabelSet<L> {}
    impl<L> LabelSet<L> {
        fn get(self) -> L {
            self.0
        }
    }
}

pub mod vertex_buffer_layout_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct ArrayStrideSet(pub wgpu::BufferAddress);
    pub struct ArrayStrideUnset(PhantomData<wgpu::BufferAddress>);
    pub trait ArrayStrideRequired {}
    pub trait ArrayStrideIsUnset: ArrayStrideRequired {}
    impl ArrayStrideRequired for ArrayStrideUnset {}
    impl ArrayStrideIsUnset for ArrayStrideUnset {}
    impl ArrayStrideRequired for ArrayStrideSet {}
    impl ArrayStrideSet {
        fn get(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct StepModeSet(pub wgpu::VertexStepMode);
    pub struct StepModeUnset(PhantomData<wgpu::VertexStepMode>);
    pub trait StepModeRequired {}
    pub trait StepModeIsUnset: StepModeRequired {}
    impl StepModeRequired for StepModeUnset {}
    impl StepModeIsUnset for StepModeUnset {}
    impl StepModeRequired for StepModeSet {}
    impl StepModeSet {
        fn get(self) -> wgpu::VertexStepMode {
            self.0
        }
    }
    pub struct AttributesSet<'a>(pub &'a [wgpu::VertexAttribute]);
    pub struct AttributesUnset<'a>(PhantomData<&'a [wgpu::VertexAttribute]>);
    pub trait AttributesRequired {}
    pub trait AttributesIsUnset: AttributesRequired {}
    impl<'a> AttributesRequired for AttributesUnset<'a> {}
    impl<'a> AttributesIsUnset for AttributesUnset<'a> {}
    impl<'a> AttributesRequired for AttributesSet<'a> {}
    impl<'a> AttributesSet<'a> {
        fn get(self) -> &'a [wgpu::VertexAttribute] {
            self.0
        }
    }
}

pub mod compute_pipeline_descriptor_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct LabelSet<'a>(pub wgpu::Label<'a>);
    pub struct LabelUnset<'a>(PhantomData<wgpu::Label<'a>>);
    pub trait LabelRequired {}
    pub trait LabelIsUnset: LabelRequired {}
    impl<'a> LabelRequired for LabelUnset<'a> {}
    impl<'a> LabelIsUnset for LabelUnset<'a> {}
    impl<'a> LabelRequired for LabelSet<'a> {}
    impl<'a> LabelSet<'a> {
        fn get(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct LayoutSet<'a>(pub Option<&'a wgpu::PipelineLayout>);
    pub struct LayoutUnset<'a>(PhantomData<Option<&'a wgpu::PipelineLayout>>);
    pub trait LayoutRequired {}
    pub trait LayoutIsUnset: LayoutRequired {}
    impl<'a> LayoutRequired for LayoutUnset<'a> {}
    impl<'a> LayoutIsUnset for LayoutUnset<'a> {}
    impl<'a> LayoutRequired for LayoutSet<'a> {}
    impl<'a> LayoutSet<'a> {
        fn get(self) -> Option<&'a wgpu::PipelineLayout> {
            self.0
        }
    }
    pub struct ModuleSet<'a>(pub &'a wgpu::ShaderModule);
    pub struct ModuleUnset<'a>(PhantomData<&'a wgpu::ShaderModule>);
    pub trait ModuleRequired {}
    pub trait ModuleIsUnset: ModuleRequired {}
    impl<'a> ModuleRequired for ModuleUnset<'a> {}
    impl<'a> ModuleIsUnset for ModuleUnset<'a> {}
    impl<'a> ModuleRequired for ModuleSet<'a> {}
    impl<'a> ModuleSet<'a> {
        fn get(self) -> &'a wgpu::ShaderModule {
            self.0
        }
    }
    pub struct EntryPointSet<'a>(pub Option<&'a str>);
    pub struct EntryPointUnset<'a>(PhantomData<Option<&'a str>>);
    pub trait EntryPointRequired {}
    pub trait EntryPointIsUnset: EntryPointRequired {}
    impl<'a> EntryPointRequired for EntryPointUnset<'a> {}
    impl<'a> EntryPointIsUnset for EntryPointUnset<'a> {}
    impl<'a> EntryPointRequired for EntryPointSet<'a> {}
    impl<'a> EntryPointSet<'a> {
        fn get(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct CompilationOptionsSet<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    pub struct CompilationOptionsUnset<'a>(PhantomData<wgpu::PipelineCompilationOptions<'a>>);
    pub trait CompilationOptionsRequired {}
    pub trait CompilationOptionsIsUnset: CompilationOptionsRequired {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsIsUnset for CompilationOptionsUnset<'a> {}
    impl<'a> CompilationOptionsRequired for CompilationOptionsSet<'a> {}
    impl<'a> CompilationOptionsSet<'a> {
        fn get(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    pub struct CacheSet<'a>(pub Option<&'a wgpu::PipelineCache>);
    pub struct CacheUnset<'a>(PhantomData<Option<&'a wgpu::PipelineCache>>);
    pub trait CacheRequired {}
    pub trait CacheIsUnset: CacheRequired {}
    impl<'a> CacheRequired for CacheUnset<'a> {}
    impl<'a> CacheIsUnset for CacheUnset<'a> {}
    impl<'a> CacheRequired for CacheSet<'a> {}
    impl<'a> CacheSet<'a> {
        fn get(self) -> Option<&'a wgpu::PipelineCache> {
            self.0
        }
    }
}

pub mod noop_backend_options_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct EnableSet(pub bool);
    pub struct EnableUnset(PhantomData<bool>);
    pub trait EnableRequired {}
    pub trait EnableIsUnset: EnableRequired {}
    impl EnableRequired for EnableUnset {}
    impl EnableIsUnset for EnableUnset {}
    impl EnableRequired for EnableSet {}
    impl EnableSet {
        fn get(self) -> bool {
            self.0
        }
    }
}

pub mod buffer_transition_builder {
    use std::{borrow::Cow, marker::PhantomData, num::NonZeroU32, ops::Range};
    pub struct BufferSet<T>(pub T);
    pub struct BufferUnset<T>(PhantomData<T>);
    pub trait BufferRequired {}
    pub trait BufferIsUnset: BufferRequired {}
    impl<T> BufferRequired for BufferUnset<T> {}
    impl<T> BufferIsUnset for BufferUnset<T> {}
    impl<T> BufferRequired for BufferSet<T> {}
    impl<T> BufferSet<T> {
        fn get(self) -> T {
            self.0
        }
    }
    pub struct StateSet(pub wgpu::BufferUses);
    pub struct StateUnset(PhantomData<wgpu::BufferUses>);
    pub trait StateRequired {}
    pub trait StateIsUnset: StateRequired {}
    impl StateRequired for StateUnset {}
    impl StateIsUnset for StateUnset {}
    impl StateRequired for StateSet {}
    impl StateSet {
        fn get(self) -> wgpu::BufferUses {
            self.0
        }
    }
}
