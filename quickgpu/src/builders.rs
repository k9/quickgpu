use crate::*;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ops::Range;

#[derive(Debug)]
pub struct RequestAdapterOptionsBaseBuilder<
    PowerPreferenceField,
    ForceFallbackAdapterField,
    CompatibleSurfaceField,
> {
    power_preference: PowerPreferenceField,
    force_fallback_adapter: ForceFallbackAdapterField,
    compatible_surface: CompatibleSurfaceField,
}
pub fn request_adapter_options_base<S>()
-> RequestAdapterOptionsBaseBuilder<Unset<wgpu::PowerPreference>, Unset<bool>, Unset<Option<S>>> {
    RequestAdapterOptionsBaseBuilder {
        power_preference: Unset(PhantomData),
        force_fallback_adapter: Unset(PhantomData),
        compatible_surface: Unset(PhantomData),
    }
}

impl<PowerPreferenceField, T0, T1>
    RequestAdapterOptionsBaseBuilder<Unset<PowerPreferenceField>, T0, T1>
{
    pub fn power_preference(
        self,
        power_preference: wgpu::PowerPreference,
    ) -> RequestAdapterOptionsBaseBuilder<Set<wgpu::PowerPreference>, T0, T1> {
        RequestAdapterOptionsBaseBuilder {
            power_preference: Set(power_preference),
            force_fallback_adapter: self.force_fallback_adapter,
            compatible_surface: self.compatible_surface,
        }
    }
}
impl<S, PowerPreferenceField, ForceFallbackAdapterField, CompatibleSurfaceField>
    RequestAdapterOptionsBaseBuilder<
        PowerPreferenceField,
        Unset<ForceFallbackAdapterField>,
        CompatibleSurfaceField,
    >
{
    pub fn force_fallback_adapter(
        self,
        force_fallback_adapter: bool,
    ) -> RequestAdapterOptionsBaseBuilder<
        PowerPreferenceField,
        Set<ForceFallbackAdapterField>,
        CompatibleSurfaceField,
    > {
        RequestAdapterOptionsBaseBuilder {
            power_preference: Unset(PhantomData),
            force_fallback_adapter: Unset(PhantomData),
            compatible_surface: Unset(PhantomData),
        }
    }
}
impl<S, PowerPreferenceField, ForceFallbackAdapterField, CompatibleSurfaceField>
    RequestAdapterOptionsBaseBuilder<
        PowerPreferenceField,
        ForceFallbackAdapterField,
        Unset<CompatibleSurfaceField>,
    >
{
    pub fn compatible_surface(
        self,
        compatible_surface: Option<S>,
    ) -> RequestAdapterOptionsBaseBuilder<
        PowerPreferenceField,
        ForceFallbackAdapterField,
        Set<CompatibleSurfaceField>,
    > {
        RequestAdapterOptionsBaseBuilder {
            power_preference: Unset(PhantomData),
            force_fallback_adapter: Unset(PhantomData),
            compatible_surface: Unset(PhantomData),
        }
    }
}
impl<S> RequestAdapterOptionsBaseBuilder<S> {
    pub fn build(self) -> wgpu::RequestAdapterOptionsBase {
        wgpu::RequestAdapterOptionsBase {}
    }
}

#[derive(Debug)]
pub struct TexelCopyBufferLayoutBuilder<OffsetField, BytesPerRowField, RowsPerImageField> {
    offset: OffsetField,
    bytes_per_row: BytesPerRowField,
    rows_per_image: RowsPerImageField,
}
pub fn texel_copy_buffer_layout()
-> TexelCopyBufferLayoutBuilder<Unset<wgpu::BufferAddress>, Unset<Option<u32>>, Unset<Option<u32>>>
{
    TexelCopyBufferLayoutBuilder {
        offset: Unset(PhantomData),
        bytes_per_row: Unset(PhantomData),
        rows_per_image: Unset(PhantomData),
    }
}
impl<OffsetField, BytesPerRowField, RowsPerImageField>
    TexelCopyBufferLayoutBuilder<Unset<OffsetField>, BytesPerRowField, RowsPerImageField>
{
    pub fn offset(
        self,
        offset: wgpu::BufferAddress,
    ) -> TexelCopyBufferLayoutBuilder<Set<OffsetField>, BytesPerRowField, RowsPerImageField> {
        TexelCopyBufferLayoutBuilder {
            offset: Unset(PhantomData),
            bytes_per_row: Unset(PhantomData),
            rows_per_image: Unset(PhantomData),
        }
    }
}
impl<OffsetField, BytesPerRowField, RowsPerImageField>
    TexelCopyBufferLayoutBuilder<OffsetField, Unset<BytesPerRowField>, RowsPerImageField>
{
    pub fn bytes_per_row(
        self,
        bytes_per_row: Option<u32>,
    ) -> TexelCopyBufferLayoutBuilder<OffsetField, Set<BytesPerRowField>, RowsPerImageField> {
        TexelCopyBufferLayoutBuilder {
            offset: Unset(PhantomData),
            bytes_per_row: Unset(PhantomData),
            rows_per_image: Unset(PhantomData),
        }
    }
}
impl<OffsetField, BytesPerRowField, RowsPerImageField>
    TexelCopyBufferLayoutBuilder<OffsetField, BytesPerRowField, Unset<RowsPerImageField>>
{
    pub fn rows_per_image(
        self,
        rows_per_image: Option<u32>,
    ) -> TexelCopyBufferLayoutBuilder<OffsetField, BytesPerRowField, Set<RowsPerImageField>> {
        TexelCopyBufferLayoutBuilder {
            offset: Unset(PhantomData),
            bytes_per_row: Unset(PhantomData),
            rows_per_image: Unset(PhantomData),
        }
    }
}
impl TexelCopyBufferLayoutBuilder {
    pub fn build(self) -> wgpu::TexelCopyBufferLayout {
        wgpu::TexelCopyBufferLayout {}
    }
}

#[derive(Debug)]
pub struct ShaderModuleDescriptorBuilder<LabelField, SourceField> {
    label: LabelField,
    source: SourceField,
}
pub fn shader_module_descriptor<'a>()
-> ShaderModuleDescriptorBuilder<Unset<wgpu::Label<'a>>, Unset<wgpu::ShaderSource<'a>>> {
    ShaderModuleDescriptorBuilder {
        label: Unset(PhantomData),
        source: Unset(PhantomData),
    }
}
impl<'a, LabelField, SourceField> ShaderModuleDescriptorBuilder<Unset<LabelField>, SourceField> {
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> ShaderModuleDescriptorBuilder<Set<LabelField>, SourceField> {
        ShaderModuleDescriptorBuilder {
            label: Unset(PhantomData),
            source: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, SourceField> ShaderModuleDescriptorBuilder<LabelField, Unset<SourceField>> {
    pub fn source(
        self,
        source: wgpu::ShaderSource<'a>,
    ) -> ShaderModuleDescriptorBuilder<LabelField, Set<SourceField>> {
        ShaderModuleDescriptorBuilder {
            label: Unset(PhantomData),
            source: Unset(PhantomData),
        }
    }
}
impl<'a> ShaderModuleDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::ShaderModuleDescriptor {
        wgpu::ShaderModuleDescriptor {}
    }
}

#[derive(Debug)]
pub struct BufferDescriptorBuilder<LabelField, SizeField, UsageField, MappedAtCreationField> {
    label: LabelField,
    size: SizeField,
    usage: UsageField,
    mapped_at_creation: MappedAtCreationField,
}
pub fn buffer_descriptor<'a>() -> BufferDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<wgpu::BufferAddress>,
    Unset<wgpu::BufferUsages>,
    Unset<bool>,
> {
    BufferDescriptorBuilder {
        label: Unset(PhantomData),
        size: Unset(PhantomData),
        usage: Unset(PhantomData),
        mapped_at_creation: Unset(PhantomData),
    }
}
impl<'a, LabelField, SizeField, UsageField, MappedAtCreationField>
    BufferDescriptorBuilder<Unset<LabelField>, SizeField, UsageField, MappedAtCreationField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> BufferDescriptorBuilder<Set<LabelField>, SizeField, UsageField, MappedAtCreationField>
    {
        BufferDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            usage: Unset(PhantomData),
            mapped_at_creation: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, SizeField, UsageField, MappedAtCreationField>
    BufferDescriptorBuilder<LabelField, Unset<SizeField>, UsageField, MappedAtCreationField>
{
    pub fn size(
        self,
        size: wgpu::BufferAddress,
    ) -> BufferDescriptorBuilder<LabelField, Set<SizeField>, UsageField, MappedAtCreationField>
    {
        BufferDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            usage: Unset(PhantomData),
            mapped_at_creation: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, SizeField, UsageField, MappedAtCreationField>
    BufferDescriptorBuilder<LabelField, SizeField, Unset<UsageField>, MappedAtCreationField>
{
    pub fn usage(
        self,
        usage: wgpu::BufferUsages,
    ) -> BufferDescriptorBuilder<LabelField, SizeField, Set<UsageField>, MappedAtCreationField>
    {
        BufferDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            usage: Unset(PhantomData),
            mapped_at_creation: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, SizeField, UsageField, MappedAtCreationField>
    BufferDescriptorBuilder<LabelField, SizeField, UsageField, Unset<MappedAtCreationField>>
{
    pub fn mapped_at_creation(
        self,
        mapped_at_creation: bool,
    ) -> BufferDescriptorBuilder<LabelField, SizeField, UsageField, Set<MappedAtCreationField>>
    {
        BufferDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            usage: Unset(PhantomData),
            mapped_at_creation: Unset(PhantomData),
        }
    }
}
impl<'a> BufferDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::BufferDescriptor {
        wgpu::BufferDescriptor {}
    }
}

#[derive(Debug)]
pub struct ShaderModuleDescriptorPassthroughBuilder<
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
> {
    entry_point: EntryPointField,
    label: LabelField,
    num_workgroups: NumWorkgroupsField,
    runtime_checks: RuntimeChecksField,
    spirv: SpirvField,
    dxil: DxilField,
    msl: MslField,
    hlsl: HlslField,
    glsl: GlslField,
    wgsl: WgslField,
}
pub fn shader_module_descriptor_passthrough<'a>() -> ShaderModuleDescriptorPassthroughBuilder<
    Unset<String>,
    Unset<wgpu::Label<'a>>,
    Unset<(u32, u32, u32)>,
    Unset<impl Nested<wgpu::ShaderRuntimeChecks>>,
    Unset<Option<Cow<'a, [u32]>>>,
    Unset<Option<Cow<'a, [u8]>>>,
    Unset<Option<Cow<'a, str>>>,
    Unset<Option<Cow<'a, str>>>,
    Unset<Option<Cow<'a, str>>>,
    Unset<Option<Cow<'a, str>>>,
> {
    ShaderModuleDescriptorPassthroughBuilder {
        entry_point: Unset(PhantomData),
        label: Unset(PhantomData),
        num_workgroups: Unset(PhantomData),
        runtime_checks: Unset(PhantomData),
        spirv: Unset(PhantomData),
        dxil: Unset(PhantomData),
        msl: Unset(PhantomData),
        hlsl: Unset(PhantomData),
        glsl: Unset(PhantomData),
        wgsl: Unset(PhantomData),
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        Unset<EntryPointField>,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn entry_point(
        self,
        entry_point: String,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        Set<EntryPointField>,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        Unset<LabelField>,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        Set<LabelField>,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        Unset<NumWorkgroupsField>,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn num_workgroups(
        self,
        num_workgroups: (u32, u32, u32),
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        Set<NumWorkgroupsField>,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        Unset<RuntimeChecksField>,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn runtime_checks(
        self,
        runtime_checks: impl Nested<wgpu::ShaderRuntimeChecks>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        Set<RuntimeChecksField>,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        Unset<SpirvField>,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn spirv(
        self,
        spirv: Option<Cow<'a, [u32]>>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        Set<SpirvField>,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        Unset<DxilField>,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn dxil(
        self,
        dxil: Option<Cow<'a, [u8]>>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        Set<DxilField>,
        MslField,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        Unset<MslField>,
        HlslField,
        GlslField,
        WgslField,
    >
{
    pub fn msl(
        self,
        msl: Option<Cow<'a, str>>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        Set<MslField>,
        HlslField,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        Unset<HlslField>,
        GlslField,
        WgslField,
    >
{
    pub fn hlsl(
        self,
        hlsl: Option<Cow<'a, str>>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        Set<HlslField>,
        GlslField,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        Unset<GlslField>,
        WgslField,
    >
{
    pub fn glsl(
        self,
        glsl: Option<Cow<'a, str>>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        Set<GlslField>,
        WgslField,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    EntryPointField,
    LabelField,
    NumWorkgroupsField,
    RuntimeChecksField,
    SpirvField,
    DxilField,
    MslField,
    HlslField,
    GlslField,
    WgslField,
>
    ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        Unset<WgslField>,
    >
{
    pub fn wgsl(
        self,
        wgsl: Option<Cow<'a, str>>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        EntryPointField,
        LabelField,
        NumWorkgroupsField,
        RuntimeChecksField,
        SpirvField,
        DxilField,
        MslField,
        HlslField,
        GlslField,
        Set<WgslField>,
    > {
        ShaderModuleDescriptorPassthroughBuilder {
            entry_point: Unset(PhantomData),
            label: Unset(PhantomData),
            num_workgroups: Unset(PhantomData),
            runtime_checks: Unset(PhantomData),
            spirv: Unset(PhantomData),
            dxil: Unset(PhantomData),
            msl: Unset(PhantomData),
            hlsl: Unset(PhantomData),
            glsl: Unset(PhantomData),
            wgsl: Unset(PhantomData),
        }
    }
}
impl<'a> ShaderModuleDescriptorPassthroughBuilder<'a> {
    pub fn build(self) -> wgpu::ShaderModuleDescriptorPassthrough {
        wgpu::ShaderModuleDescriptorPassthrough {}
    }
}

#[derive(Debug)]
pub struct CreateBlasDescriptorBuilder<LabelField, FlagsField, UpdateModeField> {
    label: LabelField,
    flags: FlagsField,
    update_mode: UpdateModeField,
}
pub fn create_blas_descriptor<'a>() -> CreateBlasDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<wgpu::wgt::AccelerationStructureFlags>,
    Unset<wgpu::wgt::AccelerationStructureUpdateMode>,
> {
    CreateBlasDescriptorBuilder {
        label: Unset(PhantomData),
        flags: Unset(PhantomData),
        update_mode: Unset(PhantomData),
    }
}
impl<'a, LabelField, FlagsField, UpdateModeField>
    CreateBlasDescriptorBuilder<Unset<LabelField>, FlagsField, UpdateModeField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> CreateBlasDescriptorBuilder<Set<LabelField>, FlagsField, UpdateModeField> {
        CreateBlasDescriptorBuilder {
            label: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, FlagsField, UpdateModeField>
    CreateBlasDescriptorBuilder<LabelField, Unset<FlagsField>, UpdateModeField>
{
    pub fn flags(
        self,
        flags: wgpu::wgt::AccelerationStructureFlags,
    ) -> CreateBlasDescriptorBuilder<LabelField, Set<FlagsField>, UpdateModeField> {
        CreateBlasDescriptorBuilder {
            label: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, FlagsField, UpdateModeField>
    CreateBlasDescriptorBuilder<LabelField, FlagsField, Unset<UpdateModeField>>
{
    pub fn update_mode(
        self,
        update_mode: wgpu::wgt::AccelerationStructureUpdateMode,
    ) -> CreateBlasDescriptorBuilder<LabelField, FlagsField, Set<UpdateModeField>> {
        CreateBlasDescriptorBuilder {
            label: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a> CreateBlasDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::CreateBlasDescriptor {
        wgpu::CreateBlasDescriptor {}
    }
}

#[derive(Debug)]
pub struct ImageSubresourceRangeBuilder<
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
> {
    aspect: AspectField,
    base_mip_level: BaseMipLevelField,
    mip_level_count: MipLevelCountField,
    base_array_layer: BaseArrayLayerField,
    array_layer_count: ArrayLayerCountField,
}
pub fn image_subresource_range() -> ImageSubresourceRangeBuilder<
    Unset<wgpu::TextureAspect>,
    Unset<u32>,
    Unset<Option<u32>>,
    Unset<u32>,
    Unset<Option<u32>>,
> {
    ImageSubresourceRangeBuilder {
        aspect: Unset(PhantomData),
        base_mip_level: Unset(PhantomData),
        mip_level_count: Unset(PhantomData),
        base_array_layer: Unset(PhantomData),
        array_layer_count: Unset(PhantomData),
    }
}
impl<AspectField, BaseMipLevelField, MipLevelCountField, BaseArrayLayerField, ArrayLayerCountField>
    ImageSubresourceRangeBuilder<
        Unset<AspectField>,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn aspect(
        self,
        aspect: wgpu::TextureAspect,
    ) -> ImageSubresourceRangeBuilder<
        Set<AspectField>,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        ImageSubresourceRangeBuilder {
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<AspectField, BaseMipLevelField, MipLevelCountField, BaseArrayLayerField, ArrayLayerCountField>
    ImageSubresourceRangeBuilder<
        AspectField,
        Unset<BaseMipLevelField>,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn base_mip_level(
        self,
        base_mip_level: u32,
    ) -> ImageSubresourceRangeBuilder<
        AspectField,
        Set<BaseMipLevelField>,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        ImageSubresourceRangeBuilder {
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<AspectField, BaseMipLevelField, MipLevelCountField, BaseArrayLayerField, ArrayLayerCountField>
    ImageSubresourceRangeBuilder<
        AspectField,
        BaseMipLevelField,
        Unset<MipLevelCountField>,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn mip_level_count(
        self,
        mip_level_count: Option<u32>,
    ) -> ImageSubresourceRangeBuilder<
        AspectField,
        BaseMipLevelField,
        Set<MipLevelCountField>,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        ImageSubresourceRangeBuilder {
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<AspectField, BaseMipLevelField, MipLevelCountField, BaseArrayLayerField, ArrayLayerCountField>
    ImageSubresourceRangeBuilder<
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        Unset<BaseArrayLayerField>,
        ArrayLayerCountField,
    >
{
    pub fn base_array_layer(
        self,
        base_array_layer: u32,
    ) -> ImageSubresourceRangeBuilder<
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        Set<BaseArrayLayerField>,
        ArrayLayerCountField,
    > {
        ImageSubresourceRangeBuilder {
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<AspectField, BaseMipLevelField, MipLevelCountField, BaseArrayLayerField, ArrayLayerCountField>
    ImageSubresourceRangeBuilder<
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        Unset<ArrayLayerCountField>,
    >
{
    pub fn array_layer_count(
        self,
        array_layer_count: Option<u32>,
    ) -> ImageSubresourceRangeBuilder<
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        Set<ArrayLayerCountField>,
    > {
        ImageSubresourceRangeBuilder {
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl ImageSubresourceRangeBuilder {
    pub fn build(self) -> wgpu::ImageSubresourceRange {
        wgpu::ImageSubresourceRange {}
    }
}

#[derive(Debug)]
pub struct BindGroupDescriptorBuilder<LabelField, LayoutField, EntriesField> {
    label: LabelField,
    layout: LayoutField,
    entries: EntriesField,
}
pub fn bind_group_descriptor<'a>() -> BindGroupDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<&'a wgpu::BindGroupLayout>,
    Unset<&'a [wgpu::BindGroupEntry<'a>]>,
> {
    BindGroupDescriptorBuilder {
        label: Unset(PhantomData),
        layout: Unset(PhantomData),
        entries: Unset(PhantomData),
    }
}
impl<'a, LabelField, LayoutField, EntriesField>
    BindGroupDescriptorBuilder<Unset<LabelField>, LayoutField, EntriesField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> BindGroupDescriptorBuilder<Set<LabelField>, LayoutField, EntriesField> {
        BindGroupDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            entries: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, EntriesField>
    BindGroupDescriptorBuilder<LabelField, Unset<LayoutField>, EntriesField>
{
    pub fn layout(
        self,
        layout: &'a wgpu::BindGroupLayout,
    ) -> BindGroupDescriptorBuilder<LabelField, Set<LayoutField>, EntriesField> {
        BindGroupDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            entries: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, EntriesField>
    BindGroupDescriptorBuilder<LabelField, LayoutField, Unset<EntriesField>>
{
    pub fn entries(
        self,
        entries: &'a [wgpu::BindGroupEntry<'a>],
    ) -> BindGroupDescriptorBuilder<LabelField, LayoutField, Set<EntriesField>> {
        BindGroupDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            entries: Unset(PhantomData),
        }
    }
}
impl<'a> BindGroupDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::BindGroupDescriptor {
        wgpu::BindGroupDescriptor {}
    }
}

#[derive(Debug)]
pub struct Extent3DBuilder<WidthField, HeightField, DepthOrArrayLayersField> {
    width: WidthField,
    height: HeightField,
    depth_or_array_layers: DepthOrArrayLayersField,
}
pub fn extent_3_d() -> Extent3DBuilder<Unset<u32>, Unset<u32>, Unset<u32>> {
    Extent3DBuilder {
        width: Unset(PhantomData),
        height: Unset(PhantomData),
        depth_or_array_layers: Unset(PhantomData),
    }
}
impl<WidthField, HeightField, DepthOrArrayLayersField>
    Extent3DBuilder<Unset<WidthField>, HeightField, DepthOrArrayLayersField>
{
    pub fn width(
        self,
        width: u32,
    ) -> Extent3DBuilder<Set<WidthField>, HeightField, DepthOrArrayLayersField> {
        Extent3DBuilder {
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            depth_or_array_layers: Unset(PhantomData),
        }
    }
}
impl<WidthField, HeightField, DepthOrArrayLayersField>
    Extent3DBuilder<WidthField, Unset<HeightField>, DepthOrArrayLayersField>
{
    pub fn height(
        self,
        height: u32,
    ) -> Extent3DBuilder<WidthField, Set<HeightField>, DepthOrArrayLayersField> {
        Extent3DBuilder {
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            depth_or_array_layers: Unset(PhantomData),
        }
    }
}
impl<WidthField, HeightField, DepthOrArrayLayersField>
    Extent3DBuilder<WidthField, HeightField, Unset<DepthOrArrayLayersField>>
{
    pub fn depth_or_array_layers(
        self,
        depth_or_array_layers: u32,
    ) -> Extent3DBuilder<WidthField, HeightField, Set<DepthOrArrayLayersField>> {
        Extent3DBuilder {
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            depth_or_array_layers: Unset(PhantomData),
        }
    }
}
impl Extent3DBuilder {
    pub fn build(self) -> wgpu::Extent3d {
        wgpu::Extent3d {}
    }
}

#[derive(Debug)]
pub struct CommandEncoderDescriptorBuilder<LabelField> {
    label: LabelField,
}
pub fn command_encoder_descriptor<'a>() -> CommandEncoderDescriptorBuilder<Unset<wgpu::Label<'a>>> {
    CommandEncoderDescriptorBuilder {
        label: Unset(PhantomData),
    }
}
impl<'a, LabelField> CommandEncoderDescriptorBuilder<Unset<LabelField>> {
    pub fn label(self, label: wgpu::Label<'a>) -> CommandEncoderDescriptorBuilder<Set<LabelField>> {
        CommandEncoderDescriptorBuilder {
            label: Unset(PhantomData),
        }
    }
}
impl<'a> CommandEncoderDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::CommandEncoderDescriptor {
        wgpu::CommandEncoderDescriptor {}
    }
}

#[derive(Debug)]
pub struct SamplerDescriptorBuilder<
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
> {
    label: LabelField,
    address_mode_u: AddressModeUField,
    address_mode_v: AddressModeVField,
    address_mode_w: AddressModeWField,
    mag_filter: MagFilterField,
    min_filter: MinFilterField,
    mipmap_filter: MipmapFilterField,
    lod_min_clamp: LodMinClampField,
    lod_max_clamp: LodMaxClampField,
    compare: CompareField,
    anisotropy_clamp: AnisotropyClampField,
    border_color: BorderColorField,
}
pub fn sampler_descriptor<'a>() -> SamplerDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<wgpu::AddressMode>,
    Unset<wgpu::AddressMode>,
    Unset<wgpu::AddressMode>,
    Unset<wgpu::FilterMode>,
    Unset<wgpu::FilterMode>,
    Unset<wgpu::FilterMode>,
    Unset<f32>,
    Unset<f32>,
    Unset<Option<wgpu::CompareFunction>>,
    Unset<u16>,
    Unset<Option<wgpu::SamplerBorderColor>>,
> {
    SamplerDescriptorBuilder {
        label: Unset(PhantomData),
        address_mode_u: Unset(PhantomData),
        address_mode_v: Unset(PhantomData),
        address_mode_w: Unset(PhantomData),
        mag_filter: Unset(PhantomData),
        min_filter: Unset(PhantomData),
        mipmap_filter: Unset(PhantomData),
        lod_min_clamp: Unset(PhantomData),
        lod_max_clamp: Unset(PhantomData),
        compare: Unset(PhantomData),
        anisotropy_clamp: Unset(PhantomData),
        border_color: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        Unset<LabelField>,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> SamplerDescriptorBuilder<
        Set<LabelField>,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        Unset<AddressModeUField>,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn address_mode_u(
        self,
        address_mode_u: wgpu::AddressMode,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        Set<AddressModeUField>,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        Unset<AddressModeVField>,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn address_mode_v(
        self,
        address_mode_v: wgpu::AddressMode,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        Set<AddressModeVField>,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        Unset<AddressModeWField>,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn address_mode_w(
        self,
        address_mode_w: wgpu::AddressMode,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        Set<AddressModeWField>,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        Unset<MagFilterField>,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn mag_filter(
        self,
        mag_filter: wgpu::FilterMode,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        Set<MagFilterField>,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        Unset<MinFilterField>,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn min_filter(
        self,
        min_filter: wgpu::FilterMode,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        Set<MinFilterField>,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        Unset<MipmapFilterField>,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn mipmap_filter(
        self,
        mipmap_filter: wgpu::FilterMode,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        Set<MipmapFilterField>,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        Unset<LodMinClampField>,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn lod_min_clamp(
        self,
        lod_min_clamp: f32,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        Set<LodMinClampField>,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        Unset<LodMaxClampField>,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn lod_max_clamp(
        self,
        lod_max_clamp: f32,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        Set<LodMaxClampField>,
        CompareField,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        Unset<CompareField>,
        AnisotropyClampField,
        BorderColorField,
    >
{
    pub fn compare(
        self,
        compare: Option<wgpu::CompareFunction>,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        Set<CompareField>,
        AnisotropyClampField,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        Unset<AnisotropyClampField>,
        BorderColorField,
    >
{
    pub fn anisotropy_clamp(
        self,
        anisotropy_clamp: u16,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        Set<AnisotropyClampField>,
        BorderColorField,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    AddressModeUField,
    AddressModeVField,
    AddressModeWField,
    MagFilterField,
    MinFilterField,
    MipmapFilterField,
    LodMinClampField,
    LodMaxClampField,
    CompareField,
    AnisotropyClampField,
    BorderColorField,
>
    SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        Unset<BorderColorField>,
    >
{
    pub fn border_color(
        self,
        border_color: Option<wgpu::SamplerBorderColor>,
    ) -> SamplerDescriptorBuilder<
        LabelField,
        AddressModeUField,
        AddressModeVField,
        AddressModeWField,
        MagFilterField,
        MinFilterField,
        MipmapFilterField,
        LodMinClampField,
        LodMaxClampField,
        CompareField,
        AnisotropyClampField,
        Set<BorderColorField>,
    > {
        SamplerDescriptorBuilder {
            label: Unset(PhantomData),
            address_mode_u: Unset(PhantomData),
            address_mode_v: Unset(PhantomData),
            address_mode_w: Unset(PhantomData),
            mag_filter: Unset(PhantomData),
            min_filter: Unset(PhantomData),
            mipmap_filter: Unset(PhantomData),
            lod_min_clamp: Unset(PhantomData),
            lod_max_clamp: Unset(PhantomData),
            compare: Unset(PhantomData),
            anisotropy_clamp: Unset(PhantomData),
            border_color: Unset(PhantomData),
        }
    }
}
impl<'a> SamplerDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::SamplerDescriptor {
        wgpu::SamplerDescriptor {}
    }
}

#[derive(Debug)]
pub struct DepthStencilStateBuilder<
    FormatField,
    DepthWriteEnabledField,
    DepthCompareField,
    StencilField,
    BiasField,
> {
    format: FormatField,
    depth_write_enabled: DepthWriteEnabledField,
    depth_compare: DepthCompareField,
    stencil: StencilField,
    bias: BiasField,
}
pub fn depth_stencil_state() -> DepthStencilStateBuilder<
    Unset<wgpu::TextureFormat>,
    Unset<bool>,
    Unset<wgpu::CompareFunction>,
    Unset<impl Nested<wgpu::StencilState>>,
    Unset<impl Nested<wgpu::DepthBiasState>>,
> {
    DepthStencilStateBuilder {
        format: Unset(PhantomData),
        depth_write_enabled: Unset(PhantomData),
        depth_compare: Unset(PhantomData),
        stencil: Unset(PhantomData),
        bias: Unset(PhantomData),
    }
}
impl<FormatField, DepthWriteEnabledField, DepthCompareField, StencilField, BiasField>
    DepthStencilStateBuilder<
        Unset<FormatField>,
        DepthWriteEnabledField,
        DepthCompareField,
        StencilField,
        BiasField,
    >
{
    pub fn format(
        self,
        format: wgpu::TextureFormat,
    ) -> DepthStencilStateBuilder<
        Set<FormatField>,
        DepthWriteEnabledField,
        DepthCompareField,
        StencilField,
        BiasField,
    > {
        DepthStencilStateBuilder {
            format: Unset(PhantomData),
            depth_write_enabled: Unset(PhantomData),
            depth_compare: Unset(PhantomData),
            stencil: Unset(PhantomData),
            bias: Unset(PhantomData),
        }
    }
}
impl<FormatField, DepthWriteEnabledField, DepthCompareField, StencilField, BiasField>
    DepthStencilStateBuilder<
        FormatField,
        Unset<DepthWriteEnabledField>,
        DepthCompareField,
        StencilField,
        BiasField,
    >
{
    pub fn depth_write_enabled(
        self,
        depth_write_enabled: bool,
    ) -> DepthStencilStateBuilder<
        FormatField,
        Set<DepthWriteEnabledField>,
        DepthCompareField,
        StencilField,
        BiasField,
    > {
        DepthStencilStateBuilder {
            format: Unset(PhantomData),
            depth_write_enabled: Unset(PhantomData),
            depth_compare: Unset(PhantomData),
            stencil: Unset(PhantomData),
            bias: Unset(PhantomData),
        }
    }
}
impl<FormatField, DepthWriteEnabledField, DepthCompareField, StencilField, BiasField>
    DepthStencilStateBuilder<
        FormatField,
        DepthWriteEnabledField,
        Unset<DepthCompareField>,
        StencilField,
        BiasField,
    >
{
    pub fn depth_compare(
        self,
        depth_compare: wgpu::CompareFunction,
    ) -> DepthStencilStateBuilder<
        FormatField,
        DepthWriteEnabledField,
        Set<DepthCompareField>,
        StencilField,
        BiasField,
    > {
        DepthStencilStateBuilder {
            format: Unset(PhantomData),
            depth_write_enabled: Unset(PhantomData),
            depth_compare: Unset(PhantomData),
            stencil: Unset(PhantomData),
            bias: Unset(PhantomData),
        }
    }
}
impl<FormatField, DepthWriteEnabledField, DepthCompareField, StencilField, BiasField>
    DepthStencilStateBuilder<
        FormatField,
        DepthWriteEnabledField,
        DepthCompareField,
        Unset<StencilField>,
        BiasField,
    >
{
    pub fn stencil(
        self,
        stencil: impl Nested<wgpu::StencilState>,
    ) -> DepthStencilStateBuilder<
        FormatField,
        DepthWriteEnabledField,
        DepthCompareField,
        Set<StencilField>,
        BiasField,
    > {
        DepthStencilStateBuilder {
            format: Unset(PhantomData),
            depth_write_enabled: Unset(PhantomData),
            depth_compare: Unset(PhantomData),
            stencil: Unset(PhantomData),
            bias: Unset(PhantomData),
        }
    }
}
impl<FormatField, DepthWriteEnabledField, DepthCompareField, StencilField, BiasField>
    DepthStencilStateBuilder<
        FormatField,
        DepthWriteEnabledField,
        DepthCompareField,
        StencilField,
        Unset<BiasField>,
    >
{
    pub fn bias(
        self,
        bias: impl Nested<wgpu::DepthBiasState>,
    ) -> DepthStencilStateBuilder<
        FormatField,
        DepthWriteEnabledField,
        DepthCompareField,
        StencilField,
        Set<BiasField>,
    > {
        DepthStencilStateBuilder {
            format: Unset(PhantomData),
            depth_write_enabled: Unset(PhantomData),
            depth_compare: Unset(PhantomData),
            stencil: Unset(PhantomData),
            bias: Unset(PhantomData),
        }
    }
}
impl DepthStencilStateBuilder {
    pub fn build(self) -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {}
    }
}

#[derive(Debug)]
pub struct DepthBiasStateBuilder<ConstantField, SlopeScaleField, ClampField> {
    constant: ConstantField,
    slope_scale: SlopeScaleField,
    clamp: ClampField,
}
pub fn depth_bias_state() -> DepthBiasStateBuilder<Unset<i32>, Unset<f32>, Unset<f32>> {
    DepthBiasStateBuilder {
        constant: Unset(PhantomData),
        slope_scale: Unset(PhantomData),
        clamp: Unset(PhantomData),
    }
}
impl<ConstantField, SlopeScaleField, ClampField>
    DepthBiasStateBuilder<Unset<ConstantField>, SlopeScaleField, ClampField>
{
    pub fn constant(
        self,
        constant: i32,
    ) -> DepthBiasStateBuilder<Set<ConstantField>, SlopeScaleField, ClampField> {
        DepthBiasStateBuilder {
            constant: Unset(PhantomData),
            slope_scale: Unset(PhantomData),
            clamp: Unset(PhantomData),
        }
    }
}
impl<ConstantField, SlopeScaleField, ClampField>
    DepthBiasStateBuilder<ConstantField, Unset<SlopeScaleField>, ClampField>
{
    pub fn slope_scale(
        self,
        slope_scale: f32,
    ) -> DepthBiasStateBuilder<ConstantField, Set<SlopeScaleField>, ClampField> {
        DepthBiasStateBuilder {
            constant: Unset(PhantomData),
            slope_scale: Unset(PhantomData),
            clamp: Unset(PhantomData),
        }
    }
}
impl<ConstantField, SlopeScaleField, ClampField>
    DepthBiasStateBuilder<ConstantField, SlopeScaleField, Unset<ClampField>>
{
    pub fn clamp(
        self,
        clamp: f32,
    ) -> DepthBiasStateBuilder<ConstantField, SlopeScaleField, Set<ClampField>> {
        DepthBiasStateBuilder {
            constant: Unset(PhantomData),
            slope_scale: Unset(PhantomData),
            clamp: Unset(PhantomData),
        }
    }
}
impl DepthBiasStateBuilder {
    pub fn build(self) -> wgpu::DepthBiasState {
        wgpu::DepthBiasState {}
    }
}

#[derive(Debug)]
pub struct PipelineCacheDescriptorBuilder<LabelField, DataField, FallbackField> {
    label: LabelField,
    data: DataField,
    fallback: FallbackField,
}
pub fn pipeline_cache_descriptor<'a>()
-> PipelineCacheDescriptorBuilder<Unset<wgpu::Label<'a>>, Unset<Option<&'a [u8]>>, Unset<bool>> {
    PipelineCacheDescriptorBuilder {
        label: Unset(PhantomData),
        data: Unset(PhantomData),
        fallback: Unset(PhantomData),
    }
}
impl<'a, LabelField, DataField, FallbackField>
    PipelineCacheDescriptorBuilder<Unset<LabelField>, DataField, FallbackField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> PipelineCacheDescriptorBuilder<Set<LabelField>, DataField, FallbackField> {
        PipelineCacheDescriptorBuilder {
            label: Unset(PhantomData),
            data: Unset(PhantomData),
            fallback: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, DataField, FallbackField>
    PipelineCacheDescriptorBuilder<LabelField, Unset<DataField>, FallbackField>
{
    pub fn data(
        self,
        data: Option<&'a [u8]>,
    ) -> PipelineCacheDescriptorBuilder<LabelField, Set<DataField>, FallbackField> {
        PipelineCacheDescriptorBuilder {
            label: Unset(PhantomData),
            data: Unset(PhantomData),
            fallback: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, DataField, FallbackField>
    PipelineCacheDescriptorBuilder<LabelField, DataField, Unset<FallbackField>>
{
    pub fn fallback(
        self,
        fallback: bool,
    ) -> PipelineCacheDescriptorBuilder<LabelField, DataField, Set<FallbackField>> {
        PipelineCacheDescriptorBuilder {
            label: Unset(PhantomData),
            data: Unset(PhantomData),
            fallback: Unset(PhantomData),
        }
    }
}
impl<'a> PipelineCacheDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::PipelineCacheDescriptor {
        wgpu::PipelineCacheDescriptor {}
    }
}

#[derive(Debug)]
pub struct VertexAttributeBuilder<FormatField, OffsetField, ShaderLocationField> {
    format: FormatField,
    offset: OffsetField,
    shader_location: ShaderLocationField,
}
pub fn vertex_attribute() -> VertexAttributeBuilder<
    Unset<wgpu::VertexFormat>,
    Unset<wgpu::BufferAddress>,
    Unset<wgpu::ShaderLocation>,
> {
    VertexAttributeBuilder {
        format: Unset(PhantomData),
        offset: Unset(PhantomData),
        shader_location: Unset(PhantomData),
    }
}
impl<FormatField, OffsetField, ShaderLocationField>
    VertexAttributeBuilder<Unset<FormatField>, OffsetField, ShaderLocationField>
{
    pub fn format(
        self,
        format: wgpu::VertexFormat,
    ) -> VertexAttributeBuilder<Set<FormatField>, OffsetField, ShaderLocationField> {
        VertexAttributeBuilder {
            format: Unset(PhantomData),
            offset: Unset(PhantomData),
            shader_location: Unset(PhantomData),
        }
    }
}
impl<FormatField, OffsetField, ShaderLocationField>
    VertexAttributeBuilder<FormatField, Unset<OffsetField>, ShaderLocationField>
{
    pub fn offset(
        self,
        offset: wgpu::BufferAddress,
    ) -> VertexAttributeBuilder<FormatField, Set<OffsetField>, ShaderLocationField> {
        VertexAttributeBuilder {
            format: Unset(PhantomData),
            offset: Unset(PhantomData),
            shader_location: Unset(PhantomData),
        }
    }
}
impl<FormatField, OffsetField, ShaderLocationField>
    VertexAttributeBuilder<FormatField, OffsetField, Unset<ShaderLocationField>>
{
    pub fn shader_location(
        self,
        shader_location: wgpu::ShaderLocation,
    ) -> VertexAttributeBuilder<FormatField, OffsetField, Set<ShaderLocationField>> {
        VertexAttributeBuilder {
            format: Unset(PhantomData),
            offset: Unset(PhantomData),
            shader_location: Unset(PhantomData),
        }
    }
}
impl VertexAttributeBuilder {
    pub fn build(self) -> wgpu::VertexAttribute {
        wgpu::VertexAttribute {}
    }
}

#[derive(Debug)]
pub struct CopyExternalImageDestInfoBuilder<
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
> {
    texture: TextureField,
    mip_level: MipLevelField,
    origin: OriginField,
    aspect: AspectField,
    color_space: ColorSpaceField,
    premultiplied_alpha: PremultipliedAlphaField,
}
pub fn copy_external_image_dest_info<T>() -> CopyExternalImageDestInfoBuilder<
    Unset<T>,
    Unset<u32>,
    Unset<impl Nested<wgpu::Origin3d>>,
    Unset<wgpu::TextureAspect>,
    Unset<wgpu::PredefinedColorSpace>,
    Unset<bool>,
> {
    CopyExternalImageDestInfoBuilder {
        texture: Unset(PhantomData),
        mip_level: Unset(PhantomData),
        origin: Unset(PhantomData),
        aspect: Unset(PhantomData),
        color_space: Unset(PhantomData),
        premultiplied_alpha: Unset(PhantomData),
    }
}
impl<
    T,
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
>
    CopyExternalImageDestInfoBuilder<
        Unset<TextureField>,
        MipLevelField,
        OriginField,
        AspectField,
        ColorSpaceField,
        PremultipliedAlphaField,
    >
{
    pub fn texture(
        self,
        texture: T,
    ) -> CopyExternalImageDestInfoBuilder<
        Set<TextureField>,
        MipLevelField,
        OriginField,
        AspectField,
        ColorSpaceField,
        PremultipliedAlphaField,
    > {
        CopyExternalImageDestInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
            color_space: Unset(PhantomData),
            premultiplied_alpha: Unset(PhantomData),
        }
    }
}
impl<
    T,
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
>
    CopyExternalImageDestInfoBuilder<
        TextureField,
        Unset<MipLevelField>,
        OriginField,
        AspectField,
        ColorSpaceField,
        PremultipliedAlphaField,
    >
{
    pub fn mip_level(
        self,
        mip_level: u32,
    ) -> CopyExternalImageDestInfoBuilder<
        TextureField,
        Set<MipLevelField>,
        OriginField,
        AspectField,
        ColorSpaceField,
        PremultipliedAlphaField,
    > {
        CopyExternalImageDestInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
            color_space: Unset(PhantomData),
            premultiplied_alpha: Unset(PhantomData),
        }
    }
}
impl<
    T,
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
>
    CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        Unset<OriginField>,
        AspectField,
        ColorSpaceField,
        PremultipliedAlphaField,
    >
{
    pub fn origin(
        self,
        origin: impl Nested<wgpu::Origin3d>,
    ) -> CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        Set<OriginField>,
        AspectField,
        ColorSpaceField,
        PremultipliedAlphaField,
    > {
        CopyExternalImageDestInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
            color_space: Unset(PhantomData),
            premultiplied_alpha: Unset(PhantomData),
        }
    }
}
impl<
    T,
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
>
    CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        OriginField,
        Unset<AspectField>,
        ColorSpaceField,
        PremultipliedAlphaField,
    >
{
    pub fn aspect(
        self,
        aspect: wgpu::TextureAspect,
    ) -> CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        OriginField,
        Set<AspectField>,
        ColorSpaceField,
        PremultipliedAlphaField,
    > {
        CopyExternalImageDestInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
            color_space: Unset(PhantomData),
            premultiplied_alpha: Unset(PhantomData),
        }
    }
}
impl<
    T,
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
>
    CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        OriginField,
        AspectField,
        Unset<ColorSpaceField>,
        PremultipliedAlphaField,
    >
{
    pub fn color_space(
        self,
        color_space: wgpu::PredefinedColorSpace,
    ) -> CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        OriginField,
        AspectField,
        Set<ColorSpaceField>,
        PremultipliedAlphaField,
    > {
        CopyExternalImageDestInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
            color_space: Unset(PhantomData),
            premultiplied_alpha: Unset(PhantomData),
        }
    }
}
impl<
    T,
    TextureField,
    MipLevelField,
    OriginField,
    AspectField,
    ColorSpaceField,
    PremultipliedAlphaField,
>
    CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        OriginField,
        AspectField,
        ColorSpaceField,
        Unset<PremultipliedAlphaField>,
    >
{
    pub fn premultiplied_alpha(
        self,
        premultiplied_alpha: bool,
    ) -> CopyExternalImageDestInfoBuilder<
        TextureField,
        MipLevelField,
        OriginField,
        AspectField,
        ColorSpaceField,
        Set<PremultipliedAlphaField>,
    > {
        CopyExternalImageDestInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
            color_space: Unset(PhantomData),
            premultiplied_alpha: Unset(PhantomData),
        }
    }
}
impl<T> CopyExternalImageDestInfoBuilder<T> {
    pub fn build(self) -> wgpu::CopyExternalImageDestInfo {
        wgpu::CopyExternalImageDestInfo {}
    }
}

#[derive(Debug)]
pub struct TexelCopyTextureInfoBaseBuilder<TextureField, MipLevelField, OriginField, AspectField> {
    texture: TextureField,
    mip_level: MipLevelField,
    origin: OriginField,
    aspect: AspectField,
}
pub fn texel_copy_texture_info_base<T>() -> TexelCopyTextureInfoBaseBuilder<
    Unset<T>,
    Unset<u32>,
    Unset<impl Nested<wgpu::Origin3d>>,
    Unset<wgpu::TextureAspect>,
> {
    TexelCopyTextureInfoBaseBuilder {
        texture: Unset(PhantomData),
        mip_level: Unset(PhantomData),
        origin: Unset(PhantomData),
        aspect: Unset(PhantomData),
    }
}
impl<T, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBaseBuilder<Unset<TextureField>, MipLevelField, OriginField, AspectField>
{
    pub fn texture(
        self,
        texture: T,
    ) -> TexelCopyTextureInfoBaseBuilder<Set<TextureField>, MipLevelField, OriginField, AspectField>
    {
        TexelCopyTextureInfoBaseBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<T, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBaseBuilder<TextureField, Unset<MipLevelField>, OriginField, AspectField>
{
    pub fn mip_level(
        self,
        mip_level: u32,
    ) -> TexelCopyTextureInfoBaseBuilder<TextureField, Set<MipLevelField>, OriginField, AspectField>
    {
        TexelCopyTextureInfoBaseBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<T, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBaseBuilder<TextureField, MipLevelField, Unset<OriginField>, AspectField>
{
    pub fn origin(
        self,
        origin: impl Nested<wgpu::Origin3d>,
    ) -> TexelCopyTextureInfoBaseBuilder<TextureField, MipLevelField, Set<OriginField>, AspectField>
    {
        TexelCopyTextureInfoBaseBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<T, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBaseBuilder<TextureField, MipLevelField, OriginField, Unset<AspectField>>
{
    pub fn aspect(
        self,
        aspect: wgpu::TextureAspect,
    ) -> TexelCopyTextureInfoBaseBuilder<TextureField, MipLevelField, OriginField, Set<AspectField>>
    {
        TexelCopyTextureInfoBaseBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<T> TexelCopyTextureInfoBaseBuilder<T> {
    pub fn build(self) -> wgpu::TexelCopyTextureInfoBase {
        wgpu::TexelCopyTextureInfoBase {}
    }
}

#[derive(Debug)]
pub struct Origin3DBuilder<XField, YField, ZField> {
    x: XField,
    y: YField,
    z: ZField,
}
pub fn origin_3_d() -> Origin3DBuilder<Unset<u32>, Unset<u32>, Unset<u32>> {
    Origin3DBuilder {
        x: Unset(PhantomData),
        y: Unset(PhantomData),
        z: Unset(PhantomData),
    }
}
impl<XField, YField, ZField> Origin3DBuilder<Unset<XField>, YField, ZField> {
    pub fn x(self, x: u32) -> Origin3DBuilder<Set<XField>, YField, ZField> {
        Origin3DBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
            z: Unset(PhantomData),
        }
    }
}
impl<XField, YField, ZField> Origin3DBuilder<XField, Unset<YField>, ZField> {
    pub fn y(self, y: u32) -> Origin3DBuilder<XField, Set<YField>, ZField> {
        Origin3DBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
            z: Unset(PhantomData),
        }
    }
}
impl<XField, YField, ZField> Origin3DBuilder<XField, YField, Unset<ZField>> {
    pub fn z(self, z: u32) -> Origin3DBuilder<XField, YField, Set<ZField>> {
        Origin3DBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
            z: Unset(PhantomData),
        }
    }
}
impl Origin3DBuilder {
    pub fn build(self) -> wgpu::Origin3d {
        wgpu::Origin3d {}
    }
}

#[derive(Debug)]
pub struct SurfaceConfigurationBuilder<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
> {
    usage: UsageField,
    format: FormatField,
    width: WidthField,
    height: HeightField,
    present_mode: PresentModeField,
    desired_maximum_frame_latency: DesiredMaximumFrameLatencyField,
    alpha_mode: AlphaModeField,
    view_formats: ViewFormatsField,
}
pub fn surface_configuration() -> SurfaceConfigurationBuilder<
    Unset<wgpu::TextureUsages>,
    Unset<wgpu::TextureFormat>,
    Unset<u32>,
    Unset<u32>,
    Unset<wgpu::PresentMode>,
    Unset<u32>,
    Unset<wgpu::CompositeAlphaMode>,
    Unset<Vec<wgpu::TextureFormat>>,
> {
    SurfaceConfigurationBuilder {
        usage: Unset(PhantomData),
        format: Unset(PhantomData),
        width: Unset(PhantomData),
        height: Unset(PhantomData),
        present_mode: Unset(PhantomData),
        desired_maximum_frame_latency: Unset(PhantomData),
        alpha_mode: Unset(PhantomData),
        view_formats: Unset(PhantomData),
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        Unset<UsageField>,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    >
{
    pub fn usage(
        self,
        usage: wgpu::TextureUsages,
    ) -> SurfaceConfigurationBuilder<
        Set<UsageField>,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        Unset<FormatField>,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    >
{
    pub fn format(
        self,
        format: wgpu::TextureFormat,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        Set<FormatField>,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        Unset<WidthField>,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    >
{
    pub fn width(
        self,
        width: u32,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        Set<WidthField>,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        Unset<HeightField>,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    >
{
    pub fn height(
        self,
        height: u32,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        Set<HeightField>,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        Unset<PresentModeField>,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    >
{
    pub fn present_mode(
        self,
        present_mode: wgpu::PresentMode,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        Set<PresentModeField>,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        Unset<DesiredMaximumFrameLatencyField>,
        AlphaModeField,
        ViewFormatsField,
    >
{
    pub fn desired_maximum_frame_latency(
        self,
        desired_maximum_frame_latency: u32,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        Set<DesiredMaximumFrameLatencyField>,
        AlphaModeField,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        Unset<AlphaModeField>,
        ViewFormatsField,
    >
{
    pub fn alpha_mode(
        self,
        alpha_mode: wgpu::CompositeAlphaMode,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        Set<AlphaModeField>,
        ViewFormatsField,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    UsageField,
    FormatField,
    WidthField,
    HeightField,
    PresentModeField,
    DesiredMaximumFrameLatencyField,
    AlphaModeField,
    ViewFormatsField,
>
    SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        Unset<ViewFormatsField>,
    >
{
    pub fn view_formats(
        self,
        view_formats: Vec<wgpu::TextureFormat>,
    ) -> SurfaceConfigurationBuilder<
        UsageField,
        FormatField,
        WidthField,
        HeightField,
        PresentModeField,
        DesiredMaximumFrameLatencyField,
        AlphaModeField,
        Set<ViewFormatsField>,
    > {
        SurfaceConfigurationBuilder {
            usage: Unset(PhantomData),
            format: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            present_mode: Unset(PhantomData),
            desired_maximum_frame_latency: Unset(PhantomData),
            alpha_mode: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl SurfaceConfigurationBuilder {
    pub fn build(self) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {}
    }
}

#[derive(Debug)]
pub struct VertexStateBuilder<ModuleField, EntryPointField, CompilationOptionsField, BuffersField> {
    module: ModuleField,
    entry_point: EntryPointField,
    compilation_options: CompilationOptionsField,
    buffers: BuffersField,
}
pub fn vertex_state<'a>() -> VertexStateBuilder<
    Unset<&'a wgpu::ShaderModule>,
    Unset<Option<&'a str>>,
    Unset<impl Nested<wgpu::PipelineCompilationOptions<'a>>>,
    Unset<&'a [wgpu::VertexBufferLayout<'a>]>,
> {
    VertexStateBuilder {
        module: Unset(PhantomData),
        entry_point: Unset(PhantomData),
        compilation_options: Unset(PhantomData),
        buffers: Unset(PhantomData),
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, BuffersField>
    VertexStateBuilder<Unset<ModuleField>, EntryPointField, CompilationOptionsField, BuffersField>
{
    pub fn module(
        self,
        module: &'a wgpu::ShaderModule,
    ) -> VertexStateBuilder<Set<ModuleField>, EntryPointField, CompilationOptionsField, BuffersField>
    {
        VertexStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            buffers: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, BuffersField>
    VertexStateBuilder<ModuleField, Unset<EntryPointField>, CompilationOptionsField, BuffersField>
{
    pub fn entry_point(
        self,
        entry_point: Option<&'a str>,
    ) -> VertexStateBuilder<ModuleField, Set<EntryPointField>, CompilationOptionsField, BuffersField>
    {
        VertexStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            buffers: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, BuffersField>
    VertexStateBuilder<ModuleField, EntryPointField, Unset<CompilationOptionsField>, BuffersField>
{
    pub fn compilation_options(
        self,
        compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    ) -> VertexStateBuilder<ModuleField, EntryPointField, Set<CompilationOptionsField>, BuffersField>
    {
        VertexStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            buffers: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, BuffersField>
    VertexStateBuilder<ModuleField, EntryPointField, CompilationOptionsField, Unset<BuffersField>>
{
    pub fn buffers(
        self,
        buffers: &'a [wgpu::VertexBufferLayout<'a>],
    ) -> VertexStateBuilder<ModuleField, EntryPointField, CompilationOptionsField, Set<BuffersField>>
    {
        VertexStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            buffers: Unset(PhantomData),
        }
    }
}
impl<'a> VertexStateBuilder<'a> {
    pub fn build(self) -> wgpu::VertexState {
        wgpu::VertexState {}
    }
}

#[derive(Debug)]
pub struct VertexBufferLayoutBuilder<ArrayStrideField, StepModeField, AttributesField> {
    array_stride: ArrayStrideField,
    step_mode: StepModeField,
    attributes: AttributesField,
}
pub fn vertex_buffer_layout<'a>() -> VertexBufferLayoutBuilder<
    Unset<wgpu::BufferAddress>,
    Unset<wgpu::VertexStepMode>,
    Unset<&'a [wgpu::VertexAttribute]>,
> {
    VertexBufferLayoutBuilder {
        array_stride: Unset(PhantomData),
        step_mode: Unset(PhantomData),
        attributes: Unset(PhantomData),
    }
}
impl<'a, ArrayStrideField, StepModeField, AttributesField>
    VertexBufferLayoutBuilder<Unset<ArrayStrideField>, StepModeField, AttributesField>
{
    pub fn array_stride(
        self,
        array_stride: wgpu::BufferAddress,
    ) -> VertexBufferLayoutBuilder<Set<ArrayStrideField>, StepModeField, AttributesField> {
        VertexBufferLayoutBuilder {
            array_stride: Unset(PhantomData),
            step_mode: Unset(PhantomData),
            attributes: Unset(PhantomData),
        }
    }
}
impl<'a, ArrayStrideField, StepModeField, AttributesField>
    VertexBufferLayoutBuilder<ArrayStrideField, Unset<StepModeField>, AttributesField>
{
    pub fn step_mode(
        self,
        step_mode: wgpu::VertexStepMode,
    ) -> VertexBufferLayoutBuilder<ArrayStrideField, Set<StepModeField>, AttributesField> {
        VertexBufferLayoutBuilder {
            array_stride: Unset(PhantomData),
            step_mode: Unset(PhantomData),
            attributes: Unset(PhantomData),
        }
    }
}
impl<'a, ArrayStrideField, StepModeField, AttributesField>
    VertexBufferLayoutBuilder<ArrayStrideField, StepModeField, Unset<AttributesField>>
{
    pub fn attributes(
        self,
        attributes: &'a [wgpu::VertexAttribute],
    ) -> VertexBufferLayoutBuilder<ArrayStrideField, StepModeField, Set<AttributesField>> {
        VertexBufferLayoutBuilder {
            array_stride: Unset(PhantomData),
            step_mode: Unset(PhantomData),
            attributes: Unset(PhantomData),
        }
    }
}
impl<'a> VertexBufferLayoutBuilder<'a> {
    pub fn build(self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {}
    }
}

#[derive(Debug)]
pub struct CompilationInfoBuilder<MessagesField> {
    messages: MessagesField,
}
pub fn compilation_info() -> CompilationInfoBuilder<Unset<Vec<wgpu::CompilationMessage>>> {
    CompilationInfoBuilder {
        messages: Unset(PhantomData),
    }
}
impl<MessagesField> CompilationInfoBuilder<Unset<MessagesField>> {
    pub fn messages(
        self,
        messages: Vec<wgpu::CompilationMessage>,
    ) -> CompilationInfoBuilder<Set<MessagesField>> {
        CompilationInfoBuilder {
            messages: Unset(PhantomData),
        }
    }
}
impl CompilationInfoBuilder {
    pub fn build(self) -> wgpu::CompilationInfo {
        wgpu::CompilationInfo {}
    }
}

#[derive(Debug)]
pub struct PipelineLayoutDescriptorBuilder<
    LabelField,
    BindGroupLayoutsField,
    PushConstantRangesField,
> {
    label: LabelField,
    bind_group_layouts: BindGroupLayoutsField,
    push_constant_ranges: PushConstantRangesField,
}
pub fn pipeline_layout_descriptor<'a>() -> PipelineLayoutDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<&'a [&'a wgpu::BindGroupLayout]>,
    Unset<&'a [wgpu::PushConstantRange]>,
> {
    PipelineLayoutDescriptorBuilder {
        label: Unset(PhantomData),
        bind_group_layouts: Unset(PhantomData),
        push_constant_ranges: Unset(PhantomData),
    }
}
impl<'a, LabelField, BindGroupLayoutsField, PushConstantRangesField>
    PipelineLayoutDescriptorBuilder<
        Unset<LabelField>,
        BindGroupLayoutsField,
        PushConstantRangesField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> PipelineLayoutDescriptorBuilder<
        Set<LabelField>,
        BindGroupLayoutsField,
        PushConstantRangesField,
    > {
        PipelineLayoutDescriptorBuilder {
            label: Unset(PhantomData),
            bind_group_layouts: Unset(PhantomData),
            push_constant_ranges: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, BindGroupLayoutsField, PushConstantRangesField>
    PipelineLayoutDescriptorBuilder<
        LabelField,
        Unset<BindGroupLayoutsField>,
        PushConstantRangesField,
    >
{
    pub fn bind_group_layouts(
        self,
        bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    ) -> PipelineLayoutDescriptorBuilder<
        LabelField,
        Set<BindGroupLayoutsField>,
        PushConstantRangesField,
    > {
        PipelineLayoutDescriptorBuilder {
            label: Unset(PhantomData),
            bind_group_layouts: Unset(PhantomData),
            push_constant_ranges: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, BindGroupLayoutsField, PushConstantRangesField>
    PipelineLayoutDescriptorBuilder<
        LabelField,
        BindGroupLayoutsField,
        Unset<PushConstantRangesField>,
    >
{
    pub fn push_constant_ranges(
        self,
        push_constant_ranges: &'a [wgpu::PushConstantRange],
    ) -> PipelineLayoutDescriptorBuilder<
        LabelField,
        BindGroupLayoutsField,
        Set<PushConstantRangesField>,
    > {
        PipelineLayoutDescriptorBuilder {
            label: Unset(PhantomData),
            bind_group_layouts: Unset(PhantomData),
            push_constant_ranges: Unset(PhantomData),
        }
    }
}
impl<'a> PipelineLayoutDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::PipelineLayoutDescriptor {
        wgpu::PipelineLayoutDescriptor {}
    }
}

#[derive(Debug)]
pub struct CoreCountersBuilder {}
pub fn core_counters() -> CoreCountersBuilder {
    CoreCountersBuilder {}
}
impl CoreCountersBuilder {
    pub fn build(self) -> wgpu::CoreCounters {
        wgpu::CoreCounters {}
    }
}

#[derive(Debug)]
pub struct ComputePipelineDescriptorBuilder<
    LabelField,
    LayoutField,
    ModuleField,
    EntryPointField,
    CompilationOptionsField,
    CacheField,
> {
    label: LabelField,
    layout: LayoutField,
    module: ModuleField,
    entry_point: EntryPointField,
    compilation_options: CompilationOptionsField,
    cache: CacheField,
}
pub fn compute_pipeline_descriptor<'a>() -> ComputePipelineDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<Option<&'a wgpu::PipelineLayout>>,
    Unset<&'a wgpu::ShaderModule>,
    Unset<Option<&'a str>>,
    Unset<impl Nested<wgpu::PipelineCompilationOptions<'a>>>,
    Unset<Option<&'a wgpu::PipelineCache>>,
> {
    ComputePipelineDescriptorBuilder {
        label: Unset(PhantomData),
        layout: Unset(PhantomData),
        module: Unset(PhantomData),
        entry_point: Unset(PhantomData),
        compilation_options: Unset(PhantomData),
        cache: Unset(PhantomData),
    }
}
impl<'a, LabelField, LayoutField, ModuleField, EntryPointField, CompilationOptionsField, CacheField>
    ComputePipelineDescriptorBuilder<
        Unset<LabelField>,
        LayoutField,
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        CacheField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> ComputePipelineDescriptorBuilder<
        Set<LabelField>,
        LayoutField,
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        CacheField,
    > {
        ComputePipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, ModuleField, EntryPointField, CompilationOptionsField, CacheField>
    ComputePipelineDescriptorBuilder<
        LabelField,
        Unset<LayoutField>,
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        CacheField,
    >
{
    pub fn layout(
        self,
        layout: Option<&'a wgpu::PipelineLayout>,
    ) -> ComputePipelineDescriptorBuilder<
        LabelField,
        Set<LayoutField>,
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        CacheField,
    > {
        ComputePipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, ModuleField, EntryPointField, CompilationOptionsField, CacheField>
    ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        Unset<ModuleField>,
        EntryPointField,
        CompilationOptionsField,
        CacheField,
    >
{
    pub fn module(
        self,
        module: &'a wgpu::ShaderModule,
    ) -> ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        Set<ModuleField>,
        EntryPointField,
        CompilationOptionsField,
        CacheField,
    > {
        ComputePipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, ModuleField, EntryPointField, CompilationOptionsField, CacheField>
    ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        ModuleField,
        Unset<EntryPointField>,
        CompilationOptionsField,
        CacheField,
    >
{
    pub fn entry_point(
        self,
        entry_point: Option<&'a str>,
    ) -> ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        ModuleField,
        Set<EntryPointField>,
        CompilationOptionsField,
        CacheField,
    > {
        ComputePipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, ModuleField, EntryPointField, CompilationOptionsField, CacheField>
    ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        ModuleField,
        EntryPointField,
        Unset<CompilationOptionsField>,
        CacheField,
    >
{
    pub fn compilation_options(
        self,
        compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    ) -> ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        ModuleField,
        EntryPointField,
        Set<CompilationOptionsField>,
        CacheField,
    > {
        ComputePipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, LayoutField, ModuleField, EntryPointField, CompilationOptionsField, CacheField>
    ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        Unset<CacheField>,
    >
{
    pub fn cache(
        self,
        cache: Option<&'a wgpu::PipelineCache>,
    ) -> ComputePipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        Set<CacheField>,
    > {
        ComputePipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a> ComputePipelineDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::ComputePipelineDescriptor {
        wgpu::ComputePipelineDescriptor {}
    }
}

#[derive(Debug)]
pub struct InstanceDescriptorBuilder<
    BackendsField,
    FlagsField,
    MemoryBudgetThresholdsField,
    BackendOptionsField,
> {
    backends: BackendsField,
    flags: FlagsField,
    memory_budget_thresholds: MemoryBudgetThresholdsField,
    backend_options: BackendOptionsField,
}
pub fn instance_descriptor() -> InstanceDescriptorBuilder<
    Unset<wgpu::Backends>,
    Unset<wgpu::InstanceFlags>,
    Unset<impl Nested<wgpu::MemoryBudgetThresholds>>,
    Unset<impl Nested<wgpu::BackendOptions>>,
> {
    InstanceDescriptorBuilder {
        backends: Unset(PhantomData),
        flags: Unset(PhantomData),
        memory_budget_thresholds: Unset(PhantomData),
        backend_options: Unset(PhantomData),
    }
}
impl<BackendsField, FlagsField, MemoryBudgetThresholdsField, BackendOptionsField>
    InstanceDescriptorBuilder<
        Unset<BackendsField>,
        FlagsField,
        MemoryBudgetThresholdsField,
        BackendOptionsField,
    >
{
    pub fn backends(
        self,
        backends: wgpu::Backends,
    ) -> InstanceDescriptorBuilder<
        Set<BackendsField>,
        FlagsField,
        MemoryBudgetThresholdsField,
        BackendOptionsField,
    > {
        InstanceDescriptorBuilder {
            backends: Unset(PhantomData),
            flags: Unset(PhantomData),
            memory_budget_thresholds: Unset(PhantomData),
            backend_options: Unset(PhantomData),
        }
    }
}
impl<BackendsField, FlagsField, MemoryBudgetThresholdsField, BackendOptionsField>
    InstanceDescriptorBuilder<
        BackendsField,
        Unset<FlagsField>,
        MemoryBudgetThresholdsField,
        BackendOptionsField,
    >
{
    pub fn flags(
        self,
        flags: wgpu::InstanceFlags,
    ) -> InstanceDescriptorBuilder<
        BackendsField,
        Set<FlagsField>,
        MemoryBudgetThresholdsField,
        BackendOptionsField,
    > {
        InstanceDescriptorBuilder {
            backends: Unset(PhantomData),
            flags: Unset(PhantomData),
            memory_budget_thresholds: Unset(PhantomData),
            backend_options: Unset(PhantomData),
        }
    }
}
impl<BackendsField, FlagsField, MemoryBudgetThresholdsField, BackendOptionsField>
    InstanceDescriptorBuilder<
        BackendsField,
        FlagsField,
        Unset<MemoryBudgetThresholdsField>,
        BackendOptionsField,
    >
{
    pub fn memory_budget_thresholds(
        self,
        memory_budget_thresholds: impl Nested<wgpu::MemoryBudgetThresholds>,
    ) -> InstanceDescriptorBuilder<
        BackendsField,
        FlagsField,
        Set<MemoryBudgetThresholdsField>,
        BackendOptionsField,
    > {
        InstanceDescriptorBuilder {
            backends: Unset(PhantomData),
            flags: Unset(PhantomData),
            memory_budget_thresholds: Unset(PhantomData),
            backend_options: Unset(PhantomData),
        }
    }
}
impl<BackendsField, FlagsField, MemoryBudgetThresholdsField, BackendOptionsField>
    InstanceDescriptorBuilder<
        BackendsField,
        FlagsField,
        MemoryBudgetThresholdsField,
        Unset<BackendOptionsField>,
    >
{
    pub fn backend_options(
        self,
        backend_options: impl Nested<wgpu::BackendOptions>,
    ) -> InstanceDescriptorBuilder<
        BackendsField,
        FlagsField,
        MemoryBudgetThresholdsField,
        Set<BackendOptionsField>,
    > {
        InstanceDescriptorBuilder {
            backends: Unset(PhantomData),
            flags: Unset(PhantomData),
            memory_budget_thresholds: Unset(PhantomData),
            backend_options: Unset(PhantomData),
        }
    }
}
impl InstanceDescriptorBuilder {
    pub fn build(self) -> wgpu::InstanceDescriptor {
        wgpu::InstanceDescriptor {}
    }
}

#[derive(Debug)]
pub struct TextureViewDescriptorBuilder<
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
> {
    label: LabelField,
    format: FormatField,
    dimension: DimensionField,
    usage: UsageField,
    aspect: AspectField,
    base_mip_level: BaseMipLevelField,
    mip_level_count: MipLevelCountField,
    base_array_layer: BaseArrayLayerField,
    array_layer_count: ArrayLayerCountField,
}
pub fn texture_view_descriptor<'a>() -> TextureViewDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<Option<wgpu::TextureFormat>>,
    Unset<Option<wgpu::TextureViewDimension>>,
    Unset<Option<wgpu::TextureUsages>>,
    Unset<wgpu::TextureAspect>,
    Unset<u32>,
    Unset<Option<u32>>,
    Unset<u32>,
    Unset<Option<u32>>,
> {
    TextureViewDescriptorBuilder {
        label: Unset(PhantomData),
        format: Unset(PhantomData),
        dimension: Unset(PhantomData),
        usage: Unset(PhantomData),
        aspect: Unset(PhantomData),
        base_mip_level: Unset(PhantomData),
        mip_level_count: Unset(PhantomData),
        base_array_layer: Unset(PhantomData),
        array_layer_count: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        Unset<LabelField>,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> TextureViewDescriptorBuilder<
        Set<LabelField>,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        Unset<FormatField>,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn format(
        self,
        format: Option<wgpu::TextureFormat>,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        Set<FormatField>,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        Unset<DimensionField>,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn dimension(
        self,
        dimension: Option<wgpu::TextureViewDimension>,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        Set<DimensionField>,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        Unset<UsageField>,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn usage(
        self,
        usage: Option<wgpu::TextureUsages>,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        Set<UsageField>,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        Unset<AspectField>,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn aspect(
        self,
        aspect: wgpu::TextureAspect,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        Set<AspectField>,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        Unset<BaseMipLevelField>,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn base_mip_level(
        self,
        base_mip_level: u32,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        Set<BaseMipLevelField>,
        MipLevelCountField,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        Unset<MipLevelCountField>,
        BaseArrayLayerField,
        ArrayLayerCountField,
    >
{
    pub fn mip_level_count(
        self,
        mip_level_count: Option<u32>,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        Set<MipLevelCountField>,
        BaseArrayLayerField,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        Unset<BaseArrayLayerField>,
        ArrayLayerCountField,
    >
{
    pub fn base_array_layer(
        self,
        base_array_layer: u32,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        Set<BaseArrayLayerField>,
        ArrayLayerCountField,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    FormatField,
    DimensionField,
    UsageField,
    AspectField,
    BaseMipLevelField,
    MipLevelCountField,
    BaseArrayLayerField,
    ArrayLayerCountField,
>
    TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        Unset<ArrayLayerCountField>,
    >
{
    pub fn array_layer_count(
        self,
        array_layer_count: Option<u32>,
    ) -> TextureViewDescriptorBuilder<
        LabelField,
        FormatField,
        DimensionField,
        UsageField,
        AspectField,
        BaseMipLevelField,
        MipLevelCountField,
        BaseArrayLayerField,
        Set<ArrayLayerCountField>,
    > {
        TextureViewDescriptorBuilder {
            label: Unset(PhantomData),
            format: Unset(PhantomData),
            dimension: Unset(PhantomData),
            usage: Unset(PhantomData),
            aspect: Unset(PhantomData),
            base_mip_level: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            base_array_layer: Unset(PhantomData),
            array_layer_count: Unset(PhantomData),
        }
    }
}
impl<'a> TextureViewDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::TextureViewDescriptor {
        wgpu::TextureViewDescriptor {}
    }
}

#[derive(Debug)]
pub struct ComputePassTimestampWritesBuilder<
    QuerySetField,
    BeginningOfPassWriteIndexField,
    EndOfPassWriteIndexField,
> {
    query_set: QuerySetField,
    beginning_of_pass_write_index: BeginningOfPassWriteIndexField,
    end_of_pass_write_index: EndOfPassWriteIndexField,
}
pub fn compute_pass_timestamp_writes<'a>() -> ComputePassTimestampWritesBuilder<
    Unset<&'a wgpu::QuerySet>,
    Unset<Option<u32>>,
    Unset<Option<u32>>,
> {
    ComputePassTimestampWritesBuilder {
        query_set: Unset(PhantomData),
        beginning_of_pass_write_index: Unset(PhantomData),
        end_of_pass_write_index: Unset(PhantomData),
    }
}
impl<'a, QuerySetField, BeginningOfPassWriteIndexField, EndOfPassWriteIndexField>
    ComputePassTimestampWritesBuilder<
        Unset<QuerySetField>,
        BeginningOfPassWriteIndexField,
        EndOfPassWriteIndexField,
    >
{
    pub fn query_set(
        self,
        query_set: &'a wgpu::QuerySet,
    ) -> ComputePassTimestampWritesBuilder<
        Set<QuerySetField>,
        BeginningOfPassWriteIndexField,
        EndOfPassWriteIndexField,
    > {
        ComputePassTimestampWritesBuilder {
            query_set: Unset(PhantomData),
            beginning_of_pass_write_index: Unset(PhantomData),
            end_of_pass_write_index: Unset(PhantomData),
        }
    }
}
impl<'a, QuerySetField, BeginningOfPassWriteIndexField, EndOfPassWriteIndexField>
    ComputePassTimestampWritesBuilder<
        QuerySetField,
        Unset<BeginningOfPassWriteIndexField>,
        EndOfPassWriteIndexField,
    >
{
    pub fn beginning_of_pass_write_index(
        self,
        beginning_of_pass_write_index: Option<u32>,
    ) -> ComputePassTimestampWritesBuilder<
        QuerySetField,
        Set<BeginningOfPassWriteIndexField>,
        EndOfPassWriteIndexField,
    > {
        ComputePassTimestampWritesBuilder {
            query_set: Unset(PhantomData),
            beginning_of_pass_write_index: Unset(PhantomData),
            end_of_pass_write_index: Unset(PhantomData),
        }
    }
}
impl<'a, QuerySetField, BeginningOfPassWriteIndexField, EndOfPassWriteIndexField>
    ComputePassTimestampWritesBuilder<
        QuerySetField,
        BeginningOfPassWriteIndexField,
        Unset<EndOfPassWriteIndexField>,
    >
{
    pub fn end_of_pass_write_index(
        self,
        end_of_pass_write_index: Option<u32>,
    ) -> ComputePassTimestampWritesBuilder<
        QuerySetField,
        BeginningOfPassWriteIndexField,
        Set<EndOfPassWriteIndexField>,
    > {
        ComputePassTimestampWritesBuilder {
            query_set: Unset(PhantomData),
            beginning_of_pass_write_index: Unset(PhantomData),
            end_of_pass_write_index: Unset(PhantomData),
        }
    }
}
impl<'a> ComputePassTimestampWritesBuilder<'a> {
    pub fn build(self) -> wgpu::ComputePassTimestampWrites {
        wgpu::ComputePassTimestampWrites {}
    }
}

#[derive(Debug)]
pub struct PushConstantRangeBuilder<StagesField, RangeField> {
    stages: StagesField,
    range: RangeField,
}
pub fn push_constant_range()
-> PushConstantRangeBuilder<Unset<wgpu::ShaderStages>, Unset<Range<u32>>> {
    PushConstantRangeBuilder {
        stages: Unset(PhantomData),
        range: Unset(PhantomData),
    }
}
impl<StagesField, RangeField> PushConstantRangeBuilder<Unset<StagesField>, RangeField> {
    pub fn stages(
        self,
        stages: wgpu::ShaderStages,
    ) -> PushConstantRangeBuilder<Set<StagesField>, RangeField> {
        PushConstantRangeBuilder {
            stages: Unset(PhantomData),
            range: Unset(PhantomData),
        }
    }
}
impl<StagesField, RangeField> PushConstantRangeBuilder<StagesField, Unset<RangeField>> {
    pub fn range(
        self,
        range: Range<u32>,
    ) -> PushConstantRangeBuilder<StagesField, Set<RangeField>> {
        PushConstantRangeBuilder {
            stages: Unset(PhantomData),
            range: Unset(PhantomData),
        }
    }
}
impl PushConstantRangeBuilder {
    pub fn build(self) -> wgpu::PushConstantRange {
        wgpu::PushConstantRange {}
    }
}

#[derive(Debug)]
pub struct PipelineCompilationOptionsBuilder<ConstantsField, ZeroInitializeWorkgroupMemoryField> {
    constants: ConstantsField,
    zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryField,
}
pub fn pipeline_compilation_options<'a>()
-> PipelineCompilationOptionsBuilder<Unset<&'a [(&'a str, f64)]>, Unset<bool>> {
    PipelineCompilationOptionsBuilder {
        constants: Unset(PhantomData),
        zero_initialize_workgroup_memory: Unset(PhantomData),
    }
}
impl<'a, ConstantsField, ZeroInitializeWorkgroupMemoryField>
    PipelineCompilationOptionsBuilder<Unset<ConstantsField>, ZeroInitializeWorkgroupMemoryField>
{
    pub fn constants(
        self,
        constants: &'a [(&'a str, f64)],
    ) -> PipelineCompilationOptionsBuilder<Set<ConstantsField>, ZeroInitializeWorkgroupMemoryField>
    {
        PipelineCompilationOptionsBuilder {
            constants: Unset(PhantomData),
            zero_initialize_workgroup_memory: Unset(PhantomData),
        }
    }
}
impl<'a, ConstantsField, ZeroInitializeWorkgroupMemoryField>
    PipelineCompilationOptionsBuilder<ConstantsField, Unset<ZeroInitializeWorkgroupMemoryField>>
{
    pub fn zero_initialize_workgroup_memory(
        self,
        zero_initialize_workgroup_memory: bool,
    ) -> PipelineCompilationOptionsBuilder<ConstantsField, Set<ZeroInitializeWorkgroupMemoryField>>
    {
        PipelineCompilationOptionsBuilder {
            constants: Unset(PhantomData),
            zero_initialize_workgroup_memory: Unset(PhantomData),
        }
    }
}
impl<'a> PipelineCompilationOptionsBuilder<'a> {
    pub fn build(self) -> wgpu::PipelineCompilationOptions {
        wgpu::PipelineCompilationOptions {}
    }
}

#[derive(Debug)]
pub struct BindGroupEntryBuilder<BindingField, ResourceField> {
    binding: BindingField,
    resource: ResourceField,
}
pub fn bind_group_entry<'a>() -> BindGroupEntryBuilder<Unset<u32>, Unset<wgpu::BindingResource<'a>>>
{
    BindGroupEntryBuilder {
        binding: Unset(PhantomData),
        resource: Unset(PhantomData),
    }
}
impl<'a, BindingField, ResourceField> BindGroupEntryBuilder<Unset<BindingField>, ResourceField> {
    pub fn binding(self, binding: u32) -> BindGroupEntryBuilder<Set<BindingField>, ResourceField> {
        BindGroupEntryBuilder {
            binding: Unset(PhantomData),
            resource: Unset(PhantomData),
        }
    }
}
impl<'a, BindingField, ResourceField> BindGroupEntryBuilder<BindingField, Unset<ResourceField>> {
    pub fn resource(
        self,
        resource: wgpu::BindingResource<'a>,
    ) -> BindGroupEntryBuilder<BindingField, Set<ResourceField>> {
        BindGroupEntryBuilder {
            binding: Unset(PhantomData),
            resource: Unset(PhantomData),
        }
    }
}
impl<'a> BindGroupEntryBuilder<'a> {
    pub fn build(self) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {}
    }
}

#[derive(Debug)]
pub struct DeviceDescriptorBuilder<
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
> {
    label: LabelField,
    required_features: RequiredFeaturesField,
    required_limits: RequiredLimitsField,
    experimental_features: ExperimentalFeaturesField,
    memory_hints: MemoryHintsField,
    trace: TraceField,
}
pub fn device_descriptor<'a>() -> DeviceDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<wgpu::Features>,
    Unset<wgpu::Limits>,
    Unset<wgpu::ExperimentalFeatures>,
    Unset<wgpu::MemoryHints>,
    Unset<wgpu::Trace>,
> {
    DeviceDescriptorBuilder {
        label: Unset(PhantomData),
        required_features: Unset(PhantomData),
        required_limits: Unset(PhantomData),
        experimental_features: Unset(PhantomData),
        memory_hints: Unset(PhantomData),
        trace: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
>
    DeviceDescriptorBuilder<
        Unset<LabelField>,
        RequiredFeaturesField,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        MemoryHintsField,
        TraceField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> DeviceDescriptorBuilder<
        Set<LabelField>,
        RequiredFeaturesField,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        MemoryHintsField,
        TraceField,
    > {
        DeviceDescriptorBuilder {
            label: Unset(PhantomData),
            required_features: Unset(PhantomData),
            required_limits: Unset(PhantomData),
            experimental_features: Unset(PhantomData),
            memory_hints: Unset(PhantomData),
            trace: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
>
    DeviceDescriptorBuilder<
        LabelField,
        Unset<RequiredFeaturesField>,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        MemoryHintsField,
        TraceField,
    >
{
    pub fn required_features(
        self,
        required_features: wgpu::Features,
    ) -> DeviceDescriptorBuilder<
        LabelField,
        Set<RequiredFeaturesField>,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        MemoryHintsField,
        TraceField,
    > {
        DeviceDescriptorBuilder {
            label: Unset(PhantomData),
            required_features: Unset(PhantomData),
            required_limits: Unset(PhantomData),
            experimental_features: Unset(PhantomData),
            memory_hints: Unset(PhantomData),
            trace: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
>
    DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        Unset<RequiredLimitsField>,
        ExperimentalFeaturesField,
        MemoryHintsField,
        TraceField,
    >
{
    pub fn required_limits(
        self,
        required_limits: wgpu::Limits,
    ) -> DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        Set<RequiredLimitsField>,
        ExperimentalFeaturesField,
        MemoryHintsField,
        TraceField,
    > {
        DeviceDescriptorBuilder {
            label: Unset(PhantomData),
            required_features: Unset(PhantomData),
            required_limits: Unset(PhantomData),
            experimental_features: Unset(PhantomData),
            memory_hints: Unset(PhantomData),
            trace: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
>
    DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        RequiredLimitsField,
        Unset<ExperimentalFeaturesField>,
        MemoryHintsField,
        TraceField,
    >
{
    pub fn experimental_features(
        self,
        experimental_features: wgpu::ExperimentalFeatures,
    ) -> DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        RequiredLimitsField,
        Set<ExperimentalFeaturesField>,
        MemoryHintsField,
        TraceField,
    > {
        DeviceDescriptorBuilder {
            label: Unset(PhantomData),
            required_features: Unset(PhantomData),
            required_limits: Unset(PhantomData),
            experimental_features: Unset(PhantomData),
            memory_hints: Unset(PhantomData),
            trace: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
>
    DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        Unset<MemoryHintsField>,
        TraceField,
    >
{
    pub fn memory_hints(
        self,
        memory_hints: wgpu::MemoryHints,
    ) -> DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        Set<MemoryHintsField>,
        TraceField,
    > {
        DeviceDescriptorBuilder {
            label: Unset(PhantomData),
            required_features: Unset(PhantomData),
            required_limits: Unset(PhantomData),
            experimental_features: Unset(PhantomData),
            memory_hints: Unset(PhantomData),
            trace: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    RequiredFeaturesField,
    RequiredLimitsField,
    ExperimentalFeaturesField,
    MemoryHintsField,
    TraceField,
>
    DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        MemoryHintsField,
        Unset<TraceField>,
    >
{
    pub fn trace(
        self,
        trace: wgpu::Trace,
    ) -> DeviceDescriptorBuilder<
        LabelField,
        RequiredFeaturesField,
        RequiredLimitsField,
        ExperimentalFeaturesField,
        MemoryHintsField,
        Set<TraceField>,
    > {
        DeviceDescriptorBuilder {
            label: Unset(PhantomData),
            required_features: Unset(PhantomData),
            required_limits: Unset(PhantomData),
            experimental_features: Unset(PhantomData),
            memory_hints: Unset(PhantomData),
            trace: Unset(PhantomData),
        }
    }
}
impl<'a> DeviceDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::DeviceDescriptor {
        wgpu::DeviceDescriptor {}
    }
}

#[derive(Debug)]
pub struct RenderPassDescriptorBuilder<
    LabelField,
    ColorAttachmentsField,
    DepthStencilAttachmentField,
    TimestampWritesField,
    OcclusionQuerySetField,
> {
    label: LabelField,
    color_attachments: ColorAttachmentsField,
    depth_stencil_attachment: DepthStencilAttachmentField,
    timestamp_writes: TimestampWritesField,
    occlusion_query_set: OcclusionQuerySetField,
}
pub fn render_pass_descriptor<'a>() -> RenderPassDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>,
    Unset<Option<impl Nested<wgpu::RenderPassDepthStencilAttachment<'a>>>>,
    Unset<Option<impl Nested<wgpu::RenderPassTimestampWrites<'a>>>>,
    Unset<Option<&'a wgpu::QuerySet>>,
> {
    RenderPassDescriptorBuilder {
        label: Unset(PhantomData),
        color_attachments: Unset(PhantomData),
        depth_stencil_attachment: Unset(PhantomData),
        timestamp_writes: Unset(PhantomData),
        occlusion_query_set: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    ColorAttachmentsField,
    DepthStencilAttachmentField,
    TimestampWritesField,
    OcclusionQuerySetField,
>
    RenderPassDescriptorBuilder<
        Unset<LabelField>,
        ColorAttachmentsField,
        DepthStencilAttachmentField,
        TimestampWritesField,
        OcclusionQuerySetField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> RenderPassDescriptorBuilder<
        Set<LabelField>,
        ColorAttachmentsField,
        DepthStencilAttachmentField,
        TimestampWritesField,
        OcclusionQuerySetField,
    > {
        RenderPassDescriptorBuilder {
            label: Unset(PhantomData),
            color_attachments: Unset(PhantomData),
            depth_stencil_attachment: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
            occlusion_query_set: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    ColorAttachmentsField,
    DepthStencilAttachmentField,
    TimestampWritesField,
    OcclusionQuerySetField,
>
    RenderPassDescriptorBuilder<
        LabelField,
        Unset<ColorAttachmentsField>,
        DepthStencilAttachmentField,
        TimestampWritesField,
        OcclusionQuerySetField,
    >
{
    pub fn color_attachments(
        self,
        color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
    ) -> RenderPassDescriptorBuilder<
        LabelField,
        Set<ColorAttachmentsField>,
        DepthStencilAttachmentField,
        TimestampWritesField,
        OcclusionQuerySetField,
    > {
        RenderPassDescriptorBuilder {
            label: Unset(PhantomData),
            color_attachments: Unset(PhantomData),
            depth_stencil_attachment: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
            occlusion_query_set: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    ColorAttachmentsField,
    DepthStencilAttachmentField,
    TimestampWritesField,
    OcclusionQuerySetField,
>
    RenderPassDescriptorBuilder<
        LabelField,
        ColorAttachmentsField,
        Unset<DepthStencilAttachmentField>,
        TimestampWritesField,
        OcclusionQuerySetField,
    >
{
    pub fn depth_stencil_attachment(
        self,
        depth_stencil_attachment: Option<impl Nested<wgpu::RenderPassDepthStencilAttachment<'a>>>,
    ) -> RenderPassDescriptorBuilder<
        LabelField,
        ColorAttachmentsField,
        Set<DepthStencilAttachmentField>,
        TimestampWritesField,
        OcclusionQuerySetField,
    > {
        RenderPassDescriptorBuilder {
            label: Unset(PhantomData),
            color_attachments: Unset(PhantomData),
            depth_stencil_attachment: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
            occlusion_query_set: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    ColorAttachmentsField,
    DepthStencilAttachmentField,
    TimestampWritesField,
    OcclusionQuerySetField,
>
    RenderPassDescriptorBuilder<
        LabelField,
        ColorAttachmentsField,
        DepthStencilAttachmentField,
        Unset<TimestampWritesField>,
        OcclusionQuerySetField,
    >
{
    pub fn timestamp_writes(
        self,
        timestamp_writes: Option<impl Nested<wgpu::RenderPassTimestampWrites<'a>>>,
    ) -> RenderPassDescriptorBuilder<
        LabelField,
        ColorAttachmentsField,
        DepthStencilAttachmentField,
        Set<TimestampWritesField>,
        OcclusionQuerySetField,
    > {
        RenderPassDescriptorBuilder {
            label: Unset(PhantomData),
            color_attachments: Unset(PhantomData),
            depth_stencil_attachment: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
            occlusion_query_set: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    ColorAttachmentsField,
    DepthStencilAttachmentField,
    TimestampWritesField,
    OcclusionQuerySetField,
>
    RenderPassDescriptorBuilder<
        LabelField,
        ColorAttachmentsField,
        DepthStencilAttachmentField,
        TimestampWritesField,
        Unset<OcclusionQuerySetField>,
    >
{
    pub fn occlusion_query_set(
        self,
        occlusion_query_set: Option<&'a wgpu::QuerySet>,
    ) -> RenderPassDescriptorBuilder<
        LabelField,
        ColorAttachmentsField,
        DepthStencilAttachmentField,
        TimestampWritesField,
        Set<OcclusionQuerySetField>,
    > {
        RenderPassDescriptorBuilder {
            label: Unset(PhantomData),
            color_attachments: Unset(PhantomData),
            depth_stencil_attachment: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
            occlusion_query_set: Unset(PhantomData),
        }
    }
}
impl<'a> RenderPassDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::RenderPassDescriptor {
        wgpu::RenderPassDescriptor {}
    }
}

#[derive(Debug)]
pub struct RenderPipelineDescriptorBuilder<
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
> {
    label: LabelField,
    layout: LayoutField,
    vertex: VertexField,
    primitive: PrimitiveField,
    depth_stencil: DepthStencilField,
    multisample: MultisampleField,
    fragment: FragmentField,
    multiview: MultiviewField,
    cache: CacheField,
}
pub fn render_pipeline_descriptor<'a>() -> RenderPipelineDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<Option<&'a wgpu::PipelineLayout>>,
    Unset<impl Nested<wgpu::VertexState<'a>>>,
    Unset<impl Nested<wgpu::PrimitiveState>>,
    Unset<Option<impl Nested<wgpu::DepthStencilState>>>,
    Unset<impl Nested<wgpu::MultisampleState>>,
    Unset<Option<impl Nested<wgpu::FragmentState<'a>>>>,
    Unset<Option<NonZeroU32>>,
    Unset<Option<&'a wgpu::PipelineCache>>,
> {
    RenderPipelineDescriptorBuilder {
        label: Unset(PhantomData),
        layout: Unset(PhantomData),
        vertex: Unset(PhantomData),
        primitive: Unset(PhantomData),
        depth_stencil: Unset(PhantomData),
        multisample: Unset(PhantomData),
        fragment: Unset(PhantomData),
        multiview: Unset(PhantomData),
        cache: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        Unset<LabelField>,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> RenderPipelineDescriptorBuilder<
        Set<LabelField>,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        Unset<LayoutField>,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn layout(
        self,
        layout: Option<&'a wgpu::PipelineLayout>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        Set<LayoutField>,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        Unset<VertexField>,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn vertex(
        self,
        vertex: impl Nested<wgpu::VertexState<'a>>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        Set<VertexField>,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        Unset<PrimitiveField>,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn primitive(
        self,
        primitive: impl Nested<wgpu::PrimitiveState>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        Set<PrimitiveField>,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        Unset<DepthStencilField>,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn depth_stencil(
        self,
        depth_stencil: Option<impl Nested<wgpu::DepthStencilState>>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        Set<DepthStencilField>,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        Unset<MultisampleField>,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn multisample(
        self,
        multisample: impl Nested<wgpu::MultisampleState>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        Set<MultisampleField>,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        Unset<FragmentField>,
        MultiviewField,
        CacheField,
    >
{
    pub fn fragment(
        self,
        fragment: Option<impl Nested<wgpu::FragmentState<'a>>>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        Set<FragmentField>,
        MultiviewField,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        Unset<MultiviewField>,
        CacheField,
    >
{
    pub fn multiview(
        self,
        multiview: Option<NonZeroU32>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        Set<MultiviewField>,
        CacheField,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    VertexField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        Unset<CacheField>,
    >
{
    pub fn cache(
        self,
        cache: Option<&'a wgpu::PipelineCache>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        VertexField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        Set<CacheField>,
    > {
        RenderPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            vertex: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a> RenderPipelineDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::RenderPipelineDescriptor {
        wgpu::RenderPipelineDescriptor {}
    }
}

#[derive(Debug)]
pub struct TexelCopyTextureInfoBuilder<TextureField, MipLevelField, OriginField, AspectField> {
    texture: TextureField,
    mip_level: MipLevelField,
    origin: OriginField,
    aspect: AspectField,
}
pub fn texel_copy_texture_info<'a>() -> TexelCopyTextureInfoBuilder<
    Unset<&'a wgpu::Texture>,
    Unset<u32>,
    Unset<impl Nested<wgpu::Origin3d>>,
    Unset<wgpu::TextureAspect>,
> {
    TexelCopyTextureInfoBuilder {
        texture: Unset(PhantomData),
        mip_level: Unset(PhantomData),
        origin: Unset(PhantomData),
        aspect: Unset(PhantomData),
    }
}
impl<'a, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBuilder<Unset<TextureField>, MipLevelField, OriginField, AspectField>
{
    pub fn texture(
        self,
        texture: &'a wgpu::Texture,
    ) -> TexelCopyTextureInfoBuilder<Set<TextureField>, MipLevelField, OriginField, AspectField>
    {
        TexelCopyTextureInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<'a, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBuilder<TextureField, Unset<MipLevelField>, OriginField, AspectField>
{
    pub fn mip_level(
        self,
        mip_level: u32,
    ) -> TexelCopyTextureInfoBuilder<TextureField, Set<MipLevelField>, OriginField, AspectField>
    {
        TexelCopyTextureInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<'a, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBuilder<TextureField, MipLevelField, Unset<OriginField>, AspectField>
{
    pub fn origin(
        self,
        origin: impl Nested<wgpu::Origin3d>,
    ) -> TexelCopyTextureInfoBuilder<TextureField, MipLevelField, Set<OriginField>, AspectField>
    {
        TexelCopyTextureInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<'a, TextureField, MipLevelField, OriginField, AspectField>
    TexelCopyTextureInfoBuilder<TextureField, MipLevelField, OriginField, Unset<AspectField>>
{
    pub fn aspect(
        self,
        aspect: wgpu::TextureAspect,
    ) -> TexelCopyTextureInfoBuilder<TextureField, MipLevelField, OriginField, Set<AspectField>>
    {
        TexelCopyTextureInfoBuilder {
            texture: Unset(PhantomData),
            mip_level: Unset(PhantomData),
            origin: Unset(PhantomData),
            aspect: Unset(PhantomData),
        }
    }
}
impl<'a> TexelCopyTextureInfoBuilder<'a> {
    pub fn build(self) -> wgpu::TexelCopyTextureInfo {
        wgpu::TexelCopyTextureInfo {}
    }
}

#[derive(Debug)]
pub struct MemoryBudgetThresholdsBuilder<ForResourceCreationField, ForDeviceLossField> {
    for_resource_creation: ForResourceCreationField,
    for_device_loss: ForDeviceLossField,
}
pub fn memory_budget_thresholds()
-> MemoryBudgetThresholdsBuilder<Unset<Option<u8>>, Unset<Option<u8>>> {
    MemoryBudgetThresholdsBuilder {
        for_resource_creation: Unset(PhantomData),
        for_device_loss: Unset(PhantomData),
    }
}
impl<ForResourceCreationField, ForDeviceLossField>
    MemoryBudgetThresholdsBuilder<Unset<ForResourceCreationField>, ForDeviceLossField>
{
    pub fn for_resource_creation(
        self,
        for_resource_creation: Option<u8>,
    ) -> MemoryBudgetThresholdsBuilder<Set<ForResourceCreationField>, ForDeviceLossField> {
        MemoryBudgetThresholdsBuilder {
            for_resource_creation: Unset(PhantomData),
            for_device_loss: Unset(PhantomData),
        }
    }
}
impl<ForResourceCreationField, ForDeviceLossField>
    MemoryBudgetThresholdsBuilder<ForResourceCreationField, Unset<ForDeviceLossField>>
{
    pub fn for_device_loss(
        self,
        for_device_loss: Option<u8>,
    ) -> MemoryBudgetThresholdsBuilder<ForResourceCreationField, Set<ForDeviceLossField>> {
        MemoryBudgetThresholdsBuilder {
            for_resource_creation: Unset(PhantomData),
            for_device_loss: Unset(PhantomData),
        }
    }
}
impl MemoryBudgetThresholdsBuilder {
    pub fn build(self) -> wgpu::MemoryBudgetThresholds {
        wgpu::MemoryBudgetThresholds {}
    }
}

#[derive(Debug)]
pub struct RenderBundleDescriptorBuilder<LabelField> {
    label: LabelField,
}
pub fn render_bundle_descriptor<'a>() -> RenderBundleDescriptorBuilder<Unset<wgpu::Label<'a>>> {
    RenderBundleDescriptorBuilder {
        label: Unset(PhantomData),
    }
}
impl<'a, LabelField> RenderBundleDescriptorBuilder<Unset<LabelField>> {
    pub fn label(self, label: wgpu::Label<'a>) -> RenderBundleDescriptorBuilder<Set<LabelField>> {
        RenderBundleDescriptorBuilder {
            label: Unset(PhantomData),
        }
    }
}
impl<'a> RenderBundleDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::RenderBundleDescriptor {
        wgpu::RenderBundleDescriptor {}
    }
}

#[derive(Debug)]
pub struct Origin2DBuilder<XField, YField> {
    x: XField,
    y: YField,
}
pub fn origin_2_d() -> Origin2DBuilder<Unset<u32>, Unset<u32>> {
    Origin2DBuilder {
        x: Unset(PhantomData),
        y: Unset(PhantomData),
    }
}
impl<XField, YField> Origin2DBuilder<Unset<XField>, YField> {
    pub fn x(self, x: u32) -> Origin2DBuilder<Set<XField>, YField> {
        Origin2DBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
        }
    }
}
impl<XField, YField> Origin2DBuilder<XField, Unset<YField>> {
    pub fn y(self, y: u32) -> Origin2DBuilder<XField, Set<YField>> {
        Origin2DBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
        }
    }
}
impl Origin2DBuilder {
    pub fn build(self) -> wgpu::Origin2d {
        wgpu::Origin2d {}
    }
}

#[derive(Debug)]
pub struct MeshStateBuilder<ModuleField, EntryPointField, CompilationOptionsField> {
    module: ModuleField,
    entry_point: EntryPointField,
    compilation_options: CompilationOptionsField,
}
pub fn mesh_state<'a>() -> MeshStateBuilder<
    Unset<&'a wgpu::ShaderModule>,
    Unset<Option<&'a str>>,
    Unset<impl Nested<wgpu::PipelineCompilationOptions<'a>>>,
> {
    MeshStateBuilder {
        module: Unset(PhantomData),
        entry_point: Unset(PhantomData),
        compilation_options: Unset(PhantomData),
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField>
    MeshStateBuilder<Unset<ModuleField>, EntryPointField, CompilationOptionsField>
{
    pub fn module(
        self,
        module: &'a wgpu::ShaderModule,
    ) -> MeshStateBuilder<Set<ModuleField>, EntryPointField, CompilationOptionsField> {
        MeshStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField>
    MeshStateBuilder<ModuleField, Unset<EntryPointField>, CompilationOptionsField>
{
    pub fn entry_point(
        self,
        entry_point: Option<&'a str>,
    ) -> MeshStateBuilder<ModuleField, Set<EntryPointField>, CompilationOptionsField> {
        MeshStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField>
    MeshStateBuilder<ModuleField, EntryPointField, Unset<CompilationOptionsField>>
{
    pub fn compilation_options(
        self,
        compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    ) -> MeshStateBuilder<ModuleField, EntryPointField, Set<CompilationOptionsField>> {
        MeshStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
        }
    }
}
impl<'a> MeshStateBuilder<'a> {
    pub fn build(self) -> wgpu::MeshState {
        wgpu::MeshState {}
    }
}

#[derive(Debug)]
pub struct RenderBundleEncoderDescriptorBuilder<
    LabelField,
    ColorFormatsField,
    DepthStencilField,
    SampleCountField,
    MultiviewField,
> {
    label: LabelField,
    color_formats: ColorFormatsField,
    depth_stencil: DepthStencilField,
    sample_count: SampleCountField,
    multiview: MultiviewField,
}
pub fn render_bundle_encoder_descriptor<'a>() -> RenderBundleEncoderDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<&'a [Option<wgpu::TextureFormat>]>,
    Unset<Option<impl Nested<wgpu::RenderBundleDepthStencil>>>,
    Unset<u32>,
    Unset<Option<NonZeroU32>>,
> {
    RenderBundleEncoderDescriptorBuilder {
        label: Unset(PhantomData),
        color_formats: Unset(PhantomData),
        depth_stencil: Unset(PhantomData),
        sample_count: Unset(PhantomData),
        multiview: Unset(PhantomData),
    }
}
impl<'a, LabelField, ColorFormatsField, DepthStencilField, SampleCountField, MultiviewField>
    RenderBundleEncoderDescriptorBuilder<
        Unset<LabelField>,
        ColorFormatsField,
        DepthStencilField,
        SampleCountField,
        MultiviewField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> RenderBundleEncoderDescriptorBuilder<
        Set<LabelField>,
        ColorFormatsField,
        DepthStencilField,
        SampleCountField,
        MultiviewField,
    > {
        RenderBundleEncoderDescriptorBuilder {
            label: Unset(PhantomData),
            color_formats: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            multiview: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, ColorFormatsField, DepthStencilField, SampleCountField, MultiviewField>
    RenderBundleEncoderDescriptorBuilder<
        LabelField,
        Unset<ColorFormatsField>,
        DepthStencilField,
        SampleCountField,
        MultiviewField,
    >
{
    pub fn color_formats(
        self,
        color_formats: &'a [Option<wgpu::TextureFormat>],
    ) -> RenderBundleEncoderDescriptorBuilder<
        LabelField,
        Set<ColorFormatsField>,
        DepthStencilField,
        SampleCountField,
        MultiviewField,
    > {
        RenderBundleEncoderDescriptorBuilder {
            label: Unset(PhantomData),
            color_formats: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            multiview: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, ColorFormatsField, DepthStencilField, SampleCountField, MultiviewField>
    RenderBundleEncoderDescriptorBuilder<
        LabelField,
        ColorFormatsField,
        Unset<DepthStencilField>,
        SampleCountField,
        MultiviewField,
    >
{
    pub fn depth_stencil(
        self,
        depth_stencil: Option<impl Nested<wgpu::RenderBundleDepthStencil>>,
    ) -> RenderBundleEncoderDescriptorBuilder<
        LabelField,
        ColorFormatsField,
        Set<DepthStencilField>,
        SampleCountField,
        MultiviewField,
    > {
        RenderBundleEncoderDescriptorBuilder {
            label: Unset(PhantomData),
            color_formats: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            multiview: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, ColorFormatsField, DepthStencilField, SampleCountField, MultiviewField>
    RenderBundleEncoderDescriptorBuilder<
        LabelField,
        ColorFormatsField,
        DepthStencilField,
        Unset<SampleCountField>,
        MultiviewField,
    >
{
    pub fn sample_count(
        self,
        sample_count: u32,
    ) -> RenderBundleEncoderDescriptorBuilder<
        LabelField,
        ColorFormatsField,
        DepthStencilField,
        Set<SampleCountField>,
        MultiviewField,
    > {
        RenderBundleEncoderDescriptorBuilder {
            label: Unset(PhantomData),
            color_formats: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            multiview: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, ColorFormatsField, DepthStencilField, SampleCountField, MultiviewField>
    RenderBundleEncoderDescriptorBuilder<
        LabelField,
        ColorFormatsField,
        DepthStencilField,
        SampleCountField,
        Unset<MultiviewField>,
    >
{
    pub fn multiview(
        self,
        multiview: Option<NonZeroU32>,
    ) -> RenderBundleEncoderDescriptorBuilder<
        LabelField,
        ColorFormatsField,
        DepthStencilField,
        SampleCountField,
        Set<MultiviewField>,
    > {
        RenderBundleEncoderDescriptorBuilder {
            label: Unset(PhantomData),
            color_formats: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            multiview: Unset(PhantomData),
        }
    }
}
impl<'a> RenderBundleEncoderDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::RenderBundleEncoderDescriptor {
        wgpu::RenderBundleEncoderDescriptor {}
    }
}

#[derive(Debug)]
pub struct ExternalTextureTransferFunctionBuilder<AField, BField, GField, KField> {
    a: AField,
    b: BField,
    g: GField,
    k: KField,
}
pub fn external_texture_transfer_function()
-> ExternalTextureTransferFunctionBuilder<Unset<f32>, Unset<f32>, Unset<f32>, Unset<f32>> {
    ExternalTextureTransferFunctionBuilder {
        a: Unset(PhantomData),
        b: Unset(PhantomData),
        g: Unset(PhantomData),
        k: Unset(PhantomData),
    }
}
impl<AField, BField, GField, KField>
    ExternalTextureTransferFunctionBuilder<Unset<AField>, BField, GField, KField>
{
    pub fn a(
        self,
        a: f32,
    ) -> ExternalTextureTransferFunctionBuilder<Set<AField>, BField, GField, KField> {
        ExternalTextureTransferFunctionBuilder {
            a: Unset(PhantomData),
            b: Unset(PhantomData),
            g: Unset(PhantomData),
            k: Unset(PhantomData),
        }
    }
}
impl<AField, BField, GField, KField>
    ExternalTextureTransferFunctionBuilder<AField, Unset<BField>, GField, KField>
{
    pub fn b(
        self,
        b: f32,
    ) -> ExternalTextureTransferFunctionBuilder<AField, Set<BField>, GField, KField> {
        ExternalTextureTransferFunctionBuilder {
            a: Unset(PhantomData),
            b: Unset(PhantomData),
            g: Unset(PhantomData),
            k: Unset(PhantomData),
        }
    }
}
impl<AField, BField, GField, KField>
    ExternalTextureTransferFunctionBuilder<AField, BField, Unset<GField>, KField>
{
    pub fn g(
        self,
        g: f32,
    ) -> ExternalTextureTransferFunctionBuilder<AField, BField, Set<GField>, KField> {
        ExternalTextureTransferFunctionBuilder {
            a: Unset(PhantomData),
            b: Unset(PhantomData),
            g: Unset(PhantomData),
            k: Unset(PhantomData),
        }
    }
}
impl<AField, BField, GField, KField>
    ExternalTextureTransferFunctionBuilder<AField, BField, GField, Unset<KField>>
{
    pub fn k(
        self,
        k: f32,
    ) -> ExternalTextureTransferFunctionBuilder<AField, BField, GField, Set<KField>> {
        ExternalTextureTransferFunctionBuilder {
            a: Unset(PhantomData),
            b: Unset(PhantomData),
            g: Unset(PhantomData),
            k: Unset(PhantomData),
        }
    }
}
impl ExternalTextureTransferFunctionBuilder {
    pub fn build(self) -> wgpu::ExternalTextureTransferFunction {
        wgpu::ExternalTextureTransferFunction {}
    }
}

#[derive(Debug)]
pub struct TextureTransitionBuilder<TextureField, SelectorField, StateField> {
    texture: TextureField,
    selector: SelectorField,
    state: StateField,
}
pub fn texture_transition<T>() -> TextureTransitionBuilder<
    Unset<T>,
    Unset<Option<wgpu::wgt::TextureSelector>>,
    Unset<wgpu::TextureUses>,
> {
    TextureTransitionBuilder {
        texture: Unset(PhantomData),
        selector: Unset(PhantomData),
        state: Unset(PhantomData),
    }
}
impl<T, TextureField, SelectorField, StateField>
    TextureTransitionBuilder<Unset<TextureField>, SelectorField, StateField>
{
    pub fn texture(
        self,
        texture: T,
    ) -> TextureTransitionBuilder<Set<TextureField>, SelectorField, StateField> {
        TextureTransitionBuilder {
            texture: Unset(PhantomData),
            selector: Unset(PhantomData),
            state: Unset(PhantomData),
        }
    }
}
impl<T, TextureField, SelectorField, StateField>
    TextureTransitionBuilder<TextureField, Unset<SelectorField>, StateField>
{
    pub fn selector(
        self,
        selector: Option<wgpu::wgt::TextureSelector>,
    ) -> TextureTransitionBuilder<TextureField, Set<SelectorField>, StateField> {
        TextureTransitionBuilder {
            texture: Unset(PhantomData),
            selector: Unset(PhantomData),
            state: Unset(PhantomData),
        }
    }
}
impl<T, TextureField, SelectorField, StateField>
    TextureTransitionBuilder<TextureField, SelectorField, Unset<StateField>>
{
    pub fn state(
        self,
        state: wgpu::TextureUses,
    ) -> TextureTransitionBuilder<TextureField, SelectorField, Set<StateField>> {
        TextureTransitionBuilder {
            texture: Unset(PhantomData),
            selector: Unset(PhantomData),
            state: Unset(PhantomData),
        }
    }
}
impl<T> TextureTransitionBuilder<T> {
    pub fn build(self) -> wgpu::TextureTransition {
        wgpu::TextureTransition {}
    }
}

#[derive(Debug)]
pub struct BlendStateBuilder<ColorField, AlphaField> {
    color: ColorField,
    alpha: AlphaField,
}
pub fn blend_state() -> BlendStateBuilder<
    Unset<impl Nested<wgpu::BlendComponent>>,
    Unset<impl Nested<wgpu::BlendComponent>>,
> {
    BlendStateBuilder {
        color: Unset(PhantomData),
        alpha: Unset(PhantomData),
    }
}
impl<ColorField, AlphaField> BlendStateBuilder<Unset<ColorField>, AlphaField> {
    pub fn color(
        self,
        color: impl Nested<wgpu::BlendComponent>,
    ) -> BlendStateBuilder<Set<ColorField>, AlphaField> {
        BlendStateBuilder {
            color: Unset(PhantomData),
            alpha: Unset(PhantomData),
        }
    }
}
impl<ColorField, AlphaField> BlendStateBuilder<ColorField, Unset<AlphaField>> {
    pub fn alpha(
        self,
        alpha: impl Nested<wgpu::BlendComponent>,
    ) -> BlendStateBuilder<ColorField, Set<AlphaField>> {
        BlendStateBuilder {
            color: Unset(PhantomData),
            alpha: Unset(PhantomData),
        }
    }
}
impl BlendStateBuilder {
    pub fn build(self) -> wgpu::BlendState {
        wgpu::BlendState {}
    }
}

#[derive(Debug)]
pub struct DrawIndirectArgsBuilder<
    VertexCountField,
    InstanceCountField,
    FirstVertexField,
    FirstInstanceField,
> {
    vertex_count: VertexCountField,
    instance_count: InstanceCountField,
    first_vertex: FirstVertexField,
    first_instance: FirstInstanceField,
}
pub fn draw_indirect_args()
-> DrawIndirectArgsBuilder<Unset<u32>, Unset<u32>, Unset<u32>, Unset<u32>> {
    DrawIndirectArgsBuilder {
        vertex_count: Unset(PhantomData),
        instance_count: Unset(PhantomData),
        first_vertex: Unset(PhantomData),
        first_instance: Unset(PhantomData),
    }
}
impl<VertexCountField, InstanceCountField, FirstVertexField, FirstInstanceField>
    DrawIndirectArgsBuilder<
        Unset<VertexCountField>,
        InstanceCountField,
        FirstVertexField,
        FirstInstanceField,
    >
{
    pub fn vertex_count(
        self,
        vertex_count: u32,
    ) -> DrawIndirectArgsBuilder<
        Set<VertexCountField>,
        InstanceCountField,
        FirstVertexField,
        FirstInstanceField,
    > {
        DrawIndirectArgsBuilder {
            vertex_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<VertexCountField, InstanceCountField, FirstVertexField, FirstInstanceField>
    DrawIndirectArgsBuilder<
        VertexCountField,
        Unset<InstanceCountField>,
        FirstVertexField,
        FirstInstanceField,
    >
{
    pub fn instance_count(
        self,
        instance_count: u32,
    ) -> DrawIndirectArgsBuilder<
        VertexCountField,
        Set<InstanceCountField>,
        FirstVertexField,
        FirstInstanceField,
    > {
        DrawIndirectArgsBuilder {
            vertex_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<VertexCountField, InstanceCountField, FirstVertexField, FirstInstanceField>
    DrawIndirectArgsBuilder<
        VertexCountField,
        InstanceCountField,
        Unset<FirstVertexField>,
        FirstInstanceField,
    >
{
    pub fn first_vertex(
        self,
        first_vertex: u32,
    ) -> DrawIndirectArgsBuilder<
        VertexCountField,
        InstanceCountField,
        Set<FirstVertexField>,
        FirstInstanceField,
    > {
        DrawIndirectArgsBuilder {
            vertex_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<VertexCountField, InstanceCountField, FirstVertexField, FirstInstanceField>
    DrawIndirectArgsBuilder<
        VertexCountField,
        InstanceCountField,
        FirstVertexField,
        Unset<FirstInstanceField>,
    >
{
    pub fn first_instance(
        self,
        first_instance: u32,
    ) -> DrawIndirectArgsBuilder<
        VertexCountField,
        InstanceCountField,
        FirstVertexField,
        Set<FirstInstanceField>,
    > {
        DrawIndirectArgsBuilder {
            vertex_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl DrawIndirectArgsBuilder {
    pub fn build(self) -> wgpu::util::DrawIndirectArgs {
        wgpu::util::DrawIndirectArgs {}
    }
}

#[derive(Debug)]
pub struct CreateTlasDescriptorBuilder<LabelField, MaxInstancesField, FlagsField, UpdateModeField> {
    label: LabelField,
    max_instances: MaxInstancesField,
    flags: FlagsField,
    update_mode: UpdateModeField,
}
pub fn create_tlas_descriptor<'a>() -> CreateTlasDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<u32>,
    Unset<wgpu::wgt::AccelerationStructureFlags>,
    Unset<wgpu::wgt::AccelerationStructureUpdateMode>,
> {
    CreateTlasDescriptorBuilder {
        label: Unset(PhantomData),
        max_instances: Unset(PhantomData),
        flags: Unset(PhantomData),
        update_mode: Unset(PhantomData),
    }
}
impl<'a, LabelField, MaxInstancesField, FlagsField, UpdateModeField>
    CreateTlasDescriptorBuilder<Unset<LabelField>, MaxInstancesField, FlagsField, UpdateModeField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> CreateTlasDescriptorBuilder<Set<LabelField>, MaxInstancesField, FlagsField, UpdateModeField>
    {
        CreateTlasDescriptorBuilder {
            label: Unset(PhantomData),
            max_instances: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, MaxInstancesField, FlagsField, UpdateModeField>
    CreateTlasDescriptorBuilder<LabelField, Unset<MaxInstancesField>, FlagsField, UpdateModeField>
{
    pub fn max_instances(
        self,
        max_instances: u32,
    ) -> CreateTlasDescriptorBuilder<LabelField, Set<MaxInstancesField>, FlagsField, UpdateModeField>
    {
        CreateTlasDescriptorBuilder {
            label: Unset(PhantomData),
            max_instances: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, MaxInstancesField, FlagsField, UpdateModeField>
    CreateTlasDescriptorBuilder<LabelField, MaxInstancesField, Unset<FlagsField>, UpdateModeField>
{
    pub fn flags(
        self,
        flags: wgpu::wgt::AccelerationStructureFlags,
    ) -> CreateTlasDescriptorBuilder<LabelField, MaxInstancesField, Set<FlagsField>, UpdateModeField>
    {
        CreateTlasDescriptorBuilder {
            label: Unset(PhantomData),
            max_instances: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, MaxInstancesField, FlagsField, UpdateModeField>
    CreateTlasDescriptorBuilder<LabelField, MaxInstancesField, FlagsField, Unset<UpdateModeField>>
{
    pub fn update_mode(
        self,
        update_mode: wgpu::wgt::AccelerationStructureUpdateMode,
    ) -> CreateTlasDescriptorBuilder<LabelField, MaxInstancesField, FlagsField, Set<UpdateModeField>>
    {
        CreateTlasDescriptorBuilder {
            label: Unset(PhantomData),
            max_instances: Unset(PhantomData),
            flags: Unset(PhantomData),
            update_mode: Unset(PhantomData),
        }
    }
}
impl<'a> CreateTlasDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::CreateTlasDescriptor {
        wgpu::CreateTlasDescriptor {}
    }
}

#[derive(Debug)]
pub struct BlasBuildEntryBuilder<BlasField, GeometryField> {
    blas: BlasField,
    geometry: GeometryField,
}
pub fn blas_build_entry<'a>()
-> BlasBuildEntryBuilder<Unset<&'a wgpu::Blas>, Unset<wgpu::BlasGeometries<'a>>> {
    BlasBuildEntryBuilder {
        blas: Unset(PhantomData),
        geometry: Unset(PhantomData),
    }
}
impl<'a, BlasField, GeometryField> BlasBuildEntryBuilder<Unset<BlasField>, GeometryField> {
    pub fn blas(
        self,
        blas: &'a wgpu::Blas,
    ) -> BlasBuildEntryBuilder<Set<BlasField>, GeometryField> {
        BlasBuildEntryBuilder {
            blas: Unset(PhantomData),
            geometry: Unset(PhantomData),
        }
    }
}
impl<'a, BlasField, GeometryField> BlasBuildEntryBuilder<BlasField, Unset<GeometryField>> {
    pub fn geometry(
        self,
        geometry: wgpu::BlasGeometries<'a>,
    ) -> BlasBuildEntryBuilder<BlasField, Set<GeometryField>> {
        BlasBuildEntryBuilder {
            blas: Unset(PhantomData),
            geometry: Unset(PhantomData),
        }
    }
}
impl<'a> BlasBuildEntryBuilder<'a> {
    pub fn build(self) -> wgpu::BlasBuildEntry {
        wgpu::BlasBuildEntry {}
    }
}

#[derive(Debug)]
pub struct BindGroupLayoutEntryBuilder<BindingField, VisibilityField, TyField, CountField> {
    binding: BindingField,
    visibility: VisibilityField,
    ty: TyField,
    count: CountField,
}
pub fn bind_group_layout_entry() -> BindGroupLayoutEntryBuilder<
    Unset<u32>,
    Unset<wgpu::ShaderStages>,
    Unset<wgpu::BindingType>,
    Unset<Option<NonZeroU32>>,
> {
    BindGroupLayoutEntryBuilder {
        binding: Unset(PhantomData),
        visibility: Unset(PhantomData),
        ty: Unset(PhantomData),
        count: Unset(PhantomData),
    }
}
impl<BindingField, VisibilityField, TyField, CountField>
    BindGroupLayoutEntryBuilder<Unset<BindingField>, VisibilityField, TyField, CountField>
{
    pub fn binding(
        self,
        binding: u32,
    ) -> BindGroupLayoutEntryBuilder<Set<BindingField>, VisibilityField, TyField, CountField> {
        BindGroupLayoutEntryBuilder {
            binding: Unset(PhantomData),
            visibility: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl<BindingField, VisibilityField, TyField, CountField>
    BindGroupLayoutEntryBuilder<BindingField, Unset<VisibilityField>, TyField, CountField>
{
    pub fn visibility(
        self,
        visibility: wgpu::ShaderStages,
    ) -> BindGroupLayoutEntryBuilder<BindingField, Set<VisibilityField>, TyField, CountField> {
        BindGroupLayoutEntryBuilder {
            binding: Unset(PhantomData),
            visibility: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl<BindingField, VisibilityField, TyField, CountField>
    BindGroupLayoutEntryBuilder<BindingField, VisibilityField, Unset<TyField>, CountField>
{
    pub fn ty(
        self,
        ty: wgpu::BindingType,
    ) -> BindGroupLayoutEntryBuilder<BindingField, VisibilityField, Set<TyField>, CountField> {
        BindGroupLayoutEntryBuilder {
            binding: Unset(PhantomData),
            visibility: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl<BindingField, VisibilityField, TyField, CountField>
    BindGroupLayoutEntryBuilder<BindingField, VisibilityField, TyField, Unset<CountField>>
{
    pub fn count(
        self,
        count: Option<NonZeroU32>,
    ) -> BindGroupLayoutEntryBuilder<BindingField, VisibilityField, TyField, Set<CountField>> {
        BindGroupLayoutEntryBuilder {
            binding: Unset(PhantomData),
            visibility: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl BindGroupLayoutEntryBuilder {
    pub fn build(self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {}
    }
}

#[derive(Debug)]
pub struct ComputePassDescriptorBuilder<LabelField, TimestampWritesField> {
    label: LabelField,
    timestamp_writes: TimestampWritesField,
}
pub fn compute_pass_descriptor<'a>() -> ComputePassDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<Option<impl Nested<wgpu::ComputePassTimestampWrites<'a>>>>,
> {
    ComputePassDescriptorBuilder {
        label: Unset(PhantomData),
        timestamp_writes: Unset(PhantomData),
    }
}
impl<'a, LabelField, TimestampWritesField>
    ComputePassDescriptorBuilder<Unset<LabelField>, TimestampWritesField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> ComputePassDescriptorBuilder<Set<LabelField>, TimestampWritesField> {
        ComputePassDescriptorBuilder {
            label: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, TimestampWritesField>
    ComputePassDescriptorBuilder<LabelField, Unset<TimestampWritesField>>
{
    pub fn timestamp_writes(
        self,
        timestamp_writes: Option<impl Nested<wgpu::ComputePassTimestampWrites<'a>>>,
    ) -> ComputePassDescriptorBuilder<LabelField, Set<TimestampWritesField>> {
        ComputePassDescriptorBuilder {
            label: Unset(PhantomData),
            timestamp_writes: Unset(PhantomData),
        }
    }
}
impl<'a> ComputePassDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::ComputePassDescriptor {
        wgpu::ComputePassDescriptor {}
    }
}

#[derive(Debug)]
pub struct RenderPassDepthStencilAttachmentBuilder<ViewField, DepthOpsField, StencilOpsField> {
    view: ViewField,
    depth_ops: DepthOpsField,
    stencil_ops: StencilOpsField,
}
pub fn render_pass_depth_stencil_attachment<'tex>() -> RenderPassDepthStencilAttachmentBuilder<
    Unset<&'tex wgpu::TextureView>,
    Unset<Option<impl Nested<wgpu::Operations<f32>>>>,
    Unset<Option<impl Nested<wgpu::Operations<u32>>>>,
> {
    RenderPassDepthStencilAttachmentBuilder {
        view: Unset(PhantomData),
        depth_ops: Unset(PhantomData),
        stencil_ops: Unset(PhantomData),
    }
}
impl<'tex, ViewField, DepthOpsField, StencilOpsField>
    RenderPassDepthStencilAttachmentBuilder<Unset<ViewField>, DepthOpsField, StencilOpsField>
{
    pub fn view(
        self,
        view: &'tex wgpu::TextureView,
    ) -> RenderPassDepthStencilAttachmentBuilder<Set<ViewField>, DepthOpsField, StencilOpsField>
    {
        RenderPassDepthStencilAttachmentBuilder {
            view: Unset(PhantomData),
            depth_ops: Unset(PhantomData),
            stencil_ops: Unset(PhantomData),
        }
    }
}
impl<'tex, ViewField, DepthOpsField, StencilOpsField>
    RenderPassDepthStencilAttachmentBuilder<ViewField, Unset<DepthOpsField>, StencilOpsField>
{
    pub fn depth_ops(
        self,
        depth_ops: Option<impl Nested<wgpu::Operations<f32>>>,
    ) -> RenderPassDepthStencilAttachmentBuilder<ViewField, Set<DepthOpsField>, StencilOpsField>
    {
        RenderPassDepthStencilAttachmentBuilder {
            view: Unset(PhantomData),
            depth_ops: Unset(PhantomData),
            stencil_ops: Unset(PhantomData),
        }
    }
}
impl<'tex, ViewField, DepthOpsField, StencilOpsField>
    RenderPassDepthStencilAttachmentBuilder<ViewField, DepthOpsField, Unset<StencilOpsField>>
{
    pub fn stencil_ops(
        self,
        stencil_ops: Option<impl Nested<wgpu::Operations<u32>>>,
    ) -> RenderPassDepthStencilAttachmentBuilder<ViewField, DepthOpsField, Set<StencilOpsField>>
    {
        RenderPassDepthStencilAttachmentBuilder {
            view: Unset(PhantomData),
            depth_ops: Unset(PhantomData),
            stencil_ops: Unset(PhantomData),
        }
    }
}
impl<'tex> RenderPassDepthStencilAttachmentBuilder<'tex> {
    pub fn build(self) -> wgpu::RenderPassDepthStencilAttachment {
        wgpu::RenderPassDepthStencilAttachment {}
    }
}

#[derive(Debug)]
pub struct DownlevelLimitsBuilder {}
pub fn downlevel_limits() -> DownlevelLimitsBuilder {
    DownlevelLimitsBuilder {}
}
impl DownlevelLimitsBuilder {
    pub fn build(self) -> wgpu::DownlevelLimits {
        wgpu::DownlevelLimits {}
    }
}

#[derive(Debug)]
pub struct ColorTargetStateBuilder<FormatField, BlendField, WriteMaskField> {
    format: FormatField,
    blend: BlendField,
    write_mask: WriteMaskField,
}
pub fn color_target_state() -> ColorTargetStateBuilder<
    Unset<wgpu::TextureFormat>,
    Unset<Option<impl Nested<wgpu::BlendState>>>,
    Unset<wgpu::ColorWrites>,
> {
    ColorTargetStateBuilder {
        format: Unset(PhantomData),
        blend: Unset(PhantomData),
        write_mask: Unset(PhantomData),
    }
}
impl<FormatField, BlendField, WriteMaskField>
    ColorTargetStateBuilder<Unset<FormatField>, BlendField, WriteMaskField>
{
    pub fn format(
        self,
        format: wgpu::TextureFormat,
    ) -> ColorTargetStateBuilder<Set<FormatField>, BlendField, WriteMaskField> {
        ColorTargetStateBuilder {
            format: Unset(PhantomData),
            blend: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl<FormatField, BlendField, WriteMaskField>
    ColorTargetStateBuilder<FormatField, Unset<BlendField>, WriteMaskField>
{
    pub fn blend(
        self,
        blend: Option<impl Nested<wgpu::BlendState>>,
    ) -> ColorTargetStateBuilder<FormatField, Set<BlendField>, WriteMaskField> {
        ColorTargetStateBuilder {
            format: Unset(PhantomData),
            blend: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl<FormatField, BlendField, WriteMaskField>
    ColorTargetStateBuilder<FormatField, BlendField, Unset<WriteMaskField>>
{
    pub fn write_mask(
        self,
        write_mask: wgpu::ColorWrites,
    ) -> ColorTargetStateBuilder<FormatField, BlendField, Set<WriteMaskField>> {
        ColorTargetStateBuilder {
            format: Unset(PhantomData),
            blend: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl ColorTargetStateBuilder {
    pub fn build(self) -> wgpu::ColorTargetState {
        wgpu::ColorTargetState {}
    }
}

#[derive(Debug)]
pub struct RequestAdapterOptionsBuilder<
    PowerPreferenceField,
    ForceFallbackAdapterField,
    CompatibleSurfaceField,
> {
    power_preference: PowerPreferenceField,
    force_fallback_adapter: ForceFallbackAdapterField,
    compatible_surface: CompatibleSurfaceField,
}
pub fn request_adapter_options<'a, 'b>() -> RequestAdapterOptionsBuilder<
    Unset<wgpu::PowerPreference>,
    Unset<bool>,
    Unset<Option<&'a wgpu::Surface<'b>>>,
> {
    RequestAdapterOptionsBuilder {
        power_preference: Unset(PhantomData),
        force_fallback_adapter: Unset(PhantomData),
        compatible_surface: Unset(PhantomData),
    }
}
impl<'a, 'b, PowerPreferenceField, ForceFallbackAdapterField, CompatibleSurfaceField>
    RequestAdapterOptionsBuilder<
        Unset<PowerPreferenceField>,
        ForceFallbackAdapterField,
        CompatibleSurfaceField,
    >
{
    pub fn power_preference(
        self,
        power_preference: wgpu::PowerPreference,
    ) -> RequestAdapterOptionsBuilder<
        Set<PowerPreferenceField>,
        ForceFallbackAdapterField,
        CompatibleSurfaceField,
    > {
        RequestAdapterOptionsBuilder {
            power_preference: Unset(PhantomData),
            force_fallback_adapter: Unset(PhantomData),
            compatible_surface: Unset(PhantomData),
        }
    }
}
impl<'a, 'b, PowerPreferenceField, ForceFallbackAdapterField, CompatibleSurfaceField>
    RequestAdapterOptionsBuilder<
        PowerPreferenceField,
        Unset<ForceFallbackAdapterField>,
        CompatibleSurfaceField,
    >
{
    pub fn force_fallback_adapter(
        self,
        force_fallback_adapter: bool,
    ) -> RequestAdapterOptionsBuilder<
        PowerPreferenceField,
        Set<ForceFallbackAdapterField>,
        CompatibleSurfaceField,
    > {
        RequestAdapterOptionsBuilder {
            power_preference: Unset(PhantomData),
            force_fallback_adapter: Unset(PhantomData),
            compatible_surface: Unset(PhantomData),
        }
    }
}
impl<'a, 'b, PowerPreferenceField, ForceFallbackAdapterField, CompatibleSurfaceField>
    RequestAdapterOptionsBuilder<
        PowerPreferenceField,
        ForceFallbackAdapterField,
        Unset<CompatibleSurfaceField>,
    >
{
    pub fn compatible_surface(
        self,
        compatible_surface: Option<&'a wgpu::Surface<'b>>,
    ) -> RequestAdapterOptionsBuilder<
        PowerPreferenceField,
        ForceFallbackAdapterField,
        Set<CompatibleSurfaceField>,
    > {
        RequestAdapterOptionsBuilder {
            power_preference: Unset(PhantomData),
            force_fallback_adapter: Unset(PhantomData),
            compatible_surface: Unset(PhantomData),
        }
    }
}
impl<'a, 'b> RequestAdapterOptionsBuilder<'a, 'b> {
    pub fn build(self) -> wgpu::RequestAdapterOptions {
        wgpu::RequestAdapterOptions {}
    }
}

#[derive(Debug)]
pub struct OperationsBuilder<LoadField, StoreField> {
    load: LoadField,
    store: StoreField,
}
pub fn operations<V: Default>() -> OperationsBuilder<Unset<wgpu::LoadOp<V>>, Unset<wgpu::StoreOp>> {
    OperationsBuilder {
        load: Unset(PhantomData),
        store: Unset(PhantomData),
    }
}
impl<V, LoadField, StoreField> OperationsBuilder<Unset<LoadField>, StoreField> {
    pub fn load(self, load: wgpu::LoadOp<V>) -> OperationsBuilder<Set<LoadField>, StoreField> {
        OperationsBuilder {
            load: Unset(PhantomData),
            store: Unset(PhantomData),
        }
    }
}
impl<V, LoadField, StoreField> OperationsBuilder<LoadField, Unset<StoreField>> {
    pub fn store(self, store: wgpu::StoreOp) -> OperationsBuilder<LoadField, Set<StoreField>> {
        OperationsBuilder {
            load: Unset(PhantomData),
            store: Unset(PhantomData),
        }
    }
}
impl<V> OperationsBuilder<V> {
    pub fn build(self) -> wgpu::Operations {
        wgpu::Operations {}
    }
}

#[derive(Debug)]
pub struct NoopBackendOptionsBuilder<EnableField> {
    enable: EnableField,
}
pub fn noop_backend_options() -> NoopBackendOptionsBuilder<Unset<bool>> {
    NoopBackendOptionsBuilder {
        enable: Unset(PhantomData),
    }
}
impl<EnableField> NoopBackendOptionsBuilder<Unset<EnableField>> {
    pub fn enable(self, enable: bool) -> NoopBackendOptionsBuilder<Set<EnableField>> {
        NoopBackendOptionsBuilder {
            enable: Unset(PhantomData),
        }
    }
}
impl NoopBackendOptionsBuilder {
    pub fn build(self) -> wgpu::NoopBackendOptions {
        wgpu::NoopBackendOptions {}
    }
}

#[derive(Debug)]
pub struct BufferInitDescriptorBuilder<LabelField, ContentsField, UsageField> {
    label: LabelField,
    contents: ContentsField,
    usage: UsageField,
}
pub fn buffer_init_descriptor<'a>()
-> BufferInitDescriptorBuilder<Unset<wgpu::Label<'a>>, Unset<&'a [u8]>, Unset<wgpu::BufferUsages>> {
    BufferInitDescriptorBuilder {
        label: Unset(PhantomData),
        contents: Unset(PhantomData),
        usage: Unset(PhantomData),
    }
}
impl<'a, LabelField, ContentsField, UsageField>
    BufferInitDescriptorBuilder<Unset<LabelField>, ContentsField, UsageField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> BufferInitDescriptorBuilder<Set<LabelField>, ContentsField, UsageField> {
        BufferInitDescriptorBuilder {
            label: Unset(PhantomData),
            contents: Unset(PhantomData),
            usage: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, ContentsField, UsageField>
    BufferInitDescriptorBuilder<LabelField, Unset<ContentsField>, UsageField>
{
    pub fn contents(
        self,
        contents: &'a [u8],
    ) -> BufferInitDescriptorBuilder<LabelField, Set<ContentsField>, UsageField> {
        BufferInitDescriptorBuilder {
            label: Unset(PhantomData),
            contents: Unset(PhantomData),
            usage: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, ContentsField, UsageField>
    BufferInitDescriptorBuilder<LabelField, ContentsField, Unset<UsageField>>
{
    pub fn usage(
        self,
        usage: wgpu::BufferUsages,
    ) -> BufferInitDescriptorBuilder<LabelField, ContentsField, Set<UsageField>> {
        BufferInitDescriptorBuilder {
            label: Unset(PhantomData),
            contents: Unset(PhantomData),
            usage: Unset(PhantomData),
        }
    }
}
impl<'a> BufferInitDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::util::BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {}
    }
}

#[derive(Debug)]
pub struct ShaderRuntimeChecksBuilder<BoundsChecksField, ForceLoopBoundingField> {
    bounds_checks: BoundsChecksField,
    force_loop_bounding: ForceLoopBoundingField,
}
pub fn shader_runtime_checks() -> ShaderRuntimeChecksBuilder<Unset<bool>, Unset<bool>> {
    ShaderRuntimeChecksBuilder {
        bounds_checks: Unset(PhantomData),
        force_loop_bounding: Unset(PhantomData),
    }
}
impl<BoundsChecksField, ForceLoopBoundingField>
    ShaderRuntimeChecksBuilder<Unset<BoundsChecksField>, ForceLoopBoundingField>
{
    pub fn bounds_checks(
        self,
        bounds_checks: bool,
    ) -> ShaderRuntimeChecksBuilder<Set<BoundsChecksField>, ForceLoopBoundingField> {
        ShaderRuntimeChecksBuilder {
            bounds_checks: Unset(PhantomData),
            force_loop_bounding: Unset(PhantomData),
        }
    }
}
impl<BoundsChecksField, ForceLoopBoundingField>
    ShaderRuntimeChecksBuilder<BoundsChecksField, Unset<ForceLoopBoundingField>>
{
    pub fn force_loop_bounding(
        self,
        force_loop_bounding: bool,
    ) -> ShaderRuntimeChecksBuilder<BoundsChecksField, Set<ForceLoopBoundingField>> {
        ShaderRuntimeChecksBuilder {
            bounds_checks: Unset(PhantomData),
            force_loop_bounding: Unset(PhantomData),
        }
    }
}
impl ShaderRuntimeChecksBuilder {
    pub fn build(self) -> wgpu::ShaderRuntimeChecks {
        wgpu::ShaderRuntimeChecks {}
    }
}

#[derive(Debug)]
pub struct DrawIndexedIndirectArgsBuilder<
    IndexCountField,
    InstanceCountField,
    FirstIndexField,
    BaseVertexField,
    FirstInstanceField,
> {
    index_count: IndexCountField,
    instance_count: InstanceCountField,
    first_index: FirstIndexField,
    base_vertex: BaseVertexField,
    first_instance: FirstInstanceField,
}
pub fn draw_indexed_indirect_args()
-> DrawIndexedIndirectArgsBuilder<Unset<u32>, Unset<u32>, Unset<u32>, Unset<i32>, Unset<u32>> {
    DrawIndexedIndirectArgsBuilder {
        index_count: Unset(PhantomData),
        instance_count: Unset(PhantomData),
        first_index: Unset(PhantomData),
        base_vertex: Unset(PhantomData),
        first_instance: Unset(PhantomData),
    }
}
impl<IndexCountField, InstanceCountField, FirstIndexField, BaseVertexField, FirstInstanceField>
    DrawIndexedIndirectArgsBuilder<
        Unset<IndexCountField>,
        InstanceCountField,
        FirstIndexField,
        BaseVertexField,
        FirstInstanceField,
    >
{
    pub fn index_count(
        self,
        index_count: u32,
    ) -> DrawIndexedIndirectArgsBuilder<
        Set<IndexCountField>,
        InstanceCountField,
        FirstIndexField,
        BaseVertexField,
        FirstInstanceField,
    > {
        DrawIndexedIndirectArgsBuilder {
            index_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_index: Unset(PhantomData),
            base_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<IndexCountField, InstanceCountField, FirstIndexField, BaseVertexField, FirstInstanceField>
    DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        Unset<InstanceCountField>,
        FirstIndexField,
        BaseVertexField,
        FirstInstanceField,
    >
{
    pub fn instance_count(
        self,
        instance_count: u32,
    ) -> DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        Set<InstanceCountField>,
        FirstIndexField,
        BaseVertexField,
        FirstInstanceField,
    > {
        DrawIndexedIndirectArgsBuilder {
            index_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_index: Unset(PhantomData),
            base_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<IndexCountField, InstanceCountField, FirstIndexField, BaseVertexField, FirstInstanceField>
    DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        InstanceCountField,
        Unset<FirstIndexField>,
        BaseVertexField,
        FirstInstanceField,
    >
{
    pub fn first_index(
        self,
        first_index: u32,
    ) -> DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        InstanceCountField,
        Set<FirstIndexField>,
        BaseVertexField,
        FirstInstanceField,
    > {
        DrawIndexedIndirectArgsBuilder {
            index_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_index: Unset(PhantomData),
            base_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<IndexCountField, InstanceCountField, FirstIndexField, BaseVertexField, FirstInstanceField>
    DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        InstanceCountField,
        FirstIndexField,
        Unset<BaseVertexField>,
        FirstInstanceField,
    >
{
    pub fn base_vertex(
        self,
        base_vertex: i32,
    ) -> DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        InstanceCountField,
        FirstIndexField,
        Set<BaseVertexField>,
        FirstInstanceField,
    > {
        DrawIndexedIndirectArgsBuilder {
            index_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_index: Unset(PhantomData),
            base_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl<IndexCountField, InstanceCountField, FirstIndexField, BaseVertexField, FirstInstanceField>
    DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        InstanceCountField,
        FirstIndexField,
        BaseVertexField,
        Unset<FirstInstanceField>,
    >
{
    pub fn first_instance(
        self,
        first_instance: u32,
    ) -> DrawIndexedIndirectArgsBuilder<
        IndexCountField,
        InstanceCountField,
        FirstIndexField,
        BaseVertexField,
        Set<FirstInstanceField>,
    > {
        DrawIndexedIndirectArgsBuilder {
            index_count: Unset(PhantomData),
            instance_count: Unset(PhantomData),
            first_index: Unset(PhantomData),
            base_vertex: Unset(PhantomData),
            first_instance: Unset(PhantomData),
        }
    }
}
impl DrawIndexedIndirectArgsBuilder {
    pub fn build(self) -> wgpu::util::DrawIndexedIndirectArgs {
        wgpu::util::DrawIndexedIndirectArgs {}
    }
}

#[derive(Debug)]
pub struct RenderPassColorAttachmentBuilder<
    ViewField,
    DepthSliceField,
    ResolveTargetField,
    OpsField,
> {
    view: ViewField,
    depth_slice: DepthSliceField,
    resolve_target: ResolveTargetField,
    ops: OpsField,
}
pub fn render_pass_color_attachment<'tex>() -> RenderPassColorAttachmentBuilder<
    Unset<&'tex wgpu::TextureView>,
    Unset<Option<u32>>,
    Unset<Option<&'tex wgpu::TextureView>>,
    Unset<impl Nested<wgpu::Operations<wgpu::Color>>>,
> {
    RenderPassColorAttachmentBuilder {
        view: Unset(PhantomData),
        depth_slice: Unset(PhantomData),
        resolve_target: Unset(PhantomData),
        ops: Unset(PhantomData),
    }
}
impl<'tex, ViewField, DepthSliceField, ResolveTargetField, OpsField>
    RenderPassColorAttachmentBuilder<
        Unset<ViewField>,
        DepthSliceField,
        ResolveTargetField,
        OpsField,
    >
{
    pub fn view(
        self,
        view: &'tex wgpu::TextureView,
    ) -> RenderPassColorAttachmentBuilder<
        Set<ViewField>,
        DepthSliceField,
        ResolveTargetField,
        OpsField,
    > {
        RenderPassColorAttachmentBuilder {
            view: Unset(PhantomData),
            depth_slice: Unset(PhantomData),
            resolve_target: Unset(PhantomData),
            ops: Unset(PhantomData),
        }
    }
}
impl<'tex, ViewField, DepthSliceField, ResolveTargetField, OpsField>
    RenderPassColorAttachmentBuilder<
        ViewField,
        Unset<DepthSliceField>,
        ResolveTargetField,
        OpsField,
    >
{
    pub fn depth_slice(
        self,
        depth_slice: Option<u32>,
    ) -> RenderPassColorAttachmentBuilder<
        ViewField,
        Set<DepthSliceField>,
        ResolveTargetField,
        OpsField,
    > {
        RenderPassColorAttachmentBuilder {
            view: Unset(PhantomData),
            depth_slice: Unset(PhantomData),
            resolve_target: Unset(PhantomData),
            ops: Unset(PhantomData),
        }
    }
}
impl<'tex, ViewField, DepthSliceField, ResolveTargetField, OpsField>
    RenderPassColorAttachmentBuilder<
        ViewField,
        DepthSliceField,
        Unset<ResolveTargetField>,
        OpsField,
    >
{
    pub fn resolve_target(
        self,
        resolve_target: Option<&'tex wgpu::TextureView>,
    ) -> RenderPassColorAttachmentBuilder<
        ViewField,
        DepthSliceField,
        Set<ResolveTargetField>,
        OpsField,
    > {
        RenderPassColorAttachmentBuilder {
            view: Unset(PhantomData),
            depth_slice: Unset(PhantomData),
            resolve_target: Unset(PhantomData),
            ops: Unset(PhantomData),
        }
    }
}
impl<'tex, ViewField, DepthSliceField, ResolveTargetField, OpsField>
    RenderPassColorAttachmentBuilder<
        ViewField,
        DepthSliceField,
        ResolveTargetField,
        Unset<OpsField>,
    >
{
    pub fn ops(
        self,
        ops: impl Nested<wgpu::Operations<wgpu::Color>>,
    ) -> RenderPassColorAttachmentBuilder<
        ViewField,
        DepthSliceField,
        ResolveTargetField,
        Set<OpsField>,
    > {
        RenderPassColorAttachmentBuilder {
            view: Unset(PhantomData),
            depth_slice: Unset(PhantomData),
            resolve_target: Unset(PhantomData),
            ops: Unset(PhantomData),
        }
    }
}
impl<'tex> RenderPassColorAttachmentBuilder<'tex> {
    pub fn build(self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {}
    }
}

#[derive(Debug)]
pub struct TaskStateBuilder<ModuleField, EntryPointField, CompilationOptionsField> {
    module: ModuleField,
    entry_point: EntryPointField,
    compilation_options: CompilationOptionsField,
}
pub fn task_state<'a>() -> TaskStateBuilder<
    Unset<&'a wgpu::ShaderModule>,
    Unset<Option<&'a str>>,
    Unset<impl Nested<wgpu::PipelineCompilationOptions<'a>>>,
> {
    TaskStateBuilder {
        module: Unset(PhantomData),
        entry_point: Unset(PhantomData),
        compilation_options: Unset(PhantomData),
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField>
    TaskStateBuilder<Unset<ModuleField>, EntryPointField, CompilationOptionsField>
{
    pub fn module(
        self,
        module: &'a wgpu::ShaderModule,
    ) -> TaskStateBuilder<Set<ModuleField>, EntryPointField, CompilationOptionsField> {
        TaskStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField>
    TaskStateBuilder<ModuleField, Unset<EntryPointField>, CompilationOptionsField>
{
    pub fn entry_point(
        self,
        entry_point: Option<&'a str>,
    ) -> TaskStateBuilder<ModuleField, Set<EntryPointField>, CompilationOptionsField> {
        TaskStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField>
    TaskStateBuilder<ModuleField, EntryPointField, Unset<CompilationOptionsField>>
{
    pub fn compilation_options(
        self,
        compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    ) -> TaskStateBuilder<ModuleField, EntryPointField, Set<CompilationOptionsField>> {
        TaskStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
        }
    }
}
impl<'a> TaskStateBuilder<'a> {
    pub fn build(self) -> wgpu::TaskState {
        wgpu::TaskState {}
    }
}

#[derive(Debug)]
pub struct RenderBundleDepthStencilBuilder<FormatField, DepthReadOnlyField, StencilReadOnlyField> {
    format: FormatField,
    depth_read_only: DepthReadOnlyField,
    stencil_read_only: StencilReadOnlyField,
}
pub fn render_bundle_depth_stencil()
-> RenderBundleDepthStencilBuilder<Unset<wgpu::TextureFormat>, Unset<bool>, Unset<bool>> {
    RenderBundleDepthStencilBuilder {
        format: Unset(PhantomData),
        depth_read_only: Unset(PhantomData),
        stencil_read_only: Unset(PhantomData),
    }
}
impl<FormatField, DepthReadOnlyField, StencilReadOnlyField>
    RenderBundleDepthStencilBuilder<Unset<FormatField>, DepthReadOnlyField, StencilReadOnlyField>
{
    pub fn format(
        self,
        format: wgpu::TextureFormat,
    ) -> RenderBundleDepthStencilBuilder<Set<FormatField>, DepthReadOnlyField, StencilReadOnlyField>
    {
        RenderBundleDepthStencilBuilder {
            format: Unset(PhantomData),
            depth_read_only: Unset(PhantomData),
            stencil_read_only: Unset(PhantomData),
        }
    }
}
impl<FormatField, DepthReadOnlyField, StencilReadOnlyField>
    RenderBundleDepthStencilBuilder<FormatField, Unset<DepthReadOnlyField>, StencilReadOnlyField>
{
    pub fn depth_read_only(
        self,
        depth_read_only: bool,
    ) -> RenderBundleDepthStencilBuilder<FormatField, Set<DepthReadOnlyField>, StencilReadOnlyField>
    {
        RenderBundleDepthStencilBuilder {
            format: Unset(PhantomData),
            depth_read_only: Unset(PhantomData),
            stencil_read_only: Unset(PhantomData),
        }
    }
}
impl<FormatField, DepthReadOnlyField, StencilReadOnlyField>
    RenderBundleDepthStencilBuilder<FormatField, DepthReadOnlyField, Unset<StencilReadOnlyField>>
{
    pub fn stencil_read_only(
        self,
        stencil_read_only: bool,
    ) -> RenderBundleDepthStencilBuilder<FormatField, DepthReadOnlyField, Set<StencilReadOnlyField>>
    {
        RenderBundleDepthStencilBuilder {
            format: Unset(PhantomData),
            depth_read_only: Unset(PhantomData),
            stencil_read_only: Unset(PhantomData),
        }
    }
}
impl RenderBundleDepthStencilBuilder {
    pub fn build(self) -> wgpu::RenderBundleDepthStencil {
        wgpu::RenderBundleDepthStencil {}
    }
}

#[derive(Debug)]
pub struct StencilFaceStateBuilder<CompareField, FailOpField, DepthFailOpField, PassOpField> {
    compare: CompareField,
    fail_op: FailOpField,
    depth_fail_op: DepthFailOpField,
    pass_op: PassOpField,
}
pub fn stencil_face_state() -> StencilFaceStateBuilder<
    Unset<wgpu::CompareFunction>,
    Unset<wgpu::StencilOperation>,
    Unset<wgpu::StencilOperation>,
    Unset<wgpu::StencilOperation>,
> {
    StencilFaceStateBuilder {
        compare: Unset(PhantomData),
        fail_op: Unset(PhantomData),
        depth_fail_op: Unset(PhantomData),
        pass_op: Unset(PhantomData),
    }
}
impl<CompareField, FailOpField, DepthFailOpField, PassOpField>
    StencilFaceStateBuilder<Unset<CompareField>, FailOpField, DepthFailOpField, PassOpField>
{
    pub fn compare(
        self,
        compare: wgpu::CompareFunction,
    ) -> StencilFaceStateBuilder<Set<CompareField>, FailOpField, DepthFailOpField, PassOpField>
    {
        StencilFaceStateBuilder {
            compare: Unset(PhantomData),
            fail_op: Unset(PhantomData),
            depth_fail_op: Unset(PhantomData),
            pass_op: Unset(PhantomData),
        }
    }
}
impl<CompareField, FailOpField, DepthFailOpField, PassOpField>
    StencilFaceStateBuilder<CompareField, Unset<FailOpField>, DepthFailOpField, PassOpField>
{
    pub fn fail_op(
        self,
        fail_op: wgpu::StencilOperation,
    ) -> StencilFaceStateBuilder<CompareField, Set<FailOpField>, DepthFailOpField, PassOpField>
    {
        StencilFaceStateBuilder {
            compare: Unset(PhantomData),
            fail_op: Unset(PhantomData),
            depth_fail_op: Unset(PhantomData),
            pass_op: Unset(PhantomData),
        }
    }
}
impl<CompareField, FailOpField, DepthFailOpField, PassOpField>
    StencilFaceStateBuilder<CompareField, FailOpField, Unset<DepthFailOpField>, PassOpField>
{
    pub fn depth_fail_op(
        self,
        depth_fail_op: wgpu::StencilOperation,
    ) -> StencilFaceStateBuilder<CompareField, FailOpField, Set<DepthFailOpField>, PassOpField>
    {
        StencilFaceStateBuilder {
            compare: Unset(PhantomData),
            fail_op: Unset(PhantomData),
            depth_fail_op: Unset(PhantomData),
            pass_op: Unset(PhantomData),
        }
    }
}
impl<CompareField, FailOpField, DepthFailOpField, PassOpField>
    StencilFaceStateBuilder<CompareField, FailOpField, DepthFailOpField, Unset<PassOpField>>
{
    pub fn pass_op(
        self,
        pass_op: wgpu::StencilOperation,
    ) -> StencilFaceStateBuilder<CompareField, FailOpField, DepthFailOpField, Set<PassOpField>>
    {
        StencilFaceStateBuilder {
            compare: Unset(PhantomData),
            fail_op: Unset(PhantomData),
            depth_fail_op: Unset(PhantomData),
            pass_op: Unset(PhantomData),
        }
    }
}
impl StencilFaceStateBuilder {
    pub fn build(self) -> wgpu::StencilFaceState {
        wgpu::StencilFaceState {}
    }
}

#[derive(Debug)]
pub struct ColorBuilder<RField, GField, BField, AField> {
    r: RField,
    g: GField,
    b: BField,
    a: AField,
}
pub fn color() -> ColorBuilder<Unset<f64>, Unset<f64>, Unset<f64>, Unset<f64>> {
    ColorBuilder {
        r: Unset(PhantomData),
        g: Unset(PhantomData),
        b: Unset(PhantomData),
        a: Unset(PhantomData),
    }
}
impl<RField, GField, BField, AField> ColorBuilder<Unset<RField>, GField, BField, AField> {
    pub fn r(self, r: f64) -> ColorBuilder<Set<RField>, GField, BField, AField> {
        ColorBuilder {
            r: Unset(PhantomData),
            g: Unset(PhantomData),
            b: Unset(PhantomData),
            a: Unset(PhantomData),
        }
    }
}
impl<RField, GField, BField, AField> ColorBuilder<RField, Unset<GField>, BField, AField> {
    pub fn g(self, g: f64) -> ColorBuilder<RField, Set<GField>, BField, AField> {
        ColorBuilder {
            r: Unset(PhantomData),
            g: Unset(PhantomData),
            b: Unset(PhantomData),
            a: Unset(PhantomData),
        }
    }
}
impl<RField, GField, BField, AField> ColorBuilder<RField, GField, Unset<BField>, AField> {
    pub fn b(self, b: f64) -> ColorBuilder<RField, GField, Set<BField>, AField> {
        ColorBuilder {
            r: Unset(PhantomData),
            g: Unset(PhantomData),
            b: Unset(PhantomData),
            a: Unset(PhantomData),
        }
    }
}
impl<RField, GField, BField, AField> ColorBuilder<RField, GField, BField, Unset<AField>> {
    pub fn a(self, a: f64) -> ColorBuilder<RField, GField, BField, Set<AField>> {
        ColorBuilder {
            r: Unset(PhantomData),
            g: Unset(PhantomData),
            b: Unset(PhantomData),
            a: Unset(PhantomData),
        }
    }
}
impl ColorBuilder {
    pub fn build(self) -> wgpu::Color {
        wgpu::Color {}
    }
}

#[derive(Debug)]
pub struct BufferTransitionBuilder<BufferField, StateField> {
    buffer: BufferField,
    state: StateField,
}
pub fn buffer_transition<T>() -> BufferTransitionBuilder<Unset<T>, Unset<wgpu::BufferUses>> {
    BufferTransitionBuilder {
        buffer: Unset(PhantomData),
        state: Unset(PhantomData),
    }
}
impl<T, BufferField, StateField> BufferTransitionBuilder<Unset<BufferField>, StateField> {
    pub fn buffer(self, buffer: T) -> BufferTransitionBuilder<Set<BufferField>, StateField> {
        BufferTransitionBuilder {
            buffer: Unset(PhantomData),
            state: Unset(PhantomData),
        }
    }
}
impl<T, BufferField, StateField> BufferTransitionBuilder<BufferField, Unset<StateField>> {
    pub fn state(
        self,
        state: wgpu::BufferUses,
    ) -> BufferTransitionBuilder<BufferField, Set<StateField>> {
        BufferTransitionBuilder {
            buffer: Unset(PhantomData),
            state: Unset(PhantomData),
        }
    }
}
impl<T> BufferTransitionBuilder<T> {
    pub fn build(self) -> wgpu::BufferTransition {
        wgpu::BufferTransition {}
    }
}

#[derive(Debug)]
pub struct BindGroupLayoutDescriptorBuilder<LabelField, EntriesField> {
    label: LabelField,
    entries: EntriesField,
}
pub fn bind_group_layout_descriptor<'a>()
-> BindGroupLayoutDescriptorBuilder<Unset<wgpu::Label<'a>>, Unset<&'a [wgpu::BindGroupLayoutEntry]>>
{
    BindGroupLayoutDescriptorBuilder {
        label: Unset(PhantomData),
        entries: Unset(PhantomData),
    }
}
impl<'a, LabelField, EntriesField>
    BindGroupLayoutDescriptorBuilder<Unset<LabelField>, EntriesField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> BindGroupLayoutDescriptorBuilder<Set<LabelField>, EntriesField> {
        BindGroupLayoutDescriptorBuilder {
            label: Unset(PhantomData),
            entries: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, EntriesField>
    BindGroupLayoutDescriptorBuilder<LabelField, Unset<EntriesField>>
{
    pub fn entries(
        self,
        entries: &'a [wgpu::BindGroupLayoutEntry],
    ) -> BindGroupLayoutDescriptorBuilder<LabelField, Set<EntriesField>> {
        BindGroupLayoutDescriptorBuilder {
            label: Unset(PhantomData),
            entries: Unset(PhantomData),
        }
    }
}
impl<'a> BindGroupLayoutDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::BindGroupLayoutDescriptor {
        wgpu::BindGroupLayoutDescriptor {}
    }
}

#[derive(Debug)]
pub struct QuerySetDescriptorBuilder<LabelField, TyField, CountField> {
    label: LabelField,
    ty: TyField,
    count: CountField,
}
pub fn query_set_descriptor<'a>()
-> QuerySetDescriptorBuilder<Unset<wgpu::Label<'a>>, Unset<wgpu::QueryType>, Unset<u32>> {
    QuerySetDescriptorBuilder {
        label: Unset(PhantomData),
        ty: Unset(PhantomData),
        count: Unset(PhantomData),
    }
}
impl<'a, LabelField, TyField, CountField>
    QuerySetDescriptorBuilder<Unset<LabelField>, TyField, CountField>
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> QuerySetDescriptorBuilder<Set<LabelField>, TyField, CountField> {
        QuerySetDescriptorBuilder {
            label: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, TyField, CountField>
    QuerySetDescriptorBuilder<LabelField, Unset<TyField>, CountField>
{
    pub fn ty(
        self,
        ty: wgpu::QueryType,
    ) -> QuerySetDescriptorBuilder<LabelField, Set<TyField>, CountField> {
        QuerySetDescriptorBuilder {
            label: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl<'a, LabelField, TyField, CountField>
    QuerySetDescriptorBuilder<LabelField, TyField, Unset<CountField>>
{
    pub fn count(
        self,
        count: u32,
    ) -> QuerySetDescriptorBuilder<LabelField, TyField, Set<CountField>> {
        QuerySetDescriptorBuilder {
            label: Unset(PhantomData),
            ty: Unset(PhantomData),
            count: Unset(PhantomData),
        }
    }
}
impl<'a> QuerySetDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::QuerySetDescriptor {
        wgpu::QuerySetDescriptor {}
    }
}

#[derive(Debug)]
pub struct BlasTriangleGeometryBuilder<
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
> {
    size: SizeField,
    vertex_buffer: VertexBufferField,
    first_vertex: FirstVertexField,
    vertex_stride: VertexStrideField,
    index_buffer: IndexBufferField,
    first_index: FirstIndexField,
    transform_buffer: TransformBufferField,
    transform_buffer_offset: TransformBufferOffsetField,
}
pub fn blas_triangle_geometry<'a>() -> BlasTriangleGeometryBuilder<
    Unset<&'a wgpu::BlasTriangleGeometrySizeDescriptor>,
    Unset<&'a wgpu::Buffer>,
    Unset<u32>,
    Unset<wgpu::BufferAddress>,
    Unset<Option<&'a wgpu::Buffer>>,
    Unset<Option<u32>>,
    Unset<Option<&'a wgpu::Buffer>>,
    Unset<Option<wgpu::BufferAddress>>,
> {
    BlasTriangleGeometryBuilder {
        size: Unset(PhantomData),
        vertex_buffer: Unset(PhantomData),
        first_vertex: Unset(PhantomData),
        vertex_stride: Unset(PhantomData),
        index_buffer: Unset(PhantomData),
        first_index: Unset(PhantomData),
        transform_buffer: Unset(PhantomData),
        transform_buffer_offset: Unset(PhantomData),
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        Unset<SizeField>,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    >
{
    pub fn size(
        self,
        size: &'a wgpu::BlasTriangleGeometrySizeDescriptor,
    ) -> BlasTriangleGeometryBuilder<
        Set<SizeField>,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        Unset<VertexBufferField>,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    >
{
    pub fn vertex_buffer(
        self,
        vertex_buffer: &'a wgpu::Buffer,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        Set<VertexBufferField>,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        Unset<FirstVertexField>,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    >
{
    pub fn first_vertex(
        self,
        first_vertex: u32,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        Set<FirstVertexField>,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        Unset<VertexStrideField>,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    >
{
    pub fn vertex_stride(
        self,
        vertex_stride: wgpu::BufferAddress,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        Set<VertexStrideField>,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        Unset<IndexBufferField>,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    >
{
    pub fn index_buffer(
        self,
        index_buffer: Option<&'a wgpu::Buffer>,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        Set<IndexBufferField>,
        FirstIndexField,
        TransformBufferField,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        Unset<FirstIndexField>,
        TransformBufferField,
        TransformBufferOffsetField,
    >
{
    pub fn first_index(
        self,
        first_index: Option<u32>,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        Set<FirstIndexField>,
        TransformBufferField,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        Unset<TransformBufferField>,
        TransformBufferOffsetField,
    >
{
    pub fn transform_buffer(
        self,
        transform_buffer: Option<&'a wgpu::Buffer>,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        Set<TransformBufferField>,
        TransformBufferOffsetField,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    SizeField,
    VertexBufferField,
    FirstVertexField,
    VertexStrideField,
    IndexBufferField,
    FirstIndexField,
    TransformBufferField,
    TransformBufferOffsetField,
>
    BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        Unset<TransformBufferOffsetField>,
    >
{
    pub fn transform_buffer_offset(
        self,
        transform_buffer_offset: Option<wgpu::BufferAddress>,
    ) -> BlasTriangleGeometryBuilder<
        SizeField,
        VertexBufferField,
        FirstVertexField,
        VertexStrideField,
        IndexBufferField,
        FirstIndexField,
        TransformBufferField,
        Set<TransformBufferOffsetField>,
    > {
        BlasTriangleGeometryBuilder {
            size: Unset(PhantomData),
            vertex_buffer: Unset(PhantomData),
            first_vertex: Unset(PhantomData),
            vertex_stride: Unset(PhantomData),
            index_buffer: Unset(PhantomData),
            first_index: Unset(PhantomData),
            transform_buffer: Unset(PhantomData),
            transform_buffer_offset: Unset(PhantomData),
        }
    }
}
impl<'a> BlasTriangleGeometryBuilder<'a> {
    pub fn build(self) -> wgpu::BlasTriangleGeometry {
        wgpu::BlasTriangleGeometry {}
    }
}

#[derive(Debug)]
pub struct FragmentStateBuilder<ModuleField, EntryPointField, CompilationOptionsField, TargetsField>
{
    module: ModuleField,
    entry_point: EntryPointField,
    compilation_options: CompilationOptionsField,
    targets: TargetsField,
}
pub fn fragment_state<'a>() -> FragmentStateBuilder<
    Unset<&'a wgpu::ShaderModule>,
    Unset<Option<&'a str>>,
    Unset<impl Nested<wgpu::PipelineCompilationOptions<'a>>>,
    Unset<&'a [Option<wgpu::ColorTargetState>]>,
> {
    FragmentStateBuilder {
        module: Unset(PhantomData),
        entry_point: Unset(PhantomData),
        compilation_options: Unset(PhantomData),
        targets: Unset(PhantomData),
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, TargetsField>
    FragmentStateBuilder<Unset<ModuleField>, EntryPointField, CompilationOptionsField, TargetsField>
{
    pub fn module(
        self,
        module: &'a wgpu::ShaderModule,
    ) -> FragmentStateBuilder<
        Set<ModuleField>,
        EntryPointField,
        CompilationOptionsField,
        TargetsField,
    > {
        FragmentStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            targets: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, TargetsField>
    FragmentStateBuilder<ModuleField, Unset<EntryPointField>, CompilationOptionsField, TargetsField>
{
    pub fn entry_point(
        self,
        entry_point: Option<&'a str>,
    ) -> FragmentStateBuilder<
        ModuleField,
        Set<EntryPointField>,
        CompilationOptionsField,
        TargetsField,
    > {
        FragmentStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            targets: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, TargetsField>
    FragmentStateBuilder<ModuleField, EntryPointField, Unset<CompilationOptionsField>, TargetsField>
{
    pub fn compilation_options(
        self,
        compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
    ) -> FragmentStateBuilder<
        ModuleField,
        EntryPointField,
        Set<CompilationOptionsField>,
        TargetsField,
    > {
        FragmentStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            targets: Unset(PhantomData),
        }
    }
}
impl<'a, ModuleField, EntryPointField, CompilationOptionsField, TargetsField>
    FragmentStateBuilder<ModuleField, EntryPointField, CompilationOptionsField, Unset<TargetsField>>
{
    pub fn targets(
        self,
        targets: &'a [Option<wgpu::ColorTargetState>],
    ) -> FragmentStateBuilder<
        ModuleField,
        EntryPointField,
        CompilationOptionsField,
        Set<TargetsField>,
    > {
        FragmentStateBuilder {
            module: Unset(PhantomData),
            entry_point: Unset(PhantomData),
            compilation_options: Unset(PhantomData),
            targets: Unset(PhantomData),
        }
    }
}
impl<'a> FragmentStateBuilder<'a> {
    pub fn build(self) -> wgpu::FragmentState {
        wgpu::FragmentState {}
    }
}

#[derive(Debug)]
pub struct Dx12BackendOptionsBuilder<
    ShaderCompilerField,
    PresentationSystemField,
    LatencyWaitableObjectField,
> {
    shader_compiler: ShaderCompilerField,
    presentation_system: PresentationSystemField,
    latency_waitable_object: LatencyWaitableObjectField,
}
pub fn dx_12_backend_options() -> Dx12BackendOptionsBuilder<
    Unset<wgpu::Dx12Compiler>,
    Unset<wgpu::wgt::Dx12SwapchainKind>,
    Unset<wgpu::wgt::Dx12UseFrameLatencyWaitableObject>,
> {
    Dx12BackendOptionsBuilder {
        shader_compiler: Unset(PhantomData),
        presentation_system: Unset(PhantomData),
        latency_waitable_object: Unset(PhantomData),
    }
}
impl<ShaderCompilerField, PresentationSystemField, LatencyWaitableObjectField>
    Dx12BackendOptionsBuilder<
        Unset<ShaderCompilerField>,
        PresentationSystemField,
        LatencyWaitableObjectField,
    >
{
    pub fn shader_compiler(
        self,
        shader_compiler: wgpu::Dx12Compiler,
    ) -> Dx12BackendOptionsBuilder<
        Set<ShaderCompilerField>,
        PresentationSystemField,
        LatencyWaitableObjectField,
    > {
        Dx12BackendOptionsBuilder {
            shader_compiler: Unset(PhantomData),
            presentation_system: Unset(PhantomData),
            latency_waitable_object: Unset(PhantomData),
        }
    }
}
impl<ShaderCompilerField, PresentationSystemField, LatencyWaitableObjectField>
    Dx12BackendOptionsBuilder<
        ShaderCompilerField,
        Unset<PresentationSystemField>,
        LatencyWaitableObjectField,
    >
{
    pub fn presentation_system(
        self,
        presentation_system: wgpu::wgt::Dx12SwapchainKind,
    ) -> Dx12BackendOptionsBuilder<
        ShaderCompilerField,
        Set<PresentationSystemField>,
        LatencyWaitableObjectField,
    > {
        Dx12BackendOptionsBuilder {
            shader_compiler: Unset(PhantomData),
            presentation_system: Unset(PhantomData),
            latency_waitable_object: Unset(PhantomData),
        }
    }
}
impl<ShaderCompilerField, PresentationSystemField, LatencyWaitableObjectField>
    Dx12BackendOptionsBuilder<
        ShaderCompilerField,
        PresentationSystemField,
        Unset<LatencyWaitableObjectField>,
    >
{
    pub fn latency_waitable_object(
        self,
        latency_waitable_object: wgpu::wgt::Dx12UseFrameLatencyWaitableObject,
    ) -> Dx12BackendOptionsBuilder<
        ShaderCompilerField,
        PresentationSystemField,
        Set<LatencyWaitableObjectField>,
    > {
        Dx12BackendOptionsBuilder {
            shader_compiler: Unset(PhantomData),
            presentation_system: Unset(PhantomData),
            latency_waitable_object: Unset(PhantomData),
        }
    }
}
impl Dx12BackendOptionsBuilder {
    pub fn build(self) -> wgpu::Dx12BackendOptions {
        wgpu::Dx12BackendOptions {}
    }
}

#[derive(Debug)]
pub struct TextureDescriptorBuilder<
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
> {
    label: LabelField,
    size: SizeField,
    mip_level_count: MipLevelCountField,
    sample_count: SampleCountField,
    dimension: DimensionField,
    format: FormatField,
    usage: UsageField,
    view_formats: ViewFormatsField,
}
pub fn texture_descriptor<'a>() -> TextureDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<impl Nested<wgpu::Extent3d>>,
    Unset<u32>,
    Unset<u32>,
    Unset<wgpu::TextureDimension>,
    Unset<wgpu::TextureFormat>,
    Unset<wgpu::TextureUsages>,
    Unset<&'a [wgpu::TextureFormat]>,
> {
    TextureDescriptorBuilder {
        label: Unset(PhantomData),
        size: Unset(PhantomData),
        mip_level_count: Unset(PhantomData),
        sample_count: Unset(PhantomData),
        dimension: Unset(PhantomData),
        format: Unset(PhantomData),
        usage: Unset(PhantomData),
        view_formats: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        Unset<LabelField>,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> TextureDescriptorBuilder<
        Set<LabelField>,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        Unset<SizeField>,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    >
{
    pub fn size(
        self,
        size: impl Nested<wgpu::Extent3d>,
    ) -> TextureDescriptorBuilder<
        LabelField,
        Set<SizeField>,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        SizeField,
        Unset<MipLevelCountField>,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    >
{
    pub fn mip_level_count(
        self,
        mip_level_count: u32,
    ) -> TextureDescriptorBuilder<
        LabelField,
        SizeField,
        Set<MipLevelCountField>,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        Unset<SampleCountField>,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    >
{
    pub fn sample_count(
        self,
        sample_count: u32,
    ) -> TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        Set<SampleCountField>,
        DimensionField,
        FormatField,
        UsageField,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        Unset<DimensionField>,
        FormatField,
        UsageField,
        ViewFormatsField,
    >
{
    pub fn dimension(
        self,
        dimension: wgpu::TextureDimension,
    ) -> TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        Set<DimensionField>,
        FormatField,
        UsageField,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        Unset<FormatField>,
        UsageField,
        ViewFormatsField,
    >
{
    pub fn format(
        self,
        format: wgpu::TextureFormat,
    ) -> TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        Set<FormatField>,
        UsageField,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        Unset<UsageField>,
        ViewFormatsField,
    >
{
    pub fn usage(
        self,
        usage: wgpu::TextureUsages,
    ) -> TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        Set<UsageField>,
        ViewFormatsField,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    SizeField,
    MipLevelCountField,
    SampleCountField,
    DimensionField,
    FormatField,
    UsageField,
    ViewFormatsField,
>
    TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        Unset<ViewFormatsField>,
    >
{
    pub fn view_formats(
        self,
        view_formats: &'a [wgpu::TextureFormat],
    ) -> TextureDescriptorBuilder<
        LabelField,
        SizeField,
        MipLevelCountField,
        SampleCountField,
        DimensionField,
        FormatField,
        UsageField,
        Set<ViewFormatsField>,
    > {
        TextureDescriptorBuilder {
            label: Unset(PhantomData),
            size: Unset(PhantomData),
            mip_level_count: Unset(PhantomData),
            sample_count: Unset(PhantomData),
            dimension: Unset(PhantomData),
            format: Unset(PhantomData),
            usage: Unset(PhantomData),
            view_formats: Unset(PhantomData),
        }
    }
}
impl<'a> TextureDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::TextureDescriptor {
        wgpu::TextureDescriptor {}
    }
}

#[derive(Debug)]
pub struct MultisampleStateBuilder<CountField, MaskField, AlphaToCoverageEnabledField> {
    count: CountField,
    mask: MaskField,
    alpha_to_coverage_enabled: AlphaToCoverageEnabledField,
}
pub fn multisample_state() -> MultisampleStateBuilder<Unset<u32>, Unset<u64>, Unset<bool>> {
    MultisampleStateBuilder {
        count: Unset(PhantomData),
        mask: Unset(PhantomData),
        alpha_to_coverage_enabled: Unset(PhantomData),
    }
}
impl<CountField, MaskField, AlphaToCoverageEnabledField>
    MultisampleStateBuilder<Unset<CountField>, MaskField, AlphaToCoverageEnabledField>
{
    pub fn count(
        self,
        count: u32,
    ) -> MultisampleStateBuilder<Set<CountField>, MaskField, AlphaToCoverageEnabledField> {
        MultisampleStateBuilder {
            count: Unset(PhantomData),
            mask: Unset(PhantomData),
            alpha_to_coverage_enabled: Unset(PhantomData),
        }
    }
}
impl<CountField, MaskField, AlphaToCoverageEnabledField>
    MultisampleStateBuilder<CountField, Unset<MaskField>, AlphaToCoverageEnabledField>
{
    pub fn mask(
        self,
        mask: u64,
    ) -> MultisampleStateBuilder<CountField, Set<MaskField>, AlphaToCoverageEnabledField> {
        MultisampleStateBuilder {
            count: Unset(PhantomData),
            mask: Unset(PhantomData),
            alpha_to_coverage_enabled: Unset(PhantomData),
        }
    }
}
impl<CountField, MaskField, AlphaToCoverageEnabledField>
    MultisampleStateBuilder<CountField, MaskField, Unset<AlphaToCoverageEnabledField>>
{
    pub fn alpha_to_coverage_enabled(
        self,
        alpha_to_coverage_enabled: bool,
    ) -> MultisampleStateBuilder<CountField, MaskField, Set<AlphaToCoverageEnabledField>> {
        MultisampleStateBuilder {
            count: Unset(PhantomData),
            mask: Unset(PhantomData),
            alpha_to_coverage_enabled: Unset(PhantomData),
        }
    }
}
impl MultisampleStateBuilder {
    pub fn build(self) -> wgpu::MultisampleState {
        wgpu::MultisampleState {}
    }
}

#[derive(Debug)]
pub struct DispatchIndirectArgsBuilder<XField, YField, ZField> {
    x: XField,
    y: YField,
    z: ZField,
}
pub fn dispatch_indirect_args() -> DispatchIndirectArgsBuilder<Unset<u32>, Unset<u32>, Unset<u32>> {
    DispatchIndirectArgsBuilder {
        x: Unset(PhantomData),
        y: Unset(PhantomData),
        z: Unset(PhantomData),
    }
}
impl<XField, YField, ZField> DispatchIndirectArgsBuilder<Unset<XField>, YField, ZField> {
    pub fn x(self, x: u32) -> DispatchIndirectArgsBuilder<Set<XField>, YField, ZField> {
        DispatchIndirectArgsBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
            z: Unset(PhantomData),
        }
    }
}
impl<XField, YField, ZField> DispatchIndirectArgsBuilder<XField, Unset<YField>, ZField> {
    pub fn y(self, y: u32) -> DispatchIndirectArgsBuilder<XField, Set<YField>, ZField> {
        DispatchIndirectArgsBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
            z: Unset(PhantomData),
        }
    }
}
impl<XField, YField, ZField> DispatchIndirectArgsBuilder<XField, YField, Unset<ZField>> {
    pub fn z(self, z: u32) -> DispatchIndirectArgsBuilder<XField, YField, Set<ZField>> {
        DispatchIndirectArgsBuilder {
            x: Unset(PhantomData),
            y: Unset(PhantomData),
            z: Unset(PhantomData),
        }
    }
}
impl DispatchIndirectArgsBuilder {
    pub fn build(self) -> wgpu::util::DispatchIndirectArgs {
        wgpu::util::DispatchIndirectArgs {}
    }
}

#[derive(Debug)]
pub struct StencilStateBuilder<FrontField, BackField, ReadMaskField, WriteMaskField> {
    front: FrontField,
    back: BackField,
    read_mask: ReadMaskField,
    write_mask: WriteMaskField,
}
pub fn stencil_state() -> StencilStateBuilder<
    Unset<impl Nested<wgpu::StencilFaceState>>,
    Unset<impl Nested<wgpu::StencilFaceState>>,
    Unset<u32>,
    Unset<u32>,
> {
    StencilStateBuilder {
        front: Unset(PhantomData),
        back: Unset(PhantomData),
        read_mask: Unset(PhantomData),
        write_mask: Unset(PhantomData),
    }
}
impl<FrontField, BackField, ReadMaskField, WriteMaskField>
    StencilStateBuilder<Unset<FrontField>, BackField, ReadMaskField, WriteMaskField>
{
    pub fn front(
        self,
        front: impl Nested<wgpu::StencilFaceState>,
    ) -> StencilStateBuilder<Set<FrontField>, BackField, ReadMaskField, WriteMaskField> {
        StencilStateBuilder {
            front: Unset(PhantomData),
            back: Unset(PhantomData),
            read_mask: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl<FrontField, BackField, ReadMaskField, WriteMaskField>
    StencilStateBuilder<FrontField, Unset<BackField>, ReadMaskField, WriteMaskField>
{
    pub fn back(
        self,
        back: impl Nested<wgpu::StencilFaceState>,
    ) -> StencilStateBuilder<FrontField, Set<BackField>, ReadMaskField, WriteMaskField> {
        StencilStateBuilder {
            front: Unset(PhantomData),
            back: Unset(PhantomData),
            read_mask: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl<FrontField, BackField, ReadMaskField, WriteMaskField>
    StencilStateBuilder<FrontField, BackField, Unset<ReadMaskField>, WriteMaskField>
{
    pub fn read_mask(
        self,
        read_mask: u32,
    ) -> StencilStateBuilder<FrontField, BackField, Set<ReadMaskField>, WriteMaskField> {
        StencilStateBuilder {
            front: Unset(PhantomData),
            back: Unset(PhantomData),
            read_mask: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl<FrontField, BackField, ReadMaskField, WriteMaskField>
    StencilStateBuilder<FrontField, BackField, ReadMaskField, Unset<WriteMaskField>>
{
    pub fn write_mask(
        self,
        write_mask: u32,
    ) -> StencilStateBuilder<FrontField, BackField, ReadMaskField, Set<WriteMaskField>> {
        StencilStateBuilder {
            front: Unset(PhantomData),
            back: Unset(PhantomData),
            read_mask: Unset(PhantomData),
            write_mask: Unset(PhantomData),
        }
    }
}
impl StencilStateBuilder {
    pub fn build(self) -> wgpu::StencilState {
        wgpu::StencilState {}
    }
}

#[derive(Debug)]
pub struct PrimitiveStateBuilder<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
> {
    topology: TopologyField,
    strip_index_format: StripIndexFormatField,
    front_face: FrontFaceField,
    cull_mode: CullModeField,
    unclipped_depth: UnclippedDepthField,
    polygon_mode: PolygonModeField,
    conservative: ConservativeField,
}
pub fn primitive_state() -> PrimitiveStateBuilder<
    Unset<wgpu::PrimitiveTopology>,
    Unset<Option<wgpu::IndexFormat>>,
    Unset<wgpu::FrontFace>,
    Unset<Option<wgpu::Face>>,
    Unset<bool>,
    Unset<wgpu::PolygonMode>,
    Unset<bool>,
> {
    PrimitiveStateBuilder {
        topology: Unset(PhantomData),
        strip_index_format: Unset(PhantomData),
        front_face: Unset(PhantomData),
        cull_mode: Unset(PhantomData),
        unclipped_depth: Unset(PhantomData),
        polygon_mode: Unset(PhantomData),
        conservative: Unset(PhantomData),
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        Unset<TopologyField>,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    >
{
    pub fn topology(
        self,
        topology: wgpu::PrimitiveTopology,
    ) -> PrimitiveStateBuilder<
        Set<TopologyField>,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        TopologyField,
        Unset<StripIndexFormatField>,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    >
{
    pub fn strip_index_format(
        self,
        strip_index_format: Option<wgpu::IndexFormat>,
    ) -> PrimitiveStateBuilder<
        TopologyField,
        Set<StripIndexFormatField>,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        Unset<FrontFaceField>,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    >
{
    pub fn front_face(
        self,
        front_face: wgpu::FrontFace,
    ) -> PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        Set<FrontFaceField>,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        Unset<CullModeField>,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    >
{
    pub fn cull_mode(
        self,
        cull_mode: Option<wgpu::Face>,
    ) -> PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        Set<CullModeField>,
        UnclippedDepthField,
        PolygonModeField,
        ConservativeField,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        Unset<UnclippedDepthField>,
        PolygonModeField,
        ConservativeField,
    >
{
    pub fn unclipped_depth(
        self,
        unclipped_depth: bool,
    ) -> PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        Set<UnclippedDepthField>,
        PolygonModeField,
        ConservativeField,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        Unset<PolygonModeField>,
        ConservativeField,
    >
{
    pub fn polygon_mode(
        self,
        polygon_mode: wgpu::PolygonMode,
    ) -> PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        Set<PolygonModeField>,
        ConservativeField,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl<
    TopologyField,
    StripIndexFormatField,
    FrontFaceField,
    CullModeField,
    UnclippedDepthField,
    PolygonModeField,
    ConservativeField,
>
    PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        Unset<ConservativeField>,
    >
{
    pub fn conservative(
        self,
        conservative: bool,
    ) -> PrimitiveStateBuilder<
        TopologyField,
        StripIndexFormatField,
        FrontFaceField,
        CullModeField,
        UnclippedDepthField,
        PolygonModeField,
        Set<ConservativeField>,
    > {
        PrimitiveStateBuilder {
            topology: Unset(PhantomData),
            strip_index_format: Unset(PhantomData),
            front_face: Unset(PhantomData),
            cull_mode: Unset(PhantomData),
            unclipped_depth: Unset(PhantomData),
            polygon_mode: Unset(PhantomData),
            conservative: Unset(PhantomData),
        }
    }
}
impl PrimitiveStateBuilder {
    pub fn build(self) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {}
    }
}

#[derive(Debug)]
pub struct GlBackendOptionsBuilder<GlesMinorVersionField, FenceBehaviorField> {
    gles_minor_version: GlesMinorVersionField,
    fence_behavior: FenceBehaviorField,
}
pub fn gl_backend_options()
-> GlBackendOptionsBuilder<Unset<wgpu::Gles3MinorVersion>, Unset<wgpu::GlFenceBehavior>> {
    GlBackendOptionsBuilder {
        gles_minor_version: Unset(PhantomData),
        fence_behavior: Unset(PhantomData),
    }
}
impl<GlesMinorVersionField, FenceBehaviorField>
    GlBackendOptionsBuilder<Unset<GlesMinorVersionField>, FenceBehaviorField>
{
    pub fn gles_minor_version(
        self,
        gles_minor_version: wgpu::Gles3MinorVersion,
    ) -> GlBackendOptionsBuilder<Set<GlesMinorVersionField>, FenceBehaviorField> {
        GlBackendOptionsBuilder {
            gles_minor_version: Unset(PhantomData),
            fence_behavior: Unset(PhantomData),
        }
    }
}
impl<GlesMinorVersionField, FenceBehaviorField>
    GlBackendOptionsBuilder<GlesMinorVersionField, Unset<FenceBehaviorField>>
{
    pub fn fence_behavior(
        self,
        fence_behavior: wgpu::GlFenceBehavior,
    ) -> GlBackendOptionsBuilder<GlesMinorVersionField, Set<FenceBehaviorField>> {
        GlBackendOptionsBuilder {
            gles_minor_version: Unset(PhantomData),
            fence_behavior: Unset(PhantomData),
        }
    }
}
impl GlBackendOptionsBuilder {
    pub fn build(self) -> wgpu::GlBackendOptions {
        wgpu::GlBackendOptions {}
    }
}

#[derive(Debug)]
pub struct BufferBindingBuilder<BufferField, OffsetField, SizeField> {
    buffer: BufferField,
    offset: OffsetField,
    size: SizeField,
}
pub fn buffer_binding<'a>() -> BufferBindingBuilder<
    Unset<&'a wgpu::Buffer>,
    Unset<wgpu::BufferAddress>,
    Unset<Option<wgpu::BufferSize>>,
> {
    BufferBindingBuilder {
        buffer: Unset(PhantomData),
        offset: Unset(PhantomData),
        size: Unset(PhantomData),
    }
}
impl<'a, BufferField, OffsetField, SizeField>
    BufferBindingBuilder<Unset<BufferField>, OffsetField, SizeField>
{
    pub fn buffer(
        self,
        buffer: &'a wgpu::Buffer,
    ) -> BufferBindingBuilder<Set<BufferField>, OffsetField, SizeField> {
        BufferBindingBuilder {
            buffer: Unset(PhantomData),
            offset: Unset(PhantomData),
            size: Unset(PhantomData),
        }
    }
}
impl<'a, BufferField, OffsetField, SizeField>
    BufferBindingBuilder<BufferField, Unset<OffsetField>, SizeField>
{
    pub fn offset(
        self,
        offset: wgpu::BufferAddress,
    ) -> BufferBindingBuilder<BufferField, Set<OffsetField>, SizeField> {
        BufferBindingBuilder {
            buffer: Unset(PhantomData),
            offset: Unset(PhantomData),
            size: Unset(PhantomData),
        }
    }
}
impl<'a, BufferField, OffsetField, SizeField>
    BufferBindingBuilder<BufferField, OffsetField, Unset<SizeField>>
{
    pub fn size(
        self,
        size: Option<wgpu::BufferSize>,
    ) -> BufferBindingBuilder<BufferField, OffsetField, Set<SizeField>> {
        BufferBindingBuilder {
            buffer: Unset(PhantomData),
            offset: Unset(PhantomData),
            size: Unset(PhantomData),
        }
    }
}
impl<'a> BufferBindingBuilder<'a> {
    pub fn build(self) -> wgpu::BufferBinding {
        wgpu::BufferBinding {}
    }
}

#[derive(Debug)]
pub struct ExternalTextureDescriptorBuilder<
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
> {
    label: LabelField,
    width: WidthField,
    height: HeightField,
    format: FormatField,
    yuv_conversion_matrix: YuvConversionMatrixField,
    gamut_conversion_matrix: GamutConversionMatrixField,
    src_transfer_function: SrcTransferFunctionField,
    dst_transfer_function: DstTransferFunctionField,
    sample_transform: SampleTransformField,
    load_transform: LoadTransformField,
}
pub fn external_texture_descriptor<'a>() -> ExternalTextureDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<u32>,
    Unset<u32>,
    Unset<wgpu::ExternalTextureFormat>,
    Unset<[f32; 16]>,
    Unset<[f32; 9]>,
    Unset<impl Nested<wgpu::ExternalTextureTransferFunction>>,
    Unset<impl Nested<wgpu::ExternalTextureTransferFunction>>,
    Unset<[f32; 6]>,
    Unset<[f32; 6]>,
> {
    ExternalTextureDescriptorBuilder {
        label: Unset(PhantomData),
        width: Unset(PhantomData),
        height: Unset(PhantomData),
        format: Unset(PhantomData),
        yuv_conversion_matrix: Unset(PhantomData),
        gamut_conversion_matrix: Unset(PhantomData),
        src_transfer_function: Unset(PhantomData),
        dst_transfer_function: Unset(PhantomData),
        sample_transform: Unset(PhantomData),
        load_transform: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        Unset<LabelField>,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> ExternalTextureDescriptorBuilder<
        Set<LabelField>,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        Unset<WidthField>,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn width(
        self,
        width: u32,
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        Set<WidthField>,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        Unset<HeightField>,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn height(
        self,
        height: u32,
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        Set<HeightField>,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        Unset<FormatField>,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn format(
        self,
        format: wgpu::ExternalTextureFormat,
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        Set<FormatField>,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        Unset<YuvConversionMatrixField>,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn yuv_conversion_matrix(
        self,
        yuv_conversion_matrix: [f32; 16],
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        Set<YuvConversionMatrixField>,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        Unset<GamutConversionMatrixField>,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn gamut_conversion_matrix(
        self,
        gamut_conversion_matrix: [f32; 9],
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        Set<GamutConversionMatrixField>,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        Unset<SrcTransferFunctionField>,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn src_transfer_function(
        self,
        src_transfer_function: impl Nested<wgpu::ExternalTextureTransferFunction>,
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        Set<SrcTransferFunctionField>,
        DstTransferFunctionField,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        Unset<DstTransferFunctionField>,
        SampleTransformField,
        LoadTransformField,
    >
{
    pub fn dst_transfer_function(
        self,
        dst_transfer_function: impl Nested<wgpu::ExternalTextureTransferFunction>,
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        Set<DstTransferFunctionField>,
        SampleTransformField,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        Unset<SampleTransformField>,
        LoadTransformField,
    >
{
    pub fn sample_transform(
        self,
        sample_transform: [f32; 6],
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        Set<SampleTransformField>,
        LoadTransformField,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    WidthField,
    HeightField,
    FormatField,
    YuvConversionMatrixField,
    GamutConversionMatrixField,
    SrcTransferFunctionField,
    DstTransferFunctionField,
    SampleTransformField,
    LoadTransformField,
>
    ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        Unset<LoadTransformField>,
    >
{
    pub fn load_transform(
        self,
        load_transform: [f32; 6],
    ) -> ExternalTextureDescriptorBuilder<
        LabelField,
        WidthField,
        HeightField,
        FormatField,
        YuvConversionMatrixField,
        GamutConversionMatrixField,
        SrcTransferFunctionField,
        DstTransferFunctionField,
        SampleTransformField,
        Set<LoadTransformField>,
    > {
        ExternalTextureDescriptorBuilder {
            label: Unset(PhantomData),
            width: Unset(PhantomData),
            height: Unset(PhantomData),
            format: Unset(PhantomData),
            yuv_conversion_matrix: Unset(PhantomData),
            gamut_conversion_matrix: Unset(PhantomData),
            src_transfer_function: Unset(PhantomData),
            dst_transfer_function: Unset(PhantomData),
            sample_transform: Unset(PhantomData),
            load_transform: Unset(PhantomData),
        }
    }
}
impl<'a> ExternalTextureDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::ExternalTextureDescriptor {
        wgpu::ExternalTextureDescriptor {}
    }
}

#[derive(Debug)]
pub struct CommandBufferDescriptorBuilder<LabelField> {
    label: LabelField,
}
pub fn command_buffer_descriptor<L: Default>() -> CommandBufferDescriptorBuilder<Unset<L>> {
    CommandBufferDescriptorBuilder {
        label: Unset(PhantomData),
    }
}
impl<L, LabelField> CommandBufferDescriptorBuilder<Unset<LabelField>> {
    pub fn label(self, label: L) -> CommandBufferDescriptorBuilder<Set<LabelField>> {
        CommandBufferDescriptorBuilder {
            label: Unset(PhantomData),
        }
    }
}
impl<L> CommandBufferDescriptorBuilder<L> {
    pub fn build(self) -> wgpu::CommandBufferDescriptor {
        wgpu::CommandBufferDescriptor {}
    }
}

#[derive(Debug)]
pub struct BlendComponentBuilder<SrcFactorField, DstFactorField, OperationField> {
    src_factor: SrcFactorField,
    dst_factor: DstFactorField,
    operation: OperationField,
}
pub fn blend_component() -> BlendComponentBuilder<
    Unset<wgpu::BlendFactor>,
    Unset<wgpu::BlendFactor>,
    Unset<wgpu::BlendOperation>,
> {
    BlendComponentBuilder {
        src_factor: Unset(PhantomData),
        dst_factor: Unset(PhantomData),
        operation: Unset(PhantomData),
    }
}
impl<SrcFactorField, DstFactorField, OperationField>
    BlendComponentBuilder<Unset<SrcFactorField>, DstFactorField, OperationField>
{
    pub fn src_factor(
        self,
        src_factor: wgpu::BlendFactor,
    ) -> BlendComponentBuilder<Set<SrcFactorField>, DstFactorField, OperationField> {
        BlendComponentBuilder {
            src_factor: Unset(PhantomData),
            dst_factor: Unset(PhantomData),
            operation: Unset(PhantomData),
        }
    }
}
impl<SrcFactorField, DstFactorField, OperationField>
    BlendComponentBuilder<SrcFactorField, Unset<DstFactorField>, OperationField>
{
    pub fn dst_factor(
        self,
        dst_factor: wgpu::BlendFactor,
    ) -> BlendComponentBuilder<SrcFactorField, Set<DstFactorField>, OperationField> {
        BlendComponentBuilder {
            src_factor: Unset(PhantomData),
            dst_factor: Unset(PhantomData),
            operation: Unset(PhantomData),
        }
    }
}
impl<SrcFactorField, DstFactorField, OperationField>
    BlendComponentBuilder<SrcFactorField, DstFactorField, Unset<OperationField>>
{
    pub fn operation(
        self,
        operation: wgpu::BlendOperation,
    ) -> BlendComponentBuilder<SrcFactorField, DstFactorField, Set<OperationField>> {
        BlendComponentBuilder {
            src_factor: Unset(PhantomData),
            dst_factor: Unset(PhantomData),
            operation: Unset(PhantomData),
        }
    }
}
impl BlendComponentBuilder {
    pub fn build(self) -> wgpu::BlendComponent {
        wgpu::BlendComponent {}
    }
}

#[derive(Debug)]
pub struct BackendOptionsBuilder<GlField, Dx12Field, NoopField> {
    gl: GlField,
    dx12: Dx12Field,
    noop: NoopField,
}
pub fn backend_options() -> BackendOptionsBuilder<
    Unset<impl Nested<wgpu::GlBackendOptions>>,
    Unset<impl Nested<wgpu::Dx12BackendOptions>>,
    Unset<impl Nested<wgpu::NoopBackendOptions>>,
> {
    BackendOptionsBuilder {
        gl: Unset(PhantomData),
        dx12: Unset(PhantomData),
        noop: Unset(PhantomData),
    }
}
impl<GlField, Dx12Field, NoopField> BackendOptionsBuilder<Unset<GlField>, Dx12Field, NoopField> {
    pub fn gl(
        self,
        gl: impl Nested<wgpu::GlBackendOptions>,
    ) -> BackendOptionsBuilder<Set<GlField>, Dx12Field, NoopField> {
        BackendOptionsBuilder {
            gl: Unset(PhantomData),
            dx12: Unset(PhantomData),
            noop: Unset(PhantomData),
        }
    }
}
impl<GlField, Dx12Field, NoopField> BackendOptionsBuilder<GlField, Unset<Dx12Field>, NoopField> {
    pub fn dx12(
        self,
        dx12: impl Nested<wgpu::Dx12BackendOptions>,
    ) -> BackendOptionsBuilder<GlField, Set<Dx12Field>, NoopField> {
        BackendOptionsBuilder {
            gl: Unset(PhantomData),
            dx12: Unset(PhantomData),
            noop: Unset(PhantomData),
        }
    }
}
impl<GlField, Dx12Field, NoopField> BackendOptionsBuilder<GlField, Dx12Field, Unset<NoopField>> {
    pub fn noop(
        self,
        noop: impl Nested<wgpu::NoopBackendOptions>,
    ) -> BackendOptionsBuilder<GlField, Dx12Field, Set<NoopField>> {
        BackendOptionsBuilder {
            gl: Unset(PhantomData),
            dx12: Unset(PhantomData),
            noop: Unset(PhantomData),
        }
    }
}
impl BackendOptionsBuilder {
    pub fn build(self) -> wgpu::BackendOptions {
        wgpu::BackendOptions {}
    }
}

#[derive(Debug)]
pub struct MeshPipelineDescriptorBuilder<
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
> {
    label: LabelField,
    layout: LayoutField,
    task: TaskField,
    mesh: MeshField,
    primitive: PrimitiveField,
    depth_stencil: DepthStencilField,
    multisample: MultisampleField,
    fragment: FragmentField,
    multiview: MultiviewField,
    cache: CacheField,
}
pub fn mesh_pipeline_descriptor<'a>() -> MeshPipelineDescriptorBuilder<
    Unset<wgpu::Label<'a>>,
    Unset<Option<&'a wgpu::PipelineLayout>>,
    Unset<Option<impl Nested<wgpu::TaskState<'a>>>>,
    Unset<impl Nested<wgpu::MeshState<'a>>>,
    Unset<impl Nested<wgpu::PrimitiveState>>,
    Unset<Option<impl Nested<wgpu::DepthStencilState>>>,
    Unset<impl Nested<wgpu::MultisampleState>>,
    Unset<Option<impl Nested<wgpu::FragmentState<'a>>>>,
    Unset<Option<NonZeroU32>>,
    Unset<Option<&'a wgpu::PipelineCache>>,
> {
    MeshPipelineDescriptorBuilder {
        label: Unset(PhantomData),
        layout: Unset(PhantomData),
        task: Unset(PhantomData),
        mesh: Unset(PhantomData),
        primitive: Unset(PhantomData),
        depth_stencil: Unset(PhantomData),
        multisample: Unset(PhantomData),
        fragment: Unset(PhantomData),
        multiview: Unset(PhantomData),
        cache: Unset(PhantomData),
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        Unset<LabelField>,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn label(
        self,
        label: wgpu::Label<'a>,
    ) -> MeshPipelineDescriptorBuilder<
        Set<LabelField>,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        Unset<LayoutField>,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn layout(
        self,
        layout: Option<&'a wgpu::PipelineLayout>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        Set<LayoutField>,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        Unset<TaskField>,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn task(
        self,
        task: Option<impl Nested<wgpu::TaskState<'a>>>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        Set<TaskField>,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        Unset<MeshField>,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn mesh(
        self,
        mesh: impl Nested<wgpu::MeshState<'a>>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        Set<MeshField>,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        Unset<PrimitiveField>,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn primitive(
        self,
        primitive: impl Nested<wgpu::PrimitiveState>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        Set<PrimitiveField>,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        Unset<DepthStencilField>,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn depth_stencil(
        self,
        depth_stencil: Option<impl Nested<wgpu::DepthStencilState>>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        Set<DepthStencilField>,
        MultisampleField,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        Unset<MultisampleField>,
        FragmentField,
        MultiviewField,
        CacheField,
    >
{
    pub fn multisample(
        self,
        multisample: impl Nested<wgpu::MultisampleState>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        Set<MultisampleField>,
        FragmentField,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        Unset<FragmentField>,
        MultiviewField,
        CacheField,
    >
{
    pub fn fragment(
        self,
        fragment: Option<impl Nested<wgpu::FragmentState<'a>>>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        Set<FragmentField>,
        MultiviewField,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        Unset<MultiviewField>,
        CacheField,
    >
{
    pub fn multiview(
        self,
        multiview: Option<NonZeroU32>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        Set<MultiviewField>,
        CacheField,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<
    'a,
    LabelField,
    LayoutField,
    TaskField,
    MeshField,
    PrimitiveField,
    DepthStencilField,
    MultisampleField,
    FragmentField,
    MultiviewField,
    CacheField,
>
    MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        Unset<CacheField>,
    >
{
    pub fn cache(
        self,
        cache: Option<&'a wgpu::PipelineCache>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelField,
        LayoutField,
        TaskField,
        MeshField,
        PrimitiveField,
        DepthStencilField,
        MultisampleField,
        FragmentField,
        MultiviewField,
        Set<CacheField>,
    > {
        MeshPipelineDescriptorBuilder {
            label: Unset(PhantomData),
            layout: Unset(PhantomData),
            task: Unset(PhantomData),
            mesh: Unset(PhantomData),
            primitive: Unset(PhantomData),
            depth_stencil: Unset(PhantomData),
            multisample: Unset(PhantomData),
            fragment: Unset(PhantomData),
            multiview: Unset(PhantomData),
            cache: Unset(PhantomData),
        }
    }
}
impl<'a> MeshPipelineDescriptorBuilder<'a> {
    pub fn build(self) -> wgpu::MeshPipelineDescriptor {
        wgpu::MeshPipelineDescriptor {}
    }
}

#[derive(Debug)]
pub struct RenderPassTimestampWritesBuilder<
    QuerySetField,
    BeginningOfPassWriteIndexField,
    EndOfPassWriteIndexField,
> {
    query_set: QuerySetField,
    beginning_of_pass_write_index: BeginningOfPassWriteIndexField,
    end_of_pass_write_index: EndOfPassWriteIndexField,
}
pub fn render_pass_timestamp_writes<'a>() -> RenderPassTimestampWritesBuilder<
    Unset<&'a wgpu::QuerySet>,
    Unset<Option<u32>>,
    Unset<Option<u32>>,
> {
    RenderPassTimestampWritesBuilder {
        query_set: Unset(PhantomData),
        beginning_of_pass_write_index: Unset(PhantomData),
        end_of_pass_write_index: Unset(PhantomData),
    }
}
impl<'a, QuerySetField, BeginningOfPassWriteIndexField, EndOfPassWriteIndexField>
    RenderPassTimestampWritesBuilder<
        Unset<QuerySetField>,
        BeginningOfPassWriteIndexField,
        EndOfPassWriteIndexField,
    >
{
    pub fn query_set(
        self,
        query_set: &'a wgpu::QuerySet,
    ) -> RenderPassTimestampWritesBuilder<
        Set<QuerySetField>,
        BeginningOfPassWriteIndexField,
        EndOfPassWriteIndexField,
    > {
        RenderPassTimestampWritesBuilder {
            query_set: Unset(PhantomData),
            beginning_of_pass_write_index: Unset(PhantomData),
            end_of_pass_write_index: Unset(PhantomData),
        }
    }
}
impl<'a, QuerySetField, BeginningOfPassWriteIndexField, EndOfPassWriteIndexField>
    RenderPassTimestampWritesBuilder<
        QuerySetField,
        Unset<BeginningOfPassWriteIndexField>,
        EndOfPassWriteIndexField,
    >
{
    pub fn beginning_of_pass_write_index(
        self,
        beginning_of_pass_write_index: Option<u32>,
    ) -> RenderPassTimestampWritesBuilder<
        QuerySetField,
        Set<BeginningOfPassWriteIndexField>,
        EndOfPassWriteIndexField,
    > {
        RenderPassTimestampWritesBuilder {
            query_set: Unset(PhantomData),
            beginning_of_pass_write_index: Unset(PhantomData),
            end_of_pass_write_index: Unset(PhantomData),
        }
    }
}
impl<'a, QuerySetField, BeginningOfPassWriteIndexField, EndOfPassWriteIndexField>
    RenderPassTimestampWritesBuilder<
        QuerySetField,
        BeginningOfPassWriteIndexField,
        Unset<EndOfPassWriteIndexField>,
    >
{
    pub fn end_of_pass_write_index(
        self,
        end_of_pass_write_index: Option<u32>,
    ) -> RenderPassTimestampWritesBuilder<
        QuerySetField,
        BeginningOfPassWriteIndexField,
        Set<EndOfPassWriteIndexField>,
    > {
        RenderPassTimestampWritesBuilder {
            query_set: Unset(PhantomData),
            beginning_of_pass_write_index: Unset(PhantomData),
            end_of_pass_write_index: Unset(PhantomData),
        }
    }
}
impl<'a> RenderPassTimestampWritesBuilder<'a> {
    pub fn build(self) -> wgpu::RenderPassTimestampWrites {
        wgpu::RenderPassTimestampWrites {}
    }
}

#[derive(Debug)]
pub struct TexelCopyBufferInfoBuilder<BufferField, LayoutField> {
    buffer: BufferField,
    layout: LayoutField,
}
pub fn texel_copy_buffer_info<'a>() -> TexelCopyBufferInfoBuilder<
    Unset<&'a wgpu::Buffer>,
    Unset<impl Nested<wgpu::TexelCopyBufferLayout>>,
> {
    TexelCopyBufferInfoBuilder {
        buffer: Unset(PhantomData),
        layout: Unset(PhantomData),
    }
}
impl<'a, BufferField, LayoutField> TexelCopyBufferInfoBuilder<Unset<BufferField>, LayoutField> {
    pub fn buffer(
        self,
        buffer: &'a wgpu::Buffer,
    ) -> TexelCopyBufferInfoBuilder<Set<BufferField>, LayoutField> {
        TexelCopyBufferInfoBuilder {
            buffer: Unset(PhantomData),
            layout: Unset(PhantomData),
        }
    }
}
impl<'a, BufferField, LayoutField> TexelCopyBufferInfoBuilder<BufferField, Unset<LayoutField>> {
    pub fn layout(
        self,
        layout: impl Nested<wgpu::TexelCopyBufferLayout>,
    ) -> TexelCopyBufferInfoBuilder<BufferField, Set<LayoutField>> {
        TexelCopyBufferInfoBuilder {
            buffer: Unset(PhantomData),
            layout: Unset(PhantomData),
        }
    }
}
impl<'a> TexelCopyBufferInfoBuilder<'a> {
    pub fn build(self) -> wgpu::TexelCopyBufferInfo {
        wgpu::TexelCopyBufferInfo {}
    }
}

#[derive(Debug)]
pub struct TexelCopyBufferInfoBaseBuilder<BufferField, LayoutField> {
    buffer: BufferField,
    layout: LayoutField,
}
pub fn texel_copy_buffer_info_base<B>()
-> TexelCopyBufferInfoBaseBuilder<Unset<B>, Unset<impl Nested<wgpu::TexelCopyBufferLayout>>> {
    TexelCopyBufferInfoBaseBuilder {
        buffer: Unset(PhantomData),
        layout: Unset(PhantomData),
    }
}
impl<B, BufferField, LayoutField> TexelCopyBufferInfoBaseBuilder<Unset<BufferField>, LayoutField> {
    pub fn buffer(
        self,
        buffer: B,
    ) -> TexelCopyBufferInfoBaseBuilder<Set<BufferField>, LayoutField> {
        TexelCopyBufferInfoBaseBuilder {
            buffer: Unset(PhantomData),
            layout: Unset(PhantomData),
        }
    }
}
impl<B, BufferField, LayoutField> TexelCopyBufferInfoBaseBuilder<BufferField, Unset<LayoutField>> {
    pub fn layout(
        self,
        layout: impl Nested<wgpu::TexelCopyBufferLayout>,
    ) -> TexelCopyBufferInfoBaseBuilder<BufferField, Set<LayoutField>> {
        TexelCopyBufferInfoBaseBuilder {
            buffer: Unset(PhantomData),
            layout: Unset(PhantomData),
        }
    }
}
impl<B> TexelCopyBufferInfoBaseBuilder<B> {
    pub fn build(self) -> wgpu::TexelCopyBufferInfoBase {
        wgpu::TexelCopyBufferInfoBase {}
    }
}
