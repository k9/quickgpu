mod common {
    pub use crate::Nested;
    pub use std::{borrow::Cow, num::NonZeroU32, ops::Range};
    pub trait IsRequired {}
    pub trait IsUnset {}
    pub trait IsUnsetOptional {}
    pub trait IsOptional {}
    pub trait ResolveOptional<T>: IsOptional {
        fn resolve(self) -> T;
    }
}

pub use builder_texel_copy_texture_info::texel_copy_texture_info;
pub mod builder_texel_copy_texture_info {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texel_copy_texture_info() -> TexelCopyTextureInfoBuilder<
        UnsetTexture,
        UnsetMipLevel,
        UnsetOriginOptional,
        UnsetAspectOptional,
    > {
        TexelCopyTextureInfoBuilder::new()
    }
    pub struct TexelCopyTextureInfoBuilder<T0, T1, T2, T3> {
        texture: T0,
        mip_level: T1,
        origin: T2,
        aspect: T3,
    }
    impl
        TexelCopyTextureInfoBuilder<
            UnsetTexture,
            UnsetMipLevel,
            UnsetOriginOptional,
            UnsetAspectOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                texture: UnsetTexture,
                mip_level: UnsetMipLevel,
                origin: UnsetOriginOptional,
                aspect: UnsetAspectOptional,
            }
        }
    }
    pub struct UnsetTexture;
    impl IsRequired for UnsetTexture {}
    impl IsUnset for UnsetTexture {}
    pub struct TextureValue<'a>(pub &'a wgpu::Texture);
    impl<'a> IsRequired for TextureValue<'a> {}
    pub struct UnsetMipLevel;
    impl IsRequired for UnsetMipLevel {}
    impl IsUnset for UnsetMipLevel {}
    pub struct MipLevelValue(pub u32);
    impl IsRequired for MipLevelValue {}
    pub struct UnsetOriginOptional;
    impl IsOptional for UnsetOriginOptional {}
    impl IsUnsetOptional for UnsetOriginOptional {}
    impl ResolveOptional<wgpu::Origin3d> for UnsetOriginOptional {
        fn resolve(self) -> wgpu::Origin3d {
            Default::default()
        }
    }
    pub struct OriginOptionalValue(pub wgpu::Origin3d);
    impl IsOptional for OriginOptionalValue {}
    impl ResolveOptional<wgpu::Origin3d> for OriginOptionalValue {
        fn resolve(self) -> wgpu::Origin3d {
            self.0
        }
    }
    pub struct UnsetAspectOptional;
    impl IsOptional for UnsetAspectOptional {}
    impl IsUnsetOptional for UnsetAspectOptional {}
    impl ResolveOptional<wgpu::TextureAspect> for UnsetAspectOptional {
        fn resolve(self) -> wgpu::TextureAspect {
            Default::default()
        }
    }
    pub struct AspectOptionalValue(pub wgpu::TextureAspect);
    impl IsOptional for AspectOptionalValue {}
    impl ResolveOptional<wgpu::TextureAspect> for AspectOptionalValue {
        fn resolve(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsRequired, T2: IsOptional, T3: IsOptional>
        TexelCopyTextureInfoBuilder<T0, T1, T2, T3>
    {
        pub fn texture<'a>(
            self,
            texture: &'a wgpu::Texture,
        ) -> TexelCopyTextureInfoBuilder<TextureValue<'a>, T1, T2, T3>
        where
            T0: IsUnset,
        {
            TexelCopyTextureInfoBuilder {
                texture: TextureValue(texture),
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: self.aspect,
            }
        }
        pub fn mip_level(
            self,
            mip_level: u32,
        ) -> TexelCopyTextureInfoBuilder<T0, MipLevelValue, T2, T3>
        where
            T1: IsUnset,
        {
            TexelCopyTextureInfoBuilder {
                texture: self.texture,
                mip_level: MipLevelValue(mip_level),
                origin: self.origin,
                aspect: self.aspect,
            }
        }
        pub fn origin(
            self,
            origin: impl Nested<wgpu::Origin3d>,
        ) -> TexelCopyTextureInfoBuilder<T0, T1, OriginOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            TexelCopyTextureInfoBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: OriginOptionalValue(origin.unnest()),
                aspect: self.aspect,
            }
        }
        pub fn aspect(
            self,
            aspect: wgpu::TextureAspect,
        ) -> TexelCopyTextureInfoBuilder<T0, T1, T2, AspectOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            TexelCopyTextureInfoBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: AspectOptionalValue(aspect),
            }
        }
    }
    impl<'a, ROrigin, RAspect>
        TexelCopyTextureInfoBuilder<TextureValue<'a>, MipLevelValue, ROrigin, RAspect>
    {
        pub fn build(self) -> wgpu::TexelCopyTextureInfo<'a>
        where
            ROrigin: ResolveOptional<wgpu::Origin3d>,
            RAspect: ResolveOptional<wgpu::TextureAspect>,
        {
            wgpu::TexelCopyTextureInfo {
                texture: self.texture.0,
                mip_level: self.mip_level.0,
                origin: self.origin.resolve(),
                aspect: self.aspect.resolve(),
            }
        }
    }
}

pub use builder_noop_backend_options::noop_backend_options;
pub mod builder_noop_backend_options {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn noop_backend_options() -> NoopBackendOptionsBuilder<UnsetEnableOptional> {
        NoopBackendOptionsBuilder::new()
    }
    pub struct NoopBackendOptionsBuilder<T0> {
        enable: T0,
    }
    impl NoopBackendOptionsBuilder<UnsetEnableOptional> {
        pub fn new() -> Self {
            Self {
                enable: UnsetEnableOptional,
            }
        }
    }
    pub struct UnsetEnableOptional;
    impl IsOptional for UnsetEnableOptional {}
    impl IsUnsetOptional for UnsetEnableOptional {}
    impl ResolveOptional<bool> for UnsetEnableOptional {
        fn resolve(self) -> bool {
            Default::default()
        }
    }
    pub struct EnableOptionalValue(pub bool);
    impl IsOptional for EnableOptionalValue {}
    impl ResolveOptional<bool> for EnableOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    impl<T0: IsOptional> NoopBackendOptionsBuilder<T0> {
        pub fn enable(self, enable: bool) -> NoopBackendOptionsBuilder<EnableOptionalValue>
        where
            T0: IsUnsetOptional,
        {
            NoopBackendOptionsBuilder {
                enable: EnableOptionalValue(enable),
            }
        }
    }
    impl<REnable> NoopBackendOptionsBuilder<REnable> {
        pub fn build(self) -> wgpu::NoopBackendOptions
        where
            REnable: ResolveOptional<bool>,
        {
            wgpu::NoopBackendOptions {
                enable: self.enable.resolve(),
            }
        }
    }
    impl<REnable> Nested<wgpu::NoopBackendOptions> for NoopBackendOptionsBuilder<REnable>
    where
        REnable: ResolveOptional<bool>,
    {
        fn unnest(self) -> wgpu::NoopBackendOptions {
            self.build()
        }
    }
    impl Nested<wgpu::NoopBackendOptions> for wgpu::NoopBackendOptions {
        fn unnest(self) -> wgpu::NoopBackendOptions {
            self
        }
    }
}

pub use builder_compute_pipeline_descriptor::compute_pipeline_descriptor;
pub mod builder_compute_pipeline_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn compute_pipeline_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> ComputePipelineDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetLayoutOptional,
        UnsetModule,
        UnsetEntryPointOptional,
        UnsetCompilationOptionsOptional,
        UnsetCacheOptional,
    > {
        ComputePipelineDescriptorBuilder::new().label(label)
    }
    pub struct ComputePipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5> {
        label: T0,
        layout: T1,
        module: T2,
        entry_point: T3,
        compilation_options: T4,
        cache: T5,
    }
    impl
        ComputePipelineDescriptorBuilder<
            UnsetLabelOptional,
            UnsetLayoutOptional,
            UnsetModule,
            UnsetEntryPointOptional,
            UnsetCompilationOptionsOptional,
            UnsetCacheOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                layout: UnsetLayoutOptional,
                module: UnsetModule,
                entry_point: UnsetEntryPointOptional,
                compilation_options: UnsetCompilationOptionsOptional,
                cache: UnsetCacheOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetLayoutOptional;
    impl IsOptional for UnsetLayoutOptional {}
    impl IsUnsetOptional for UnsetLayoutOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineLayout>> for UnsetLayoutOptional {
        fn resolve(self) -> Option<&'a wgpu::PipelineLayout> {
            Default::default()
        }
    }
    pub struct LayoutOptionalValue<'a>(pub Option<&'a wgpu::PipelineLayout>);
    impl<'a> IsOptional for LayoutOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineLayout>> for LayoutOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::PipelineLayout> {
            self.0
        }
    }
    pub struct UnsetModule;
    impl IsRequired for UnsetModule {}
    impl IsUnset for UnsetModule {}
    pub struct ModuleValue<'a>(pub &'a wgpu::ShaderModule);
    impl<'a> IsRequired for ModuleValue<'a> {}
    pub struct UnsetEntryPointOptional;
    impl IsOptional for UnsetEntryPointOptional {}
    impl IsUnsetOptional for UnsetEntryPointOptional {}
    impl<'a> ResolveOptional<Option<&'a str>> for UnsetEntryPointOptional {
        fn resolve(self) -> Option<&'a str> {
            Default::default()
        }
    }
    pub struct EntryPointOptionalValue<'a>(pub Option<&'a str>);
    impl<'a> IsOptional for EntryPointOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a str>> for EntryPointOptionalValue<'a> {
        fn resolve(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct UnsetCompilationOptionsOptional;
    impl IsOptional for UnsetCompilationOptionsOptional {}
    impl IsUnsetOptional for UnsetCompilationOptionsOptional {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>> for UnsetCompilationOptionsOptional {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            Default::default()
        }
    }
    pub struct CompilationOptionsOptionalValue<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    impl<'a> IsOptional for CompilationOptionsOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>>
        for CompilationOptionsOptionalValue<'a>
    {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    pub struct UnsetCacheOptional;
    impl IsOptional for UnsetCacheOptional {}
    impl IsUnsetOptional for UnsetCacheOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineCache>> for UnsetCacheOptional {
        fn resolve(self) -> Option<&'a wgpu::PipelineCache> {
            Default::default()
        }
    }
    pub struct CacheOptionalValue<'a>(pub Option<&'a wgpu::PipelineCache>);
    impl<'a> IsOptional for CacheOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineCache>> for CacheOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::PipelineCache> {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsRequired,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
    > ComputePipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> ComputePipelineDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4, T5>
        where
            T0: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: LabelOptionalValue(label),
                layout: self.layout,
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                cache: self.cache,
            }
        }
        pub fn maybe_layout<'a>(
            self,
            layout: Option<&'a wgpu::PipelineLayout>,
        ) -> ComputePipelineDescriptorBuilder<T0, LayoutOptionalValue<'a>, T2, T3, T4, T5>
        where
            T1: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: LayoutOptionalValue(layout),
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                cache: self.cache,
            }
        }
        pub fn layout<'a>(
            self,
            layout: &'a wgpu::PipelineLayout,
        ) -> ComputePipelineDescriptorBuilder<T0, LayoutOptionalValue<'a>, T2, T3, T4, T5>
        where
            T1: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: LayoutOptionalValue(Some(layout)),
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                cache: self.cache,
            }
        }
        pub fn module<'a>(
            self,
            module: &'a wgpu::ShaderModule,
        ) -> ComputePipelineDescriptorBuilder<T0, T1, ModuleValue<'a>, T3, T4, T5>
        where
            T2: IsUnset,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                module: ModuleValue(module),
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                cache: self.cache,
            }
        }
        pub fn maybe_entry_point<'a>(
            self,
            entry_point: Option<&'a str>,
        ) -> ComputePipelineDescriptorBuilder<T0, T1, T2, EntryPointOptionalValue<'a>, T4, T5>
        where
            T3: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                module: self.module,
                entry_point: EntryPointOptionalValue(entry_point),
                compilation_options: self.compilation_options,
                cache: self.cache,
            }
        }
        pub fn entry_point<'a>(
            self,
            entry_point: &'a str,
        ) -> ComputePipelineDescriptorBuilder<T0, T1, T2, EntryPointOptionalValue<'a>, T4, T5>
        where
            T3: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                module: self.module,
                entry_point: EntryPointOptionalValue(Some(entry_point)),
                compilation_options: self.compilation_options,
                cache: self.cache,
            }
        }
        pub fn compilation_options<'a>(
            self,
            compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
        ) -> ComputePipelineDescriptorBuilder<T0, T1, T2, T3, CompilationOptionsOptionalValue<'a>, T5>
        where
            T4: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: CompilationOptionsOptionalValue(compilation_options.unnest()),
                cache: self.cache,
            }
        }
        pub fn maybe_cache<'a>(
            self,
            cache: Option<&'a wgpu::PipelineCache>,
        ) -> ComputePipelineDescriptorBuilder<T0, T1, T2, T3, T4, CacheOptionalValue<'a>>
        where
            T5: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                cache: CacheOptionalValue(cache),
            }
        }
        pub fn cache<'a>(
            self,
            cache: &'a wgpu::PipelineCache,
        ) -> ComputePipelineDescriptorBuilder<T0, T1, T2, T3, T4, CacheOptionalValue<'a>>
        where
            T5: IsUnsetOptional,
        {
            ComputePipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                cache: CacheOptionalValue(Some(cache)),
            }
        }
    }
    impl<'a, RLabel, RLayout, REntryPoint, RCompilationOptions, RCache>
        ComputePipelineDescriptorBuilder<
            RLabel,
            RLayout,
            ModuleValue<'a>,
            REntryPoint,
            RCompilationOptions,
            RCache,
        >
    {
        pub fn build(self) -> wgpu::ComputePipelineDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
            REntryPoint: ResolveOptional<Option<&'a str>>,
            RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
            RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
        {
            wgpu::ComputePipelineDescriptor {
                label: self.label.resolve(),
                layout: self.layout.resolve(),
                module: self.module.0,
                entry_point: self.entry_point.resolve(),
                compilation_options: self.compilation_options.resolve(),
                cache: self.cache.resolve(),
            }
        }
    }
    impl<'a, RLabel, RLayout, REntryPoint, RCompilationOptions, RCache>
        Nested<wgpu::ComputePipelineDescriptor<'a>>
        for ComputePipelineDescriptorBuilder<
            RLabel,
            RLayout,
            ModuleValue<'a>,
            REntryPoint,
            RCompilationOptions,
            RCache,
        >
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
        REntryPoint: ResolveOptional<Option<&'a str>>,
        RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
        RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
    {
        fn unnest(self) -> wgpu::ComputePipelineDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::ComputePipelineDescriptor<'a>> for wgpu::ComputePipelineDescriptor<'a> {
        fn unnest(self) -> wgpu::ComputePipelineDescriptor<'a> {
            self
        }
    }
}

pub use builder_task_state::task_state;
pub mod builder_task_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn task_state()
    -> TaskStateBuilder<UnsetModule, UnsetEntryPointOptional, UnsetCompilationOptionsOptional> {
        TaskStateBuilder::new()
    }
    pub struct TaskStateBuilder<T0, T1, T2> {
        module: T0,
        entry_point: T1,
        compilation_options: T2,
    }
    impl TaskStateBuilder<UnsetModule, UnsetEntryPointOptional, UnsetCompilationOptionsOptional> {
        pub fn new() -> Self {
            Self {
                module: UnsetModule,
                entry_point: UnsetEntryPointOptional,
                compilation_options: UnsetCompilationOptionsOptional,
            }
        }
    }
    pub struct UnsetModule;
    impl IsRequired for UnsetModule {}
    impl IsUnset for UnsetModule {}
    pub struct ModuleValue<'a>(pub &'a wgpu::ShaderModule);
    impl<'a> IsRequired for ModuleValue<'a> {}
    pub struct UnsetEntryPointOptional;
    impl IsOptional for UnsetEntryPointOptional {}
    impl IsUnsetOptional for UnsetEntryPointOptional {}
    impl<'a> ResolveOptional<Option<&'a str>> for UnsetEntryPointOptional {
        fn resolve(self) -> Option<&'a str> {
            Default::default()
        }
    }
    pub struct EntryPointOptionalValue<'a>(pub Option<&'a str>);
    impl<'a> IsOptional for EntryPointOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a str>> for EntryPointOptionalValue<'a> {
        fn resolve(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct UnsetCompilationOptionsOptional;
    impl IsOptional for UnsetCompilationOptionsOptional {}
    impl IsUnsetOptional for UnsetCompilationOptionsOptional {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>> for UnsetCompilationOptionsOptional {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            Default::default()
        }
    }
    pub struct CompilationOptionsOptionalValue<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    impl<'a> IsOptional for CompilationOptionsOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>>
        for CompilationOptionsOptionalValue<'a>
    {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional> TaskStateBuilder<T0, T1, T2> {
        pub fn module<'a>(
            self,
            module: &'a wgpu::ShaderModule,
        ) -> TaskStateBuilder<ModuleValue<'a>, T1, T2>
        where
            T0: IsUnset,
        {
            TaskStateBuilder {
                module: ModuleValue(module),
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
            }
        }
        pub fn maybe_entry_point<'a>(
            self,
            entry_point: Option<&'a str>,
        ) -> TaskStateBuilder<T0, EntryPointOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            TaskStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(entry_point),
                compilation_options: self.compilation_options,
            }
        }
        pub fn entry_point<'a>(
            self,
            entry_point: &'a str,
        ) -> TaskStateBuilder<T0, EntryPointOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            TaskStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(Some(entry_point)),
                compilation_options: self.compilation_options,
            }
        }
        pub fn compilation_options<'a>(
            self,
            compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
        ) -> TaskStateBuilder<T0, T1, CompilationOptionsOptionalValue<'a>>
        where
            T2: IsUnsetOptional,
        {
            TaskStateBuilder {
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: CompilationOptionsOptionalValue(compilation_options.unnest()),
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions>
        TaskStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions>
    {
        pub fn build(self) -> wgpu::TaskState<'a>
        where
            REntryPoint: ResolveOptional<Option<&'a str>>,
            RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
        {
            wgpu::TaskState {
                module: self.module.0,
                entry_point: self.entry_point.resolve(),
                compilation_options: self.compilation_options.resolve(),
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions> Nested<wgpu::TaskState<'a>>
        for TaskStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions>
    where
        REntryPoint: ResolveOptional<Option<&'a str>>,
        RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
    {
        fn unnest(self) -> wgpu::TaskState<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::TaskState<'a>> for wgpu::TaskState<'a> {
        fn unnest(self) -> wgpu::TaskState<'a> {
            self
        }
    }
}

pub use builder_extent_3_d::extent_3_d;
pub mod builder_extent_3_d {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn extent_3_d()
    -> Extent3dBuilder<UnsetWidthOptional, UnsetHeightOptional, UnsetDepthOrArrayLayersOptional>
    {
        Extent3dBuilder::new()
    }
    pub struct Extent3dBuilder<T0, T1, T2> {
        width: T0,
        height: T1,
        depth_or_array_layers: T2,
    }
    impl Extent3dBuilder<UnsetWidthOptional, UnsetHeightOptional, UnsetDepthOrArrayLayersOptional> {
        pub fn new() -> Self {
            Self {
                width: UnsetWidthOptional,
                height: UnsetHeightOptional,
                depth_or_array_layers: UnsetDepthOrArrayLayersOptional,
            }
        }
    }
    pub struct UnsetWidthOptional;
    impl IsOptional for UnsetWidthOptional {}
    impl IsUnsetOptional for UnsetWidthOptional {}
    impl ResolveOptional<u32> for UnsetWidthOptional {
        fn resolve(self) -> u32 {
            1
        }
    }
    pub struct WidthOptionalValue(pub u32);
    impl IsOptional for WidthOptionalValue {}
    impl ResolveOptional<u32> for WidthOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetHeightOptional;
    impl IsOptional for UnsetHeightOptional {}
    impl IsUnsetOptional for UnsetHeightOptional {}
    impl ResolveOptional<u32> for UnsetHeightOptional {
        fn resolve(self) -> u32 {
            1
        }
    }
    pub struct HeightOptionalValue(pub u32);
    impl IsOptional for HeightOptionalValue {}
    impl ResolveOptional<u32> for HeightOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetDepthOrArrayLayersOptional;
    impl IsOptional for UnsetDepthOrArrayLayersOptional {}
    impl IsUnsetOptional for UnsetDepthOrArrayLayersOptional {}
    impl ResolveOptional<u32> for UnsetDepthOrArrayLayersOptional {
        fn resolve(self) -> u32 {
            1
        }
    }
    pub struct DepthOrArrayLayersOptionalValue(pub u32);
    impl IsOptional for DepthOrArrayLayersOptionalValue {}
    impl ResolveOptional<u32> for DepthOrArrayLayersOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> Extent3dBuilder<T0, T1, T2> {
        pub fn width(self, width: u32) -> Extent3dBuilder<WidthOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            Extent3dBuilder {
                width: WidthOptionalValue(width),
                height: self.height,
                depth_or_array_layers: self.depth_or_array_layers,
            }
        }
        pub fn height(self, height: u32) -> Extent3dBuilder<T0, HeightOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            Extent3dBuilder {
                width: self.width,
                height: HeightOptionalValue(height),
                depth_or_array_layers: self.depth_or_array_layers,
            }
        }
        pub fn depth_or_array_layers(
            self,
            depth_or_array_layers: u32,
        ) -> Extent3dBuilder<T0, T1, DepthOrArrayLayersOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            Extent3dBuilder {
                width: self.width,
                height: self.height,
                depth_or_array_layers: DepthOrArrayLayersOptionalValue(depth_or_array_layers),
            }
        }
    }
    impl<RWidth, RHeight, RDepthOrArrayLayers> Extent3dBuilder<RWidth, RHeight, RDepthOrArrayLayers> {
        pub fn build(self) -> wgpu::Extent3d
        where
            RWidth: ResolveOptional<u32>,
            RHeight: ResolveOptional<u32>,
            RDepthOrArrayLayers: ResolveOptional<u32>,
        {
            wgpu::Extent3d {
                width: self.width.resolve(),
                height: self.height.resolve(),
                depth_or_array_layers: self.depth_or_array_layers.resolve(),
            }
        }
    }
    impl<RWidth, RHeight, RDepthOrArrayLayers> Nested<wgpu::Extent3d>
        for Extent3dBuilder<RWidth, RHeight, RDepthOrArrayLayers>
    where
        RWidth: ResolveOptional<u32>,
        RHeight: ResolveOptional<u32>,
        RDepthOrArrayLayers: ResolveOptional<u32>,
    {
        fn unnest(self) -> wgpu::Extent3d {
            self.build()
        }
    }
    impl Nested<wgpu::Extent3d> for wgpu::Extent3d {
        fn unnest(self) -> wgpu::Extent3d {
            self
        }
    }
}

pub use builder_blend_component::blend_component;
pub mod builder_blend_component {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn blend_component()
    -> BlendComponentBuilder<UnsetSrcFactorOptional, UnsetDstFactorOptional, UnsetOperationOptional>
    {
        BlendComponentBuilder::new()
    }
    pub struct BlendComponentBuilder<T0, T1, T2> {
        src_factor: T0,
        dst_factor: T1,
        operation: T2,
    }
    impl BlendComponentBuilder<UnsetSrcFactorOptional, UnsetDstFactorOptional, UnsetOperationOptional> {
        pub fn new() -> Self {
            Self {
                src_factor: UnsetSrcFactorOptional,
                dst_factor: UnsetDstFactorOptional,
                operation: UnsetOperationOptional,
            }
        }
    }
    pub struct UnsetSrcFactorOptional;
    impl IsOptional for UnsetSrcFactorOptional {}
    impl IsUnsetOptional for UnsetSrcFactorOptional {}
    impl ResolveOptional<wgpu::BlendFactor> for UnsetSrcFactorOptional {
        fn resolve(self) -> wgpu::BlendFactor {
            wgpu::BlendFactor::One
        }
    }
    pub struct SrcFactorOptionalValue(pub wgpu::BlendFactor);
    impl IsOptional for SrcFactorOptionalValue {}
    impl ResolveOptional<wgpu::BlendFactor> for SrcFactorOptionalValue {
        fn resolve(self) -> wgpu::BlendFactor {
            self.0
        }
    }
    pub struct UnsetDstFactorOptional;
    impl IsOptional for UnsetDstFactorOptional {}
    impl IsUnsetOptional for UnsetDstFactorOptional {}
    impl ResolveOptional<wgpu::BlendFactor> for UnsetDstFactorOptional {
        fn resolve(self) -> wgpu::BlendFactor {
            wgpu::BlendFactor::Zero
        }
    }
    pub struct DstFactorOptionalValue(pub wgpu::BlendFactor);
    impl IsOptional for DstFactorOptionalValue {}
    impl ResolveOptional<wgpu::BlendFactor> for DstFactorOptionalValue {
        fn resolve(self) -> wgpu::BlendFactor {
            self.0
        }
    }
    pub struct UnsetOperationOptional;
    impl IsOptional for UnsetOperationOptional {}
    impl IsUnsetOptional for UnsetOperationOptional {}
    impl ResolveOptional<wgpu::BlendOperation> for UnsetOperationOptional {
        fn resolve(self) -> wgpu::BlendOperation {
            Default::default()
        }
    }
    pub struct OperationOptionalValue(pub wgpu::BlendOperation);
    impl IsOptional for OperationOptionalValue {}
    impl ResolveOptional<wgpu::BlendOperation> for OperationOptionalValue {
        fn resolve(self) -> wgpu::BlendOperation {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> BlendComponentBuilder<T0, T1, T2> {
        pub fn src_factor(
            self,
            src_factor: wgpu::BlendFactor,
        ) -> BlendComponentBuilder<SrcFactorOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            BlendComponentBuilder {
                src_factor: SrcFactorOptionalValue(src_factor),
                dst_factor: self.dst_factor,
                operation: self.operation,
            }
        }
        pub fn dst_factor(
            self,
            dst_factor: wgpu::BlendFactor,
        ) -> BlendComponentBuilder<T0, DstFactorOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            BlendComponentBuilder {
                src_factor: self.src_factor,
                dst_factor: DstFactorOptionalValue(dst_factor),
                operation: self.operation,
            }
        }
        pub fn operation(
            self,
            operation: wgpu::BlendOperation,
        ) -> BlendComponentBuilder<T0, T1, OperationOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            BlendComponentBuilder {
                src_factor: self.src_factor,
                dst_factor: self.dst_factor,
                operation: OperationOptionalValue(operation),
            }
        }
    }
    impl<RSrcFactor, RDstFactor, ROperation> BlendComponentBuilder<RSrcFactor, RDstFactor, ROperation> {
        pub fn build(self) -> wgpu::BlendComponent
        where
            RSrcFactor: ResolveOptional<wgpu::BlendFactor>,
            RDstFactor: ResolveOptional<wgpu::BlendFactor>,
            ROperation: ResolveOptional<wgpu::BlendOperation>,
        {
            wgpu::BlendComponent {
                src_factor: self.src_factor.resolve(),
                dst_factor: self.dst_factor.resolve(),
                operation: self.operation.resolve(),
            }
        }
    }
    impl<RSrcFactor, RDstFactor, ROperation> Nested<wgpu::BlendComponent>
        for BlendComponentBuilder<RSrcFactor, RDstFactor, ROperation>
    where
        RSrcFactor: ResolveOptional<wgpu::BlendFactor>,
        RDstFactor: ResolveOptional<wgpu::BlendFactor>,
        ROperation: ResolveOptional<wgpu::BlendOperation>,
    {
        fn unnest(self) -> wgpu::BlendComponent {
            self.build()
        }
    }
    impl Nested<wgpu::BlendComponent> for wgpu::BlendComponent {
        fn unnest(self) -> wgpu::BlendComponent {
            self
        }
    }
}

pub use builder_command_buffer_descriptor::command_buffer_descriptor;
pub mod builder_command_buffer_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn command_buffer_descriptor<L: Default>(
        label: L,
    ) -> CommandBufferDescriptorBuilder<LabelOptionalValue<L>> {
        CommandBufferDescriptorBuilder::new().label(label)
    }
    pub struct CommandBufferDescriptorBuilder<T0> {
        label: T0,
    }
    impl CommandBufferDescriptorBuilder<UnsetLabelOptional> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<L: Default> ResolveOptional<L> for UnsetLabelOptional {
        fn resolve(self) -> L {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<L: Default>(pub L);
    impl<L: Default> IsOptional for LabelOptionalValue<L> {}
    impl<L: Default> ResolveOptional<L> for LabelOptionalValue<L> {
        fn resolve(self) -> L {
            self.0
        }
    }
    impl<T0: IsOptional> CommandBufferDescriptorBuilder<T0> {
        pub fn label<L: Default>(
            self,
            label: L,
        ) -> CommandBufferDescriptorBuilder<LabelOptionalValue<L>>
        where
            T0: IsUnsetOptional,
        {
            CommandBufferDescriptorBuilder {
                label: LabelOptionalValue(label),
            }
        }
    }
    impl<RLabel> CommandBufferDescriptorBuilder<RLabel> {
        pub fn build<L: Default>(self) -> wgpu::CommandBufferDescriptor<L>
        where
            RLabel: ResolveOptional<L>,
        {
            wgpu::CommandBufferDescriptor {
                label: self.label.resolve(),
            }
        }
    }
    impl<L: Default, RLabel> Nested<wgpu::CommandBufferDescriptor<L>>
        for CommandBufferDescriptorBuilder<RLabel>
    where
        RLabel: ResolveOptional<L>,
    {
        fn unnest(self) -> wgpu::CommandBufferDescriptor<L> {
            self.build()
        }
    }
    impl<L> Nested<wgpu::CommandBufferDescriptor<L>> for wgpu::CommandBufferDescriptor<L> {
        fn unnest(self) -> wgpu::CommandBufferDescriptor<L> {
            self
        }
    }
}

pub use builder_primitive_state::primitive_state;
pub mod builder_primitive_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn primitive_state() -> PrimitiveStateBuilder<
        UnsetTopologyOptional,
        UnsetStripIndexFormatOptional,
        UnsetFrontFaceOptional,
        UnsetCullModeOptional,
        UnsetUnclippedDepthOptional,
        UnsetPolygonModeOptional,
        UnsetConservativeOptional,
    > {
        PrimitiveStateBuilder::new()
    }
    pub struct PrimitiveStateBuilder<T0, T1, T2, T3, T4, T5, T6> {
        topology: T0,
        strip_index_format: T1,
        front_face: T2,
        cull_mode: T3,
        unclipped_depth: T4,
        polygon_mode: T5,
        conservative: T6,
    }
    impl
        PrimitiveStateBuilder<
            UnsetTopologyOptional,
            UnsetStripIndexFormatOptional,
            UnsetFrontFaceOptional,
            UnsetCullModeOptional,
            UnsetUnclippedDepthOptional,
            UnsetPolygonModeOptional,
            UnsetConservativeOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                topology: UnsetTopologyOptional,
                strip_index_format: UnsetStripIndexFormatOptional,
                front_face: UnsetFrontFaceOptional,
                cull_mode: UnsetCullModeOptional,
                unclipped_depth: UnsetUnclippedDepthOptional,
                polygon_mode: UnsetPolygonModeOptional,
                conservative: UnsetConservativeOptional,
            }
        }
    }
    pub struct UnsetTopologyOptional;
    impl IsOptional for UnsetTopologyOptional {}
    impl IsUnsetOptional for UnsetTopologyOptional {}
    impl ResolveOptional<wgpu::PrimitiveTopology> for UnsetTopologyOptional {
        fn resolve(self) -> wgpu::PrimitiveTopology {
            Default::default()
        }
    }
    pub struct TopologyOptionalValue(pub wgpu::PrimitiveTopology);
    impl IsOptional for TopologyOptionalValue {}
    impl ResolveOptional<wgpu::PrimitiveTopology> for TopologyOptionalValue {
        fn resolve(self) -> wgpu::PrimitiveTopology {
            self.0
        }
    }
    pub struct UnsetStripIndexFormatOptional;
    impl IsOptional for UnsetStripIndexFormatOptional {}
    impl IsUnsetOptional for UnsetStripIndexFormatOptional {}
    impl ResolveOptional<Option<wgpu::IndexFormat>> for UnsetStripIndexFormatOptional {
        fn resolve(self) -> Option<wgpu::IndexFormat> {
            Default::default()
        }
    }
    pub struct StripIndexFormatOptionalValue(pub Option<wgpu::IndexFormat>);
    impl IsOptional for StripIndexFormatOptionalValue {}
    impl ResolveOptional<Option<wgpu::IndexFormat>> for StripIndexFormatOptionalValue {
        fn resolve(self) -> Option<wgpu::IndexFormat> {
            self.0
        }
    }
    pub struct UnsetFrontFaceOptional;
    impl IsOptional for UnsetFrontFaceOptional {}
    impl IsUnsetOptional for UnsetFrontFaceOptional {}
    impl ResolveOptional<wgpu::FrontFace> for UnsetFrontFaceOptional {
        fn resolve(self) -> wgpu::FrontFace {
            Default::default()
        }
    }
    pub struct FrontFaceOptionalValue(pub wgpu::FrontFace);
    impl IsOptional for FrontFaceOptionalValue {}
    impl ResolveOptional<wgpu::FrontFace> for FrontFaceOptionalValue {
        fn resolve(self) -> wgpu::FrontFace {
            self.0
        }
    }
    pub struct UnsetCullModeOptional;
    impl IsOptional for UnsetCullModeOptional {}
    impl IsUnsetOptional for UnsetCullModeOptional {}
    impl ResolveOptional<Option<wgpu::Face>> for UnsetCullModeOptional {
        fn resolve(self) -> Option<wgpu::Face> {
            Default::default()
        }
    }
    pub struct CullModeOptionalValue(pub Option<wgpu::Face>);
    impl IsOptional for CullModeOptionalValue {}
    impl ResolveOptional<Option<wgpu::Face>> for CullModeOptionalValue {
        fn resolve(self) -> Option<wgpu::Face> {
            self.0
        }
    }
    pub struct UnsetUnclippedDepthOptional;
    impl IsOptional for UnsetUnclippedDepthOptional {}
    impl IsUnsetOptional for UnsetUnclippedDepthOptional {}
    impl ResolveOptional<bool> for UnsetUnclippedDepthOptional {
        fn resolve(self) -> bool {
            Default::default()
        }
    }
    pub struct UnclippedDepthOptionalValue(pub bool);
    impl IsOptional for UnclippedDepthOptionalValue {}
    impl ResolveOptional<bool> for UnclippedDepthOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    pub struct UnsetPolygonModeOptional;
    impl IsOptional for UnsetPolygonModeOptional {}
    impl IsUnsetOptional for UnsetPolygonModeOptional {}
    impl ResolveOptional<wgpu::PolygonMode> for UnsetPolygonModeOptional {
        fn resolve(self) -> wgpu::PolygonMode {
            Default::default()
        }
    }
    pub struct PolygonModeOptionalValue(pub wgpu::PolygonMode);
    impl IsOptional for PolygonModeOptionalValue {}
    impl ResolveOptional<wgpu::PolygonMode> for PolygonModeOptionalValue {
        fn resolve(self) -> wgpu::PolygonMode {
            self.0
        }
    }
    pub struct UnsetConservativeOptional;
    impl IsOptional for UnsetConservativeOptional {}
    impl IsUnsetOptional for UnsetConservativeOptional {}
    impl ResolveOptional<bool> for UnsetConservativeOptional {
        fn resolve(self) -> bool {
            Default::default()
        }
    }
    pub struct ConservativeOptionalValue(pub bool);
    impl IsOptional for ConservativeOptionalValue {}
    impl ResolveOptional<bool> for ConservativeOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsOptional,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
    > PrimitiveStateBuilder<T0, T1, T2, T3, T4, T5, T6>
    {
        pub fn topology(
            self,
            topology: wgpu::PrimitiveTopology,
        ) -> PrimitiveStateBuilder<TopologyOptionalValue, T1, T2, T3, T4, T5, T6>
        where
            T0: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: TopologyOptionalValue(topology),
                strip_index_format: self.strip_index_format,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn maybe_strip_index_format(
            self,
            strip_index_format: Option<wgpu::IndexFormat>,
        ) -> PrimitiveStateBuilder<T0, StripIndexFormatOptionalValue, T2, T3, T4, T5, T6>
        where
            T1: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: StripIndexFormatOptionalValue(strip_index_format),
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn strip_index_format(
            self,
            strip_index_format: wgpu::IndexFormat,
        ) -> PrimitiveStateBuilder<T0, StripIndexFormatOptionalValue, T2, T3, T4, T5, T6>
        where
            T1: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: StripIndexFormatOptionalValue(Some(strip_index_format)),
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn front_face(
            self,
            front_face: wgpu::FrontFace,
        ) -> PrimitiveStateBuilder<T0, T1, FrontFaceOptionalValue, T3, T4, T5, T6>
        where
            T2: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                front_face: FrontFaceOptionalValue(front_face),
                cull_mode: self.cull_mode,
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn maybe_cull_mode(
            self,
            cull_mode: Option<wgpu::Face>,
        ) -> PrimitiveStateBuilder<T0, T1, T2, CullModeOptionalValue, T4, T5, T6>
        where
            T3: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                front_face: self.front_face,
                cull_mode: CullModeOptionalValue(cull_mode),
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn cull_mode(
            self,
            cull_mode: wgpu::Face,
        ) -> PrimitiveStateBuilder<T0, T1, T2, CullModeOptionalValue, T4, T5, T6>
        where
            T3: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                front_face: self.front_face,
                cull_mode: CullModeOptionalValue(Some(cull_mode)),
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn unclipped_depth(
            self,
            unclipped_depth: bool,
        ) -> PrimitiveStateBuilder<T0, T1, T2, T3, UnclippedDepthOptionalValue, T5, T6>
        where
            T4: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                unclipped_depth: UnclippedDepthOptionalValue(unclipped_depth),
                polygon_mode: self.polygon_mode,
                conservative: self.conservative,
            }
        }
        pub fn polygon_mode(
            self,
            polygon_mode: wgpu::PolygonMode,
        ) -> PrimitiveStateBuilder<T0, T1, T2, T3, T4, PolygonModeOptionalValue, T6>
        where
            T5: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                unclipped_depth: self.unclipped_depth,
                polygon_mode: PolygonModeOptionalValue(polygon_mode),
                conservative: self.conservative,
            }
        }
        pub fn conservative(
            self,
            conservative: bool,
        ) -> PrimitiveStateBuilder<T0, T1, T2, T3, T4, T5, ConservativeOptionalValue>
        where
            T6: IsUnsetOptional,
        {
            PrimitiveStateBuilder {
                topology: self.topology,
                strip_index_format: self.strip_index_format,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                unclipped_depth: self.unclipped_depth,
                polygon_mode: self.polygon_mode,
                conservative: ConservativeOptionalValue(conservative),
            }
        }
    }
    impl<
        RTopology,
        RStripIndexFormat,
        RFrontFace,
        RCullMode,
        RUnclippedDepth,
        RPolygonMode,
        RConservative,
    >
        PrimitiveStateBuilder<
            RTopology,
            RStripIndexFormat,
            RFrontFace,
            RCullMode,
            RUnclippedDepth,
            RPolygonMode,
            RConservative,
        >
    {
        pub fn build(self) -> wgpu::PrimitiveState
        where
            RTopology: ResolveOptional<wgpu::PrimitiveTopology>,
            RStripIndexFormat: ResolveOptional<Option<wgpu::IndexFormat>>,
            RFrontFace: ResolveOptional<wgpu::FrontFace>,
            RCullMode: ResolveOptional<Option<wgpu::Face>>,
            RUnclippedDepth: ResolveOptional<bool>,
            RPolygonMode: ResolveOptional<wgpu::PolygonMode>,
            RConservative: ResolveOptional<bool>,
        {
            wgpu::PrimitiveState {
                topology: self.topology.resolve(),
                strip_index_format: self.strip_index_format.resolve(),
                front_face: self.front_face.resolve(),
                cull_mode: self.cull_mode.resolve(),
                unclipped_depth: self.unclipped_depth.resolve(),
                polygon_mode: self.polygon_mode.resolve(),
                conservative: self.conservative.resolve(),
            }
        }
    }
    impl<
        RTopology,
        RStripIndexFormat,
        RFrontFace,
        RCullMode,
        RUnclippedDepth,
        RPolygonMode,
        RConservative,
    > Nested<wgpu::PrimitiveState>
        for PrimitiveStateBuilder<
            RTopology,
            RStripIndexFormat,
            RFrontFace,
            RCullMode,
            RUnclippedDepth,
            RPolygonMode,
            RConservative,
        >
    where
        RTopology: ResolveOptional<wgpu::PrimitiveTopology>,
        RStripIndexFormat: ResolveOptional<Option<wgpu::IndexFormat>>,
        RFrontFace: ResolveOptional<wgpu::FrontFace>,
        RCullMode: ResolveOptional<Option<wgpu::Face>>,
        RUnclippedDepth: ResolveOptional<bool>,
        RPolygonMode: ResolveOptional<wgpu::PolygonMode>,
        RConservative: ResolveOptional<bool>,
    {
        fn unnest(self) -> wgpu::PrimitiveState {
            self.build()
        }
    }
    impl Nested<wgpu::PrimitiveState> for wgpu::PrimitiveState {
        fn unnest(self) -> wgpu::PrimitiveState {
            self
        }
    }
}

pub use builder_shader_module_descriptor::shader_module_descriptor;
pub mod builder_shader_module_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn shader_module_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> ShaderModuleDescriptorBuilder<LabelOptionalValue<'a>, UnsetSource> {
        ShaderModuleDescriptorBuilder::new().label(label)
    }
    pub struct ShaderModuleDescriptorBuilder<T0, T1> {
        label: T0,
        source: T1,
    }
    impl ShaderModuleDescriptorBuilder<UnsetLabelOptional, UnsetSource> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                source: UnsetSource,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetSource;
    impl IsRequired for UnsetSource {}
    impl IsUnset for UnsetSource {}
    pub struct SourceValue<'a>(pub wgpu::ShaderSource<'a>);
    impl<'a> IsRequired for SourceValue<'a> {}
    impl<T0: IsOptional, T1: IsRequired> ShaderModuleDescriptorBuilder<T0, T1> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> ShaderModuleDescriptorBuilder<LabelOptionalValue<'a>, T1>
        where
            T0: IsUnsetOptional,
        {
            ShaderModuleDescriptorBuilder {
                label: LabelOptionalValue(label),
                source: self.source,
            }
        }
        pub fn source<'a>(
            self,
            source: wgpu::ShaderSource<'a>,
        ) -> ShaderModuleDescriptorBuilder<T0, SourceValue<'a>>
        where
            T1: IsUnset,
        {
            ShaderModuleDescriptorBuilder {
                label: self.label,
                source: SourceValue(source),
            }
        }
    }
    impl<'a, RLabel> ShaderModuleDescriptorBuilder<RLabel, SourceValue<'a>> {
        pub fn build(self) -> wgpu::ShaderModuleDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::ShaderModuleDescriptor {
                label: self.label.resolve(),
                source: self.source.0,
            }
        }
    }
    impl<'a, RLabel> Nested<wgpu::ShaderModuleDescriptor<'a>>
        for ShaderModuleDescriptorBuilder<RLabel, SourceValue<'a>>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
    {
        fn unnest(self) -> wgpu::ShaderModuleDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::ShaderModuleDescriptor<'a>> for wgpu::ShaderModuleDescriptor<'a> {
        fn unnest(self) -> wgpu::ShaderModuleDescriptor<'a> {
            self
        }
    }
}

pub use builder_dispatch_indirect_args::dispatch_indirect_args;
pub mod builder_dispatch_indirect_args {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn dispatch_indirect_args()
    -> DispatchIndirectArgsBuilder<UnsetXOptional, UnsetYOptional, UnsetZOptional> {
        DispatchIndirectArgsBuilder::new()
    }
    pub struct DispatchIndirectArgsBuilder<T0, T1, T2> {
        x: T0,
        y: T1,
        z: T2,
    }
    impl DispatchIndirectArgsBuilder<UnsetXOptional, UnsetYOptional, UnsetZOptional> {
        pub fn new() -> Self {
            Self {
                x: UnsetXOptional,
                y: UnsetYOptional,
                z: UnsetZOptional,
            }
        }
    }
    pub struct UnsetXOptional;
    impl IsOptional for UnsetXOptional {}
    impl IsUnsetOptional for UnsetXOptional {}
    impl ResolveOptional<u32> for UnsetXOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct XOptionalValue(pub u32);
    impl IsOptional for XOptionalValue {}
    impl ResolveOptional<u32> for XOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetYOptional;
    impl IsOptional for UnsetYOptional {}
    impl IsUnsetOptional for UnsetYOptional {}
    impl ResolveOptional<u32> for UnsetYOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct YOptionalValue(pub u32);
    impl IsOptional for YOptionalValue {}
    impl ResolveOptional<u32> for YOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetZOptional;
    impl IsOptional for UnsetZOptional {}
    impl IsUnsetOptional for UnsetZOptional {}
    impl ResolveOptional<u32> for UnsetZOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct ZOptionalValue(pub u32);
    impl IsOptional for ZOptionalValue {}
    impl ResolveOptional<u32> for ZOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> DispatchIndirectArgsBuilder<T0, T1, T2> {
        pub fn x(self, x: u32) -> DispatchIndirectArgsBuilder<XOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            DispatchIndirectArgsBuilder {
                x: XOptionalValue(x),
                y: self.y,
                z: self.z,
            }
        }
        pub fn y(self, y: u32) -> DispatchIndirectArgsBuilder<T0, YOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            DispatchIndirectArgsBuilder {
                x: self.x,
                y: YOptionalValue(y),
                z: self.z,
            }
        }
        pub fn z(self, z: u32) -> DispatchIndirectArgsBuilder<T0, T1, ZOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            DispatchIndirectArgsBuilder {
                x: self.x,
                y: self.y,
                z: ZOptionalValue(z),
            }
        }
    }
    impl<RX, RY, RZ> DispatchIndirectArgsBuilder<RX, RY, RZ> {
        pub fn build(self) -> wgpu::util::DispatchIndirectArgs
        where
            RX: ResolveOptional<u32>,
            RY: ResolveOptional<u32>,
            RZ: ResolveOptional<u32>,
        {
            wgpu::util::DispatchIndirectArgs {
                x: self.x.resolve(),
                y: self.y.resolve(),
                z: self.z.resolve(),
            }
        }
    }
    impl<RX, RY, RZ> Nested<wgpu::util::DispatchIndirectArgs>
        for DispatchIndirectArgsBuilder<RX, RY, RZ>
    where
        RX: ResolveOptional<u32>,
        RY: ResolveOptional<u32>,
        RZ: ResolveOptional<u32>,
    {
        fn unnest(self) -> wgpu::util::DispatchIndirectArgs {
            self.build()
        }
    }
    impl Nested<wgpu::util::DispatchIndirectArgs> for wgpu::util::DispatchIndirectArgs {
        fn unnest(self) -> wgpu::util::DispatchIndirectArgs {
            self
        }
    }
}

pub use builder_external_texture_descriptor::external_texture_descriptor;
pub mod builder_external_texture_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn external_texture_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> ExternalTextureDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetWidth,
        UnsetHeight,
        UnsetFormat,
        UnsetYuvConversionMatrix,
        UnsetGamutConversionMatrix,
        UnsetSrcTransferFunctionOptional,
        UnsetDstTransferFunctionOptional,
        UnsetSampleTransform,
        UnsetLoadTransform,
    > {
        ExternalTextureDescriptorBuilder::new().label(label)
    }
    pub struct ExternalTextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
        label: T0,
        width: T1,
        height: T2,
        format: T3,
        yuv_conversion_matrix: T4,
        gamut_conversion_matrix: T5,
        src_transfer_function: T6,
        dst_transfer_function: T7,
        sample_transform: T8,
        load_transform: T9,
    }
    impl
        ExternalTextureDescriptorBuilder<
            UnsetLabelOptional,
            UnsetWidth,
            UnsetHeight,
            UnsetFormat,
            UnsetYuvConversionMatrix,
            UnsetGamutConversionMatrix,
            UnsetSrcTransferFunctionOptional,
            UnsetDstTransferFunctionOptional,
            UnsetSampleTransform,
            UnsetLoadTransform,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                width: UnsetWidth,
                height: UnsetHeight,
                format: UnsetFormat,
                yuv_conversion_matrix: UnsetYuvConversionMatrix,
                gamut_conversion_matrix: UnsetGamutConversionMatrix,
                src_transfer_function: UnsetSrcTransferFunctionOptional,
                dst_transfer_function: UnsetDstTransferFunctionOptional,
                sample_transform: UnsetSampleTransform,
                load_transform: UnsetLoadTransform,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetWidth;
    impl IsRequired for UnsetWidth {}
    impl IsUnset for UnsetWidth {}
    pub struct WidthValue(pub u32);
    impl IsRequired for WidthValue {}
    pub struct UnsetHeight;
    impl IsRequired for UnsetHeight {}
    impl IsUnset for UnsetHeight {}
    pub struct HeightValue(pub u32);
    impl IsRequired for HeightValue {}
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::ExternalTextureFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetYuvConversionMatrix;
    impl IsRequired for UnsetYuvConversionMatrix {}
    impl IsUnset for UnsetYuvConversionMatrix {}
    pub struct YuvConversionMatrixValue(pub [f32; 16]);
    impl IsRequired for YuvConversionMatrixValue {}
    pub struct UnsetGamutConversionMatrix;
    impl IsRequired for UnsetGamutConversionMatrix {}
    impl IsUnset for UnsetGamutConversionMatrix {}
    pub struct GamutConversionMatrixValue(pub [f32; 9]);
    impl IsRequired for GamutConversionMatrixValue {}
    pub struct UnsetSrcTransferFunctionOptional;
    impl IsOptional for UnsetSrcTransferFunctionOptional {}
    impl IsUnsetOptional for UnsetSrcTransferFunctionOptional {}
    impl ResolveOptional<wgpu::ExternalTextureTransferFunction> for UnsetSrcTransferFunctionOptional {
        fn resolve(self) -> wgpu::ExternalTextureTransferFunction {
            Default::default()
        }
    }
    pub struct SrcTransferFunctionOptionalValue(pub wgpu::ExternalTextureTransferFunction);
    impl IsOptional for SrcTransferFunctionOptionalValue {}
    impl ResolveOptional<wgpu::ExternalTextureTransferFunction> for SrcTransferFunctionOptionalValue {
        fn resolve(self) -> wgpu::ExternalTextureTransferFunction {
            self.0
        }
    }
    pub struct UnsetDstTransferFunctionOptional;
    impl IsOptional for UnsetDstTransferFunctionOptional {}
    impl IsUnsetOptional for UnsetDstTransferFunctionOptional {}
    impl ResolveOptional<wgpu::ExternalTextureTransferFunction> for UnsetDstTransferFunctionOptional {
        fn resolve(self) -> wgpu::ExternalTextureTransferFunction {
            Default::default()
        }
    }
    pub struct DstTransferFunctionOptionalValue(pub wgpu::ExternalTextureTransferFunction);
    impl IsOptional for DstTransferFunctionOptionalValue {}
    impl ResolveOptional<wgpu::ExternalTextureTransferFunction> for DstTransferFunctionOptionalValue {
        fn resolve(self) -> wgpu::ExternalTextureTransferFunction {
            self.0
        }
    }
    pub struct UnsetSampleTransform;
    impl IsRequired for UnsetSampleTransform {}
    impl IsUnset for UnsetSampleTransform {}
    pub struct SampleTransformValue(pub [f32; 6]);
    impl IsRequired for SampleTransformValue {}
    pub struct UnsetLoadTransform;
    impl IsRequired for UnsetLoadTransform {}
    impl IsUnset for UnsetLoadTransform {}
    pub struct LoadTransformValue(pub [f32; 6]);
    impl IsRequired for LoadTransformValue {}
    impl<
        T0: IsOptional,
        T1: IsRequired,
        T2: IsRequired,
        T3: IsRequired,
        T4: IsRequired,
        T5: IsRequired,
        T6: IsOptional,
        T7: IsOptional,
        T8: IsRequired,
        T9: IsRequired,
    > ExternalTextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> ExternalTextureDescriptorBuilder<
            LabelOptionalValue<'a>,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T0: IsUnsetOptional,
        {
            ExternalTextureDescriptorBuilder {
                label: LabelOptionalValue(label),
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn width(
            self,
            width: u32,
        ) -> ExternalTextureDescriptorBuilder<T0, WidthValue, T2, T3, T4, T5, T6, T7, T8, T9>
        where
            T1: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: WidthValue(width),
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn height(
            self,
            height: u32,
        ) -> ExternalTextureDescriptorBuilder<T0, T1, HeightValue, T3, T4, T5, T6, T7, T8, T9>
        where
            T2: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: HeightValue(height),
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn format(
            self,
            format: wgpu::ExternalTextureFormat,
        ) -> ExternalTextureDescriptorBuilder<T0, T1, T2, FormatValue, T4, T5, T6, T7, T8, T9>
        where
            T3: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: FormatValue(format),
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn yuv_conversion_matrix(
            self,
            yuv_conversion_matrix: [f32; 16],
        ) -> ExternalTextureDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            YuvConversionMatrixValue,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T4: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: YuvConversionMatrixValue(yuv_conversion_matrix),
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn gamut_conversion_matrix(
            self,
            gamut_conversion_matrix: [f32; 9],
        ) -> ExternalTextureDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            GamutConversionMatrixValue,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T5: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: GamutConversionMatrixValue(gamut_conversion_matrix),
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn src_transfer_function(
            self,
            src_transfer_function: impl Nested<wgpu::ExternalTextureTransferFunction>,
        ) -> ExternalTextureDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            SrcTransferFunctionOptionalValue,
            T7,
            T8,
            T9,
        >
        where
            T6: IsUnsetOptional,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: SrcTransferFunctionOptionalValue(
                    src_transfer_function.unnest(),
                ),
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn dst_transfer_function(
            self,
            dst_transfer_function: impl Nested<wgpu::ExternalTextureTransferFunction>,
        ) -> ExternalTextureDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            DstTransferFunctionOptionalValue,
            T8,
            T9,
        >
        where
            T7: IsUnsetOptional,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: DstTransferFunctionOptionalValue(
                    dst_transfer_function.unnest(),
                ),
                sample_transform: self.sample_transform,
                load_transform: self.load_transform,
            }
        }
        pub fn sample_transform(
            self,
            sample_transform: [f32; 6],
        ) -> ExternalTextureDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            SampleTransformValue,
            T9,
        >
        where
            T8: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: SampleTransformValue(sample_transform),
                load_transform: self.load_transform,
            }
        }
        pub fn load_transform(
            self,
            load_transform: [f32; 6],
        ) -> ExternalTextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, LoadTransformValue>
        where
            T9: IsUnset,
        {
            ExternalTextureDescriptorBuilder {
                label: self.label,
                width: self.width,
                height: self.height,
                format: self.format,
                yuv_conversion_matrix: self.yuv_conversion_matrix,
                gamut_conversion_matrix: self.gamut_conversion_matrix,
                src_transfer_function: self.src_transfer_function,
                dst_transfer_function: self.dst_transfer_function,
                sample_transform: self.sample_transform,
                load_transform: LoadTransformValue(load_transform),
            }
        }
    }
    impl<RLabel, RSrcTransferFunction, RDstTransferFunction>
        ExternalTextureDescriptorBuilder<
            RLabel,
            WidthValue,
            HeightValue,
            FormatValue,
            YuvConversionMatrixValue,
            GamutConversionMatrixValue,
            RSrcTransferFunction,
            RDstTransferFunction,
            SampleTransformValue,
            LoadTransformValue,
        >
    {
        pub fn build<'a>(self) -> wgpu::ExternalTextureDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RSrcTransferFunction: ResolveOptional<wgpu::ExternalTextureTransferFunction>,
            RDstTransferFunction: ResolveOptional<wgpu::ExternalTextureTransferFunction>,
        {
            wgpu::ExternalTextureDescriptor {
                label: self.label.resolve(),
                width: self.width.0,
                height: self.height.0,
                format: self.format.0,
                yuv_conversion_matrix: self.yuv_conversion_matrix.0,
                gamut_conversion_matrix: self.gamut_conversion_matrix.0,
                src_transfer_function: self.src_transfer_function.resolve(),
                dst_transfer_function: self.dst_transfer_function.resolve(),
                sample_transform: self.sample_transform.0,
                load_transform: self.load_transform.0,
            }
        }
    }
}

pub use builder_backend_options::backend_options;
pub mod builder_backend_options {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn backend_options()
    -> BackendOptionsBuilder<UnsetGlOptional, UnsetDx12Optional, UnsetNoopOptional> {
        BackendOptionsBuilder::new()
    }
    pub struct BackendOptionsBuilder<T0, T1, T2> {
        gl: T0,
        dx12: T1,
        noop: T2,
    }
    impl BackendOptionsBuilder<UnsetGlOptional, UnsetDx12Optional, UnsetNoopOptional> {
        pub fn new() -> Self {
            Self {
                gl: UnsetGlOptional,
                dx12: UnsetDx12Optional,
                noop: UnsetNoopOptional,
            }
        }
    }
    pub struct UnsetGlOptional;
    impl IsOptional for UnsetGlOptional {}
    impl IsUnsetOptional for UnsetGlOptional {}
    impl ResolveOptional<wgpu::GlBackendOptions> for UnsetGlOptional {
        fn resolve(self) -> wgpu::GlBackendOptions {
            Default::default()
        }
    }
    pub struct GlOptionalValue(pub wgpu::GlBackendOptions);
    impl IsOptional for GlOptionalValue {}
    impl ResolveOptional<wgpu::GlBackendOptions> for GlOptionalValue {
        fn resolve(self) -> wgpu::GlBackendOptions {
            self.0
        }
    }
    pub struct UnsetDx12Optional;
    impl IsOptional for UnsetDx12Optional {}
    impl IsUnsetOptional for UnsetDx12Optional {}
    impl ResolveOptional<wgpu::Dx12BackendOptions> for UnsetDx12Optional {
        fn resolve(self) -> wgpu::Dx12BackendOptions {
            Default::default()
        }
    }
    pub struct Dx12OptionalValue(pub wgpu::Dx12BackendOptions);
    impl IsOptional for Dx12OptionalValue {}
    impl ResolveOptional<wgpu::Dx12BackendOptions> for Dx12OptionalValue {
        fn resolve(self) -> wgpu::Dx12BackendOptions {
            self.0
        }
    }
    pub struct UnsetNoopOptional;
    impl IsOptional for UnsetNoopOptional {}
    impl IsUnsetOptional for UnsetNoopOptional {}
    impl ResolveOptional<wgpu::NoopBackendOptions> for UnsetNoopOptional {
        fn resolve(self) -> wgpu::NoopBackendOptions {
            Default::default()
        }
    }
    pub struct NoopOptionalValue(pub wgpu::NoopBackendOptions);
    impl IsOptional for NoopOptionalValue {}
    impl ResolveOptional<wgpu::NoopBackendOptions> for NoopOptionalValue {
        fn resolve(self) -> wgpu::NoopBackendOptions {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> BackendOptionsBuilder<T0, T1, T2> {
        pub fn gl(
            self,
            gl: impl Nested<wgpu::GlBackendOptions>,
        ) -> BackendOptionsBuilder<GlOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            BackendOptionsBuilder {
                gl: GlOptionalValue(gl.unnest()),
                dx12: self.dx12,
                noop: self.noop,
            }
        }
        pub fn dx12(
            self,
            dx12: impl Nested<wgpu::Dx12BackendOptions>,
        ) -> BackendOptionsBuilder<T0, Dx12OptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            BackendOptionsBuilder {
                gl: self.gl,
                dx12: Dx12OptionalValue(dx12.unnest()),
                noop: self.noop,
            }
        }
        pub fn noop(
            self,
            noop: impl Nested<wgpu::NoopBackendOptions>,
        ) -> BackendOptionsBuilder<T0, T1, NoopOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            BackendOptionsBuilder {
                gl: self.gl,
                dx12: self.dx12,
                noop: NoopOptionalValue(noop.unnest()),
            }
        }
    }
    impl<RGl, RDx12, RNoop> BackendOptionsBuilder<RGl, RDx12, RNoop> {
        pub fn build(self) -> wgpu::BackendOptions
        where
            RGl: ResolveOptional<wgpu::GlBackendOptions>,
            RDx12: ResolveOptional<wgpu::Dx12BackendOptions>,
            RNoop: ResolveOptional<wgpu::NoopBackendOptions>,
        {
            wgpu::BackendOptions {
                gl: self.gl.resolve(),
                dx12: self.dx12.resolve(),
                noop: self.noop.resolve(),
            }
        }
    }
    impl<RGl, RDx12, RNoop> Nested<wgpu::BackendOptions> for BackendOptionsBuilder<RGl, RDx12, RNoop>
    where
        RGl: ResolveOptional<wgpu::GlBackendOptions>,
        RDx12: ResolveOptional<wgpu::Dx12BackendOptions>,
        RNoop: ResolveOptional<wgpu::NoopBackendOptions>,
    {
        fn unnest(self) -> wgpu::BackendOptions {
            self.build()
        }
    }
    impl Nested<wgpu::BackendOptions> for wgpu::BackendOptions {
        fn unnest(self) -> wgpu::BackendOptions {
            self
        }
    }
}

pub use builder_draw_indirect_args::draw_indirect_args;
pub mod builder_draw_indirect_args {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn draw_indirect_args() -> DrawIndirectArgsBuilder<
        UnsetVertexCountOptional,
        UnsetInstanceCountOptional,
        UnsetFirstVertexOptional,
        UnsetFirstInstanceOptional,
    > {
        DrawIndirectArgsBuilder::new()
    }
    pub struct DrawIndirectArgsBuilder<T0, T1, T2, T3> {
        vertex_count: T0,
        instance_count: T1,
        first_vertex: T2,
        first_instance: T3,
    }
    impl
        DrawIndirectArgsBuilder<
            UnsetVertexCountOptional,
            UnsetInstanceCountOptional,
            UnsetFirstVertexOptional,
            UnsetFirstInstanceOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                vertex_count: UnsetVertexCountOptional,
                instance_count: UnsetInstanceCountOptional,
                first_vertex: UnsetFirstVertexOptional,
                first_instance: UnsetFirstInstanceOptional,
            }
        }
    }
    pub struct UnsetVertexCountOptional;
    impl IsOptional for UnsetVertexCountOptional {}
    impl IsUnsetOptional for UnsetVertexCountOptional {}
    impl ResolveOptional<u32> for UnsetVertexCountOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct VertexCountOptionalValue(pub u32);
    impl IsOptional for VertexCountOptionalValue {}
    impl ResolveOptional<u32> for VertexCountOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetInstanceCountOptional;
    impl IsOptional for UnsetInstanceCountOptional {}
    impl IsUnsetOptional for UnsetInstanceCountOptional {}
    impl ResolveOptional<u32> for UnsetInstanceCountOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct InstanceCountOptionalValue(pub u32);
    impl IsOptional for InstanceCountOptionalValue {}
    impl ResolveOptional<u32> for InstanceCountOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetFirstVertexOptional;
    impl IsOptional for UnsetFirstVertexOptional {}
    impl IsUnsetOptional for UnsetFirstVertexOptional {}
    impl ResolveOptional<u32> for UnsetFirstVertexOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct FirstVertexOptionalValue(pub u32);
    impl IsOptional for FirstVertexOptionalValue {}
    impl ResolveOptional<u32> for FirstVertexOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetFirstInstanceOptional;
    impl IsOptional for UnsetFirstInstanceOptional {}
    impl IsUnsetOptional for UnsetFirstInstanceOptional {}
    impl ResolveOptional<u32> for UnsetFirstInstanceOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct FirstInstanceOptionalValue(pub u32);
    impl IsOptional for FirstInstanceOptionalValue {}
    impl ResolveOptional<u32> for FirstInstanceOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional>
        DrawIndirectArgsBuilder<T0, T1, T2, T3>
    {
        pub fn vertex_count(
            self,
            vertex_count: u32,
        ) -> DrawIndirectArgsBuilder<VertexCountOptionalValue, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            DrawIndirectArgsBuilder {
                vertex_count: VertexCountOptionalValue(vertex_count),
                instance_count: self.instance_count,
                first_vertex: self.first_vertex,
                first_instance: self.first_instance,
            }
        }
        pub fn instance_count(
            self,
            instance_count: u32,
        ) -> DrawIndirectArgsBuilder<T0, InstanceCountOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            DrawIndirectArgsBuilder {
                vertex_count: self.vertex_count,
                instance_count: InstanceCountOptionalValue(instance_count),
                first_vertex: self.first_vertex,
                first_instance: self.first_instance,
            }
        }
        pub fn first_vertex(
            self,
            first_vertex: u32,
        ) -> DrawIndirectArgsBuilder<T0, T1, FirstVertexOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            DrawIndirectArgsBuilder {
                vertex_count: self.vertex_count,
                instance_count: self.instance_count,
                first_vertex: FirstVertexOptionalValue(first_vertex),
                first_instance: self.first_instance,
            }
        }
        pub fn first_instance(
            self,
            first_instance: u32,
        ) -> DrawIndirectArgsBuilder<T0, T1, T2, FirstInstanceOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            DrawIndirectArgsBuilder {
                vertex_count: self.vertex_count,
                instance_count: self.instance_count,
                first_vertex: self.first_vertex,
                first_instance: FirstInstanceOptionalValue(first_instance),
            }
        }
    }
    impl<RVertexCount, RInstanceCount, RFirstVertex, RFirstInstance>
        DrawIndirectArgsBuilder<RVertexCount, RInstanceCount, RFirstVertex, RFirstInstance>
    {
        pub fn build(self) -> wgpu::util::DrawIndirectArgs
        where
            RVertexCount: ResolveOptional<u32>,
            RInstanceCount: ResolveOptional<u32>,
            RFirstVertex: ResolveOptional<u32>,
            RFirstInstance: ResolveOptional<u32>,
        {
            wgpu::util::DrawIndirectArgs {
                vertex_count: self.vertex_count.resolve(),
                instance_count: self.instance_count.resolve(),
                first_vertex: self.first_vertex.resolve(),
                first_instance: self.first_instance.resolve(),
            }
        }
    }
    impl<RVertexCount, RInstanceCount, RFirstVertex, RFirstInstance>
        Nested<wgpu::util::DrawIndirectArgs>
        for DrawIndirectArgsBuilder<RVertexCount, RInstanceCount, RFirstVertex, RFirstInstance>
    where
        RVertexCount: ResolveOptional<u32>,
        RInstanceCount: ResolveOptional<u32>,
        RFirstVertex: ResolveOptional<u32>,
        RFirstInstance: ResolveOptional<u32>,
    {
        fn unnest(self) -> wgpu::util::DrawIndirectArgs {
            self.build()
        }
    }
    impl Nested<wgpu::util::DrawIndirectArgs> for wgpu::util::DrawIndirectArgs {
        fn unnest(self) -> wgpu::util::DrawIndirectArgs {
            self
        }
    }
}

pub use builder_texel_copy_buffer_info_base::texel_copy_buffer_info_base;
pub mod builder_texel_copy_buffer_info_base {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texel_copy_buffer_info_base()
    -> TexelCopyBufferInfoBaseBuilder<UnsetBuffer, UnsetLayoutOptional> {
        TexelCopyBufferInfoBaseBuilder::new()
    }
    pub struct TexelCopyBufferInfoBaseBuilder<T0, T1> {
        buffer: T0,
        layout: T1,
    }
    impl TexelCopyBufferInfoBaseBuilder<UnsetBuffer, UnsetLayoutOptional> {
        pub fn new() -> Self {
            Self {
                buffer: UnsetBuffer,
                layout: UnsetLayoutOptional,
            }
        }
    }
    pub struct UnsetBuffer;
    impl IsRequired for UnsetBuffer {}
    impl IsUnset for UnsetBuffer {}
    pub struct BufferValue<B>(pub B);
    impl<B> IsRequired for BufferValue<B> {}
    pub struct UnsetLayoutOptional;
    impl IsOptional for UnsetLayoutOptional {}
    impl IsUnsetOptional for UnsetLayoutOptional {}
    impl ResolveOptional<wgpu::TexelCopyBufferLayout> for UnsetLayoutOptional {
        fn resolve(self) -> wgpu::TexelCopyBufferLayout {
            Default::default()
        }
    }
    pub struct LayoutOptionalValue(pub wgpu::TexelCopyBufferLayout);
    impl IsOptional for LayoutOptionalValue {}
    impl ResolveOptional<wgpu::TexelCopyBufferLayout> for LayoutOptionalValue {
        fn resolve(self) -> wgpu::TexelCopyBufferLayout {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional> TexelCopyBufferInfoBaseBuilder<T0, T1> {
        pub fn buffer<B>(self, buffer: B) -> TexelCopyBufferInfoBaseBuilder<BufferValue<B>, T1>
        where
            T0: IsUnset,
        {
            TexelCopyBufferInfoBaseBuilder {
                buffer: BufferValue(buffer),
                layout: self.layout,
            }
        }
        pub fn layout(
            self,
            layout: impl Nested<wgpu::TexelCopyBufferLayout>,
        ) -> TexelCopyBufferInfoBaseBuilder<T0, LayoutOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            TexelCopyBufferInfoBaseBuilder {
                buffer: self.buffer,
                layout: LayoutOptionalValue(layout.unnest()),
            }
        }
    }
    impl<RLayout, B> TexelCopyBufferInfoBaseBuilder<BufferValue<B>, RLayout> {
        pub fn build(self) -> wgpu::TexelCopyBufferInfoBase<B>
        where
            RLayout: ResolveOptional<wgpu::TexelCopyBufferLayout>,
        {
            wgpu::TexelCopyBufferInfoBase {
                buffer: self.buffer.0,
                layout: self.layout.resolve(),
            }
        }
    }
    impl<RLayout, B> Nested<wgpu::TexelCopyBufferInfoBase<B>>
        for TexelCopyBufferInfoBaseBuilder<BufferValue<B>, RLayout>
    where
        RLayout: ResolveOptional<wgpu::TexelCopyBufferLayout>,
    {
        fn unnest(self) -> wgpu::TexelCopyBufferInfoBase<B> {
            self.build()
        }
    }
    impl<B> Nested<wgpu::TexelCopyBufferInfoBase<B>> for wgpu::TexelCopyBufferInfoBase<B> {
        fn unnest(self) -> wgpu::TexelCopyBufferInfoBase<B> {
            self
        }
    }
}

pub use builder_shader_module_descriptor_passthrough::shader_module_descriptor_passthrough;
pub mod builder_shader_module_descriptor_passthrough {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn shader_module_descriptor_passthrough<'a>(
        label: wgpu::Label<'a>,
    ) -> ShaderModuleDescriptorPassthroughBuilder<
        UnsetEntryPointOptional,
        LabelOptionalValue<'a>,
        UnsetNumWorkgroupsOptional,
        UnsetRuntimeChecksOptional,
        UnsetSpirvOptional,
        UnsetDxilOptional,
        UnsetMslOptional,
        UnsetHlslOptional,
        UnsetGlslOptional,
        UnsetWgslOptional,
    > {
        ShaderModuleDescriptorPassthroughBuilder::new().label(label)
    }
    pub struct ShaderModuleDescriptorPassthroughBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
        entry_point: T0,
        label: T1,
        num_workgroups: T2,
        runtime_checks: T3,
        spirv: T4,
        dxil: T5,
        msl: T6,
        hlsl: T7,
        glsl: T8,
        wgsl: T9,
    }
    impl
        ShaderModuleDescriptorPassthroughBuilder<
            UnsetEntryPointOptional,
            UnsetLabelOptional,
            UnsetNumWorkgroupsOptional,
            UnsetRuntimeChecksOptional,
            UnsetSpirvOptional,
            UnsetDxilOptional,
            UnsetMslOptional,
            UnsetHlslOptional,
            UnsetGlslOptional,
            UnsetWgslOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                entry_point: UnsetEntryPointOptional,
                label: UnsetLabelOptional,
                num_workgroups: UnsetNumWorkgroupsOptional,
                runtime_checks: UnsetRuntimeChecksOptional,
                spirv: UnsetSpirvOptional,
                dxil: UnsetDxilOptional,
                msl: UnsetMslOptional,
                hlsl: UnsetHlslOptional,
                glsl: UnsetGlslOptional,
                wgsl: UnsetWgslOptional,
            }
        }
    }
    pub struct UnsetEntryPointOptional;
    impl IsOptional for UnsetEntryPointOptional {}
    impl IsUnsetOptional for UnsetEntryPointOptional {}
    impl ResolveOptional<String> for UnsetEntryPointOptional {
        fn resolve(self) -> String {
            "".into()
        }
    }
    pub struct EntryPointOptionalValue(pub String);
    impl IsOptional for EntryPointOptionalValue {}
    impl ResolveOptional<String> for EntryPointOptionalValue {
        fn resolve(self) -> String {
            self.0
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetNumWorkgroupsOptional;
    impl IsOptional for UnsetNumWorkgroupsOptional {}
    impl IsUnsetOptional for UnsetNumWorkgroupsOptional {}
    impl ResolveOptional<(u32, u32, u32)> for UnsetNumWorkgroupsOptional {
        fn resolve(self) -> (u32, u32, u32) {
            (0, 0, 0)
        }
    }
    pub struct NumWorkgroupsOptionalValue(pub (u32, u32, u32));
    impl IsOptional for NumWorkgroupsOptionalValue {}
    impl ResolveOptional<(u32, u32, u32)> for NumWorkgroupsOptionalValue {
        fn resolve(self) -> (u32, u32, u32) {
            self.0
        }
    }
    pub struct UnsetRuntimeChecksOptional;
    impl IsOptional for UnsetRuntimeChecksOptional {}
    impl IsUnsetOptional for UnsetRuntimeChecksOptional {}
    impl ResolveOptional<wgpu::ShaderRuntimeChecks> for UnsetRuntimeChecksOptional {
        fn resolve(self) -> wgpu::ShaderRuntimeChecks {
            Default::default()
        }
    }
    pub struct RuntimeChecksOptionalValue(pub wgpu::ShaderRuntimeChecks);
    impl IsOptional for RuntimeChecksOptionalValue {}
    impl ResolveOptional<wgpu::ShaderRuntimeChecks> for RuntimeChecksOptionalValue {
        fn resolve(self) -> wgpu::ShaderRuntimeChecks {
            self.0
        }
    }
    pub struct UnsetSpirvOptional;
    impl IsOptional for UnsetSpirvOptional {}
    impl IsUnsetOptional for UnsetSpirvOptional {}
    impl<'a> ResolveOptional<Option<Cow<'a, [u32]>>> for UnsetSpirvOptional {
        fn resolve(self) -> Option<Cow<'a, [u32]>> {
            Default::default()
        }
    }
    pub struct SpirvOptionalValue<'a>(pub Option<Cow<'a, [u32]>>);
    impl<'a> IsOptional for SpirvOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<Cow<'a, [u32]>>> for SpirvOptionalValue<'a> {
        fn resolve(self) -> Option<Cow<'a, [u32]>> {
            self.0
        }
    }
    pub struct UnsetDxilOptional;
    impl IsOptional for UnsetDxilOptional {}
    impl IsUnsetOptional for UnsetDxilOptional {}
    impl<'a> ResolveOptional<Option<Cow<'a, [u8]>>> for UnsetDxilOptional {
        fn resolve(self) -> Option<Cow<'a, [u8]>> {
            Default::default()
        }
    }
    pub struct DxilOptionalValue<'a>(pub Option<Cow<'a, [u8]>>);
    impl<'a> IsOptional for DxilOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<Cow<'a, [u8]>>> for DxilOptionalValue<'a> {
        fn resolve(self) -> Option<Cow<'a, [u8]>> {
            self.0
        }
    }
    pub struct UnsetMslOptional;
    impl IsOptional for UnsetMslOptional {}
    impl IsUnsetOptional for UnsetMslOptional {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for UnsetMslOptional {
        fn resolve(self) -> Option<Cow<'a, str>> {
            Default::default()
        }
    }
    pub struct MslOptionalValue<'a>(pub Option<Cow<'a, str>>);
    impl<'a> IsOptional for MslOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for MslOptionalValue<'a> {
        fn resolve(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    pub struct UnsetHlslOptional;
    impl IsOptional for UnsetHlslOptional {}
    impl IsUnsetOptional for UnsetHlslOptional {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for UnsetHlslOptional {
        fn resolve(self) -> Option<Cow<'a, str>> {
            Default::default()
        }
    }
    pub struct HlslOptionalValue<'a>(pub Option<Cow<'a, str>>);
    impl<'a> IsOptional for HlslOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for HlslOptionalValue<'a> {
        fn resolve(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    pub struct UnsetGlslOptional;
    impl IsOptional for UnsetGlslOptional {}
    impl IsUnsetOptional for UnsetGlslOptional {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for UnsetGlslOptional {
        fn resolve(self) -> Option<Cow<'a, str>> {
            Default::default()
        }
    }
    pub struct GlslOptionalValue<'a>(pub Option<Cow<'a, str>>);
    impl<'a> IsOptional for GlslOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for GlslOptionalValue<'a> {
        fn resolve(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    pub struct UnsetWgslOptional;
    impl IsOptional for UnsetWgslOptional {}
    impl IsUnsetOptional for UnsetWgslOptional {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for UnsetWgslOptional {
        fn resolve(self) -> Option<Cow<'a, str>> {
            Default::default()
        }
    }
    pub struct WgslOptionalValue<'a>(pub Option<Cow<'a, str>>);
    impl<'a> IsOptional for WgslOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<Cow<'a, str>>> for WgslOptionalValue<'a> {
        fn resolve(self) -> Option<Cow<'a, str>> {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsOptional,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
        T7: IsOptional,
        T8: IsOptional,
        T9: IsOptional,
    > ShaderModuleDescriptorPassthroughBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
    {
        pub fn entry_point(
            self,
            entry_point: String,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            EntryPointOptionalValue,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T0: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: EntryPointOptionalValue(entry_point),
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            LabelOptionalValue<'a>,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T1: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: LabelOptionalValue(label),
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn num_workgroups(
            self,
            num_workgroups: (u32, u32, u32),
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            NumWorkgroupsOptionalValue,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T2: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: NumWorkgroupsOptionalValue(num_workgroups),
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn runtime_checks(
            self,
            runtime_checks: impl Nested<wgpu::ShaderRuntimeChecks>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            RuntimeChecksOptionalValue,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T3: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: RuntimeChecksOptionalValue(runtime_checks.unnest()),
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn maybe_spirv<'a>(
            self,
            spirv: Option<Cow<'a, [u32]>>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            SpirvOptionalValue<'a>,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T4: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: SpirvOptionalValue(spirv),
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn spirv<'a>(
            self,
            spirv: Cow<'a, [u32]>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            SpirvOptionalValue<'a>,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T4: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: SpirvOptionalValue(Some(spirv)),
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn maybe_dxil<'a>(
            self,
            dxil: Option<Cow<'a, [u8]>>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            DxilOptionalValue<'a>,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T5: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: DxilOptionalValue(dxil),
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn dxil<'a>(
            self,
            dxil: Cow<'a, [u8]>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            DxilOptionalValue<'a>,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T5: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: DxilOptionalValue(Some(dxil)),
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn maybe_msl<'a>(
            self,
            msl: Option<Cow<'a, str>>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            MslOptionalValue<'a>,
            T7,
            T8,
            T9,
        >
        where
            T6: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: MslOptionalValue(msl),
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn msl<'a>(
            self,
            msl: Cow<'a, str>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            MslOptionalValue<'a>,
            T7,
            T8,
            T9,
        >
        where
            T6: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: MslOptionalValue(Some(msl)),
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn maybe_hlsl<'a>(
            self,
            hlsl: Option<Cow<'a, str>>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            HlslOptionalValue<'a>,
            T8,
            T9,
        >
        where
            T7: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: HlslOptionalValue(hlsl),
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn hlsl<'a>(
            self,
            hlsl: Cow<'a, str>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            HlslOptionalValue<'a>,
            T8,
            T9,
        >
        where
            T7: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: HlslOptionalValue(Some(hlsl)),
                glsl: self.glsl,
                wgsl: self.wgsl,
            }
        }
        pub fn maybe_glsl<'a>(
            self,
            glsl: Option<Cow<'a, str>>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            GlslOptionalValue<'a>,
            T9,
        >
        where
            T8: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: GlslOptionalValue(glsl),
                wgsl: self.wgsl,
            }
        }
        pub fn glsl<'a>(
            self,
            glsl: Cow<'a, str>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            GlslOptionalValue<'a>,
            T9,
        >
        where
            T8: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: GlslOptionalValue(Some(glsl)),
                wgsl: self.wgsl,
            }
        }
        pub fn maybe_wgsl<'a>(
            self,
            wgsl: Option<Cow<'a, str>>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            WgslOptionalValue<'a>,
        >
        where
            T9: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: WgslOptionalValue(wgsl),
            }
        }
        pub fn wgsl<'a>(
            self,
            wgsl: Cow<'a, str>,
        ) -> ShaderModuleDescriptorPassthroughBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            WgslOptionalValue<'a>,
        >
        where
            T9: IsUnsetOptional,
        {
            ShaderModuleDescriptorPassthroughBuilder {
                entry_point: self.entry_point,
                label: self.label,
                num_workgroups: self.num_workgroups,
                runtime_checks: self.runtime_checks,
                spirv: self.spirv,
                dxil: self.dxil,
                msl: self.msl,
                hlsl: self.hlsl,
                glsl: self.glsl,
                wgsl: WgslOptionalValue(Some(wgsl)),
            }
        }
    }
    impl<
        REntryPoint,
        RLabel,
        RNumWorkgroups,
        RRuntimeChecks,
        RSpirv,
        RDxil,
        RMsl,
        RHlsl,
        RGlsl,
        RWgsl,
    >
        ShaderModuleDescriptorPassthroughBuilder<
            REntryPoint,
            RLabel,
            RNumWorkgroups,
            RRuntimeChecks,
            RSpirv,
            RDxil,
            RMsl,
            RHlsl,
            RGlsl,
            RWgsl,
        >
    {
        pub fn build<'a>(self) -> wgpu::ShaderModuleDescriptorPassthrough<'a>
        where
            REntryPoint: ResolveOptional<String>,
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RNumWorkgroups: ResolveOptional<(u32, u32, u32)>,
            RRuntimeChecks: ResolveOptional<wgpu::ShaderRuntimeChecks>,
            RSpirv: ResolveOptional<Option<Cow<'a, [u32]>>>,
            RDxil: ResolveOptional<Option<Cow<'a, [u8]>>>,
            RMsl: ResolveOptional<Option<Cow<'a, str>>>,
            RHlsl: ResolveOptional<Option<Cow<'a, str>>>,
            RGlsl: ResolveOptional<Option<Cow<'a, str>>>,
            RWgsl: ResolveOptional<Option<Cow<'a, str>>>,
        {
            wgpu::ShaderModuleDescriptorPassthrough {
                entry_point: self.entry_point.resolve(),
                label: self.label.resolve(),
                num_workgroups: self.num_workgroups.resolve(),
                runtime_checks: self.runtime_checks.resolve(),
                spirv: self.spirv.resolve(),
                dxil: self.dxil.resolve(),
                msl: self.msl.resolve(),
                hlsl: self.hlsl.resolve(),
                glsl: self.glsl.resolve(),
                wgsl: self.wgsl.resolve(),
            }
        }
    }
}

pub use builder_texel_copy_buffer_info::texel_copy_buffer_info;
pub mod builder_texel_copy_buffer_info {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texel_copy_buffer_info() -> TexelCopyBufferInfoBuilder<UnsetBuffer, UnsetLayoutOptional>
    {
        TexelCopyBufferInfoBuilder::new()
    }
    pub struct TexelCopyBufferInfoBuilder<T0, T1> {
        buffer: T0,
        layout: T1,
    }
    impl TexelCopyBufferInfoBuilder<UnsetBuffer, UnsetLayoutOptional> {
        pub fn new() -> Self {
            Self {
                buffer: UnsetBuffer,
                layout: UnsetLayoutOptional,
            }
        }
    }
    pub struct UnsetBuffer;
    impl IsRequired for UnsetBuffer {}
    impl IsUnset for UnsetBuffer {}
    pub struct BufferValue<'a>(pub &'a wgpu::Buffer);
    impl<'a> IsRequired for BufferValue<'a> {}
    pub struct UnsetLayoutOptional;
    impl IsOptional for UnsetLayoutOptional {}
    impl IsUnsetOptional for UnsetLayoutOptional {}
    impl ResolveOptional<wgpu::TexelCopyBufferLayout> for UnsetLayoutOptional {
        fn resolve(self) -> wgpu::TexelCopyBufferLayout {
            Default::default()
        }
    }
    pub struct LayoutOptionalValue(pub wgpu::TexelCopyBufferLayout);
    impl IsOptional for LayoutOptionalValue {}
    impl ResolveOptional<wgpu::TexelCopyBufferLayout> for LayoutOptionalValue {
        fn resolve(self) -> wgpu::TexelCopyBufferLayout {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional> TexelCopyBufferInfoBuilder<T0, T1> {
        pub fn buffer<'a>(
            self,
            buffer: &'a wgpu::Buffer,
        ) -> TexelCopyBufferInfoBuilder<BufferValue<'a>, T1>
        where
            T0: IsUnset,
        {
            TexelCopyBufferInfoBuilder {
                buffer: BufferValue(buffer),
                layout: self.layout,
            }
        }
        pub fn layout(
            self,
            layout: impl Nested<wgpu::TexelCopyBufferLayout>,
        ) -> TexelCopyBufferInfoBuilder<T0, LayoutOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            TexelCopyBufferInfoBuilder {
                buffer: self.buffer,
                layout: LayoutOptionalValue(layout.unnest()),
            }
        }
    }
    impl<'a, RLayout> TexelCopyBufferInfoBuilder<BufferValue<'a>, RLayout> {
        pub fn build(self) -> wgpu::TexelCopyBufferInfo<'a>
        where
            RLayout: ResolveOptional<wgpu::TexelCopyBufferLayout>,
        {
            wgpu::TexelCopyBufferInfo {
                buffer: self.buffer.0,
                layout: self.layout.resolve(),
            }
        }
    }
}

pub use builder_compilation_info::compilation_info;
pub mod builder_compilation_info {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn compilation_info() -> CompilationInfoBuilder<UnsetMessages> {
        CompilationInfoBuilder::new()
    }
    pub struct CompilationInfoBuilder<T0> {
        messages: T0,
    }
    impl CompilationInfoBuilder<UnsetMessages> {
        pub fn new() -> Self {
            Self {
                messages: UnsetMessages,
            }
        }
    }
    pub struct UnsetMessages;
    impl IsRequired for UnsetMessages {}
    impl IsUnset for UnsetMessages {}
    pub struct MessagesValue(pub Vec<wgpu::CompilationMessage>);
    impl IsRequired for MessagesValue {}
    impl<T0: IsRequired> CompilationInfoBuilder<T0> {
        pub fn messages(
            self,
            messages: Vec<wgpu::CompilationMessage>,
        ) -> CompilationInfoBuilder<MessagesValue>
        where
            T0: IsUnset,
        {
            CompilationInfoBuilder {
                messages: MessagesValue(messages),
            }
        }
    }
    impl CompilationInfoBuilder<MessagesValue> {
        pub fn build(self) -> wgpu::CompilationInfo where {
            wgpu::CompilationInfo {
                messages: self.messages.0,
            }
        }
    }
    impl Nested<wgpu::CompilationInfo> for CompilationInfoBuilder<MessagesValue> {
        fn unnest(self) -> wgpu::CompilationInfo {
            self.build()
        }
    }
    impl Nested<wgpu::CompilationInfo> for wgpu::CompilationInfo {
        fn unnest(self) -> wgpu::CompilationInfo {
            self
        }
    }
}

pub use builder_texel_copy_buffer_layout::texel_copy_buffer_layout;
pub mod builder_texel_copy_buffer_layout {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texel_copy_buffer_layout() -> TexelCopyBufferLayoutBuilder<
        UnsetOffsetOptional,
        UnsetBytesPerRowOptional,
        UnsetRowsPerImageOptional,
    > {
        TexelCopyBufferLayoutBuilder::new()
    }
    pub struct TexelCopyBufferLayoutBuilder<T0, T1, T2> {
        offset: T0,
        bytes_per_row: T1,
        rows_per_image: T2,
    }
    impl
        TexelCopyBufferLayoutBuilder<
            UnsetOffsetOptional,
            UnsetBytesPerRowOptional,
            UnsetRowsPerImageOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                offset: UnsetOffsetOptional,
                bytes_per_row: UnsetBytesPerRowOptional,
                rows_per_image: UnsetRowsPerImageOptional,
            }
        }
    }
    pub struct UnsetOffsetOptional;
    impl IsOptional for UnsetOffsetOptional {}
    impl IsUnsetOptional for UnsetOffsetOptional {}
    impl ResolveOptional<wgpu::BufferAddress> for UnsetOffsetOptional {
        fn resolve(self) -> wgpu::BufferAddress {
            Default::default()
        }
    }
    pub struct OffsetOptionalValue(pub wgpu::BufferAddress);
    impl IsOptional for OffsetOptionalValue {}
    impl ResolveOptional<wgpu::BufferAddress> for OffsetOptionalValue {
        fn resolve(self) -> wgpu::BufferAddress {
            self.0
        }
    }
    pub struct UnsetBytesPerRowOptional;
    impl IsOptional for UnsetBytesPerRowOptional {}
    impl IsUnsetOptional for UnsetBytesPerRowOptional {}
    impl ResolveOptional<Option<u32>> for UnsetBytesPerRowOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct BytesPerRowOptionalValue(pub Option<u32>);
    impl IsOptional for BytesPerRowOptionalValue {}
    impl ResolveOptional<Option<u32>> for BytesPerRowOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetRowsPerImageOptional;
    impl IsOptional for UnsetRowsPerImageOptional {}
    impl IsUnsetOptional for UnsetRowsPerImageOptional {}
    impl ResolveOptional<Option<u32>> for UnsetRowsPerImageOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct RowsPerImageOptionalValue(pub Option<u32>);
    impl IsOptional for RowsPerImageOptionalValue {}
    impl ResolveOptional<Option<u32>> for RowsPerImageOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> TexelCopyBufferLayoutBuilder<T0, T1, T2> {
        pub fn offset(
            self,
            offset: wgpu::BufferAddress,
        ) -> TexelCopyBufferLayoutBuilder<OffsetOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            TexelCopyBufferLayoutBuilder {
                offset: OffsetOptionalValue(offset),
                bytes_per_row: self.bytes_per_row,
                rows_per_image: self.rows_per_image,
            }
        }
        pub fn maybe_bytes_per_row(
            self,
            bytes_per_row: Option<u32>,
        ) -> TexelCopyBufferLayoutBuilder<T0, BytesPerRowOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            TexelCopyBufferLayoutBuilder {
                offset: self.offset,
                bytes_per_row: BytesPerRowOptionalValue(bytes_per_row),
                rows_per_image: self.rows_per_image,
            }
        }
        pub fn bytes_per_row(
            self,
            bytes_per_row: u32,
        ) -> TexelCopyBufferLayoutBuilder<T0, BytesPerRowOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            TexelCopyBufferLayoutBuilder {
                offset: self.offset,
                bytes_per_row: BytesPerRowOptionalValue(Some(bytes_per_row)),
                rows_per_image: self.rows_per_image,
            }
        }
        pub fn maybe_rows_per_image(
            self,
            rows_per_image: Option<u32>,
        ) -> TexelCopyBufferLayoutBuilder<T0, T1, RowsPerImageOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            TexelCopyBufferLayoutBuilder {
                offset: self.offset,
                bytes_per_row: self.bytes_per_row,
                rows_per_image: RowsPerImageOptionalValue(rows_per_image),
            }
        }
        pub fn rows_per_image(
            self,
            rows_per_image: u32,
        ) -> TexelCopyBufferLayoutBuilder<T0, T1, RowsPerImageOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            TexelCopyBufferLayoutBuilder {
                offset: self.offset,
                bytes_per_row: self.bytes_per_row,
                rows_per_image: RowsPerImageOptionalValue(Some(rows_per_image)),
            }
        }
    }
    impl<ROffset, RBytesPerRow, RRowsPerImage>
        TexelCopyBufferLayoutBuilder<ROffset, RBytesPerRow, RRowsPerImage>
    {
        pub fn build(self) -> wgpu::TexelCopyBufferLayout
        where
            ROffset: ResolveOptional<wgpu::BufferAddress>,
            RBytesPerRow: ResolveOptional<Option<u32>>,
            RRowsPerImage: ResolveOptional<Option<u32>>,
        {
            wgpu::TexelCopyBufferLayout {
                offset: self.offset.resolve(),
                bytes_per_row: self.bytes_per_row.resolve(),
                rows_per_image: self.rows_per_image.resolve(),
            }
        }
    }
    impl<ROffset, RBytesPerRow, RRowsPerImage> Nested<wgpu::TexelCopyBufferLayout>
        for TexelCopyBufferLayoutBuilder<ROffset, RBytesPerRow, RRowsPerImage>
    where
        ROffset: ResolveOptional<wgpu::BufferAddress>,
        RBytesPerRow: ResolveOptional<Option<u32>>,
        RRowsPerImage: ResolveOptional<Option<u32>>,
    {
        fn unnest(self) -> wgpu::TexelCopyBufferLayout {
            self.build()
        }
    }
    impl Nested<wgpu::TexelCopyBufferLayout> for wgpu::TexelCopyBufferLayout {
        fn unnest(self) -> wgpu::TexelCopyBufferLayout {
            self
        }
    }
}

pub use builder_origin_3_d::origin_3_d;
pub mod builder_origin_3_d {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn origin_3_d() -> Origin3dBuilder<UnsetXOptional, UnsetYOptional, UnsetZOptional> {
        Origin3dBuilder::new()
    }
    pub struct Origin3dBuilder<T0, T1, T2> {
        x: T0,
        y: T1,
        z: T2,
    }
    impl Origin3dBuilder<UnsetXOptional, UnsetYOptional, UnsetZOptional> {
        pub fn new() -> Self {
            Self {
                x: UnsetXOptional,
                y: UnsetYOptional,
                z: UnsetZOptional,
            }
        }
    }
    pub struct UnsetXOptional;
    impl IsOptional for UnsetXOptional {}
    impl IsUnsetOptional for UnsetXOptional {}
    impl ResolveOptional<u32> for UnsetXOptional {
        fn resolve(self) -> u32 {
            0
        }
    }
    pub struct XOptionalValue(pub u32);
    impl IsOptional for XOptionalValue {}
    impl ResolveOptional<u32> for XOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetYOptional;
    impl IsOptional for UnsetYOptional {}
    impl IsUnsetOptional for UnsetYOptional {}
    impl ResolveOptional<u32> for UnsetYOptional {
        fn resolve(self) -> u32 {
            0
        }
    }
    pub struct YOptionalValue(pub u32);
    impl IsOptional for YOptionalValue {}
    impl ResolveOptional<u32> for YOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetZOptional;
    impl IsOptional for UnsetZOptional {}
    impl IsUnsetOptional for UnsetZOptional {}
    impl ResolveOptional<u32> for UnsetZOptional {
        fn resolve(self) -> u32 {
            0
        }
    }
    pub struct ZOptionalValue(pub u32);
    impl IsOptional for ZOptionalValue {}
    impl ResolveOptional<u32> for ZOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> Origin3dBuilder<T0, T1, T2> {
        pub fn x(self, x: u32) -> Origin3dBuilder<XOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            Origin3dBuilder {
                x: XOptionalValue(x),
                y: self.y,
                z: self.z,
            }
        }
        pub fn y(self, y: u32) -> Origin3dBuilder<T0, YOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            Origin3dBuilder {
                x: self.x,
                y: YOptionalValue(y),
                z: self.z,
            }
        }
        pub fn z(self, z: u32) -> Origin3dBuilder<T0, T1, ZOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            Origin3dBuilder {
                x: self.x,
                y: self.y,
                z: ZOptionalValue(z),
            }
        }
    }
    impl<RX, RY, RZ> Origin3dBuilder<RX, RY, RZ> {
        pub fn build(self) -> wgpu::Origin3d
        where
            RX: ResolveOptional<u32>,
            RY: ResolveOptional<u32>,
            RZ: ResolveOptional<u32>,
        {
            wgpu::Origin3d {
                x: self.x.resolve(),
                y: self.y.resolve(),
                z: self.z.resolve(),
            }
        }
    }
    impl<RX, RY, RZ> Nested<wgpu::Origin3d> for Origin3dBuilder<RX, RY, RZ>
    where
        RX: ResolveOptional<u32>,
        RY: ResolveOptional<u32>,
        RZ: ResolveOptional<u32>,
    {
        fn unnest(self) -> wgpu::Origin3d {
            self.build()
        }
    }
    impl Nested<wgpu::Origin3d> for wgpu::Origin3d {
        fn unnest(self) -> wgpu::Origin3d {
            self
        }
    }
}

pub use builder_mesh_pipeline_descriptor::mesh_pipeline_descriptor;
pub mod builder_mesh_pipeline_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn mesh_pipeline_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> MeshPipelineDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetLayoutOptional,
        UnsetTaskOptional,
        UnsetMesh,
        UnsetPrimitiveOptional,
        UnsetDepthStencilOptional,
        UnsetMultisampleOptional,
        UnsetFragmentOptional,
        UnsetMultiviewOptional,
        UnsetCacheOptional,
    > {
        MeshPipelineDescriptorBuilder::new().label(label)
    }
    pub struct MeshPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
        label: T0,
        layout: T1,
        task: T2,
        mesh: T3,
        primitive: T4,
        depth_stencil: T5,
        multisample: T6,
        fragment: T7,
        multiview: T8,
        cache: T9,
    }
    impl
        MeshPipelineDescriptorBuilder<
            UnsetLabelOptional,
            UnsetLayoutOptional,
            UnsetTaskOptional,
            UnsetMesh,
            UnsetPrimitiveOptional,
            UnsetDepthStencilOptional,
            UnsetMultisampleOptional,
            UnsetFragmentOptional,
            UnsetMultiviewOptional,
            UnsetCacheOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                layout: UnsetLayoutOptional,
                task: UnsetTaskOptional,
                mesh: UnsetMesh,
                primitive: UnsetPrimitiveOptional,
                depth_stencil: UnsetDepthStencilOptional,
                multisample: UnsetMultisampleOptional,
                fragment: UnsetFragmentOptional,
                multiview: UnsetMultiviewOptional,
                cache: UnsetCacheOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetLayoutOptional;
    impl IsOptional for UnsetLayoutOptional {}
    impl IsUnsetOptional for UnsetLayoutOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineLayout>> for UnsetLayoutOptional {
        fn resolve(self) -> Option<&'a wgpu::PipelineLayout> {
            Default::default()
        }
    }
    pub struct LayoutOptionalValue<'a>(pub Option<&'a wgpu::PipelineLayout>);
    impl<'a> IsOptional for LayoutOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineLayout>> for LayoutOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::PipelineLayout> {
            self.0
        }
    }
    pub struct UnsetTaskOptional;
    impl IsOptional for UnsetTaskOptional {}
    impl IsUnsetOptional for UnsetTaskOptional {}
    impl<'a> ResolveOptional<Option<wgpu::TaskState<'a>>> for UnsetTaskOptional {
        fn resolve(self) -> Option<wgpu::TaskState<'a>> {
            Default::default()
        }
    }
    pub struct TaskOptionalValue<'a>(pub Option<wgpu::TaskState<'a>>);
    impl<'a> IsOptional for TaskOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<wgpu::TaskState<'a>>> for TaskOptionalValue<'a> {
        fn resolve(self) -> Option<wgpu::TaskState<'a>> {
            self.0
        }
    }
    pub struct UnsetMesh;
    impl IsRequired for UnsetMesh {}
    impl IsUnset for UnsetMesh {}
    pub struct MeshValue<'a>(pub wgpu::MeshState<'a>);
    impl<'a> IsRequired for MeshValue<'a> {}
    pub struct UnsetPrimitiveOptional;
    impl IsOptional for UnsetPrimitiveOptional {}
    impl IsUnsetOptional for UnsetPrimitiveOptional {}
    impl ResolveOptional<wgpu::PrimitiveState> for UnsetPrimitiveOptional {
        fn resolve(self) -> wgpu::PrimitiveState {
            Default::default()
        }
    }
    pub struct PrimitiveOptionalValue(pub wgpu::PrimitiveState);
    impl IsOptional for PrimitiveOptionalValue {}
    impl ResolveOptional<wgpu::PrimitiveState> for PrimitiveOptionalValue {
        fn resolve(self) -> wgpu::PrimitiveState {
            self.0
        }
    }
    pub struct UnsetDepthStencilOptional;
    impl IsOptional for UnsetDepthStencilOptional {}
    impl IsUnsetOptional for UnsetDepthStencilOptional {}
    impl ResolveOptional<Option<wgpu::DepthStencilState>> for UnsetDepthStencilOptional {
        fn resolve(self) -> Option<wgpu::DepthStencilState> {
            Default::default()
        }
    }
    pub struct DepthStencilOptionalValue(pub Option<wgpu::DepthStencilState>);
    impl IsOptional for DepthStencilOptionalValue {}
    impl ResolveOptional<Option<wgpu::DepthStencilState>> for DepthStencilOptionalValue {
        fn resolve(self) -> Option<wgpu::DepthStencilState> {
            self.0
        }
    }
    pub struct UnsetMultisampleOptional;
    impl IsOptional for UnsetMultisampleOptional {}
    impl IsUnsetOptional for UnsetMultisampleOptional {}
    impl ResolveOptional<wgpu::MultisampleState> for UnsetMultisampleOptional {
        fn resolve(self) -> wgpu::MultisampleState {
            Default::default()
        }
    }
    pub struct MultisampleOptionalValue(pub wgpu::MultisampleState);
    impl IsOptional for MultisampleOptionalValue {}
    impl ResolveOptional<wgpu::MultisampleState> for MultisampleOptionalValue {
        fn resolve(self) -> wgpu::MultisampleState {
            self.0
        }
    }
    pub struct UnsetFragmentOptional;
    impl IsOptional for UnsetFragmentOptional {}
    impl IsUnsetOptional for UnsetFragmentOptional {}
    impl<'a> ResolveOptional<Option<wgpu::FragmentState<'a>>> for UnsetFragmentOptional {
        fn resolve(self) -> Option<wgpu::FragmentState<'a>> {
            Default::default()
        }
    }
    pub struct FragmentOptionalValue<'a>(pub Option<wgpu::FragmentState<'a>>);
    impl<'a> IsOptional for FragmentOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<wgpu::FragmentState<'a>>> for FragmentOptionalValue<'a> {
        fn resolve(self) -> Option<wgpu::FragmentState<'a>> {
            self.0
        }
    }
    pub struct UnsetMultiviewOptional;
    impl IsOptional for UnsetMultiviewOptional {}
    impl IsUnsetOptional for UnsetMultiviewOptional {}
    impl ResolveOptional<Option<NonZeroU32>> for UnsetMultiviewOptional {
        fn resolve(self) -> Option<NonZeroU32> {
            Default::default()
        }
    }
    pub struct MultiviewOptionalValue(pub Option<NonZeroU32>);
    impl IsOptional for MultiviewOptionalValue {}
    impl ResolveOptional<Option<NonZeroU32>> for MultiviewOptionalValue {
        fn resolve(self) -> Option<NonZeroU32> {
            self.0
        }
    }
    pub struct UnsetCacheOptional;
    impl IsOptional for UnsetCacheOptional {}
    impl IsUnsetOptional for UnsetCacheOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineCache>> for UnsetCacheOptional {
        fn resolve(self) -> Option<&'a wgpu::PipelineCache> {
            Default::default()
        }
    }
    pub struct CacheOptionalValue<'a>(pub Option<&'a wgpu::PipelineCache>);
    impl<'a> IsOptional for CacheOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineCache>> for CacheOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::PipelineCache> {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsOptional,
        T3: IsRequired,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
        T7: IsOptional,
        T8: IsOptional,
        T9: IsOptional,
    > MeshPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> MeshPipelineDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4, T5, T6, T7, T8, T9>
        where
            T0: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: LabelOptionalValue(label),
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_layout<'a>(
            self,
            layout: Option<&'a wgpu::PipelineLayout>,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            LayoutOptionalValue<'a>,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T1: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: LayoutOptionalValue(layout),
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn layout<'a>(
            self,
            layout: &'a wgpu::PipelineLayout,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            LayoutOptionalValue<'a>,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T1: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: LayoutOptionalValue(Some(layout)),
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_task<'a>(
            self,
            task: impl Nested<Option<wgpu::TaskState<'a>>>,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, TaskOptionalValue<'a>, T3, T4, T5, T6, T7, T8, T9>
        where
            T2: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: TaskOptionalValue(task.unnest()),
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn task<'a>(
            self,
            task: impl Nested<wgpu::TaskState<'a>>,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, TaskOptionalValue<'a>, T3, T4, T5, T6, T7, T8, T9>
        where
            T2: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: TaskOptionalValue(Some(task.unnest())),
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn mesh<'a>(
            self,
            mesh: impl Nested<wgpu::MeshState<'a>>,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, T2, MeshValue<'a>, T4, T5, T6, T7, T8, T9>
        where
            T3: IsUnset,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: MeshValue(mesh.unnest()),
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn primitive(
            self,
            primitive: impl Nested<wgpu::PrimitiveState>,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, T2, T3, PrimitiveOptionalValue, T5, T6, T7, T8, T9>
        where
            T4: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: PrimitiveOptionalValue(primitive.unnest()),
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_depth_stencil(
            self,
            depth_stencil: impl Nested<Option<wgpu::DepthStencilState>>,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            DepthStencilOptionalValue,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T5: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: DepthStencilOptionalValue(depth_stencil.unnest()),
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn depth_stencil(
            self,
            depth_stencil: impl Nested<wgpu::DepthStencilState>,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            DepthStencilOptionalValue,
            T6,
            T7,
            T8,
            T9,
        >
        where
            T5: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: DepthStencilOptionalValue(Some(depth_stencil.unnest())),
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn multisample(
            self,
            multisample: impl Nested<wgpu::MultisampleState>,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            MultisampleOptionalValue,
            T7,
            T8,
            T9,
        >
        where
            T6: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: MultisampleOptionalValue(multisample.unnest()),
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_fragment<'a>(
            self,
            fragment: impl Nested<Option<wgpu::FragmentState<'a>>>,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            FragmentOptionalValue<'a>,
            T8,
            T9,
        >
        where
            T7: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: FragmentOptionalValue(fragment.unnest()),
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn fragment<'a>(
            self,
            fragment: impl Nested<wgpu::FragmentState<'a>>,
        ) -> MeshPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            FragmentOptionalValue<'a>,
            T8,
            T9,
        >
        where
            T7: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: FragmentOptionalValue(Some(fragment.unnest())),
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_multiview(
            self,
            multiview: Option<NonZeroU32>,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, MultiviewOptionalValue, T9>
        where
            T8: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: MultiviewOptionalValue(multiview),
                cache: self.cache,
            }
        }
        pub fn multiview(
            self,
            multiview: NonZeroU32,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, MultiviewOptionalValue, T9>
        where
            T8: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: MultiviewOptionalValue(Some(multiview)),
                cache: self.cache,
            }
        }
        pub fn maybe_cache<'a>(
            self,
            cache: Option<&'a wgpu::PipelineCache>,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, CacheOptionalValue<'a>>
        where
            T9: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: CacheOptionalValue(cache),
            }
        }
        pub fn cache<'a>(
            self,
            cache: &'a wgpu::PipelineCache,
        ) -> MeshPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, CacheOptionalValue<'a>>
        where
            T9: IsUnsetOptional,
        {
            MeshPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                task: self.task,
                mesh: self.mesh,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: CacheOptionalValue(Some(cache)),
            }
        }
    }
    impl<
        'a,
        RLabel,
        RLayout,
        RTask,
        RPrimitive,
        RDepthStencil,
        RMultisample,
        RFragment,
        RMultiview,
        RCache,
    >
        MeshPipelineDescriptorBuilder<
            RLabel,
            RLayout,
            RTask,
            MeshValue<'a>,
            RPrimitive,
            RDepthStencil,
            RMultisample,
            RFragment,
            RMultiview,
            RCache,
        >
    {
        pub fn build(self) -> wgpu::MeshPipelineDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
            RTask: ResolveOptional<Option<wgpu::TaskState<'a>>>,
            RPrimitive: ResolveOptional<wgpu::PrimitiveState>,
            RDepthStencil: ResolveOptional<Option<wgpu::DepthStencilState>>,
            RMultisample: ResolveOptional<wgpu::MultisampleState>,
            RFragment: ResolveOptional<Option<wgpu::FragmentState<'a>>>,
            RMultiview: ResolveOptional<Option<NonZeroU32>>,
            RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
        {
            wgpu::MeshPipelineDescriptor {
                label: self.label.resolve(),
                layout: self.layout.resolve(),
                task: self.task.resolve(),
                mesh: self.mesh.0,
                primitive: self.primitive.resolve(),
                depth_stencil: self.depth_stencil.resolve(),
                multisample: self.multisample.resolve(),
                fragment: self.fragment.resolve(),
                multiview: self.multiview.resolve(),
                cache: self.cache.resolve(),
            }
        }
    }
    impl<
        'a,
        RLabel,
        RLayout,
        RTask,
        RPrimitive,
        RDepthStencil,
        RMultisample,
        RFragment,
        RMultiview,
        RCache,
    > Nested<wgpu::MeshPipelineDescriptor<'a>>
        for MeshPipelineDescriptorBuilder<
            RLabel,
            RLayout,
            RTask,
            MeshValue<'a>,
            RPrimitive,
            RDepthStencil,
            RMultisample,
            RFragment,
            RMultiview,
            RCache,
        >
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
        RTask: ResolveOptional<Option<wgpu::TaskState<'a>>>,
        RPrimitive: ResolveOptional<wgpu::PrimitiveState>,
        RDepthStencil: ResolveOptional<Option<wgpu::DepthStencilState>>,
        RMultisample: ResolveOptional<wgpu::MultisampleState>,
        RFragment: ResolveOptional<Option<wgpu::FragmentState<'a>>>,
        RMultiview: ResolveOptional<Option<NonZeroU32>>,
        RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
    {
        fn unnest(self) -> wgpu::MeshPipelineDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::MeshPipelineDescriptor<'a>> for wgpu::MeshPipelineDescriptor<'a> {
        fn unnest(self) -> wgpu::MeshPipelineDescriptor<'a> {
            self
        }
    }
}

pub use builder_device_descriptor::device_descriptor;
pub mod builder_device_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn device_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> DeviceDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetRequiredFeaturesOptional,
        UnsetRequiredLimitsOptional,
        UnsetExperimentalFeaturesOptional,
        UnsetMemoryHintsOptional,
        UnsetTraceOptional,
    > {
        DeviceDescriptorBuilder::new().label(label)
    }
    pub struct DeviceDescriptorBuilder<T0, T1, T2, T3, T4, T5> {
        label: T0,
        required_features: T1,
        required_limits: T2,
        experimental_features: T3,
        memory_hints: T4,
        trace: T5,
    }
    impl
        DeviceDescriptorBuilder<
            UnsetLabelOptional,
            UnsetRequiredFeaturesOptional,
            UnsetRequiredLimitsOptional,
            UnsetExperimentalFeaturesOptional,
            UnsetMemoryHintsOptional,
            UnsetTraceOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                required_features: UnsetRequiredFeaturesOptional,
                required_limits: UnsetRequiredLimitsOptional,
                experimental_features: UnsetExperimentalFeaturesOptional,
                memory_hints: UnsetMemoryHintsOptional,
                trace: UnsetTraceOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetRequiredFeaturesOptional;
    impl IsOptional for UnsetRequiredFeaturesOptional {}
    impl IsUnsetOptional for UnsetRequiredFeaturesOptional {}
    impl ResolveOptional<wgpu::Features> for UnsetRequiredFeaturesOptional {
        fn resolve(self) -> wgpu::Features {
            Default::default()
        }
    }
    pub struct RequiredFeaturesOptionalValue(pub wgpu::Features);
    impl IsOptional for RequiredFeaturesOptionalValue {}
    impl ResolveOptional<wgpu::Features> for RequiredFeaturesOptionalValue {
        fn resolve(self) -> wgpu::Features {
            self.0
        }
    }
    pub struct UnsetRequiredLimitsOptional;
    impl IsOptional for UnsetRequiredLimitsOptional {}
    impl IsUnsetOptional for UnsetRequiredLimitsOptional {}
    impl ResolveOptional<wgpu::Limits> for UnsetRequiredLimitsOptional {
        fn resolve(self) -> wgpu::Limits {
            Default::default()
        }
    }
    pub struct RequiredLimitsOptionalValue(pub wgpu::Limits);
    impl IsOptional for RequiredLimitsOptionalValue {}
    impl ResolveOptional<wgpu::Limits> for RequiredLimitsOptionalValue {
        fn resolve(self) -> wgpu::Limits {
            self.0
        }
    }
    pub struct UnsetExperimentalFeaturesOptional;
    impl IsOptional for UnsetExperimentalFeaturesOptional {}
    impl IsUnsetOptional for UnsetExperimentalFeaturesOptional {}
    impl ResolveOptional<wgpu::ExperimentalFeatures> for UnsetExperimentalFeaturesOptional {
        fn resolve(self) -> wgpu::ExperimentalFeatures {
            Default::default()
        }
    }
    pub struct ExperimentalFeaturesOptionalValue(pub wgpu::ExperimentalFeatures);
    impl IsOptional for ExperimentalFeaturesOptionalValue {}
    impl ResolveOptional<wgpu::ExperimentalFeatures> for ExperimentalFeaturesOptionalValue {
        fn resolve(self) -> wgpu::ExperimentalFeatures {
            self.0
        }
    }
    pub struct UnsetMemoryHintsOptional;
    impl IsOptional for UnsetMemoryHintsOptional {}
    impl IsUnsetOptional for UnsetMemoryHintsOptional {}
    impl ResolveOptional<wgpu::MemoryHints> for UnsetMemoryHintsOptional {
        fn resolve(self) -> wgpu::MemoryHints {
            Default::default()
        }
    }
    pub struct MemoryHintsOptionalValue(pub wgpu::MemoryHints);
    impl IsOptional for MemoryHintsOptionalValue {}
    impl ResolveOptional<wgpu::MemoryHints> for MemoryHintsOptionalValue {
        fn resolve(self) -> wgpu::MemoryHints {
            self.0
        }
    }
    pub struct UnsetTraceOptional;
    impl IsOptional for UnsetTraceOptional {}
    impl IsUnsetOptional for UnsetTraceOptional {}
    impl ResolveOptional<wgpu::Trace> for UnsetTraceOptional {
        fn resolve(self) -> wgpu::Trace {
            Default::default()
        }
    }
    pub struct TraceOptionalValue(pub wgpu::Trace);
    impl IsOptional for TraceOptionalValue {}
    impl ResolveOptional<wgpu::Trace> for TraceOptionalValue {
        fn resolve(self) -> wgpu::Trace {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsOptional,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
    > DeviceDescriptorBuilder<T0, T1, T2, T3, T4, T5>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> DeviceDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4, T5>
        where
            T0: IsUnsetOptional,
        {
            DeviceDescriptorBuilder {
                label: LabelOptionalValue(label),
                required_features: self.required_features,
                required_limits: self.required_limits,
                experimental_features: self.experimental_features,
                memory_hints: self.memory_hints,
                trace: self.trace,
            }
        }
        pub fn required_features(
            self,
            required_features: wgpu::Features,
        ) -> DeviceDescriptorBuilder<T0, RequiredFeaturesOptionalValue, T2, T3, T4, T5>
        where
            T1: IsUnsetOptional,
        {
            DeviceDescriptorBuilder {
                label: self.label,
                required_features: RequiredFeaturesOptionalValue(required_features),
                required_limits: self.required_limits,
                experimental_features: self.experimental_features,
                memory_hints: self.memory_hints,
                trace: self.trace,
            }
        }
        pub fn required_limits(
            self,
            required_limits: wgpu::Limits,
        ) -> DeviceDescriptorBuilder<T0, T1, RequiredLimitsOptionalValue, T3, T4, T5>
        where
            T2: IsUnsetOptional,
        {
            DeviceDescriptorBuilder {
                label: self.label,
                required_features: self.required_features,
                required_limits: RequiredLimitsOptionalValue(required_limits),
                experimental_features: self.experimental_features,
                memory_hints: self.memory_hints,
                trace: self.trace,
            }
        }
        pub fn experimental_features(
            self,
            experimental_features: wgpu::ExperimentalFeatures,
        ) -> DeviceDescriptorBuilder<T0, T1, T2, ExperimentalFeaturesOptionalValue, T4, T5>
        where
            T3: IsUnsetOptional,
        {
            DeviceDescriptorBuilder {
                label: self.label,
                required_features: self.required_features,
                required_limits: self.required_limits,
                experimental_features: ExperimentalFeaturesOptionalValue(experimental_features),
                memory_hints: self.memory_hints,
                trace: self.trace,
            }
        }
        pub fn memory_hints(
            self,
            memory_hints: wgpu::MemoryHints,
        ) -> DeviceDescriptorBuilder<T0, T1, T2, T3, MemoryHintsOptionalValue, T5>
        where
            T4: IsUnsetOptional,
        {
            DeviceDescriptorBuilder {
                label: self.label,
                required_features: self.required_features,
                required_limits: self.required_limits,
                experimental_features: self.experimental_features,
                memory_hints: MemoryHintsOptionalValue(memory_hints),
                trace: self.trace,
            }
        }
        pub fn trace(
            self,
            trace: wgpu::Trace,
        ) -> DeviceDescriptorBuilder<T0, T1, T2, T3, T4, TraceOptionalValue>
        where
            T5: IsUnsetOptional,
        {
            DeviceDescriptorBuilder {
                label: self.label,
                required_features: self.required_features,
                required_limits: self.required_limits,
                experimental_features: self.experimental_features,
                memory_hints: self.memory_hints,
                trace: TraceOptionalValue(trace),
            }
        }
    }
    impl<RLabel, RRequiredFeatures, RRequiredLimits, RExperimentalFeatures, RMemoryHints, RTrace>
        DeviceDescriptorBuilder<
            RLabel,
            RRequiredFeatures,
            RRequiredLimits,
            RExperimentalFeatures,
            RMemoryHints,
            RTrace,
        >
    {
        pub fn build<'a>(self) -> wgpu::DeviceDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RRequiredFeatures: ResolveOptional<wgpu::Features>,
            RRequiredLimits: ResolveOptional<wgpu::Limits>,
            RExperimentalFeatures: ResolveOptional<wgpu::ExperimentalFeatures>,
            RMemoryHints: ResolveOptional<wgpu::MemoryHints>,
            RTrace: ResolveOptional<wgpu::Trace>,
        {
            wgpu::DeviceDescriptor {
                label: self.label.resolve(),
                required_features: self.required_features.resolve(),
                required_limits: self.required_limits.resolve(),
                experimental_features: self.experimental_features.resolve(),
                memory_hints: self.memory_hints.resolve(),
                trace: self.trace.resolve(),
            }
        }
    }
}

pub use builder_render_pipeline_descriptor::render_pipeline_descriptor;
pub mod builder_render_pipeline_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_pipeline_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> RenderPipelineDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetLayoutOptional,
        UnsetVertex,
        UnsetPrimitiveOptional,
        UnsetDepthStencilOptional,
        UnsetMultisampleOptional,
        UnsetFragmentOptional,
        UnsetMultiviewOptional,
        UnsetCacheOptional,
    > {
        RenderPipelineDescriptorBuilder::new().label(label)
    }
    pub struct RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
        label: T0,
        layout: T1,
        vertex: T2,
        primitive: T3,
        depth_stencil: T4,
        multisample: T5,
        fragment: T6,
        multiview: T7,
        cache: T8,
    }
    impl
        RenderPipelineDescriptorBuilder<
            UnsetLabelOptional,
            UnsetLayoutOptional,
            UnsetVertex,
            UnsetPrimitiveOptional,
            UnsetDepthStencilOptional,
            UnsetMultisampleOptional,
            UnsetFragmentOptional,
            UnsetMultiviewOptional,
            UnsetCacheOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                layout: UnsetLayoutOptional,
                vertex: UnsetVertex,
                primitive: UnsetPrimitiveOptional,
                depth_stencil: UnsetDepthStencilOptional,
                multisample: UnsetMultisampleOptional,
                fragment: UnsetFragmentOptional,
                multiview: UnsetMultiviewOptional,
                cache: UnsetCacheOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetLayoutOptional;
    impl IsOptional for UnsetLayoutOptional {}
    impl IsUnsetOptional for UnsetLayoutOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineLayout>> for UnsetLayoutOptional {
        fn resolve(self) -> Option<&'a wgpu::PipelineLayout> {
            Default::default()
        }
    }
    pub struct LayoutOptionalValue<'a>(pub Option<&'a wgpu::PipelineLayout>);
    impl<'a> IsOptional for LayoutOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineLayout>> for LayoutOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::PipelineLayout> {
            self.0
        }
    }
    pub struct UnsetVertex;
    impl IsRequired for UnsetVertex {}
    impl IsUnset for UnsetVertex {}
    pub struct VertexValue<'a>(pub wgpu::VertexState<'a>);
    impl<'a> IsRequired for VertexValue<'a> {}
    pub struct UnsetPrimitiveOptional;
    impl IsOptional for UnsetPrimitiveOptional {}
    impl IsUnsetOptional for UnsetPrimitiveOptional {}
    impl ResolveOptional<wgpu::PrimitiveState> for UnsetPrimitiveOptional {
        fn resolve(self) -> wgpu::PrimitiveState {
            Default::default()
        }
    }
    pub struct PrimitiveOptionalValue(pub wgpu::PrimitiveState);
    impl IsOptional for PrimitiveOptionalValue {}
    impl ResolveOptional<wgpu::PrimitiveState> for PrimitiveOptionalValue {
        fn resolve(self) -> wgpu::PrimitiveState {
            self.0
        }
    }
    pub struct UnsetDepthStencilOptional;
    impl IsOptional for UnsetDepthStencilOptional {}
    impl IsUnsetOptional for UnsetDepthStencilOptional {}
    impl ResolveOptional<Option<wgpu::DepthStencilState>> for UnsetDepthStencilOptional {
        fn resolve(self) -> Option<wgpu::DepthStencilState> {
            Default::default()
        }
    }
    pub struct DepthStencilOptionalValue(pub Option<wgpu::DepthStencilState>);
    impl IsOptional for DepthStencilOptionalValue {}
    impl ResolveOptional<Option<wgpu::DepthStencilState>> for DepthStencilOptionalValue {
        fn resolve(self) -> Option<wgpu::DepthStencilState> {
            self.0
        }
    }
    pub struct UnsetMultisampleOptional;
    impl IsOptional for UnsetMultisampleOptional {}
    impl IsUnsetOptional for UnsetMultisampleOptional {}
    impl ResolveOptional<wgpu::MultisampleState> for UnsetMultisampleOptional {
        fn resolve(self) -> wgpu::MultisampleState {
            Default::default()
        }
    }
    pub struct MultisampleOptionalValue(pub wgpu::MultisampleState);
    impl IsOptional for MultisampleOptionalValue {}
    impl ResolveOptional<wgpu::MultisampleState> for MultisampleOptionalValue {
        fn resolve(self) -> wgpu::MultisampleState {
            self.0
        }
    }
    pub struct UnsetFragmentOptional;
    impl IsOptional for UnsetFragmentOptional {}
    impl IsUnsetOptional for UnsetFragmentOptional {}
    impl<'a> ResolveOptional<Option<wgpu::FragmentState<'a>>> for UnsetFragmentOptional {
        fn resolve(self) -> Option<wgpu::FragmentState<'a>> {
            Default::default()
        }
    }
    pub struct FragmentOptionalValue<'a>(pub Option<wgpu::FragmentState<'a>>);
    impl<'a> IsOptional for FragmentOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<wgpu::FragmentState<'a>>> for FragmentOptionalValue<'a> {
        fn resolve(self) -> Option<wgpu::FragmentState<'a>> {
            self.0
        }
    }
    pub struct UnsetMultiviewOptional;
    impl IsOptional for UnsetMultiviewOptional {}
    impl IsUnsetOptional for UnsetMultiviewOptional {}
    impl ResolveOptional<Option<NonZeroU32>> for UnsetMultiviewOptional {
        fn resolve(self) -> Option<NonZeroU32> {
            Default::default()
        }
    }
    pub struct MultiviewOptionalValue(pub Option<NonZeroU32>);
    impl IsOptional for MultiviewOptionalValue {}
    impl ResolveOptional<Option<NonZeroU32>> for MultiviewOptionalValue {
        fn resolve(self) -> Option<NonZeroU32> {
            self.0
        }
    }
    pub struct UnsetCacheOptional;
    impl IsOptional for UnsetCacheOptional {}
    impl IsUnsetOptional for UnsetCacheOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineCache>> for UnsetCacheOptional {
        fn resolve(self) -> Option<&'a wgpu::PipelineCache> {
            Default::default()
        }
    }
    pub struct CacheOptionalValue<'a>(pub Option<&'a wgpu::PipelineCache>);
    impl<'a> IsOptional for CacheOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::PipelineCache>> for CacheOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::PipelineCache> {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsRequired,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
        T7: IsOptional,
        T8: IsOptional,
    > RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> RenderPipelineDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4, T5, T6, T7, T8>
        where
            T0: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: LabelOptionalValue(label),
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_layout<'a>(
            self,
            layout: Option<&'a wgpu::PipelineLayout>,
        ) -> RenderPipelineDescriptorBuilder<T0, LayoutOptionalValue<'a>, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: LayoutOptionalValue(layout),
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn layout<'a>(
            self,
            layout: &'a wgpu::PipelineLayout,
        ) -> RenderPipelineDescriptorBuilder<T0, LayoutOptionalValue<'a>, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: LayoutOptionalValue(Some(layout)),
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn vertex<'a>(
            self,
            vertex: impl Nested<wgpu::VertexState<'a>>,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, VertexValue<'a>, T3, T4, T5, T6, T7, T8>
        where
            T2: IsUnset,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: VertexValue(vertex.unnest()),
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn primitive(
            self,
            primitive: impl Nested<wgpu::PrimitiveState>,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, T2, PrimitiveOptionalValue, T4, T5, T6, T7, T8>
        where
            T3: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: PrimitiveOptionalValue(primitive.unnest()),
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_depth_stencil(
            self,
            depth_stencil: impl Nested<Option<wgpu::DepthStencilState>>,
        ) -> RenderPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            DepthStencilOptionalValue,
            T5,
            T6,
            T7,
            T8,
        >
        where
            T4: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: DepthStencilOptionalValue(depth_stencil.unnest()),
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn depth_stencil(
            self,
            depth_stencil: impl Nested<wgpu::DepthStencilState>,
        ) -> RenderPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            DepthStencilOptionalValue,
            T5,
            T6,
            T7,
            T8,
        >
        where
            T4: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: DepthStencilOptionalValue(Some(depth_stencil.unnest())),
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn multisample(
            self,
            multisample: impl Nested<wgpu::MultisampleState>,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, MultisampleOptionalValue, T6, T7, T8>
        where
            T5: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: MultisampleOptionalValue(multisample.unnest()),
                fragment: self.fragment,
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_fragment<'a>(
            self,
            fragment: impl Nested<Option<wgpu::FragmentState<'a>>>,
        ) -> RenderPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            FragmentOptionalValue<'a>,
            T7,
            T8,
        >
        where
            T6: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: FragmentOptionalValue(fragment.unnest()),
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn fragment<'a>(
            self,
            fragment: impl Nested<wgpu::FragmentState<'a>>,
        ) -> RenderPipelineDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            FragmentOptionalValue<'a>,
            T7,
            T8,
        >
        where
            T6: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: FragmentOptionalValue(Some(fragment.unnest())),
                multiview: self.multiview,
                cache: self.cache,
            }
        }
        pub fn maybe_multiview(
            self,
            multiview: Option<NonZeroU32>,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, MultiviewOptionalValue, T8>
        where
            T7: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: MultiviewOptionalValue(multiview),
                cache: self.cache,
            }
        }
        pub fn multiview(
            self,
            multiview: NonZeroU32,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, MultiviewOptionalValue, T8>
        where
            T7: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: MultiviewOptionalValue(Some(multiview)),
                cache: self.cache,
            }
        }
        pub fn maybe_cache<'a>(
            self,
            cache: Option<&'a wgpu::PipelineCache>,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, CacheOptionalValue<'a>>
        where
            T8: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: CacheOptionalValue(cache),
            }
        }
        pub fn cache<'a>(
            self,
            cache: &'a wgpu::PipelineCache,
        ) -> RenderPipelineDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, CacheOptionalValue<'a>>
        where
            T8: IsUnsetOptional,
        {
            RenderPipelineDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                vertex: self.vertex,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: self.fragment,
                multiview: self.multiview,
                cache: CacheOptionalValue(Some(cache)),
            }
        }
    }
    impl<
        'a,
        RLabel,
        RLayout,
        RPrimitive,
        RDepthStencil,
        RMultisample,
        RFragment,
        RMultiview,
        RCache,
    >
        RenderPipelineDescriptorBuilder<
            RLabel,
            RLayout,
            VertexValue<'a>,
            RPrimitive,
            RDepthStencil,
            RMultisample,
            RFragment,
            RMultiview,
            RCache,
        >
    {
        pub fn build(self) -> wgpu::RenderPipelineDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
            RPrimitive: ResolveOptional<wgpu::PrimitiveState>,
            RDepthStencil: ResolveOptional<Option<wgpu::DepthStencilState>>,
            RMultisample: ResolveOptional<wgpu::MultisampleState>,
            RFragment: ResolveOptional<Option<wgpu::FragmentState<'a>>>,
            RMultiview: ResolveOptional<Option<NonZeroU32>>,
            RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
        {
            wgpu::RenderPipelineDescriptor {
                label: self.label.resolve(),
                layout: self.layout.resolve(),
                vertex: self.vertex.0,
                primitive: self.primitive.resolve(),
                depth_stencil: self.depth_stencil.resolve(),
                multisample: self.multisample.resolve(),
                fragment: self.fragment.resolve(),
                multiview: self.multiview.resolve(),
                cache: self.cache.resolve(),
            }
        }
    }
    impl<
        'a,
        RLabel,
        RLayout,
        RPrimitive,
        RDepthStencil,
        RMultisample,
        RFragment,
        RMultiview,
        RCache,
    > Nested<wgpu::RenderPipelineDescriptor<'a>>
        for RenderPipelineDescriptorBuilder<
            RLabel,
            RLayout,
            VertexValue<'a>,
            RPrimitive,
            RDepthStencil,
            RMultisample,
            RFragment,
            RMultiview,
            RCache,
        >
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RLayout: ResolveOptional<Option<&'a wgpu::PipelineLayout>>,
        RPrimitive: ResolveOptional<wgpu::PrimitiveState>,
        RDepthStencil: ResolveOptional<Option<wgpu::DepthStencilState>>,
        RMultisample: ResolveOptional<wgpu::MultisampleState>,
        RFragment: ResolveOptional<Option<wgpu::FragmentState<'a>>>,
        RMultiview: ResolveOptional<Option<NonZeroU32>>,
        RCache: ResolveOptional<Option<&'a wgpu::PipelineCache>>,
    {
        fn unnest(self) -> wgpu::RenderPipelineDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::RenderPipelineDescriptor<'a>> for wgpu::RenderPipelineDescriptor<'a> {
        fn unnest(self) -> wgpu::RenderPipelineDescriptor<'a> {
            self
        }
    }
}

pub use builder_texel_copy_texture_info_base::texel_copy_texture_info_base;
pub mod builder_texel_copy_texture_info_base {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texel_copy_texture_info_base() -> TexelCopyTextureInfoBaseBuilder<
        UnsetTexture,
        UnsetMipLevel,
        UnsetOriginOptional,
        UnsetAspectOptional,
    > {
        TexelCopyTextureInfoBaseBuilder::new()
    }
    pub struct TexelCopyTextureInfoBaseBuilder<T0, T1, T2, T3> {
        texture: T0,
        mip_level: T1,
        origin: T2,
        aspect: T3,
    }
    impl
        TexelCopyTextureInfoBaseBuilder<
            UnsetTexture,
            UnsetMipLevel,
            UnsetOriginOptional,
            UnsetAspectOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                texture: UnsetTexture,
                mip_level: UnsetMipLevel,
                origin: UnsetOriginOptional,
                aspect: UnsetAspectOptional,
            }
        }
    }
    pub struct UnsetTexture;
    impl IsRequired for UnsetTexture {}
    impl IsUnset for UnsetTexture {}
    pub struct TextureValue<T>(pub T);
    impl<T> IsRequired for TextureValue<T> {}
    pub struct UnsetMipLevel;
    impl IsRequired for UnsetMipLevel {}
    impl IsUnset for UnsetMipLevel {}
    pub struct MipLevelValue(pub u32);
    impl IsRequired for MipLevelValue {}
    pub struct UnsetOriginOptional;
    impl IsOptional for UnsetOriginOptional {}
    impl IsUnsetOptional for UnsetOriginOptional {}
    impl ResolveOptional<wgpu::Origin3d> for UnsetOriginOptional {
        fn resolve(self) -> wgpu::Origin3d {
            Default::default()
        }
    }
    pub struct OriginOptionalValue(pub wgpu::Origin3d);
    impl IsOptional for OriginOptionalValue {}
    impl ResolveOptional<wgpu::Origin3d> for OriginOptionalValue {
        fn resolve(self) -> wgpu::Origin3d {
            self.0
        }
    }
    pub struct UnsetAspectOptional;
    impl IsOptional for UnsetAspectOptional {}
    impl IsUnsetOptional for UnsetAspectOptional {}
    impl ResolveOptional<wgpu::TextureAspect> for UnsetAspectOptional {
        fn resolve(self) -> wgpu::TextureAspect {
            Default::default()
        }
    }
    pub struct AspectOptionalValue(pub wgpu::TextureAspect);
    impl IsOptional for AspectOptionalValue {}
    impl ResolveOptional<wgpu::TextureAspect> for AspectOptionalValue {
        fn resolve(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsRequired, T2: IsOptional, T3: IsOptional>
        TexelCopyTextureInfoBaseBuilder<T0, T1, T2, T3>
    {
        pub fn texture<T>(
            self,
            texture: T,
        ) -> TexelCopyTextureInfoBaseBuilder<TextureValue<T>, T1, T2, T3>
        where
            T0: IsUnset,
        {
            TexelCopyTextureInfoBaseBuilder {
                texture: TextureValue(texture),
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: self.aspect,
            }
        }
        pub fn mip_level(
            self,
            mip_level: u32,
        ) -> TexelCopyTextureInfoBaseBuilder<T0, MipLevelValue, T2, T3>
        where
            T1: IsUnset,
        {
            TexelCopyTextureInfoBaseBuilder {
                texture: self.texture,
                mip_level: MipLevelValue(mip_level),
                origin: self.origin,
                aspect: self.aspect,
            }
        }
        pub fn origin(
            self,
            origin: impl Nested<wgpu::Origin3d>,
        ) -> TexelCopyTextureInfoBaseBuilder<T0, T1, OriginOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            TexelCopyTextureInfoBaseBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: OriginOptionalValue(origin.unnest()),
                aspect: self.aspect,
            }
        }
        pub fn aspect(
            self,
            aspect: wgpu::TextureAspect,
        ) -> TexelCopyTextureInfoBaseBuilder<T0, T1, T2, AspectOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            TexelCopyTextureInfoBaseBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: AspectOptionalValue(aspect),
            }
        }
    }
    impl<ROrigin, RAspect, T>
        TexelCopyTextureInfoBaseBuilder<TextureValue<T>, MipLevelValue, ROrigin, RAspect>
    {
        pub fn build(self) -> wgpu::TexelCopyTextureInfoBase<T>
        where
            ROrigin: ResolveOptional<wgpu::Origin3d>,
            RAspect: ResolveOptional<wgpu::TextureAspect>,
        {
            wgpu::TexelCopyTextureInfoBase {
                texture: self.texture.0,
                mip_level: self.mip_level.0,
                origin: self.origin.resolve(),
                aspect: self.aspect.resolve(),
            }
        }
    }
    impl<ROrigin, RAspect, T> Nested<wgpu::TexelCopyTextureInfoBase<T>>
        for TexelCopyTextureInfoBaseBuilder<TextureValue<T>, MipLevelValue, ROrigin, RAspect>
    where
        ROrigin: ResolveOptional<wgpu::Origin3d>,
        RAspect: ResolveOptional<wgpu::TextureAspect>,
    {
        fn unnest(self) -> wgpu::TexelCopyTextureInfoBase<T> {
            self.build()
        }
    }
    impl<T> Nested<wgpu::TexelCopyTextureInfoBase<T>> for wgpu::TexelCopyTextureInfoBase<T> {
        fn unnest(self) -> wgpu::TexelCopyTextureInfoBase<T> {
            self
        }
    }
}

pub use builder_external_texture_transfer_function::external_texture_transfer_function;
pub mod builder_external_texture_transfer_function {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn external_texture_transfer_function() -> ExternalTextureTransferFunctionBuilder<
        UnsetAOptional,
        UnsetBOptional,
        UnsetGOptional,
        UnsetKOptional,
    > {
        ExternalTextureTransferFunctionBuilder::new()
    }
    pub struct ExternalTextureTransferFunctionBuilder<T0, T1, T2, T3> {
        a: T0,
        b: T1,
        g: T2,
        k: T3,
    }
    impl
        ExternalTextureTransferFunctionBuilder<
            UnsetAOptional,
            UnsetBOptional,
            UnsetGOptional,
            UnsetKOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                a: UnsetAOptional,
                b: UnsetBOptional,
                g: UnsetGOptional,
                k: UnsetKOptional,
            }
        }
    }
    pub struct UnsetAOptional;
    impl IsOptional for UnsetAOptional {}
    impl IsUnsetOptional for UnsetAOptional {}
    impl ResolveOptional<f32> for UnsetAOptional {
        fn resolve(self) -> f32 {
            1.0
        }
    }
    pub struct AOptionalValue(pub f32);
    impl IsOptional for AOptionalValue {}
    impl ResolveOptional<f32> for AOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    pub struct UnsetBOptional;
    impl IsOptional for UnsetBOptional {}
    impl IsUnsetOptional for UnsetBOptional {}
    impl ResolveOptional<f32> for UnsetBOptional {
        fn resolve(self) -> f32 {
            1.0
        }
    }
    pub struct BOptionalValue(pub f32);
    impl IsOptional for BOptionalValue {}
    impl ResolveOptional<f32> for BOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    pub struct UnsetGOptional;
    impl IsOptional for UnsetGOptional {}
    impl IsUnsetOptional for UnsetGOptional {}
    impl ResolveOptional<f32> for UnsetGOptional {
        fn resolve(self) -> f32 {
            1.0
        }
    }
    pub struct GOptionalValue(pub f32);
    impl IsOptional for GOptionalValue {}
    impl ResolveOptional<f32> for GOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    pub struct UnsetKOptional;
    impl IsOptional for UnsetKOptional {}
    impl IsUnsetOptional for UnsetKOptional {}
    impl ResolveOptional<f32> for UnsetKOptional {
        fn resolve(self) -> f32 {
            1.0
        }
    }
    pub struct KOptionalValue(pub f32);
    impl IsOptional for KOptionalValue {}
    impl ResolveOptional<f32> for KOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional>
        ExternalTextureTransferFunctionBuilder<T0, T1, T2, T3>
    {
        pub fn a(self, a: f32) -> ExternalTextureTransferFunctionBuilder<AOptionalValue, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            ExternalTextureTransferFunctionBuilder {
                a: AOptionalValue(a),
                b: self.b,
                g: self.g,
                k: self.k,
            }
        }
        pub fn b(self, b: f32) -> ExternalTextureTransferFunctionBuilder<T0, BOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            ExternalTextureTransferFunctionBuilder {
                a: self.a,
                b: BOptionalValue(b),
                g: self.g,
                k: self.k,
            }
        }
        pub fn g(self, g: f32) -> ExternalTextureTransferFunctionBuilder<T0, T1, GOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            ExternalTextureTransferFunctionBuilder {
                a: self.a,
                b: self.b,
                g: GOptionalValue(g),
                k: self.k,
            }
        }
        pub fn k(self, k: f32) -> ExternalTextureTransferFunctionBuilder<T0, T1, T2, KOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            ExternalTextureTransferFunctionBuilder {
                a: self.a,
                b: self.b,
                g: self.g,
                k: KOptionalValue(k),
            }
        }
    }
    impl<RA, RB, RG, RK> ExternalTextureTransferFunctionBuilder<RA, RB, RG, RK> {
        pub fn build(self) -> wgpu::ExternalTextureTransferFunction
        where
            RA: ResolveOptional<f32>,
            RB: ResolveOptional<f32>,
            RG: ResolveOptional<f32>,
            RK: ResolveOptional<f32>,
        {
            wgpu::ExternalTextureTransferFunction {
                a: self.a.resolve(),
                b: self.b.resolve(),
                g: self.g.resolve(),
                k: self.k.resolve(),
            }
        }
    }
    impl<RA, RB, RG, RK> Nested<wgpu::ExternalTextureTransferFunction>
        for ExternalTextureTransferFunctionBuilder<RA, RB, RG, RK>
    where
        RA: ResolveOptional<f32>,
        RB: ResolveOptional<f32>,
        RG: ResolveOptional<f32>,
        RK: ResolveOptional<f32>,
    {
        fn unnest(self) -> wgpu::ExternalTextureTransferFunction {
            self.build()
        }
    }
    impl Nested<wgpu::ExternalTextureTransferFunction> for wgpu::ExternalTextureTransferFunction {
        fn unnest(self) -> wgpu::ExternalTextureTransferFunction {
            self
        }
    }
}

pub use builder_memory_budget_thresholds::memory_budget_thresholds;
pub mod builder_memory_budget_thresholds {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn memory_budget_thresholds()
    -> MemoryBudgetThresholdsBuilder<UnsetForResourceCreationOptional, UnsetForDeviceLossOptional>
    {
        MemoryBudgetThresholdsBuilder::new()
    }
    pub struct MemoryBudgetThresholdsBuilder<T0, T1> {
        for_resource_creation: T0,
        for_device_loss: T1,
    }
    impl MemoryBudgetThresholdsBuilder<UnsetForResourceCreationOptional, UnsetForDeviceLossOptional> {
        pub fn new() -> Self {
            Self {
                for_resource_creation: UnsetForResourceCreationOptional,
                for_device_loss: UnsetForDeviceLossOptional,
            }
        }
    }
    pub struct UnsetForResourceCreationOptional;
    impl IsOptional for UnsetForResourceCreationOptional {}
    impl IsUnsetOptional for UnsetForResourceCreationOptional {}
    impl ResolveOptional<Option<u8>> for UnsetForResourceCreationOptional {
        fn resolve(self) -> Option<u8> {
            Default::default()
        }
    }
    pub struct ForResourceCreationOptionalValue(pub Option<u8>);
    impl IsOptional for ForResourceCreationOptionalValue {}
    impl ResolveOptional<Option<u8>> for ForResourceCreationOptionalValue {
        fn resolve(self) -> Option<u8> {
            self.0
        }
    }
    pub struct UnsetForDeviceLossOptional;
    impl IsOptional for UnsetForDeviceLossOptional {}
    impl IsUnsetOptional for UnsetForDeviceLossOptional {}
    impl ResolveOptional<Option<u8>> for UnsetForDeviceLossOptional {
        fn resolve(self) -> Option<u8> {
            Default::default()
        }
    }
    pub struct ForDeviceLossOptionalValue(pub Option<u8>);
    impl IsOptional for ForDeviceLossOptionalValue {}
    impl ResolveOptional<Option<u8>> for ForDeviceLossOptionalValue {
        fn resolve(self) -> Option<u8> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional> MemoryBudgetThresholdsBuilder<T0, T1> {
        pub fn maybe_for_resource_creation(
            self,
            for_resource_creation: Option<u8>,
        ) -> MemoryBudgetThresholdsBuilder<ForResourceCreationOptionalValue, T1>
        where
            T0: IsUnsetOptional,
        {
            MemoryBudgetThresholdsBuilder {
                for_resource_creation: ForResourceCreationOptionalValue(for_resource_creation),
                for_device_loss: self.for_device_loss,
            }
        }
        pub fn for_resource_creation(
            self,
            for_resource_creation: u8,
        ) -> MemoryBudgetThresholdsBuilder<ForResourceCreationOptionalValue, T1>
        where
            T0: IsUnsetOptional,
        {
            MemoryBudgetThresholdsBuilder {
                for_resource_creation: ForResourceCreationOptionalValue(Some(
                    for_resource_creation,
                )),
                for_device_loss: self.for_device_loss,
            }
        }
        pub fn maybe_for_device_loss(
            self,
            for_device_loss: Option<u8>,
        ) -> MemoryBudgetThresholdsBuilder<T0, ForDeviceLossOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            MemoryBudgetThresholdsBuilder {
                for_resource_creation: self.for_resource_creation,
                for_device_loss: ForDeviceLossOptionalValue(for_device_loss),
            }
        }
        pub fn for_device_loss(
            self,
            for_device_loss: u8,
        ) -> MemoryBudgetThresholdsBuilder<T0, ForDeviceLossOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            MemoryBudgetThresholdsBuilder {
                for_resource_creation: self.for_resource_creation,
                for_device_loss: ForDeviceLossOptionalValue(Some(for_device_loss)),
            }
        }
    }
    impl<RForResourceCreation, RForDeviceLoss>
        MemoryBudgetThresholdsBuilder<RForResourceCreation, RForDeviceLoss>
    {
        pub fn build(self) -> wgpu::MemoryBudgetThresholds
        where
            RForResourceCreation: ResolveOptional<Option<u8>>,
            RForDeviceLoss: ResolveOptional<Option<u8>>,
        {
            wgpu::MemoryBudgetThresholds {
                for_resource_creation: self.for_resource_creation.resolve(),
                for_device_loss: self.for_device_loss.resolve(),
            }
        }
    }
    impl<RForResourceCreation, RForDeviceLoss> Nested<wgpu::MemoryBudgetThresholds>
        for MemoryBudgetThresholdsBuilder<RForResourceCreation, RForDeviceLoss>
    where
        RForResourceCreation: ResolveOptional<Option<u8>>,
        RForDeviceLoss: ResolveOptional<Option<u8>>,
    {
        fn unnest(self) -> wgpu::MemoryBudgetThresholds {
            self.build()
        }
    }
    impl Nested<wgpu::MemoryBudgetThresholds> for wgpu::MemoryBudgetThresholds {
        fn unnest(self) -> wgpu::MemoryBudgetThresholds {
            self
        }
    }
}

pub use builder_render_bundle_encoder_descriptor::render_bundle_encoder_descriptor;
pub mod builder_render_bundle_encoder_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_bundle_encoder_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> RenderBundleEncoderDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetColorFormatsOptional,
        UnsetDepthStencilOptional,
        UnsetSampleCountOptional,
        UnsetMultiviewOptional,
    > {
        RenderBundleEncoderDescriptorBuilder::new().label(label)
    }
    pub struct RenderBundleEncoderDescriptorBuilder<T0, T1, T2, T3, T4> {
        label: T0,
        color_formats: T1,
        depth_stencil: T2,
        sample_count: T3,
        multiview: T4,
    }
    impl
        RenderBundleEncoderDescriptorBuilder<
            UnsetLabelOptional,
            UnsetColorFormatsOptional,
            UnsetDepthStencilOptional,
            UnsetSampleCountOptional,
            UnsetMultiviewOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                color_formats: UnsetColorFormatsOptional,
                depth_stencil: UnsetDepthStencilOptional,
                sample_count: UnsetSampleCountOptional,
                multiview: UnsetMultiviewOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetColorFormatsOptional;
    impl IsOptional for UnsetColorFormatsOptional {}
    impl IsUnsetOptional for UnsetColorFormatsOptional {}
    impl<'a> ResolveOptional<&'a [Option<wgpu::TextureFormat>]> for UnsetColorFormatsOptional {
        fn resolve(self) -> &'a [Option<wgpu::TextureFormat>] {
            Default::default()
        }
    }
    pub struct ColorFormatsOptionalValue<'a>(pub &'a [Option<wgpu::TextureFormat>]);
    impl<'a> IsOptional for ColorFormatsOptionalValue<'a> {}
    impl<'a> ResolveOptional<&'a [Option<wgpu::TextureFormat>]> for ColorFormatsOptionalValue<'a> {
        fn resolve(self) -> &'a [Option<wgpu::TextureFormat>] {
            self.0
        }
    }
    pub struct UnsetDepthStencilOptional;
    impl IsOptional for UnsetDepthStencilOptional {}
    impl IsUnsetOptional for UnsetDepthStencilOptional {}
    impl ResolveOptional<Option<wgpu::RenderBundleDepthStencil>> for UnsetDepthStencilOptional {
        fn resolve(self) -> Option<wgpu::RenderBundleDepthStencil> {
            Default::default()
        }
    }
    pub struct DepthStencilOptionalValue(pub Option<wgpu::RenderBundleDepthStencil>);
    impl IsOptional for DepthStencilOptionalValue {}
    impl ResolveOptional<Option<wgpu::RenderBundleDepthStencil>> for DepthStencilOptionalValue {
        fn resolve(self) -> Option<wgpu::RenderBundleDepthStencil> {
            self.0
        }
    }
    pub struct UnsetSampleCountOptional;
    impl IsOptional for UnsetSampleCountOptional {}
    impl IsUnsetOptional for UnsetSampleCountOptional {}
    impl ResolveOptional<u32> for UnsetSampleCountOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct SampleCountOptionalValue(pub u32);
    impl IsOptional for SampleCountOptionalValue {}
    impl ResolveOptional<u32> for SampleCountOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetMultiviewOptional;
    impl IsOptional for UnsetMultiviewOptional {}
    impl IsUnsetOptional for UnsetMultiviewOptional {}
    impl ResolveOptional<Option<NonZeroU32>> for UnsetMultiviewOptional {
        fn resolve(self) -> Option<NonZeroU32> {
            Default::default()
        }
    }
    pub struct MultiviewOptionalValue(pub Option<NonZeroU32>);
    impl IsOptional for MultiviewOptionalValue {}
    impl ResolveOptional<Option<NonZeroU32>> for MultiviewOptionalValue {
        fn resolve(self) -> Option<NonZeroU32> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional, T4: IsOptional>
        RenderBundleEncoderDescriptorBuilder<T0, T1, T2, T3, T4>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> RenderBundleEncoderDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4>
        where
            T0: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: LabelOptionalValue(label),
                color_formats: self.color_formats,
                depth_stencil: self.depth_stencil,
                sample_count: self.sample_count,
                multiview: self.multiview,
            }
        }
        pub fn color_formats<'a>(
            self,
            color_formats: &'a [Option<wgpu::TextureFormat>],
        ) -> RenderBundleEncoderDescriptorBuilder<T0, ColorFormatsOptionalValue<'a>, T2, T3, T4>
        where
            T1: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: self.label,
                color_formats: ColorFormatsOptionalValue(color_formats),
                depth_stencil: self.depth_stencil,
                sample_count: self.sample_count,
                multiview: self.multiview,
            }
        }
        pub fn maybe_depth_stencil(
            self,
            depth_stencil: impl Nested<Option<wgpu::RenderBundleDepthStencil>>,
        ) -> RenderBundleEncoderDescriptorBuilder<T0, T1, DepthStencilOptionalValue, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: self.label,
                color_formats: self.color_formats,
                depth_stencil: DepthStencilOptionalValue(depth_stencil.unnest()),
                sample_count: self.sample_count,
                multiview: self.multiview,
            }
        }
        pub fn depth_stencil(
            self,
            depth_stencil: impl Nested<wgpu::RenderBundleDepthStencil>,
        ) -> RenderBundleEncoderDescriptorBuilder<T0, T1, DepthStencilOptionalValue, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: self.label,
                color_formats: self.color_formats,
                depth_stencil: DepthStencilOptionalValue(Some(depth_stencil.unnest())),
                sample_count: self.sample_count,
                multiview: self.multiview,
            }
        }
        pub fn sample_count(
            self,
            sample_count: u32,
        ) -> RenderBundleEncoderDescriptorBuilder<T0, T1, T2, SampleCountOptionalValue, T4>
        where
            T3: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: self.label,
                color_formats: self.color_formats,
                depth_stencil: self.depth_stencil,
                sample_count: SampleCountOptionalValue(sample_count),
                multiview: self.multiview,
            }
        }
        pub fn maybe_multiview(
            self,
            multiview: Option<NonZeroU32>,
        ) -> RenderBundleEncoderDescriptorBuilder<T0, T1, T2, T3, MultiviewOptionalValue>
        where
            T4: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: self.label,
                color_formats: self.color_formats,
                depth_stencil: self.depth_stencil,
                sample_count: self.sample_count,
                multiview: MultiviewOptionalValue(multiview),
            }
        }
        pub fn multiview(
            self,
            multiview: NonZeroU32,
        ) -> RenderBundleEncoderDescriptorBuilder<T0, T1, T2, T3, MultiviewOptionalValue>
        where
            T4: IsUnsetOptional,
        {
            RenderBundleEncoderDescriptorBuilder {
                label: self.label,
                color_formats: self.color_formats,
                depth_stencil: self.depth_stencil,
                sample_count: self.sample_count,
                multiview: MultiviewOptionalValue(Some(multiview)),
            }
        }
    }
    impl<RLabel, RColorFormats, RDepthStencil, RSampleCount, RMultiview>
        RenderBundleEncoderDescriptorBuilder<
            RLabel,
            RColorFormats,
            RDepthStencil,
            RSampleCount,
            RMultiview,
        >
    {
        pub fn build<'a>(self) -> wgpu::RenderBundleEncoderDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RColorFormats: ResolveOptional<&'a [Option<wgpu::TextureFormat>]>,
            RDepthStencil: ResolveOptional<Option<wgpu::RenderBundleDepthStencil>>,
            RSampleCount: ResolveOptional<u32>,
            RMultiview: ResolveOptional<Option<NonZeroU32>>,
        {
            wgpu::RenderBundleEncoderDescriptor {
                label: self.label.resolve(),
                color_formats: self.color_formats.resolve(),
                depth_stencil: self.depth_stencil.resolve(),
                sample_count: self.sample_count.resolve(),
                multiview: self.multiview.resolve(),
            }
        }
    }
    impl<'a, RLabel, RColorFormats, RDepthStencil, RSampleCount, RMultiview>
        Nested<wgpu::RenderBundleEncoderDescriptor<'a>>
        for RenderBundleEncoderDescriptorBuilder<
            RLabel,
            RColorFormats,
            RDepthStencil,
            RSampleCount,
            RMultiview,
        >
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RColorFormats: ResolveOptional<&'a [Option<wgpu::TextureFormat>]>,
        RDepthStencil: ResolveOptional<Option<wgpu::RenderBundleDepthStencil>>,
        RSampleCount: ResolveOptional<u32>,
        RMultiview: ResolveOptional<Option<NonZeroU32>>,
    {
        fn unnest(self) -> wgpu::RenderBundleEncoderDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::RenderBundleEncoderDescriptor<'a>>
        for wgpu::RenderBundleEncoderDescriptor<'a>
    {
        fn unnest(self) -> wgpu::RenderBundleEncoderDescriptor<'a> {
            self
        }
    }
}

pub use builder_render_pass_depth_stencil_attachment::render_pass_depth_stencil_attachment;
pub mod builder_render_pass_depth_stencil_attachment {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_pass_depth_stencil_attachment() -> RenderPassDepthStencilAttachmentBuilder<
        UnsetView,
        UnsetDepthOpsOptional,
        UnsetStencilOpsOptional,
    > {
        RenderPassDepthStencilAttachmentBuilder::new()
    }
    pub struct RenderPassDepthStencilAttachmentBuilder<T0, T1, T2> {
        view: T0,
        depth_ops: T1,
        stencil_ops: T2,
    }
    impl
        RenderPassDepthStencilAttachmentBuilder<
            UnsetView,
            UnsetDepthOpsOptional,
            UnsetStencilOpsOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                view: UnsetView,
                depth_ops: UnsetDepthOpsOptional,
                stencil_ops: UnsetStencilOpsOptional,
            }
        }
    }
    pub struct UnsetView;
    impl IsRequired for UnsetView {}
    impl IsUnset for UnsetView {}
    pub struct ViewValue<'tex>(pub &'tex wgpu::TextureView);
    impl<'tex> IsRequired for ViewValue<'tex> {}
    pub struct UnsetDepthOpsOptional;
    impl IsOptional for UnsetDepthOpsOptional {}
    impl IsUnsetOptional for UnsetDepthOpsOptional {}
    impl ResolveOptional<Option<wgpu::Operations<f32>>> for UnsetDepthOpsOptional {
        fn resolve(self) -> Option<wgpu::Operations<f32>> {
            Default::default()
        }
    }
    pub struct DepthOpsOptionalValue(pub Option<wgpu::Operations<f32>>);
    impl IsOptional for DepthOpsOptionalValue {}
    impl ResolveOptional<Option<wgpu::Operations<f32>>> for DepthOpsOptionalValue {
        fn resolve(self) -> Option<wgpu::Operations<f32>> {
            self.0
        }
    }
    pub struct UnsetStencilOpsOptional;
    impl IsOptional for UnsetStencilOpsOptional {}
    impl IsUnsetOptional for UnsetStencilOpsOptional {}
    impl ResolveOptional<Option<wgpu::Operations<u32>>> for UnsetStencilOpsOptional {
        fn resolve(self) -> Option<wgpu::Operations<u32>> {
            Default::default()
        }
    }
    pub struct StencilOpsOptionalValue(pub Option<wgpu::Operations<u32>>);
    impl IsOptional for StencilOpsOptionalValue {}
    impl ResolveOptional<Option<wgpu::Operations<u32>>> for StencilOpsOptionalValue {
        fn resolve(self) -> Option<wgpu::Operations<u32>> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional>
        RenderPassDepthStencilAttachmentBuilder<T0, T1, T2>
    {
        pub fn view<'tex>(
            self,
            view: &'tex wgpu::TextureView,
        ) -> RenderPassDepthStencilAttachmentBuilder<ViewValue<'tex>, T1, T2>
        where
            T0: IsUnset,
        {
            RenderPassDepthStencilAttachmentBuilder {
                view: ViewValue(view),
                depth_ops: self.depth_ops,
                stencil_ops: self.stencil_ops,
            }
        }
        pub fn maybe_depth_ops(
            self,
            depth_ops: impl Nested<Option<wgpu::Operations<f32>>>,
        ) -> RenderPassDepthStencilAttachmentBuilder<T0, DepthOpsOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            RenderPassDepthStencilAttachmentBuilder {
                view: self.view,
                depth_ops: DepthOpsOptionalValue(depth_ops.unnest()),
                stencil_ops: self.stencil_ops,
            }
        }
        pub fn depth_ops(
            self,
            depth_ops: impl Nested<wgpu::Operations<f32>>,
        ) -> RenderPassDepthStencilAttachmentBuilder<T0, DepthOpsOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            RenderPassDepthStencilAttachmentBuilder {
                view: self.view,
                depth_ops: DepthOpsOptionalValue(Some(depth_ops.unnest())),
                stencil_ops: self.stencil_ops,
            }
        }
        pub fn maybe_stencil_ops(
            self,
            stencil_ops: impl Nested<Option<wgpu::Operations<u32>>>,
        ) -> RenderPassDepthStencilAttachmentBuilder<T0, T1, StencilOpsOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            RenderPassDepthStencilAttachmentBuilder {
                view: self.view,
                depth_ops: self.depth_ops,
                stencil_ops: StencilOpsOptionalValue(stencil_ops.unnest()),
            }
        }
        pub fn stencil_ops(
            self,
            stencil_ops: impl Nested<wgpu::Operations<u32>>,
        ) -> RenderPassDepthStencilAttachmentBuilder<T0, T1, StencilOpsOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            RenderPassDepthStencilAttachmentBuilder {
                view: self.view,
                depth_ops: self.depth_ops,
                stencil_ops: StencilOpsOptionalValue(Some(stencil_ops.unnest())),
            }
        }
    }
    impl<'tex, RDepthOps, RStencilOps>
        RenderPassDepthStencilAttachmentBuilder<ViewValue<'tex>, RDepthOps, RStencilOps>
    {
        pub fn build(self) -> wgpu::RenderPassDepthStencilAttachment<'tex>
        where
            RDepthOps: ResolveOptional<Option<wgpu::Operations<f32>>>,
            RStencilOps: ResolveOptional<Option<wgpu::Operations<u32>>>,
        {
            wgpu::RenderPassDepthStencilAttachment {
                view: self.view.0,
                depth_ops: self.depth_ops.resolve(),
                stencil_ops: self.stencil_ops.resolve(),
            }
        }
    }
    impl<'tex, RDepthOps, RStencilOps> Nested<wgpu::RenderPassDepthStencilAttachment<'tex>>
        for RenderPassDepthStencilAttachmentBuilder<ViewValue<'tex>, RDepthOps, RStencilOps>
    where
        RDepthOps: ResolveOptional<Option<wgpu::Operations<f32>>>,
        RStencilOps: ResolveOptional<Option<wgpu::Operations<u32>>>,
    {
        fn unnest(self) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
            self.build()
        }
    }
    impl<'tex> Nested<wgpu::RenderPassDepthStencilAttachment<'tex>>
        for wgpu::RenderPassDepthStencilAttachment<'tex>
    {
        fn unnest(self) -> wgpu::RenderPassDepthStencilAttachment<'tex> {
            self
        }
    }
}

pub use builder_render_pass_timestamp_writes::render_pass_timestamp_writes;
pub mod builder_render_pass_timestamp_writes {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_pass_timestamp_writes() -> RenderPassTimestampWritesBuilder<
        UnsetQuerySet,
        UnsetBeginningOfPassWriteIndexOptional,
        UnsetEndOfPassWriteIndexOptional,
    > {
        RenderPassTimestampWritesBuilder::new()
    }
    pub struct RenderPassTimestampWritesBuilder<T0, T1, T2> {
        query_set: T0,
        beginning_of_pass_write_index: T1,
        end_of_pass_write_index: T2,
    }
    impl
        RenderPassTimestampWritesBuilder<
            UnsetQuerySet,
            UnsetBeginningOfPassWriteIndexOptional,
            UnsetEndOfPassWriteIndexOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                query_set: UnsetQuerySet,
                beginning_of_pass_write_index: UnsetBeginningOfPassWriteIndexOptional,
                end_of_pass_write_index: UnsetEndOfPassWriteIndexOptional,
            }
        }
    }
    pub struct UnsetQuerySet;
    impl IsRequired for UnsetQuerySet {}
    impl IsUnset for UnsetQuerySet {}
    pub struct QuerySetValue<'a>(pub &'a wgpu::QuerySet);
    impl<'a> IsRequired for QuerySetValue<'a> {}
    pub struct UnsetBeginningOfPassWriteIndexOptional;
    impl IsOptional for UnsetBeginningOfPassWriteIndexOptional {}
    impl IsUnsetOptional for UnsetBeginningOfPassWriteIndexOptional {}
    impl ResolveOptional<Option<u32>> for UnsetBeginningOfPassWriteIndexOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct BeginningOfPassWriteIndexOptionalValue(pub Option<u32>);
    impl IsOptional for BeginningOfPassWriteIndexOptionalValue {}
    impl ResolveOptional<Option<u32>> for BeginningOfPassWriteIndexOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetEndOfPassWriteIndexOptional;
    impl IsOptional for UnsetEndOfPassWriteIndexOptional {}
    impl IsUnsetOptional for UnsetEndOfPassWriteIndexOptional {}
    impl ResolveOptional<Option<u32>> for UnsetEndOfPassWriteIndexOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct EndOfPassWriteIndexOptionalValue(pub Option<u32>);
    impl IsOptional for EndOfPassWriteIndexOptionalValue {}
    impl ResolveOptional<Option<u32>> for EndOfPassWriteIndexOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional> RenderPassTimestampWritesBuilder<T0, T1, T2> {
        pub fn query_set<'a>(
            self,
            query_set: &'a wgpu::QuerySet,
        ) -> RenderPassTimestampWritesBuilder<QuerySetValue<'a>, T1, T2>
        where
            T0: IsUnset,
        {
            RenderPassTimestampWritesBuilder {
                query_set: QuerySetValue(query_set),
                beginning_of_pass_write_index: self.beginning_of_pass_write_index,
                end_of_pass_write_index: self.end_of_pass_write_index,
            }
        }
        pub fn maybe_beginning_of_pass_write_index(
            self,
            beginning_of_pass_write_index: Option<u32>,
        ) -> RenderPassTimestampWritesBuilder<T0, BeginningOfPassWriteIndexOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            RenderPassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: BeginningOfPassWriteIndexOptionalValue(
                    beginning_of_pass_write_index,
                ),
                end_of_pass_write_index: self.end_of_pass_write_index,
            }
        }
        pub fn beginning_of_pass_write_index(
            self,
            beginning_of_pass_write_index: u32,
        ) -> RenderPassTimestampWritesBuilder<T0, BeginningOfPassWriteIndexOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            RenderPassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: BeginningOfPassWriteIndexOptionalValue(Some(
                    beginning_of_pass_write_index,
                )),
                end_of_pass_write_index: self.end_of_pass_write_index,
            }
        }
        pub fn maybe_end_of_pass_write_index(
            self,
            end_of_pass_write_index: Option<u32>,
        ) -> RenderPassTimestampWritesBuilder<T0, T1, EndOfPassWriteIndexOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            RenderPassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: self.beginning_of_pass_write_index,
                end_of_pass_write_index: EndOfPassWriteIndexOptionalValue(end_of_pass_write_index),
            }
        }
        pub fn end_of_pass_write_index(
            self,
            end_of_pass_write_index: u32,
        ) -> RenderPassTimestampWritesBuilder<T0, T1, EndOfPassWriteIndexOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            RenderPassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: self.beginning_of_pass_write_index,
                end_of_pass_write_index: EndOfPassWriteIndexOptionalValue(Some(
                    end_of_pass_write_index,
                )),
            }
        }
    }
    impl<'a, RBeginningOfPassWriteIndex, REndOfPassWriteIndex>
        RenderPassTimestampWritesBuilder<
            QuerySetValue<'a>,
            RBeginningOfPassWriteIndex,
            REndOfPassWriteIndex,
        >
    {
        pub fn build(self) -> wgpu::RenderPassTimestampWrites<'a>
        where
            RBeginningOfPassWriteIndex: ResolveOptional<Option<u32>>,
            REndOfPassWriteIndex: ResolveOptional<Option<u32>>,
        {
            wgpu::RenderPassTimestampWrites {
                query_set: self.query_set.0,
                beginning_of_pass_write_index: self.beginning_of_pass_write_index.resolve(),
                end_of_pass_write_index: self.end_of_pass_write_index.resolve(),
            }
        }
    }
    impl<'a, RBeginningOfPassWriteIndex, REndOfPassWriteIndex>
        Nested<wgpu::RenderPassTimestampWrites<'a>>
        for RenderPassTimestampWritesBuilder<
            QuerySetValue<'a>,
            RBeginningOfPassWriteIndex,
            REndOfPassWriteIndex,
        >
    where
        RBeginningOfPassWriteIndex: ResolveOptional<Option<u32>>,
        REndOfPassWriteIndex: ResolveOptional<Option<u32>>,
    {
        fn unnest(self) -> wgpu::RenderPassTimestampWrites<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::RenderPassTimestampWrites<'a>> for wgpu::RenderPassTimestampWrites<'a> {
        fn unnest(self) -> wgpu::RenderPassTimestampWrites<'a> {
            self
        }
    }
}

pub use builder_bind_group_entry::bind_group_entry;
pub mod builder_bind_group_entry {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn bind_group_entry() -> BindGroupEntryBuilder<UnsetBinding, UnsetResource> {
        BindGroupEntryBuilder::new()
    }
    pub struct BindGroupEntryBuilder<T0, T1> {
        binding: T0,
        resource: T1,
    }
    impl BindGroupEntryBuilder<UnsetBinding, UnsetResource> {
        pub fn new() -> Self {
            Self {
                binding: UnsetBinding,
                resource: UnsetResource,
            }
        }
    }
    pub struct UnsetBinding;
    impl IsRequired for UnsetBinding {}
    impl IsUnset for UnsetBinding {}
    pub struct BindingValue(pub u32);
    impl IsRequired for BindingValue {}
    pub struct UnsetResource;
    impl IsRequired for UnsetResource {}
    impl IsUnset for UnsetResource {}
    pub struct ResourceValue<'a>(pub wgpu::BindingResource<'a>);
    impl<'a> IsRequired for ResourceValue<'a> {}
    impl<T0: IsRequired, T1: IsRequired> BindGroupEntryBuilder<T0, T1> {
        pub fn binding(self, binding: u32) -> BindGroupEntryBuilder<BindingValue, T1>
        where
            T0: IsUnset,
        {
            BindGroupEntryBuilder {
                binding: BindingValue(binding),
                resource: self.resource,
            }
        }
        pub fn resource<'a>(
            self,
            resource: wgpu::BindingResource<'a>,
        ) -> BindGroupEntryBuilder<T0, ResourceValue<'a>>
        where
            T1: IsUnset,
        {
            BindGroupEntryBuilder {
                binding: self.binding,
                resource: ResourceValue(resource),
            }
        }
    }
    impl<'a> BindGroupEntryBuilder<BindingValue, ResourceValue<'a>> {
        pub fn build(self) -> wgpu::BindGroupEntry<'a> where {
            wgpu::BindGroupEntry {
                binding: self.binding.0,
                resource: self.resource.0,
            }
        }
    }
    impl<'a> Nested<wgpu::BindGroupEntry<'a>>
        for BindGroupEntryBuilder<BindingValue, ResourceValue<'a>>
    {
        fn unnest(self) -> wgpu::BindGroupEntry<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::BindGroupEntry<'a>> for wgpu::BindGroupEntry<'a> {
        fn unnest(self) -> wgpu::BindGroupEntry<'a> {
            self
        }
    }
}

pub use builder_render_pass_color_attachment::render_pass_color_attachment;
pub mod builder_render_pass_color_attachment {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_pass_color_attachment() -> RenderPassColorAttachmentBuilder<
        UnsetView,
        UnsetDepthSliceOptional,
        UnsetResolveTargetOptional,
        UnsetOpsOptional,
    > {
        RenderPassColorAttachmentBuilder::new()
    }
    pub struct RenderPassColorAttachmentBuilder<T0, T1, T2, T3> {
        view: T0,
        depth_slice: T1,
        resolve_target: T2,
        ops: T3,
    }
    impl
        RenderPassColorAttachmentBuilder<
            UnsetView,
            UnsetDepthSliceOptional,
            UnsetResolveTargetOptional,
            UnsetOpsOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                view: UnsetView,
                depth_slice: UnsetDepthSliceOptional,
                resolve_target: UnsetResolveTargetOptional,
                ops: UnsetOpsOptional,
            }
        }
    }
    pub struct UnsetView;
    impl IsRequired for UnsetView {}
    impl IsUnset for UnsetView {}
    pub struct ViewValue<'tex>(pub &'tex wgpu::TextureView);
    impl<'tex> IsRequired for ViewValue<'tex> {}
    pub struct UnsetDepthSliceOptional;
    impl IsOptional for UnsetDepthSliceOptional {}
    impl IsUnsetOptional for UnsetDepthSliceOptional {}
    impl ResolveOptional<Option<u32>> for UnsetDepthSliceOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct DepthSliceOptionalValue(pub Option<u32>);
    impl IsOptional for DepthSliceOptionalValue {}
    impl ResolveOptional<Option<u32>> for DepthSliceOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetResolveTargetOptional;
    impl IsOptional for UnsetResolveTargetOptional {}
    impl IsUnsetOptional for UnsetResolveTargetOptional {}
    impl<'tex> ResolveOptional<Option<&'tex wgpu::TextureView>> for UnsetResolveTargetOptional {
        fn resolve(self) -> Option<&'tex wgpu::TextureView> {
            Default::default()
        }
    }
    pub struct ResolveTargetOptionalValue<'tex>(pub Option<&'tex wgpu::TextureView>);
    impl<'tex> IsOptional for ResolveTargetOptionalValue<'tex> {}
    impl<'tex> ResolveOptional<Option<&'tex wgpu::TextureView>> for ResolveTargetOptionalValue<'tex> {
        fn resolve(self) -> Option<&'tex wgpu::TextureView> {
            self.0
        }
    }
    pub struct UnsetOpsOptional;
    impl IsOptional for UnsetOpsOptional {}
    impl IsUnsetOptional for UnsetOpsOptional {}
    impl ResolveOptional<wgpu::Operations<wgpu::Color>> for UnsetOpsOptional {
        fn resolve(self) -> wgpu::Operations<wgpu::Color> {
            Default::default()
        }
    }
    pub struct OpsOptionalValue(pub wgpu::Operations<wgpu::Color>);
    impl IsOptional for OpsOptionalValue {}
    impl ResolveOptional<wgpu::Operations<wgpu::Color>> for OpsOptionalValue {
        fn resolve(self) -> wgpu::Operations<wgpu::Color> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional, T3: IsOptional>
        RenderPassColorAttachmentBuilder<T0, T1, T2, T3>
    {
        pub fn view<'tex>(
            self,
            view: &'tex wgpu::TextureView,
        ) -> RenderPassColorAttachmentBuilder<ViewValue<'tex>, T1, T2, T3>
        where
            T0: IsUnset,
        {
            RenderPassColorAttachmentBuilder {
                view: ViewValue(view),
                depth_slice: self.depth_slice,
                resolve_target: self.resolve_target,
                ops: self.ops,
            }
        }
        pub fn maybe_depth_slice(
            self,
            depth_slice: Option<u32>,
        ) -> RenderPassColorAttachmentBuilder<T0, DepthSliceOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            RenderPassColorAttachmentBuilder {
                view: self.view,
                depth_slice: DepthSliceOptionalValue(depth_slice),
                resolve_target: self.resolve_target,
                ops: self.ops,
            }
        }
        pub fn depth_slice(
            self,
            depth_slice: u32,
        ) -> RenderPassColorAttachmentBuilder<T0, DepthSliceOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            RenderPassColorAttachmentBuilder {
                view: self.view,
                depth_slice: DepthSliceOptionalValue(Some(depth_slice)),
                resolve_target: self.resolve_target,
                ops: self.ops,
            }
        }
        pub fn maybe_resolve_target<'tex>(
            self,
            resolve_target: Option<&'tex wgpu::TextureView>,
        ) -> RenderPassColorAttachmentBuilder<T0, T1, ResolveTargetOptionalValue<'tex>, T3>
        where
            T2: IsUnsetOptional,
        {
            RenderPassColorAttachmentBuilder {
                view: self.view,
                depth_slice: self.depth_slice,
                resolve_target: ResolveTargetOptionalValue(resolve_target),
                ops: self.ops,
            }
        }
        pub fn resolve_target<'tex>(
            self,
            resolve_target: &'tex wgpu::TextureView,
        ) -> RenderPassColorAttachmentBuilder<T0, T1, ResolveTargetOptionalValue<'tex>, T3>
        where
            T2: IsUnsetOptional,
        {
            RenderPassColorAttachmentBuilder {
                view: self.view,
                depth_slice: self.depth_slice,
                resolve_target: ResolveTargetOptionalValue(Some(resolve_target)),
                ops: self.ops,
            }
        }
        pub fn ops(
            self,
            ops: impl Nested<wgpu::Operations<wgpu::Color>>,
        ) -> RenderPassColorAttachmentBuilder<T0, T1, T2, OpsOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            RenderPassColorAttachmentBuilder {
                view: self.view,
                depth_slice: self.depth_slice,
                resolve_target: self.resolve_target,
                ops: OpsOptionalValue(ops.unnest()),
            }
        }
    }
    impl<'tex, RDepthSlice, RResolveTarget, ROps>
        RenderPassColorAttachmentBuilder<ViewValue<'tex>, RDepthSlice, RResolveTarget, ROps>
    {
        pub fn build(self) -> wgpu::RenderPassColorAttachment<'tex>
        where
            RDepthSlice: ResolveOptional<Option<u32>>,
            RResolveTarget: ResolveOptional<Option<&'tex wgpu::TextureView>>,
            ROps: ResolveOptional<wgpu::Operations<wgpu::Color>>,
        {
            wgpu::RenderPassColorAttachment {
                view: self.view.0,
                depth_slice: self.depth_slice.resolve(),
                resolve_target: self.resolve_target.resolve(),
                ops: self.ops.resolve(),
            }
        }
    }
    impl<'tex, RDepthSlice, RResolveTarget, ROps> Nested<wgpu::RenderPassColorAttachment<'tex>>
        for RenderPassColorAttachmentBuilder<ViewValue<'tex>, RDepthSlice, RResolveTarget, ROps>
    where
        RDepthSlice: ResolveOptional<Option<u32>>,
        RResolveTarget: ResolveOptional<Option<&'tex wgpu::TextureView>>,
        ROps: ResolveOptional<wgpu::Operations<wgpu::Color>>,
    {
        fn unnest(self) -> wgpu::RenderPassColorAttachment<'tex> {
            self.build()
        }
    }
    impl<'tex> Nested<wgpu::RenderPassColorAttachment<'tex>> for wgpu::RenderPassColorAttachment<'tex> {
        fn unnest(self) -> wgpu::RenderPassColorAttachment<'tex> {
            self
        }
    }
}

pub use builder_vertex_buffer_layout::vertex_buffer_layout;
pub mod builder_vertex_buffer_layout {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn vertex_buffer_layout()
    -> VertexBufferLayoutBuilder<UnsetArrayStride, UnsetStepModeOptional, UnsetAttributes> {
        VertexBufferLayoutBuilder::new()
    }
    pub struct VertexBufferLayoutBuilder<T0, T1, T2> {
        array_stride: T0,
        step_mode: T1,
        attributes: T2,
    }
    impl VertexBufferLayoutBuilder<UnsetArrayStride, UnsetStepModeOptional, UnsetAttributes> {
        pub fn new() -> Self {
            Self {
                array_stride: UnsetArrayStride,
                step_mode: UnsetStepModeOptional,
                attributes: UnsetAttributes,
            }
        }
    }
    pub struct UnsetArrayStride;
    impl IsRequired for UnsetArrayStride {}
    impl IsUnset for UnsetArrayStride {}
    pub struct ArrayStrideValue(pub wgpu::BufferAddress);
    impl IsRequired for ArrayStrideValue {}
    pub struct UnsetStepModeOptional;
    impl IsOptional for UnsetStepModeOptional {}
    impl IsUnsetOptional for UnsetStepModeOptional {}
    impl ResolveOptional<wgpu::VertexStepMode> for UnsetStepModeOptional {
        fn resolve(self) -> wgpu::VertexStepMode {
            Default::default()
        }
    }
    pub struct StepModeOptionalValue(pub wgpu::VertexStepMode);
    impl IsOptional for StepModeOptionalValue {}
    impl ResolveOptional<wgpu::VertexStepMode> for StepModeOptionalValue {
        fn resolve(self) -> wgpu::VertexStepMode {
            self.0
        }
    }
    pub struct UnsetAttributes;
    impl IsRequired for UnsetAttributes {}
    impl IsUnset for UnsetAttributes {}
    pub struct AttributesValue<'a>(pub &'a [wgpu::VertexAttribute]);
    impl<'a> IsRequired for AttributesValue<'a> {}
    impl<T0: IsRequired, T1: IsOptional, T2: IsRequired> VertexBufferLayoutBuilder<T0, T1, T2> {
        pub fn array_stride(
            self,
            array_stride: wgpu::BufferAddress,
        ) -> VertexBufferLayoutBuilder<ArrayStrideValue, T1, T2>
        where
            T0: IsUnset,
        {
            VertexBufferLayoutBuilder {
                array_stride: ArrayStrideValue(array_stride),
                step_mode: self.step_mode,
                attributes: self.attributes,
            }
        }
        pub fn step_mode(
            self,
            step_mode: wgpu::VertexStepMode,
        ) -> VertexBufferLayoutBuilder<T0, StepModeOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            VertexBufferLayoutBuilder {
                array_stride: self.array_stride,
                step_mode: StepModeOptionalValue(step_mode),
                attributes: self.attributes,
            }
        }
        pub fn attributes<'a>(
            self,
            attributes: &'a [wgpu::VertexAttribute],
        ) -> VertexBufferLayoutBuilder<T0, T1, AttributesValue<'a>>
        where
            T2: IsUnset,
        {
            VertexBufferLayoutBuilder {
                array_stride: self.array_stride,
                step_mode: self.step_mode,
                attributes: AttributesValue(attributes),
            }
        }
    }
    impl<'a, RStepMode> VertexBufferLayoutBuilder<ArrayStrideValue, RStepMode, AttributesValue<'a>> {
        pub fn build(self) -> wgpu::VertexBufferLayout<'a>
        where
            RStepMode: ResolveOptional<wgpu::VertexStepMode>,
        {
            wgpu::VertexBufferLayout {
                array_stride: self.array_stride.0,
                step_mode: self.step_mode.resolve(),
                attributes: self.attributes.0,
            }
        }
    }
    impl<'a, RStepMode> Nested<wgpu::VertexBufferLayout<'a>>
        for VertexBufferLayoutBuilder<ArrayStrideValue, RStepMode, AttributesValue<'a>>
    where
        RStepMode: ResolveOptional<wgpu::VertexStepMode>,
    {
        fn unnest(self) -> wgpu::VertexBufferLayout<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::VertexBufferLayout<'a>> for wgpu::VertexBufferLayout<'a> {
        fn unnest(self) -> wgpu::VertexBufferLayout<'a> {
            self
        }
    }
}

pub use builder_sampler_descriptor::sampler_descriptor;
pub mod builder_sampler_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn sampler_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> SamplerDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetAddressModeUOptional,
        UnsetAddressModeVOptional,
        UnsetAddressModeWOptional,
        UnsetMagFilterOptional,
        UnsetMinFilterOptional,
        UnsetMipmapFilterOptional,
        UnsetLodMinClampOptional,
        UnsetLodMaxClampOptional,
        UnsetCompareOptional,
        UnsetAnisotropyClampOptional,
        UnsetBorderColorOptional,
    > {
        SamplerDescriptorBuilder::new().label(label)
    }
    pub struct SamplerDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
        label: T0,
        address_mode_u: T1,
        address_mode_v: T2,
        address_mode_w: T3,
        mag_filter: T4,
        min_filter: T5,
        mipmap_filter: T6,
        lod_min_clamp: T7,
        lod_max_clamp: T8,
        compare: T9,
        anisotropy_clamp: T10,
        border_color: T11,
    }
    impl
        SamplerDescriptorBuilder<
            UnsetLabelOptional,
            UnsetAddressModeUOptional,
            UnsetAddressModeVOptional,
            UnsetAddressModeWOptional,
            UnsetMagFilterOptional,
            UnsetMinFilterOptional,
            UnsetMipmapFilterOptional,
            UnsetLodMinClampOptional,
            UnsetLodMaxClampOptional,
            UnsetCompareOptional,
            UnsetAnisotropyClampOptional,
            UnsetBorderColorOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                address_mode_u: UnsetAddressModeUOptional,
                address_mode_v: UnsetAddressModeVOptional,
                address_mode_w: UnsetAddressModeWOptional,
                mag_filter: UnsetMagFilterOptional,
                min_filter: UnsetMinFilterOptional,
                mipmap_filter: UnsetMipmapFilterOptional,
                lod_min_clamp: UnsetLodMinClampOptional,
                lod_max_clamp: UnsetLodMaxClampOptional,
                compare: UnsetCompareOptional,
                anisotropy_clamp: UnsetAnisotropyClampOptional,
                border_color: UnsetBorderColorOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetAddressModeUOptional;
    impl IsOptional for UnsetAddressModeUOptional {}
    impl IsUnsetOptional for UnsetAddressModeUOptional {}
    impl ResolveOptional<wgpu::AddressMode> for UnsetAddressModeUOptional {
        fn resolve(self) -> wgpu::AddressMode {
            Default::default()
        }
    }
    pub struct AddressModeUOptionalValue(pub wgpu::AddressMode);
    impl IsOptional for AddressModeUOptionalValue {}
    impl ResolveOptional<wgpu::AddressMode> for AddressModeUOptionalValue {
        fn resolve(self) -> wgpu::AddressMode {
            self.0
        }
    }
    pub struct UnsetAddressModeVOptional;
    impl IsOptional for UnsetAddressModeVOptional {}
    impl IsUnsetOptional for UnsetAddressModeVOptional {}
    impl ResolveOptional<wgpu::AddressMode> for UnsetAddressModeVOptional {
        fn resolve(self) -> wgpu::AddressMode {
            Default::default()
        }
    }
    pub struct AddressModeVOptionalValue(pub wgpu::AddressMode);
    impl IsOptional for AddressModeVOptionalValue {}
    impl ResolveOptional<wgpu::AddressMode> for AddressModeVOptionalValue {
        fn resolve(self) -> wgpu::AddressMode {
            self.0
        }
    }
    pub struct UnsetAddressModeWOptional;
    impl IsOptional for UnsetAddressModeWOptional {}
    impl IsUnsetOptional for UnsetAddressModeWOptional {}
    impl ResolveOptional<wgpu::AddressMode> for UnsetAddressModeWOptional {
        fn resolve(self) -> wgpu::AddressMode {
            Default::default()
        }
    }
    pub struct AddressModeWOptionalValue(pub wgpu::AddressMode);
    impl IsOptional for AddressModeWOptionalValue {}
    impl ResolveOptional<wgpu::AddressMode> for AddressModeWOptionalValue {
        fn resolve(self) -> wgpu::AddressMode {
            self.0
        }
    }
    pub struct UnsetMagFilterOptional;
    impl IsOptional for UnsetMagFilterOptional {}
    impl IsUnsetOptional for UnsetMagFilterOptional {}
    impl ResolveOptional<wgpu::FilterMode> for UnsetMagFilterOptional {
        fn resolve(self) -> wgpu::FilterMode {
            Default::default()
        }
    }
    pub struct MagFilterOptionalValue(pub wgpu::FilterMode);
    impl IsOptional for MagFilterOptionalValue {}
    impl ResolveOptional<wgpu::FilterMode> for MagFilterOptionalValue {
        fn resolve(self) -> wgpu::FilterMode {
            self.0
        }
    }
    pub struct UnsetMinFilterOptional;
    impl IsOptional for UnsetMinFilterOptional {}
    impl IsUnsetOptional for UnsetMinFilterOptional {}
    impl ResolveOptional<wgpu::FilterMode> for UnsetMinFilterOptional {
        fn resolve(self) -> wgpu::FilterMode {
            Default::default()
        }
    }
    pub struct MinFilterOptionalValue(pub wgpu::FilterMode);
    impl IsOptional for MinFilterOptionalValue {}
    impl ResolveOptional<wgpu::FilterMode> for MinFilterOptionalValue {
        fn resolve(self) -> wgpu::FilterMode {
            self.0
        }
    }
    pub struct UnsetMipmapFilterOptional;
    impl IsOptional for UnsetMipmapFilterOptional {}
    impl IsUnsetOptional for UnsetMipmapFilterOptional {}
    impl ResolveOptional<wgpu::FilterMode> for UnsetMipmapFilterOptional {
        fn resolve(self) -> wgpu::FilterMode {
            Default::default()
        }
    }
    pub struct MipmapFilterOptionalValue(pub wgpu::FilterMode);
    impl IsOptional for MipmapFilterOptionalValue {}
    impl ResolveOptional<wgpu::FilterMode> for MipmapFilterOptionalValue {
        fn resolve(self) -> wgpu::FilterMode {
            self.0
        }
    }
    pub struct UnsetLodMinClampOptional;
    impl IsOptional for UnsetLodMinClampOptional {}
    impl IsUnsetOptional for UnsetLodMinClampOptional {}
    impl ResolveOptional<f32> for UnsetLodMinClampOptional {
        fn resolve(self) -> f32 {
            0.0
        }
    }
    pub struct LodMinClampOptionalValue(pub f32);
    impl IsOptional for LodMinClampOptionalValue {}
    impl ResolveOptional<f32> for LodMinClampOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    pub struct UnsetLodMaxClampOptional;
    impl IsOptional for UnsetLodMaxClampOptional {}
    impl IsUnsetOptional for UnsetLodMaxClampOptional {}
    impl ResolveOptional<f32> for UnsetLodMaxClampOptional {
        fn resolve(self) -> f32 {
            32.0
        }
    }
    pub struct LodMaxClampOptionalValue(pub f32);
    impl IsOptional for LodMaxClampOptionalValue {}
    impl ResolveOptional<f32> for LodMaxClampOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    pub struct UnsetCompareOptional;
    impl IsOptional for UnsetCompareOptional {}
    impl IsUnsetOptional for UnsetCompareOptional {}
    impl ResolveOptional<Option<wgpu::CompareFunction>> for UnsetCompareOptional {
        fn resolve(self) -> Option<wgpu::CompareFunction> {
            Default::default()
        }
    }
    pub struct CompareOptionalValue(pub Option<wgpu::CompareFunction>);
    impl IsOptional for CompareOptionalValue {}
    impl ResolveOptional<Option<wgpu::CompareFunction>> for CompareOptionalValue {
        fn resolve(self) -> Option<wgpu::CompareFunction> {
            self.0
        }
    }
    pub struct UnsetAnisotropyClampOptional;
    impl IsOptional for UnsetAnisotropyClampOptional {}
    impl IsUnsetOptional for UnsetAnisotropyClampOptional {}
    impl ResolveOptional<u16> for UnsetAnisotropyClampOptional {
        fn resolve(self) -> u16 {
            1
        }
    }
    pub struct AnisotropyClampOptionalValue(pub u16);
    impl IsOptional for AnisotropyClampOptionalValue {}
    impl ResolveOptional<u16> for AnisotropyClampOptionalValue {
        fn resolve(self) -> u16 {
            self.0
        }
    }
    pub struct UnsetBorderColorOptional;
    impl IsOptional for UnsetBorderColorOptional {}
    impl IsUnsetOptional for UnsetBorderColorOptional {}
    impl ResolveOptional<Option<wgpu::SamplerBorderColor>> for UnsetBorderColorOptional {
        fn resolve(self) -> Option<wgpu::SamplerBorderColor> {
            Default::default()
        }
    }
    pub struct BorderColorOptionalValue(pub Option<wgpu::SamplerBorderColor>);
    impl IsOptional for BorderColorOptionalValue {}
    impl ResolveOptional<Option<wgpu::SamplerBorderColor>> for BorderColorOptionalValue {
        fn resolve(self) -> Option<wgpu::SamplerBorderColor> {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsOptional,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
        T7: IsOptional,
        T8: IsOptional,
        T9: IsOptional,
        T10: IsOptional,
        T11: IsOptional,
    > SamplerDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> SamplerDescriptorBuilder<
            LabelOptionalValue<'a>,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T0: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: LabelOptionalValue(label),
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn address_mode_u(
            self,
            address_mode_u: wgpu::AddressMode,
        ) -> SamplerDescriptorBuilder<
            T0,
            AddressModeUOptionalValue,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T1: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: AddressModeUOptionalValue(address_mode_u),
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn address_mode_v(
            self,
            address_mode_v: wgpu::AddressMode,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            AddressModeVOptionalValue,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T2: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: AddressModeVOptionalValue(address_mode_v),
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn address_mode_w(
            self,
            address_mode_w: wgpu::AddressMode,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            AddressModeWOptionalValue,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T3: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: AddressModeWOptionalValue(address_mode_w),
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn mag_filter(
            self,
            mag_filter: wgpu::FilterMode,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            MagFilterOptionalValue,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T4: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: MagFilterOptionalValue(mag_filter),
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn min_filter(
            self,
            min_filter: wgpu::FilterMode,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            MinFilterOptionalValue,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T5: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: MinFilterOptionalValue(min_filter),
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn mipmap_filter(
            self,
            mipmap_filter: wgpu::FilterMode,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            MipmapFilterOptionalValue,
            T7,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T6: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: MipmapFilterOptionalValue(mipmap_filter),
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn lod_min_clamp(
            self,
            lod_min_clamp: f32,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            LodMinClampOptionalValue,
            T8,
            T9,
            T10,
            T11,
        >
        where
            T7: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: LodMinClampOptionalValue(lod_min_clamp),
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn lod_max_clamp(
            self,
            lod_max_clamp: f32,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            LodMaxClampOptionalValue,
            T9,
            T10,
            T11,
        >
        where
            T8: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: LodMaxClampOptionalValue(lod_max_clamp),
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn maybe_compare(
            self,
            compare: Option<wgpu::CompareFunction>,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            CompareOptionalValue,
            T10,
            T11,
        >
        where
            T9: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: CompareOptionalValue(compare),
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn compare(
            self,
            compare: wgpu::CompareFunction,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            CompareOptionalValue,
            T10,
            T11,
        >
        where
            T9: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: CompareOptionalValue(Some(compare)),
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: self.border_color,
            }
        }
        pub fn anisotropy_clamp(
            self,
            anisotropy_clamp: u16,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            AnisotropyClampOptionalValue,
            T11,
        >
        where
            T10: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: AnisotropyClampOptionalValue(anisotropy_clamp),
                border_color: self.border_color,
            }
        }
        pub fn maybe_border_color(
            self,
            border_color: Option<wgpu::SamplerBorderColor>,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            BorderColorOptionalValue,
        >
        where
            T11: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: BorderColorOptionalValue(border_color),
            }
        }
        pub fn border_color(
            self,
            border_color: wgpu::SamplerBorderColor,
        ) -> SamplerDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            BorderColorOptionalValue,
        >
        where
            T11: IsUnsetOptional,
        {
            SamplerDescriptorBuilder {
                label: self.label,
                address_mode_u: self.address_mode_u,
                address_mode_v: self.address_mode_v,
                address_mode_w: self.address_mode_w,
                mag_filter: self.mag_filter,
                min_filter: self.min_filter,
                mipmap_filter: self.mipmap_filter,
                lod_min_clamp: self.lod_min_clamp,
                lod_max_clamp: self.lod_max_clamp,
                compare: self.compare,
                anisotropy_clamp: self.anisotropy_clamp,
                border_color: BorderColorOptionalValue(Some(border_color)),
            }
        }
    }
    impl<
        RLabel,
        RAddressModeU,
        RAddressModeV,
        RAddressModeW,
        RMagFilter,
        RMinFilter,
        RMipmapFilter,
        RLodMinClamp,
        RLodMaxClamp,
        RCompare,
        RAnisotropyClamp,
        RBorderColor,
    >
        SamplerDescriptorBuilder<
            RLabel,
            RAddressModeU,
            RAddressModeV,
            RAddressModeW,
            RMagFilter,
            RMinFilter,
            RMipmapFilter,
            RLodMinClamp,
            RLodMaxClamp,
            RCompare,
            RAnisotropyClamp,
            RBorderColor,
        >
    {
        pub fn build<'a>(self) -> wgpu::SamplerDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RAddressModeU: ResolveOptional<wgpu::AddressMode>,
            RAddressModeV: ResolveOptional<wgpu::AddressMode>,
            RAddressModeW: ResolveOptional<wgpu::AddressMode>,
            RMagFilter: ResolveOptional<wgpu::FilterMode>,
            RMinFilter: ResolveOptional<wgpu::FilterMode>,
            RMipmapFilter: ResolveOptional<wgpu::FilterMode>,
            RLodMinClamp: ResolveOptional<f32>,
            RLodMaxClamp: ResolveOptional<f32>,
            RCompare: ResolveOptional<Option<wgpu::CompareFunction>>,
            RAnisotropyClamp: ResolveOptional<u16>,
            RBorderColor: ResolveOptional<Option<wgpu::SamplerBorderColor>>,
        {
            wgpu::SamplerDescriptor {
                label: self.label.resolve(),
                address_mode_u: self.address_mode_u.resolve(),
                address_mode_v: self.address_mode_v.resolve(),
                address_mode_w: self.address_mode_w.resolve(),
                mag_filter: self.mag_filter.resolve(),
                min_filter: self.min_filter.resolve(),
                mipmap_filter: self.mipmap_filter.resolve(),
                lod_min_clamp: self.lod_min_clamp.resolve(),
                lod_max_clamp: self.lod_max_clamp.resolve(),
                compare: self.compare.resolve(),
                anisotropy_clamp: self.anisotropy_clamp.resolve(),
                border_color: self.border_color.resolve(),
            }
        }
    }
}

pub use builder_instance_descriptor::instance_descriptor;
pub mod builder_instance_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn instance_descriptor() -> InstanceDescriptorBuilder<
        UnsetBackendsOptional,
        UnsetFlagsOptional,
        UnsetMemoryBudgetThresholdsOptional,
        UnsetBackendOptionsOptional,
    > {
        InstanceDescriptorBuilder::new()
    }
    pub struct InstanceDescriptorBuilder<T0, T1, T2, T3> {
        backends: T0,
        flags: T1,
        memory_budget_thresholds: T2,
        backend_options: T3,
    }
    impl
        InstanceDescriptorBuilder<
            UnsetBackendsOptional,
            UnsetFlagsOptional,
            UnsetMemoryBudgetThresholdsOptional,
            UnsetBackendOptionsOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                backends: UnsetBackendsOptional,
                flags: UnsetFlagsOptional,
                memory_budget_thresholds: UnsetMemoryBudgetThresholdsOptional,
                backend_options: UnsetBackendOptionsOptional,
            }
        }
    }
    pub struct UnsetBackendsOptional;
    impl IsOptional for UnsetBackendsOptional {}
    impl IsUnsetOptional for UnsetBackendsOptional {}
    impl ResolveOptional<wgpu::Backends> for UnsetBackendsOptional {
        fn resolve(self) -> wgpu::Backends {
            Default::default()
        }
    }
    pub struct BackendsOptionalValue(pub wgpu::Backends);
    impl IsOptional for BackendsOptionalValue {}
    impl ResolveOptional<wgpu::Backends> for BackendsOptionalValue {
        fn resolve(self) -> wgpu::Backends {
            self.0
        }
    }
    pub struct UnsetFlagsOptional;
    impl IsOptional for UnsetFlagsOptional {}
    impl IsUnsetOptional for UnsetFlagsOptional {}
    impl ResolveOptional<wgpu::InstanceFlags> for UnsetFlagsOptional {
        fn resolve(self) -> wgpu::InstanceFlags {
            Default::default()
        }
    }
    pub struct FlagsOptionalValue(pub wgpu::InstanceFlags);
    impl IsOptional for FlagsOptionalValue {}
    impl ResolveOptional<wgpu::InstanceFlags> for FlagsOptionalValue {
        fn resolve(self) -> wgpu::InstanceFlags {
            self.0
        }
    }
    pub struct UnsetMemoryBudgetThresholdsOptional;
    impl IsOptional for UnsetMemoryBudgetThresholdsOptional {}
    impl IsUnsetOptional for UnsetMemoryBudgetThresholdsOptional {}
    impl ResolveOptional<wgpu::MemoryBudgetThresholds> for UnsetMemoryBudgetThresholdsOptional {
        fn resolve(self) -> wgpu::MemoryBudgetThresholds {
            Default::default()
        }
    }
    pub struct MemoryBudgetThresholdsOptionalValue(pub wgpu::MemoryBudgetThresholds);
    impl IsOptional for MemoryBudgetThresholdsOptionalValue {}
    impl ResolveOptional<wgpu::MemoryBudgetThresholds> for MemoryBudgetThresholdsOptionalValue {
        fn resolve(self) -> wgpu::MemoryBudgetThresholds {
            self.0
        }
    }
    pub struct UnsetBackendOptionsOptional;
    impl IsOptional for UnsetBackendOptionsOptional {}
    impl IsUnsetOptional for UnsetBackendOptionsOptional {}
    impl ResolveOptional<wgpu::BackendOptions> for UnsetBackendOptionsOptional {
        fn resolve(self) -> wgpu::BackendOptions {
            Default::default()
        }
    }
    pub struct BackendOptionsOptionalValue(pub wgpu::BackendOptions);
    impl IsOptional for BackendOptionsOptionalValue {}
    impl ResolveOptional<wgpu::BackendOptions> for BackendOptionsOptionalValue {
        fn resolve(self) -> wgpu::BackendOptions {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional>
        InstanceDescriptorBuilder<T0, T1, T2, T3>
    {
        pub fn backends(
            self,
            backends: wgpu::Backends,
        ) -> InstanceDescriptorBuilder<BackendsOptionalValue, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            InstanceDescriptorBuilder {
                backends: BackendsOptionalValue(backends),
                flags: self.flags,
                memory_budget_thresholds: self.memory_budget_thresholds,
                backend_options: self.backend_options,
            }
        }
        pub fn flags(
            self,
            flags: wgpu::InstanceFlags,
        ) -> InstanceDescriptorBuilder<T0, FlagsOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            InstanceDescriptorBuilder {
                backends: self.backends,
                flags: FlagsOptionalValue(flags),
                memory_budget_thresholds: self.memory_budget_thresholds,
                backend_options: self.backend_options,
            }
        }
        pub fn memory_budget_thresholds(
            self,
            memory_budget_thresholds: impl Nested<wgpu::MemoryBudgetThresholds>,
        ) -> InstanceDescriptorBuilder<T0, T1, MemoryBudgetThresholdsOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            InstanceDescriptorBuilder {
                backends: self.backends,
                flags: self.flags,
                memory_budget_thresholds: MemoryBudgetThresholdsOptionalValue(
                    memory_budget_thresholds.unnest(),
                ),
                backend_options: self.backend_options,
            }
        }
        pub fn backend_options(
            self,
            backend_options: impl Nested<wgpu::BackendOptions>,
        ) -> InstanceDescriptorBuilder<T0, T1, T2, BackendOptionsOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            InstanceDescriptorBuilder {
                backends: self.backends,
                flags: self.flags,
                memory_budget_thresholds: self.memory_budget_thresholds,
                backend_options: BackendOptionsOptionalValue(backend_options.unnest()),
            }
        }
    }
    impl<RBackends, RFlags, RMemoryBudgetThresholds, RBackendOptions>
        InstanceDescriptorBuilder<RBackends, RFlags, RMemoryBudgetThresholds, RBackendOptions>
    {
        pub fn build(self) -> wgpu::InstanceDescriptor
        where
            RBackends: ResolveOptional<wgpu::Backends>,
            RFlags: ResolveOptional<wgpu::InstanceFlags>,
            RMemoryBudgetThresholds: ResolveOptional<wgpu::MemoryBudgetThresholds>,
            RBackendOptions: ResolveOptional<wgpu::BackendOptions>,
        {
            wgpu::InstanceDescriptor {
                backends: self.backends.resolve(),
                flags: self.flags.resolve(),
                memory_budget_thresholds: self.memory_budget_thresholds.resolve(),
                backend_options: self.backend_options.resolve(),
            }
        }
    }
    impl<RBackends, RFlags, RMemoryBudgetThresholds, RBackendOptions>
        Nested<wgpu::InstanceDescriptor>
        for InstanceDescriptorBuilder<RBackends, RFlags, RMemoryBudgetThresholds, RBackendOptions>
    where
        RBackends: ResolveOptional<wgpu::Backends>,
        RFlags: ResolveOptional<wgpu::InstanceFlags>,
        RMemoryBudgetThresholds: ResolveOptional<wgpu::MemoryBudgetThresholds>,
        RBackendOptions: ResolveOptional<wgpu::BackendOptions>,
    {
        fn unnest(self) -> wgpu::InstanceDescriptor {
            self.build()
        }
    }
    impl Nested<wgpu::InstanceDescriptor> for wgpu::InstanceDescriptor {
        fn unnest(self) -> wgpu::InstanceDescriptor {
            self
        }
    }
}

pub use builder_depth_stencil_state::depth_stencil_state;
pub mod builder_depth_stencil_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn depth_stencil_state() -> DepthStencilStateBuilder<
        UnsetFormat,
        UnsetDepthWriteEnabled,
        UnsetDepthCompare,
        UnsetStencilOptional,
        UnsetBiasOptional,
    > {
        DepthStencilStateBuilder::new()
    }
    pub struct DepthStencilStateBuilder<T0, T1, T2, T3, T4> {
        format: T0,
        depth_write_enabled: T1,
        depth_compare: T2,
        stencil: T3,
        bias: T4,
    }
    impl
        DepthStencilStateBuilder<
            UnsetFormat,
            UnsetDepthWriteEnabled,
            UnsetDepthCompare,
            UnsetStencilOptional,
            UnsetBiasOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                format: UnsetFormat,
                depth_write_enabled: UnsetDepthWriteEnabled,
                depth_compare: UnsetDepthCompare,
                stencil: UnsetStencilOptional,
                bias: UnsetBiasOptional,
            }
        }
    }
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::TextureFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetDepthWriteEnabled;
    impl IsRequired for UnsetDepthWriteEnabled {}
    impl IsUnset for UnsetDepthWriteEnabled {}
    pub struct DepthWriteEnabledValue(pub bool);
    impl IsRequired for DepthWriteEnabledValue {}
    pub struct UnsetDepthCompare;
    impl IsRequired for UnsetDepthCompare {}
    impl IsUnset for UnsetDepthCompare {}
    pub struct DepthCompareValue(pub wgpu::CompareFunction);
    impl IsRequired for DepthCompareValue {}
    pub struct UnsetStencilOptional;
    impl IsOptional for UnsetStencilOptional {}
    impl IsUnsetOptional for UnsetStencilOptional {}
    impl ResolveOptional<wgpu::StencilState> for UnsetStencilOptional {
        fn resolve(self) -> wgpu::StencilState {
            Default::default()
        }
    }
    pub struct StencilOptionalValue(pub wgpu::StencilState);
    impl IsOptional for StencilOptionalValue {}
    impl ResolveOptional<wgpu::StencilState> for StencilOptionalValue {
        fn resolve(self) -> wgpu::StencilState {
            self.0
        }
    }
    pub struct UnsetBiasOptional;
    impl IsOptional for UnsetBiasOptional {}
    impl IsUnsetOptional for UnsetBiasOptional {}
    impl ResolveOptional<wgpu::DepthBiasState> for UnsetBiasOptional {
        fn resolve(self) -> wgpu::DepthBiasState {
            Default::default()
        }
    }
    pub struct BiasOptionalValue(pub wgpu::DepthBiasState);
    impl IsOptional for BiasOptionalValue {}
    impl ResolveOptional<wgpu::DepthBiasState> for BiasOptionalValue {
        fn resolve(self) -> wgpu::DepthBiasState {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsRequired, T2: IsRequired, T3: IsOptional, T4: IsOptional>
        DepthStencilStateBuilder<T0, T1, T2, T3, T4>
    {
        pub fn format(
            self,
            format: wgpu::TextureFormat,
        ) -> DepthStencilStateBuilder<FormatValue, T1, T2, T3, T4>
        where
            T0: IsUnset,
        {
            DepthStencilStateBuilder {
                format: FormatValue(format),
                depth_write_enabled: self.depth_write_enabled,
                depth_compare: self.depth_compare,
                stencil: self.stencil,
                bias: self.bias,
            }
        }
        pub fn depth_write_enabled(
            self,
            depth_write_enabled: bool,
        ) -> DepthStencilStateBuilder<T0, DepthWriteEnabledValue, T2, T3, T4>
        where
            T1: IsUnset,
        {
            DepthStencilStateBuilder {
                format: self.format,
                depth_write_enabled: DepthWriteEnabledValue(depth_write_enabled),
                depth_compare: self.depth_compare,
                stencil: self.stencil,
                bias: self.bias,
            }
        }
        pub fn depth_compare(
            self,
            depth_compare: wgpu::CompareFunction,
        ) -> DepthStencilStateBuilder<T0, T1, DepthCompareValue, T3, T4>
        where
            T2: IsUnset,
        {
            DepthStencilStateBuilder {
                format: self.format,
                depth_write_enabled: self.depth_write_enabled,
                depth_compare: DepthCompareValue(depth_compare),
                stencil: self.stencil,
                bias: self.bias,
            }
        }
        pub fn stencil(
            self,
            stencil: impl Nested<wgpu::StencilState>,
        ) -> DepthStencilStateBuilder<T0, T1, T2, StencilOptionalValue, T4>
        where
            T3: IsUnsetOptional,
        {
            DepthStencilStateBuilder {
                format: self.format,
                depth_write_enabled: self.depth_write_enabled,
                depth_compare: self.depth_compare,
                stencil: StencilOptionalValue(stencil.unnest()),
                bias: self.bias,
            }
        }
        pub fn bias(
            self,
            bias: impl Nested<wgpu::DepthBiasState>,
        ) -> DepthStencilStateBuilder<T0, T1, T2, T3, BiasOptionalValue>
        where
            T4: IsUnsetOptional,
        {
            DepthStencilStateBuilder {
                format: self.format,
                depth_write_enabled: self.depth_write_enabled,
                depth_compare: self.depth_compare,
                stencil: self.stencil,
                bias: BiasOptionalValue(bias.unnest()),
            }
        }
    }
    impl<RStencil, RBias>
        DepthStencilStateBuilder<
            FormatValue,
            DepthWriteEnabledValue,
            DepthCompareValue,
            RStencil,
            RBias,
        >
    {
        pub fn build(self) -> wgpu::DepthStencilState
        where
            RStencil: ResolveOptional<wgpu::StencilState>,
            RBias: ResolveOptional<wgpu::DepthBiasState>,
        {
            wgpu::DepthStencilState {
                format: self.format.0,
                depth_write_enabled: self.depth_write_enabled.0,
                depth_compare: self.depth_compare.0,
                stencil: self.stencil.resolve(),
                bias: self.bias.resolve(),
            }
        }
    }
    impl<RStencil, RBias> Nested<wgpu::DepthStencilState>
        for DepthStencilStateBuilder<
            FormatValue,
            DepthWriteEnabledValue,
            DepthCompareValue,
            RStencil,
            RBias,
        >
    where
        RStencil: ResolveOptional<wgpu::StencilState>,
        RBias: ResolveOptional<wgpu::DepthBiasState>,
    {
        fn unnest(self) -> wgpu::DepthStencilState {
            self.build()
        }
    }
    impl Nested<wgpu::DepthStencilState> for wgpu::DepthStencilState {
        fn unnest(self) -> wgpu::DepthStencilState {
            self
        }
    }
}

pub use builder_color::color;
pub mod builder_color {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn color() -> ColorBuilder<UnsetROptional, UnsetGOptional, UnsetBOptional, UnsetAOptional> {
        ColorBuilder::new()
    }
    pub struct ColorBuilder<T0, T1, T2, T3> {
        r: T0,
        g: T1,
        b: T2,
        a: T3,
    }
    impl ColorBuilder<UnsetROptional, UnsetGOptional, UnsetBOptional, UnsetAOptional> {
        pub fn new() -> Self {
            Self {
                r: UnsetROptional,
                g: UnsetGOptional,
                b: UnsetBOptional,
                a: UnsetAOptional,
            }
        }
    }
    pub struct UnsetROptional;
    impl IsOptional for UnsetROptional {}
    impl IsUnsetOptional for UnsetROptional {}
    impl ResolveOptional<f64> for UnsetROptional {
        fn resolve(self) -> f64 {
            Default::default()
        }
    }
    pub struct ROptionalValue(pub f64);
    impl IsOptional for ROptionalValue {}
    impl ResolveOptional<f64> for ROptionalValue {
        fn resolve(self) -> f64 {
            self.0
        }
    }
    pub struct UnsetGOptional;
    impl IsOptional for UnsetGOptional {}
    impl IsUnsetOptional for UnsetGOptional {}
    impl ResolveOptional<f64> for UnsetGOptional {
        fn resolve(self) -> f64 {
            Default::default()
        }
    }
    pub struct GOptionalValue(pub f64);
    impl IsOptional for GOptionalValue {}
    impl ResolveOptional<f64> for GOptionalValue {
        fn resolve(self) -> f64 {
            self.0
        }
    }
    pub struct UnsetBOptional;
    impl IsOptional for UnsetBOptional {}
    impl IsUnsetOptional for UnsetBOptional {}
    impl ResolveOptional<f64> for UnsetBOptional {
        fn resolve(self) -> f64 {
            Default::default()
        }
    }
    pub struct BOptionalValue(pub f64);
    impl IsOptional for BOptionalValue {}
    impl ResolveOptional<f64> for BOptionalValue {
        fn resolve(self) -> f64 {
            self.0
        }
    }
    pub struct UnsetAOptional;
    impl IsOptional for UnsetAOptional {}
    impl IsUnsetOptional for UnsetAOptional {}
    impl ResolveOptional<f64> for UnsetAOptional {
        fn resolve(self) -> f64 {
            Default::default()
        }
    }
    pub struct AOptionalValue(pub f64);
    impl IsOptional for AOptionalValue {}
    impl ResolveOptional<f64> for AOptionalValue {
        fn resolve(self) -> f64 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional> ColorBuilder<T0, T1, T2, T3> {
        pub fn r(self, r: f64) -> ColorBuilder<ROptionalValue, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            ColorBuilder {
                r: ROptionalValue(r),
                g: self.g,
                b: self.b,
                a: self.a,
            }
        }
        pub fn g(self, g: f64) -> ColorBuilder<T0, GOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            ColorBuilder {
                r: self.r,
                g: GOptionalValue(g),
                b: self.b,
                a: self.a,
            }
        }
        pub fn b(self, b: f64) -> ColorBuilder<T0, T1, BOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            ColorBuilder {
                r: self.r,
                g: self.g,
                b: BOptionalValue(b),
                a: self.a,
            }
        }
        pub fn a(self, a: f64) -> ColorBuilder<T0, T1, T2, AOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            ColorBuilder {
                r: self.r,
                g: self.g,
                b: self.b,
                a: AOptionalValue(a),
            }
        }
    }
    impl<RR, RG, RB, RA> ColorBuilder<RR, RG, RB, RA> {
        pub fn build(self) -> wgpu::Color
        where
            RR: ResolveOptional<f64>,
            RG: ResolveOptional<f64>,
            RB: ResolveOptional<f64>,
            RA: ResolveOptional<f64>,
        {
            wgpu::Color {
                r: self.r.resolve(),
                g: self.g.resolve(),
                b: self.b.resolve(),
                a: self.a.resolve(),
            }
        }
    }
    impl<RR, RG, RB, RA> Nested<wgpu::Color> for ColorBuilder<RR, RG, RB, RA>
    where
        RR: ResolveOptional<f64>,
        RG: ResolveOptional<f64>,
        RB: ResolveOptional<f64>,
        RA: ResolveOptional<f64>,
    {
        fn unnest(self) -> wgpu::Color {
            self.build()
        }
    }
    impl Nested<wgpu::Color> for wgpu::Color {
        fn unnest(self) -> wgpu::Color {
            self
        }
    }
}

pub use builder_texture_descriptor::texture_descriptor;
pub mod builder_texture_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texture_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> TextureDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetSizeOptional,
        UnsetMipLevelCount,
        UnsetSampleCount,
        UnsetDimension,
        UnsetFormat,
        UnsetUsage,
        UnsetViewFormats,
    > {
        TextureDescriptorBuilder::new().label(label)
    }
    pub struct TextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7> {
        label: T0,
        size: T1,
        mip_level_count: T2,
        sample_count: T3,
        dimension: T4,
        format: T5,
        usage: T6,
        view_formats: T7,
    }
    impl
        TextureDescriptorBuilder<
            UnsetLabelOptional,
            UnsetSizeOptional,
            UnsetMipLevelCount,
            UnsetSampleCount,
            UnsetDimension,
            UnsetFormat,
            UnsetUsage,
            UnsetViewFormats,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                size: UnsetSizeOptional,
                mip_level_count: UnsetMipLevelCount,
                sample_count: UnsetSampleCount,
                dimension: UnsetDimension,
                format: UnsetFormat,
                usage: UnsetUsage,
                view_formats: UnsetViewFormats,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetSizeOptional;
    impl IsOptional for UnsetSizeOptional {}
    impl IsUnsetOptional for UnsetSizeOptional {}
    impl ResolveOptional<wgpu::Extent3d> for UnsetSizeOptional {
        fn resolve(self) -> wgpu::Extent3d {
            Default::default()
        }
    }
    pub struct SizeOptionalValue(pub wgpu::Extent3d);
    impl IsOptional for SizeOptionalValue {}
    impl ResolveOptional<wgpu::Extent3d> for SizeOptionalValue {
        fn resolve(self) -> wgpu::Extent3d {
            self.0
        }
    }
    pub struct UnsetMipLevelCount;
    impl IsRequired for UnsetMipLevelCount {}
    impl IsUnset for UnsetMipLevelCount {}
    pub struct MipLevelCountValue(pub u32);
    impl IsRequired for MipLevelCountValue {}
    pub struct UnsetSampleCount;
    impl IsRequired for UnsetSampleCount {}
    impl IsUnset for UnsetSampleCount {}
    pub struct SampleCountValue(pub u32);
    impl IsRequired for SampleCountValue {}
    pub struct UnsetDimension;
    impl IsRequired for UnsetDimension {}
    impl IsUnset for UnsetDimension {}
    pub struct DimensionValue(pub wgpu::TextureDimension);
    impl IsRequired for DimensionValue {}
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::TextureFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetUsage;
    impl IsRequired for UnsetUsage {}
    impl IsUnset for UnsetUsage {}
    pub struct UsageValue(pub wgpu::TextureUsages);
    impl IsRequired for UsageValue {}
    pub struct UnsetViewFormats;
    impl IsRequired for UnsetViewFormats {}
    impl IsUnset for UnsetViewFormats {}
    pub struct ViewFormatsValue<'a>(pub &'a [wgpu::TextureFormat]);
    impl<'a> IsRequired for ViewFormatsValue<'a> {}
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsRequired,
        T3: IsRequired,
        T4: IsRequired,
        T5: IsRequired,
        T6: IsRequired,
        T7: IsRequired,
    > TextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> TextureDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4, T5, T6, T7>
        where
            T0: IsUnsetOptional,
        {
            TextureDescriptorBuilder {
                label: LabelOptionalValue(label),
                size: self.size,
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                dimension: self.dimension,
                format: self.format,
                usage: self.usage,
                view_formats: self.view_formats,
            }
        }
        pub fn size(
            self,
            size: impl Nested<wgpu::Extent3d>,
        ) -> TextureDescriptorBuilder<T0, SizeOptionalValue, T2, T3, T4, T5, T6, T7>
        where
            T1: IsUnsetOptional,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: SizeOptionalValue(size.unnest()),
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                dimension: self.dimension,
                format: self.format,
                usage: self.usage,
                view_formats: self.view_formats,
            }
        }
        pub fn mip_level_count(
            self,
            mip_level_count: u32,
        ) -> TextureDescriptorBuilder<T0, T1, MipLevelCountValue, T3, T4, T5, T6, T7>
        where
            T2: IsUnset,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: self.size,
                mip_level_count: MipLevelCountValue(mip_level_count),
                sample_count: self.sample_count,
                dimension: self.dimension,
                format: self.format,
                usage: self.usage,
                view_formats: self.view_formats,
            }
        }
        pub fn sample_count(
            self,
            sample_count: u32,
        ) -> TextureDescriptorBuilder<T0, T1, T2, SampleCountValue, T4, T5, T6, T7>
        where
            T3: IsUnset,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: self.size,
                mip_level_count: self.mip_level_count,
                sample_count: SampleCountValue(sample_count),
                dimension: self.dimension,
                format: self.format,
                usage: self.usage,
                view_formats: self.view_formats,
            }
        }
        pub fn dimension(
            self,
            dimension: wgpu::TextureDimension,
        ) -> TextureDescriptorBuilder<T0, T1, T2, T3, DimensionValue, T5, T6, T7>
        where
            T4: IsUnset,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: self.size,
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                dimension: DimensionValue(dimension),
                format: self.format,
                usage: self.usage,
                view_formats: self.view_formats,
            }
        }
        pub fn format(
            self,
            format: wgpu::TextureFormat,
        ) -> TextureDescriptorBuilder<T0, T1, T2, T3, T4, FormatValue, T6, T7>
        where
            T5: IsUnset,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: self.size,
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                dimension: self.dimension,
                format: FormatValue(format),
                usage: self.usage,
                view_formats: self.view_formats,
            }
        }
        pub fn usage(
            self,
            usage: wgpu::TextureUsages,
        ) -> TextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, UsageValue, T7>
        where
            T6: IsUnset,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: self.size,
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                dimension: self.dimension,
                format: self.format,
                usage: UsageValue(usage),
                view_formats: self.view_formats,
            }
        }
        pub fn view_formats<'a>(
            self,
            view_formats: &'a [wgpu::TextureFormat],
        ) -> TextureDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, ViewFormatsValue<'a>>
        where
            T7: IsUnset,
        {
            TextureDescriptorBuilder {
                label: self.label,
                size: self.size,
                mip_level_count: self.mip_level_count,
                sample_count: self.sample_count,
                dimension: self.dimension,
                format: self.format,
                usage: self.usage,
                view_formats: ViewFormatsValue(view_formats),
            }
        }
    }
    impl<'a, RLabel, RSize>
        TextureDescriptorBuilder<
            RLabel,
            RSize,
            MipLevelCountValue,
            SampleCountValue,
            DimensionValue,
            FormatValue,
            UsageValue,
            ViewFormatsValue<'a>,
        >
    {
        pub fn build(self) -> wgpu::TextureDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RSize: ResolveOptional<wgpu::Extent3d>,
        {
            wgpu::TextureDescriptor {
                label: self.label.resolve(),
                size: self.size.resolve(),
                mip_level_count: self.mip_level_count.0,
                sample_count: self.sample_count.0,
                dimension: self.dimension.0,
                format: self.format.0,
                usage: self.usage.0,
                view_formats: self.view_formats.0,
            }
        }
    }
}

pub use builder_buffer_init_descriptor::buffer_init_descriptor;
pub mod builder_buffer_init_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn buffer_init_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> BufferInitDescriptorBuilder<LabelOptionalValue<'a>, UnsetContents, UnsetUsage> {
        BufferInitDescriptorBuilder::new().label(label)
    }
    pub struct BufferInitDescriptorBuilder<T0, T1, T2> {
        label: T0,
        contents: T1,
        usage: T2,
    }
    impl BufferInitDescriptorBuilder<UnsetLabelOptional, UnsetContents, UnsetUsage> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                contents: UnsetContents,
                usage: UnsetUsage,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetContents;
    impl IsRequired for UnsetContents {}
    impl IsUnset for UnsetContents {}
    pub struct ContentsValue<'a>(pub &'a [u8]);
    impl<'a> IsRequired for ContentsValue<'a> {}
    pub struct UnsetUsage;
    impl IsRequired for UnsetUsage {}
    impl IsUnset for UnsetUsage {}
    pub struct UsageValue(pub wgpu::BufferUsages);
    impl IsRequired for UsageValue {}
    impl<T0: IsOptional, T1: IsRequired, T2: IsRequired> BufferInitDescriptorBuilder<T0, T1, T2> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> BufferInitDescriptorBuilder<LabelOptionalValue<'a>, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            BufferInitDescriptorBuilder {
                label: LabelOptionalValue(label),
                contents: self.contents,
                usage: self.usage,
            }
        }
        pub fn contents<'a>(
            self,
            contents: &'a [u8],
        ) -> BufferInitDescriptorBuilder<T0, ContentsValue<'a>, T2>
        where
            T1: IsUnset,
        {
            BufferInitDescriptorBuilder {
                label: self.label,
                contents: ContentsValue(contents),
                usage: self.usage,
            }
        }
        pub fn usage(
            self,
            usage: wgpu::BufferUsages,
        ) -> BufferInitDescriptorBuilder<T0, T1, UsageValue>
        where
            T2: IsUnset,
        {
            BufferInitDescriptorBuilder {
                label: self.label,
                contents: self.contents,
                usage: UsageValue(usage),
            }
        }
    }
    impl<'a, RLabel> BufferInitDescriptorBuilder<RLabel, ContentsValue<'a>, UsageValue> {
        pub fn build(self) -> wgpu::util::BufferInitDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::util::BufferInitDescriptor {
                label: self.label.resolve(),
                contents: self.contents.0,
                usage: self.usage.0,
            }
        }
    }
    impl<'a, RLabel> Nested<wgpu::util::BufferInitDescriptor<'a>>
        for BufferInitDescriptorBuilder<RLabel, ContentsValue<'a>, UsageValue>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
    {
        fn unnest(self) -> wgpu::util::BufferInitDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::util::BufferInitDescriptor<'a>> for wgpu::util::BufferInitDescriptor<'a> {
        fn unnest(self) -> wgpu::util::BufferInitDescriptor<'a> {
            self
        }
    }
}

pub use builder_operations::operations;
pub mod builder_operations {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn operations() -> OperationsBuilder<UnsetLoadOptional, UnsetStoreOptional> {
        OperationsBuilder::new()
    }
    pub struct OperationsBuilder<T0, T1> {
        load: T0,
        store: T1,
    }
    impl OperationsBuilder<UnsetLoadOptional, UnsetStoreOptional> {
        pub fn new() -> Self {
            Self {
                load: UnsetLoadOptional,
                store: UnsetStoreOptional,
            }
        }
    }
    pub struct UnsetLoadOptional;
    impl IsOptional for UnsetLoadOptional {}
    impl IsUnsetOptional for UnsetLoadOptional {}
    impl<V: Default> ResolveOptional<wgpu::LoadOp<V>> for UnsetLoadOptional {
        fn resolve(self) -> wgpu::LoadOp<V> {
            Default::default()
        }
    }
    pub struct LoadOptionalValue<V: Default>(pub wgpu::LoadOp<V>);
    impl<V: Default> IsOptional for LoadOptionalValue<V> {}
    impl<V: Default> ResolveOptional<wgpu::LoadOp<V>> for LoadOptionalValue<V> {
        fn resolve(self) -> wgpu::LoadOp<V> {
            self.0
        }
    }
    pub struct UnsetStoreOptional;
    impl IsOptional for UnsetStoreOptional {}
    impl IsUnsetOptional for UnsetStoreOptional {}
    impl ResolveOptional<wgpu::StoreOp> for UnsetStoreOptional {
        fn resolve(self) -> wgpu::StoreOp {
            Default::default()
        }
    }
    pub struct StoreOptionalValue(pub wgpu::StoreOp);
    impl IsOptional for StoreOptionalValue {}
    impl ResolveOptional<wgpu::StoreOp> for StoreOptionalValue {
        fn resolve(self) -> wgpu::StoreOp {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional> OperationsBuilder<T0, T1> {
        pub fn load<V: Default>(
            self,
            load: wgpu::LoadOp<V>,
        ) -> OperationsBuilder<LoadOptionalValue<V>, T1>
        where
            T0: IsUnsetOptional,
        {
            OperationsBuilder {
                load: LoadOptionalValue(load),
                store: self.store,
            }
        }
        pub fn store(self, store: wgpu::StoreOp) -> OperationsBuilder<T0, StoreOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            OperationsBuilder {
                load: self.load,
                store: StoreOptionalValue(store),
            }
        }
    }
    impl<RLoad, RStore> OperationsBuilder<RLoad, RStore> {
        pub fn build<V: Default>(self) -> wgpu::Operations<V>
        where
            RLoad: ResolveOptional<wgpu::LoadOp<V>>,
            RStore: ResolveOptional<wgpu::StoreOp>,
        {
            wgpu::Operations {
                load: self.load.resolve(),
                store: self.store.resolve(),
            }
        }
    }
    impl<V: Default, RLoad, RStore> Nested<wgpu::Operations<V>> for OperationsBuilder<RLoad, RStore>
    where
        RLoad: ResolveOptional<wgpu::LoadOp<V>>,
        RStore: ResolveOptional<wgpu::StoreOp>,
    {
        fn unnest(self) -> wgpu::Operations<V> {
            self.build()
        }
    }
    impl<V> Nested<wgpu::Operations<V>> for wgpu::Operations<V> {
        fn unnest(self) -> wgpu::Operations<V> {
            self
        }
    }
}

pub use builder_shader_runtime_checks::shader_runtime_checks;
pub mod builder_shader_runtime_checks {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn shader_runtime_checks()
    -> ShaderRuntimeChecksBuilder<UnsetBoundsChecks, UnsetForceLoopBounding> {
        ShaderRuntimeChecksBuilder::new()
    }
    pub struct ShaderRuntimeChecksBuilder<T0, T1> {
        bounds_checks: T0,
        force_loop_bounding: T1,
    }
    impl ShaderRuntimeChecksBuilder<UnsetBoundsChecks, UnsetForceLoopBounding> {
        pub fn new() -> Self {
            Self {
                bounds_checks: UnsetBoundsChecks,
                force_loop_bounding: UnsetForceLoopBounding,
            }
        }
    }
    pub struct UnsetBoundsChecks;
    impl IsRequired for UnsetBoundsChecks {}
    impl IsUnset for UnsetBoundsChecks {}
    pub struct BoundsChecksValue(pub bool);
    impl IsRequired for BoundsChecksValue {}
    pub struct UnsetForceLoopBounding;
    impl IsRequired for UnsetForceLoopBounding {}
    impl IsUnset for UnsetForceLoopBounding {}
    pub struct ForceLoopBoundingValue(pub bool);
    impl IsRequired for ForceLoopBoundingValue {}
    impl<T0: IsRequired, T1: IsRequired> ShaderRuntimeChecksBuilder<T0, T1> {
        pub fn bounds_checks(
            self,
            bounds_checks: bool,
        ) -> ShaderRuntimeChecksBuilder<BoundsChecksValue, T1>
        where
            T0: IsUnset,
        {
            ShaderRuntimeChecksBuilder {
                bounds_checks: BoundsChecksValue(bounds_checks),
                force_loop_bounding: self.force_loop_bounding,
            }
        }
        pub fn force_loop_bounding(
            self,
            force_loop_bounding: bool,
        ) -> ShaderRuntimeChecksBuilder<T0, ForceLoopBoundingValue>
        where
            T1: IsUnset,
        {
            ShaderRuntimeChecksBuilder {
                bounds_checks: self.bounds_checks,
                force_loop_bounding: ForceLoopBoundingValue(force_loop_bounding),
            }
        }
    }
    impl ShaderRuntimeChecksBuilder<BoundsChecksValue, ForceLoopBoundingValue> {
        pub fn build(self) -> wgpu::ShaderRuntimeChecks where {
            wgpu::ShaderRuntimeChecks {
                bounds_checks: self.bounds_checks.0,
                force_loop_bounding: self.force_loop_bounding.0,
            }
        }
    }
    impl Nested<wgpu::ShaderRuntimeChecks>
        for ShaderRuntimeChecksBuilder<BoundsChecksValue, ForceLoopBoundingValue>
    {
        fn unnest(self) -> wgpu::ShaderRuntimeChecks {
            self.build()
        }
    }
    impl Nested<wgpu::ShaderRuntimeChecks> for wgpu::ShaderRuntimeChecks {
        fn unnest(self) -> wgpu::ShaderRuntimeChecks {
            self
        }
    }
}

pub use builder_bind_group_descriptor::bind_group_descriptor;
pub mod builder_bind_group_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn bind_group_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> BindGroupDescriptorBuilder<LabelOptionalValue<'a>, UnsetLayout, UnsetEntries> {
        BindGroupDescriptorBuilder::new().label(label)
    }
    pub struct BindGroupDescriptorBuilder<T0, T1, T2> {
        label: T0,
        layout: T1,
        entries: T2,
    }
    impl BindGroupDescriptorBuilder<UnsetLabelOptional, UnsetLayout, UnsetEntries> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                layout: UnsetLayout,
                entries: UnsetEntries,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetLayout;
    impl IsRequired for UnsetLayout {}
    impl IsUnset for UnsetLayout {}
    pub struct LayoutValue<'a>(pub &'a wgpu::BindGroupLayout);
    impl<'a> IsRequired for LayoutValue<'a> {}
    pub struct UnsetEntries;
    impl IsRequired for UnsetEntries {}
    impl IsUnset for UnsetEntries {}
    pub struct EntriesValue<'a>(pub &'a [wgpu::BindGroupEntry<'a>]);
    impl<'a> IsRequired for EntriesValue<'a> {}
    impl<T0: IsOptional, T1: IsRequired, T2: IsRequired> BindGroupDescriptorBuilder<T0, T1, T2> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> BindGroupDescriptorBuilder<LabelOptionalValue<'a>, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            BindGroupDescriptorBuilder {
                label: LabelOptionalValue(label),
                layout: self.layout,
                entries: self.entries,
            }
        }
        pub fn layout<'a>(
            self,
            layout: &'a wgpu::BindGroupLayout,
        ) -> BindGroupDescriptorBuilder<T0, LayoutValue<'a>, T2>
        where
            T1: IsUnset,
        {
            BindGroupDescriptorBuilder {
                label: self.label,
                layout: LayoutValue(layout),
                entries: self.entries,
            }
        }
        pub fn entries<'a>(
            self,
            entries: &'a [wgpu::BindGroupEntry<'a>],
        ) -> BindGroupDescriptorBuilder<T0, T1, EntriesValue<'a>>
        where
            T2: IsUnset,
        {
            BindGroupDescriptorBuilder {
                label: self.label,
                layout: self.layout,
                entries: EntriesValue(entries),
            }
        }
    }
    impl<'a, RLabel> BindGroupDescriptorBuilder<RLabel, LayoutValue<'a>, EntriesValue<'a>> {
        pub fn build(self) -> wgpu::BindGroupDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::BindGroupDescriptor {
                label: self.label.resolve(),
                layout: self.layout.0,
                entries: self.entries.0,
            }
        }
    }
    impl<'a, RLabel> Nested<wgpu::BindGroupDescriptor<'a>>
        for BindGroupDescriptorBuilder<RLabel, LayoutValue<'a>, EntriesValue<'a>>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
    {
        fn unnest(self) -> wgpu::BindGroupDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::BindGroupDescriptor<'a>> for wgpu::BindGroupDescriptor<'a> {
        fn unnest(self) -> wgpu::BindGroupDescriptor<'a> {
            self
        }
    }
}

pub use builder_core_counters::core_counters;
pub mod builder_core_counters {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn core_counters() -> CoreCountersBuilder {
        CoreCountersBuilder::new()
    }
    pub struct CoreCountersBuilder {}
    impl CoreCountersBuilder {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl CoreCountersBuilder {}
    impl CoreCountersBuilder {
        pub fn build(self) -> wgpu::CoreCounters where {
            wgpu::CoreCounters {}
        }
    }
    impl Nested<wgpu::CoreCounters> for CoreCountersBuilder {
        fn unnest(self) -> wgpu::CoreCounters {
            self.build()
        }
    }
    impl Nested<wgpu::CoreCounters> for wgpu::CoreCounters {
        fn unnest(self) -> wgpu::CoreCounters {
            self
        }
    }
}

pub use builder_multisample_state::multisample_state;
pub mod builder_multisample_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn multisample_state() -> MultisampleStateBuilder<
        UnsetCountOptional,
        UnsetMaskOptional,
        UnsetAlphaToCoverageEnabledOptional,
    > {
        MultisampleStateBuilder::new()
    }
    pub struct MultisampleStateBuilder<T0, T1, T2> {
        count: T0,
        mask: T1,
        alpha_to_coverage_enabled: T2,
    }
    impl
        MultisampleStateBuilder<
            UnsetCountOptional,
            UnsetMaskOptional,
            UnsetAlphaToCoverageEnabledOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                count: UnsetCountOptional,
                mask: UnsetMaskOptional,
                alpha_to_coverage_enabled: UnsetAlphaToCoverageEnabledOptional,
            }
        }
    }
    pub struct UnsetCountOptional;
    impl IsOptional for UnsetCountOptional {}
    impl IsUnsetOptional for UnsetCountOptional {}
    impl ResolveOptional<u32> for UnsetCountOptional {
        fn resolve(self) -> u32 {
            1
        }
    }
    pub struct CountOptionalValue(pub u32);
    impl IsOptional for CountOptionalValue {}
    impl ResolveOptional<u32> for CountOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetMaskOptional;
    impl IsOptional for UnsetMaskOptional {}
    impl IsUnsetOptional for UnsetMaskOptional {}
    impl ResolveOptional<u64> for UnsetMaskOptional {
        fn resolve(self) -> u64 {
            !0
        }
    }
    pub struct MaskOptionalValue(pub u64);
    impl IsOptional for MaskOptionalValue {}
    impl ResolveOptional<u64> for MaskOptionalValue {
        fn resolve(self) -> u64 {
            self.0
        }
    }
    pub struct UnsetAlphaToCoverageEnabledOptional;
    impl IsOptional for UnsetAlphaToCoverageEnabledOptional {}
    impl IsUnsetOptional for UnsetAlphaToCoverageEnabledOptional {}
    impl ResolveOptional<bool> for UnsetAlphaToCoverageEnabledOptional {
        fn resolve(self) -> bool {
            false
        }
    }
    pub struct AlphaToCoverageEnabledOptionalValue(pub bool);
    impl IsOptional for AlphaToCoverageEnabledOptionalValue {}
    impl ResolveOptional<bool> for AlphaToCoverageEnabledOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> MultisampleStateBuilder<T0, T1, T2> {
        pub fn count(self, count: u32) -> MultisampleStateBuilder<CountOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            MultisampleStateBuilder {
                count: CountOptionalValue(count),
                mask: self.mask,
                alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
            }
        }
        pub fn mask(self, mask: u64) -> MultisampleStateBuilder<T0, MaskOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            MultisampleStateBuilder {
                count: self.count,
                mask: MaskOptionalValue(mask),
                alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
            }
        }
        pub fn alpha_to_coverage_enabled(
            self,
            alpha_to_coverage_enabled: bool,
        ) -> MultisampleStateBuilder<T0, T1, AlphaToCoverageEnabledOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            MultisampleStateBuilder {
                count: self.count,
                mask: self.mask,
                alpha_to_coverage_enabled: AlphaToCoverageEnabledOptionalValue(
                    alpha_to_coverage_enabled,
                ),
            }
        }
    }
    impl<RCount, RMask, RAlphaToCoverageEnabled>
        MultisampleStateBuilder<RCount, RMask, RAlphaToCoverageEnabled>
    {
        pub fn build(self) -> wgpu::MultisampleState
        where
            RCount: ResolveOptional<u32>,
            RMask: ResolveOptional<u64>,
            RAlphaToCoverageEnabled: ResolveOptional<bool>,
        {
            wgpu::MultisampleState {
                count: self.count.resolve(),
                mask: self.mask.resolve(),
                alpha_to_coverage_enabled: self.alpha_to_coverage_enabled.resolve(),
            }
        }
    }
    impl<RCount, RMask, RAlphaToCoverageEnabled> Nested<wgpu::MultisampleState>
        for MultisampleStateBuilder<RCount, RMask, RAlphaToCoverageEnabled>
    where
        RCount: ResolveOptional<u32>,
        RMask: ResolveOptional<u64>,
        RAlphaToCoverageEnabled: ResolveOptional<bool>,
    {
        fn unnest(self) -> wgpu::MultisampleState {
            self.build()
        }
    }
    impl Nested<wgpu::MultisampleState> for wgpu::MultisampleState {
        fn unnest(self) -> wgpu::MultisampleState {
            self
        }
    }
}

pub use builder_origin_2_d::origin_2_d;
pub mod builder_origin_2_d {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn origin_2_d() -> Origin2dBuilder<UnsetX, UnsetY> {
        Origin2dBuilder::new()
    }
    pub struct Origin2dBuilder<T0, T1> {
        x: T0,
        y: T1,
    }
    impl Origin2dBuilder<UnsetX, UnsetY> {
        pub fn new() -> Self {
            Self {
                x: UnsetX,
                y: UnsetY,
            }
        }
    }
    pub struct UnsetX;
    impl IsRequired for UnsetX {}
    impl IsUnset for UnsetX {}
    pub struct XValue(pub u32);
    impl IsRequired for XValue {}
    pub struct UnsetY;
    impl IsRequired for UnsetY {}
    impl IsUnset for UnsetY {}
    pub struct YValue(pub u32);
    impl IsRequired for YValue {}
    impl<T0: IsRequired, T1: IsRequired> Origin2dBuilder<T0, T1> {
        pub fn x(self, x: u32) -> Origin2dBuilder<XValue, T1>
        where
            T0: IsUnset,
        {
            Origin2dBuilder {
                x: XValue(x),
                y: self.y,
            }
        }
        pub fn y(self, y: u32) -> Origin2dBuilder<T0, YValue>
        where
            T1: IsUnset,
        {
            Origin2dBuilder {
                x: self.x,
                y: YValue(y),
            }
        }
    }
    impl Origin2dBuilder<XValue, YValue> {
        pub fn build(self) -> wgpu::Origin2d where {
            wgpu::Origin2d {
                x: self.x.0,
                y: self.y.0,
            }
        }
    }
    impl Nested<wgpu::Origin2d> for Origin2dBuilder<XValue, YValue> {
        fn unnest(self) -> wgpu::Origin2d {
            self.build()
        }
    }
    impl Nested<wgpu::Origin2d> for wgpu::Origin2d {
        fn unnest(self) -> wgpu::Origin2d {
            self
        }
    }
}

pub use builder_bind_group_layout_descriptor::bind_group_layout_descriptor;
pub mod builder_bind_group_layout_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn bind_group_layout_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> BindGroupLayoutDescriptorBuilder<LabelOptionalValue<'a>, UnsetEntries> {
        BindGroupLayoutDescriptorBuilder::new().label(label)
    }
    pub struct BindGroupLayoutDescriptorBuilder<T0, T1> {
        label: T0,
        entries: T1,
    }
    impl BindGroupLayoutDescriptorBuilder<UnsetLabelOptional, UnsetEntries> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                entries: UnsetEntries,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetEntries;
    impl IsRequired for UnsetEntries {}
    impl IsUnset for UnsetEntries {}
    pub struct EntriesValue<'a>(pub &'a [wgpu::BindGroupLayoutEntry]);
    impl<'a> IsRequired for EntriesValue<'a> {}
    impl<T0: IsOptional, T1: IsRequired> BindGroupLayoutDescriptorBuilder<T0, T1> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> BindGroupLayoutDescriptorBuilder<LabelOptionalValue<'a>, T1>
        where
            T0: IsUnsetOptional,
        {
            BindGroupLayoutDescriptorBuilder {
                label: LabelOptionalValue(label),
                entries: self.entries,
            }
        }
        pub fn entries<'a>(
            self,
            entries: &'a [wgpu::BindGroupLayoutEntry],
        ) -> BindGroupLayoutDescriptorBuilder<T0, EntriesValue<'a>>
        where
            T1: IsUnset,
        {
            BindGroupLayoutDescriptorBuilder {
                label: self.label,
                entries: EntriesValue(entries),
            }
        }
    }
    impl<'a, RLabel> BindGroupLayoutDescriptorBuilder<RLabel, EntriesValue<'a>> {
        pub fn build(self) -> wgpu::BindGroupLayoutDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::BindGroupLayoutDescriptor {
                label: self.label.resolve(),
                entries: self.entries.0,
            }
        }
    }
    impl<'a, RLabel> Nested<wgpu::BindGroupLayoutDescriptor<'a>>
        for BindGroupLayoutDescriptorBuilder<RLabel, EntriesValue<'a>>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
    {
        fn unnest(self) -> wgpu::BindGroupLayoutDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::BindGroupLayoutDescriptor<'a>> for wgpu::BindGroupLayoutDescriptor<'a> {
        fn unnest(self) -> wgpu::BindGroupLayoutDescriptor<'a> {
            self
        }
    }
}

pub use builder_query_set_descriptor::query_set_descriptor;
pub mod builder_query_set_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn query_set_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> QuerySetDescriptorBuilder<LabelOptionalValue<'a>, UnsetTy, UnsetCount> {
        QuerySetDescriptorBuilder::new().label(label)
    }
    pub struct QuerySetDescriptorBuilder<T0, T1, T2> {
        label: T0,
        ty: T1,
        count: T2,
    }
    impl QuerySetDescriptorBuilder<UnsetLabelOptional, UnsetTy, UnsetCount> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                ty: UnsetTy,
                count: UnsetCount,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetTy;
    impl IsRequired for UnsetTy {}
    impl IsUnset for UnsetTy {}
    pub struct TyValue(pub wgpu::QueryType);
    impl IsRequired for TyValue {}
    pub struct UnsetCount;
    impl IsRequired for UnsetCount {}
    impl IsUnset for UnsetCount {}
    pub struct CountValue(pub u32);
    impl IsRequired for CountValue {}
    impl<T0: IsOptional, T1: IsRequired, T2: IsRequired> QuerySetDescriptorBuilder<T0, T1, T2> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> QuerySetDescriptorBuilder<LabelOptionalValue<'a>, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            QuerySetDescriptorBuilder {
                label: LabelOptionalValue(label),
                ty: self.ty,
                count: self.count,
            }
        }
        pub fn ty(self, ty: wgpu::QueryType) -> QuerySetDescriptorBuilder<T0, TyValue, T2>
        where
            T1: IsUnset,
        {
            QuerySetDescriptorBuilder {
                label: self.label,
                ty: TyValue(ty),
                count: self.count,
            }
        }
        pub fn count(self, count: u32) -> QuerySetDescriptorBuilder<T0, T1, CountValue>
        where
            T2: IsUnset,
        {
            QuerySetDescriptorBuilder {
                label: self.label,
                ty: self.ty,
                count: CountValue(count),
            }
        }
    }
    impl<RLabel> QuerySetDescriptorBuilder<RLabel, TyValue, CountValue> {
        pub fn build<'a>(self) -> wgpu::QuerySetDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::QuerySetDescriptor {
                label: self.label.resolve(),
                ty: self.ty.0,
                count: self.count.0,
            }
        }
    }
}

pub use builder_pipeline_cache_descriptor::pipeline_cache_descriptor;
pub mod builder_pipeline_cache_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn pipeline_cache_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> PipelineCacheDescriptorBuilder<LabelOptionalValue<'a>, UnsetDataOptional, UnsetFallback>
    {
        PipelineCacheDescriptorBuilder::new().label(label)
    }
    pub struct PipelineCacheDescriptorBuilder<T0, T1, T2> {
        label: T0,
        data: T1,
        fallback: T2,
    }
    impl PipelineCacheDescriptorBuilder<UnsetLabelOptional, UnsetDataOptional, UnsetFallback> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                data: UnsetDataOptional,
                fallback: UnsetFallback,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetDataOptional;
    impl IsOptional for UnsetDataOptional {}
    impl IsUnsetOptional for UnsetDataOptional {}
    impl<'a> ResolveOptional<Option<&'a [u8]>> for UnsetDataOptional {
        fn resolve(self) -> Option<&'a [u8]> {
            Default::default()
        }
    }
    pub struct DataOptionalValue<'a>(pub Option<&'a [u8]>);
    impl<'a> IsOptional for DataOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a [u8]>> for DataOptionalValue<'a> {
        fn resolve(self) -> Option<&'a [u8]> {
            self.0
        }
    }
    pub struct UnsetFallback;
    impl IsRequired for UnsetFallback {}
    impl IsUnset for UnsetFallback {}
    pub struct FallbackValue(pub bool);
    impl IsRequired for FallbackValue {}
    impl<T0: IsOptional, T1: IsOptional, T2: IsRequired> PipelineCacheDescriptorBuilder<T0, T1, T2> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> PipelineCacheDescriptorBuilder<LabelOptionalValue<'a>, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            PipelineCacheDescriptorBuilder {
                label: LabelOptionalValue(label),
                data: self.data,
                fallback: self.fallback,
            }
        }
        pub fn maybe_data<'a>(
            self,
            data: Option<&'a [u8]>,
        ) -> PipelineCacheDescriptorBuilder<T0, DataOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            PipelineCacheDescriptorBuilder {
                label: self.label,
                data: DataOptionalValue(data),
                fallback: self.fallback,
            }
        }
        pub fn data<'a>(
            self,
            data: &'a [u8],
        ) -> PipelineCacheDescriptorBuilder<T0, DataOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            PipelineCacheDescriptorBuilder {
                label: self.label,
                data: DataOptionalValue(Some(data)),
                fallback: self.fallback,
            }
        }
        pub fn fallback(
            self,
            fallback: bool,
        ) -> PipelineCacheDescriptorBuilder<T0, T1, FallbackValue>
        where
            T2: IsUnset,
        {
            PipelineCacheDescriptorBuilder {
                label: self.label,
                data: self.data,
                fallback: FallbackValue(fallback),
            }
        }
    }
    impl<RLabel, RData> PipelineCacheDescriptorBuilder<RLabel, RData, FallbackValue> {
        pub fn build<'a>(self) -> wgpu::PipelineCacheDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RData: ResolveOptional<Option<&'a [u8]>>,
        {
            wgpu::PipelineCacheDescriptor {
                label: self.label.resolve(),
                data: self.data.resolve(),
                fallback: self.fallback.0,
            }
        }
    }
    impl<'a, RLabel, RData> Nested<wgpu::PipelineCacheDescriptor<'a>>
        for PipelineCacheDescriptorBuilder<RLabel, RData, FallbackValue>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RData: ResolveOptional<Option<&'a [u8]>>,
    {
        fn unnest(self) -> wgpu::PipelineCacheDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::PipelineCacheDescriptor<'a>> for wgpu::PipelineCacheDescriptor<'a> {
        fn unnest(self) -> wgpu::PipelineCacheDescriptor<'a> {
            self
        }
    }
}

pub use builder_command_encoder_descriptor::command_encoder_descriptor;
pub mod builder_command_encoder_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn command_encoder_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> CommandEncoderDescriptorBuilder<LabelOptionalValue<'a>> {
        CommandEncoderDescriptorBuilder::new().label(label)
    }
    pub struct CommandEncoderDescriptorBuilder<T0> {
        label: T0,
    }
    impl CommandEncoderDescriptorBuilder<UnsetLabelOptional> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    impl<T0: IsOptional> CommandEncoderDescriptorBuilder<T0> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> CommandEncoderDescriptorBuilder<LabelOptionalValue<'a>>
        where
            T0: IsUnsetOptional,
        {
            CommandEncoderDescriptorBuilder {
                label: LabelOptionalValue(label),
            }
        }
    }
    impl<RLabel> CommandEncoderDescriptorBuilder<RLabel> {
        pub fn build<'a>(self) -> wgpu::CommandEncoderDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::CommandEncoderDescriptor {
                label: self.label.resolve(),
            }
        }
    }
}

pub use builder_buffer_descriptor::buffer_descriptor;
pub mod builder_buffer_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn buffer_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> BufferDescriptorBuilder<LabelOptionalValue<'a>, UnsetSize, UnsetUsage, UnsetMappedAtCreation>
    {
        BufferDescriptorBuilder::new().label(label)
    }
    pub struct BufferDescriptorBuilder<T0, T1, T2, T3> {
        label: T0,
        size: T1,
        usage: T2,
        mapped_at_creation: T3,
    }
    impl BufferDescriptorBuilder<UnsetLabelOptional, UnsetSize, UnsetUsage, UnsetMappedAtCreation> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                size: UnsetSize,
                usage: UnsetUsage,
                mapped_at_creation: UnsetMappedAtCreation,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetSize;
    impl IsRequired for UnsetSize {}
    impl IsUnset for UnsetSize {}
    pub struct SizeValue(pub wgpu::BufferAddress);
    impl IsRequired for SizeValue {}
    pub struct UnsetUsage;
    impl IsRequired for UnsetUsage {}
    impl IsUnset for UnsetUsage {}
    pub struct UsageValue(pub wgpu::BufferUsages);
    impl IsRequired for UsageValue {}
    pub struct UnsetMappedAtCreation;
    impl IsRequired for UnsetMappedAtCreation {}
    impl IsUnset for UnsetMappedAtCreation {}
    pub struct MappedAtCreationValue(pub bool);
    impl IsRequired for MappedAtCreationValue {}
    impl<T0: IsOptional, T1: IsRequired, T2: IsRequired, T3: IsRequired>
        BufferDescriptorBuilder<T0, T1, T2, T3>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> BufferDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            BufferDescriptorBuilder {
                label: LabelOptionalValue(label),
                size: self.size,
                usage: self.usage,
                mapped_at_creation: self.mapped_at_creation,
            }
        }
        pub fn size(
            self,
            size: wgpu::BufferAddress,
        ) -> BufferDescriptorBuilder<T0, SizeValue, T2, T3>
        where
            T1: IsUnset,
        {
            BufferDescriptorBuilder {
                label: self.label,
                size: SizeValue(size),
                usage: self.usage,
                mapped_at_creation: self.mapped_at_creation,
            }
        }
        pub fn usage(
            self,
            usage: wgpu::BufferUsages,
        ) -> BufferDescriptorBuilder<T0, T1, UsageValue, T3>
        where
            T2: IsUnset,
        {
            BufferDescriptorBuilder {
                label: self.label,
                size: self.size,
                usage: UsageValue(usage),
                mapped_at_creation: self.mapped_at_creation,
            }
        }
        pub fn mapped_at_creation(
            self,
            mapped_at_creation: bool,
        ) -> BufferDescriptorBuilder<T0, T1, T2, MappedAtCreationValue>
        where
            T3: IsUnset,
        {
            BufferDescriptorBuilder {
                label: self.label,
                size: self.size,
                usage: self.usage,
                mapped_at_creation: MappedAtCreationValue(mapped_at_creation),
            }
        }
    }
    impl<RLabel> BufferDescriptorBuilder<RLabel, SizeValue, UsageValue, MappedAtCreationValue> {
        pub fn build<'a>(self) -> wgpu::BufferDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::BufferDescriptor {
                label: self.label.resolve(),
                size: self.size.0,
                usage: self.usage.0,
                mapped_at_creation: self.mapped_at_creation.0,
            }
        }
    }
}

pub use builder_blas_build_entry::blas_build_entry;
pub mod builder_blas_build_entry {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn blas_build_entry() -> BlasBuildEntryBuilder<UnsetBlas, UnsetGeometry> {
        BlasBuildEntryBuilder::new()
    }
    pub struct BlasBuildEntryBuilder<T0, T1> {
        blas: T0,
        geometry: T1,
    }
    impl BlasBuildEntryBuilder<UnsetBlas, UnsetGeometry> {
        pub fn new() -> Self {
            Self {
                blas: UnsetBlas,
                geometry: UnsetGeometry,
            }
        }
    }
    pub struct UnsetBlas;
    impl IsRequired for UnsetBlas {}
    impl IsUnset for UnsetBlas {}
    pub struct BlasValue<'a>(pub &'a wgpu::Blas);
    impl<'a> IsRequired for BlasValue<'a> {}
    pub struct UnsetGeometry;
    impl IsRequired for UnsetGeometry {}
    impl IsUnset for UnsetGeometry {}
    pub struct GeometryValue<'a>(pub wgpu::BlasGeometries<'a>);
    impl<'a> IsRequired for GeometryValue<'a> {}
    impl<T0: IsRequired, T1: IsRequired> BlasBuildEntryBuilder<T0, T1> {
        pub fn blas<'a>(self, blas: &'a wgpu::Blas) -> BlasBuildEntryBuilder<BlasValue<'a>, T1>
        where
            T0: IsUnset,
        {
            BlasBuildEntryBuilder {
                blas: BlasValue(blas),
                geometry: self.geometry,
            }
        }
        pub fn geometry<'a>(
            self,
            geometry: wgpu::BlasGeometries<'a>,
        ) -> BlasBuildEntryBuilder<T0, GeometryValue<'a>>
        where
            T1: IsUnset,
        {
            BlasBuildEntryBuilder {
                blas: self.blas,
                geometry: GeometryValue(geometry),
            }
        }
    }
    impl<'a> BlasBuildEntryBuilder<BlasValue<'a>, GeometryValue<'a>> {
        pub fn build(self) -> wgpu::BlasBuildEntry<'a> where {
            wgpu::BlasBuildEntry {
                blas: self.blas.0,
                geometry: self.geometry.0,
            }
        }
    }
    impl<'a> Nested<wgpu::BlasBuildEntry<'a>>
        for BlasBuildEntryBuilder<BlasValue<'a>, GeometryValue<'a>>
    {
        fn unnest(self) -> wgpu::BlasBuildEntry<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::BlasBuildEntry<'a>> for wgpu::BlasBuildEntry<'a> {
        fn unnest(self) -> wgpu::BlasBuildEntry<'a> {
            self
        }
    }
}

pub use builder_create_blas_descriptor::create_blas_descriptor;
pub mod builder_create_blas_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn create_blas_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> CreateBlasDescriptorBuilder<LabelOptionalValue<'a>, UnsetFlags, UnsetUpdateMode> {
        CreateBlasDescriptorBuilder::new().label(label)
    }
    pub struct CreateBlasDescriptorBuilder<T0, T1, T2> {
        label: T0,
        flags: T1,
        update_mode: T2,
    }
    impl CreateBlasDescriptorBuilder<UnsetLabelOptional, UnsetFlags, UnsetUpdateMode> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                flags: UnsetFlags,
                update_mode: UnsetUpdateMode,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetFlags;
    impl IsRequired for UnsetFlags {}
    impl IsUnset for UnsetFlags {}
    pub struct FlagsValue(pub wgpu::wgt::AccelerationStructureFlags);
    impl IsRequired for FlagsValue {}
    pub struct UnsetUpdateMode;
    impl IsRequired for UnsetUpdateMode {}
    impl IsUnset for UnsetUpdateMode {}
    pub struct UpdateModeValue(pub wgpu::wgt::AccelerationStructureUpdateMode);
    impl IsRequired for UpdateModeValue {}
    impl<T0: IsOptional, T1: IsRequired, T2: IsRequired> CreateBlasDescriptorBuilder<T0, T1, T2> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> CreateBlasDescriptorBuilder<LabelOptionalValue<'a>, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            CreateBlasDescriptorBuilder {
                label: LabelOptionalValue(label),
                flags: self.flags,
                update_mode: self.update_mode,
            }
        }
        pub fn flags(
            self,
            flags: wgpu::wgt::AccelerationStructureFlags,
        ) -> CreateBlasDescriptorBuilder<T0, FlagsValue, T2>
        where
            T1: IsUnset,
        {
            CreateBlasDescriptorBuilder {
                label: self.label,
                flags: FlagsValue(flags),
                update_mode: self.update_mode,
            }
        }
        pub fn update_mode(
            self,
            update_mode: wgpu::wgt::AccelerationStructureUpdateMode,
        ) -> CreateBlasDescriptorBuilder<T0, T1, UpdateModeValue>
        where
            T2: IsUnset,
        {
            CreateBlasDescriptorBuilder {
                label: self.label,
                flags: self.flags,
                update_mode: UpdateModeValue(update_mode),
            }
        }
    }
    impl<RLabel> CreateBlasDescriptorBuilder<RLabel, FlagsValue, UpdateModeValue> {
        pub fn build<'a>(self) -> wgpu::CreateBlasDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::CreateBlasDescriptor {
                label: self.label.resolve(),
                flags: self.flags.0,
                update_mode: self.update_mode.0,
            }
        }
    }
}

pub use builder_texture_transition::texture_transition;
pub mod builder_texture_transition {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texture_transition()
    -> TextureTransitionBuilder<UnsetTexture, UnsetSelectorOptional, UnsetState> {
        TextureTransitionBuilder::new()
    }
    pub struct TextureTransitionBuilder<T0, T1, T2> {
        texture: T0,
        selector: T1,
        state: T2,
    }
    impl TextureTransitionBuilder<UnsetTexture, UnsetSelectorOptional, UnsetState> {
        pub fn new() -> Self {
            Self {
                texture: UnsetTexture,
                selector: UnsetSelectorOptional,
                state: UnsetState,
            }
        }
    }
    pub struct UnsetTexture;
    impl IsRequired for UnsetTexture {}
    impl IsUnset for UnsetTexture {}
    pub struct TextureValue<T>(pub T);
    impl<T> IsRequired for TextureValue<T> {}
    pub struct UnsetSelectorOptional;
    impl IsOptional for UnsetSelectorOptional {}
    impl IsUnsetOptional for UnsetSelectorOptional {}
    impl ResolveOptional<Option<wgpu::wgt::TextureSelector>> for UnsetSelectorOptional {
        fn resolve(self) -> Option<wgpu::wgt::TextureSelector> {
            Default::default()
        }
    }
    pub struct SelectorOptionalValue(pub Option<wgpu::wgt::TextureSelector>);
    impl IsOptional for SelectorOptionalValue {}
    impl ResolveOptional<Option<wgpu::wgt::TextureSelector>> for SelectorOptionalValue {
        fn resolve(self) -> Option<wgpu::wgt::TextureSelector> {
            self.0
        }
    }
    pub struct UnsetState;
    impl IsRequired for UnsetState {}
    impl IsUnset for UnsetState {}
    pub struct StateValue(pub wgpu::TextureUses);
    impl IsRequired for StateValue {}
    impl<T0: IsRequired, T1: IsOptional, T2: IsRequired> TextureTransitionBuilder<T0, T1, T2> {
        pub fn texture<T>(self, texture: T) -> TextureTransitionBuilder<TextureValue<T>, T1, T2>
        where
            T0: IsUnset,
        {
            TextureTransitionBuilder {
                texture: TextureValue(texture),
                selector: self.selector,
                state: self.state,
            }
        }
        pub fn maybe_selector(
            self,
            selector: Option<wgpu::wgt::TextureSelector>,
        ) -> TextureTransitionBuilder<T0, SelectorOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            TextureTransitionBuilder {
                texture: self.texture,
                selector: SelectorOptionalValue(selector),
                state: self.state,
            }
        }
        pub fn selector(
            self,
            selector: wgpu::wgt::TextureSelector,
        ) -> TextureTransitionBuilder<T0, SelectorOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            TextureTransitionBuilder {
                texture: self.texture,
                selector: SelectorOptionalValue(Some(selector)),
                state: self.state,
            }
        }
        pub fn state(self, state: wgpu::TextureUses) -> TextureTransitionBuilder<T0, T1, StateValue>
        where
            T2: IsUnset,
        {
            TextureTransitionBuilder {
                texture: self.texture,
                selector: self.selector,
                state: StateValue(state),
            }
        }
    }
    impl<RSelector, T> TextureTransitionBuilder<TextureValue<T>, RSelector, StateValue> {
        pub fn build(self) -> wgpu::TextureTransition<T>
        where
            RSelector: ResolveOptional<Option<wgpu::wgt::TextureSelector>>,
        {
            wgpu::TextureTransition {
                texture: self.texture.0,
                selector: self.selector.resolve(),
                state: self.state.0,
            }
        }
    }
    impl<RSelector, T> Nested<wgpu::TextureTransition<T>>
        for TextureTransitionBuilder<TextureValue<T>, RSelector, StateValue>
    where
        RSelector: ResolveOptional<Option<wgpu::wgt::TextureSelector>>,
    {
        fn unnest(self) -> wgpu::TextureTransition<T> {
            self.build()
        }
    }
    impl<T> Nested<wgpu::TextureTransition<T>> for wgpu::TextureTransition<T> {
        fn unnest(self) -> wgpu::TextureTransition<T> {
            self
        }
    }
}

pub use builder_render_pass_descriptor::render_pass_descriptor;
pub mod builder_render_pass_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_pass_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> RenderPassDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetColorAttachmentsOptional,
        UnsetDepthStencilAttachmentOptional,
        UnsetTimestampWritesOptional,
        UnsetOcclusionQuerySetOptional,
    > {
        RenderPassDescriptorBuilder::new().label(label)
    }
    pub struct RenderPassDescriptorBuilder<T0, T1, T2, T3, T4> {
        label: T0,
        color_attachments: T1,
        depth_stencil_attachment: T2,
        timestamp_writes: T3,
        occlusion_query_set: T4,
    }
    impl
        RenderPassDescriptorBuilder<
            UnsetLabelOptional,
            UnsetColorAttachmentsOptional,
            UnsetDepthStencilAttachmentOptional,
            UnsetTimestampWritesOptional,
            UnsetOcclusionQuerySetOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                color_attachments: UnsetColorAttachmentsOptional,
                depth_stencil_attachment: UnsetDepthStencilAttachmentOptional,
                timestamp_writes: UnsetTimestampWritesOptional,
                occlusion_query_set: UnsetOcclusionQuerySetOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetColorAttachmentsOptional;
    impl IsOptional for UnsetColorAttachmentsOptional {}
    impl IsUnsetOptional for UnsetColorAttachmentsOptional {}
    impl<'a> ResolveOptional<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>
        for UnsetColorAttachmentsOptional
    {
        fn resolve(self) -> &'a [Option<wgpu::RenderPassColorAttachment<'a>>] {
            Default::default()
        }
    }
    pub struct ColorAttachmentsOptionalValue<'a>(
        pub &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
    );
    impl<'a> IsOptional for ColorAttachmentsOptionalValue<'a> {}
    impl<'a> ResolveOptional<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>
        for ColorAttachmentsOptionalValue<'a>
    {
        fn resolve(self) -> &'a [Option<wgpu::RenderPassColorAttachment<'a>>] {
            self.0
        }
    }
    pub struct UnsetDepthStencilAttachmentOptional;
    impl IsOptional for UnsetDepthStencilAttachmentOptional {}
    impl IsUnsetOptional for UnsetDepthStencilAttachmentOptional {}
    impl<'a> ResolveOptional<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>
        for UnsetDepthStencilAttachmentOptional
    {
        fn resolve(self) -> Option<wgpu::RenderPassDepthStencilAttachment<'a>> {
            Default::default()
        }
    }
    pub struct DepthStencilAttachmentOptionalValue<'a>(
        pub Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    );
    impl<'a> IsOptional for DepthStencilAttachmentOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>
        for DepthStencilAttachmentOptionalValue<'a>
    {
        fn resolve(self) -> Option<wgpu::RenderPassDepthStencilAttachment<'a>> {
            self.0
        }
    }
    pub struct UnsetTimestampWritesOptional;
    impl IsOptional for UnsetTimestampWritesOptional {}
    impl IsUnsetOptional for UnsetTimestampWritesOptional {}
    impl<'a> ResolveOptional<Option<wgpu::RenderPassTimestampWrites<'a>>>
        for UnsetTimestampWritesOptional
    {
        fn resolve(self) -> Option<wgpu::RenderPassTimestampWrites<'a>> {
            Default::default()
        }
    }
    pub struct TimestampWritesOptionalValue<'a>(pub Option<wgpu::RenderPassTimestampWrites<'a>>);
    impl<'a> IsOptional for TimestampWritesOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<wgpu::RenderPassTimestampWrites<'a>>>
        for TimestampWritesOptionalValue<'a>
    {
        fn resolve(self) -> Option<wgpu::RenderPassTimestampWrites<'a>> {
            self.0
        }
    }
    pub struct UnsetOcclusionQuerySetOptional;
    impl IsOptional for UnsetOcclusionQuerySetOptional {}
    impl IsUnsetOptional for UnsetOcclusionQuerySetOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::QuerySet>> for UnsetOcclusionQuerySetOptional {
        fn resolve(self) -> Option<&'a wgpu::QuerySet> {
            Default::default()
        }
    }
    pub struct OcclusionQuerySetOptionalValue<'a>(pub Option<&'a wgpu::QuerySet>);
    impl<'a> IsOptional for OcclusionQuerySetOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::QuerySet>> for OcclusionQuerySetOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::QuerySet> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional, T4: IsOptional>
        RenderPassDescriptorBuilder<T0, T1, T2, T3, T4>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> RenderPassDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4>
        where
            T0: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: LabelOptionalValue(label),
                color_attachments: self.color_attachments,
                depth_stencil_attachment: self.depth_stencil_attachment,
                timestamp_writes: self.timestamp_writes,
                occlusion_query_set: self.occlusion_query_set,
            }
        }
        pub fn color_attachments<'a>(
            self,
            color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
        ) -> RenderPassDescriptorBuilder<T0, ColorAttachmentsOptionalValue<'a>, T2, T3, T4>
        where
            T1: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: ColorAttachmentsOptionalValue(color_attachments),
                depth_stencil_attachment: self.depth_stencil_attachment,
                timestamp_writes: self.timestamp_writes,
                occlusion_query_set: self.occlusion_query_set,
            }
        }
        pub fn maybe_depth_stencil_attachment<'a>(
            self,
            depth_stencil_attachment: impl Nested<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>,
        ) -> RenderPassDescriptorBuilder<T0, T1, DepthStencilAttachmentOptionalValue<'a>, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: self.color_attachments,
                depth_stencil_attachment: DepthStencilAttachmentOptionalValue(
                    depth_stencil_attachment.unnest(),
                ),
                timestamp_writes: self.timestamp_writes,
                occlusion_query_set: self.occlusion_query_set,
            }
        }
        pub fn depth_stencil_attachment<'a>(
            self,
            depth_stencil_attachment: impl Nested<wgpu::RenderPassDepthStencilAttachment<'a>>,
        ) -> RenderPassDescriptorBuilder<T0, T1, DepthStencilAttachmentOptionalValue<'a>, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: self.color_attachments,
                depth_stencil_attachment: DepthStencilAttachmentOptionalValue(Some(
                    depth_stencil_attachment.unnest(),
                )),
                timestamp_writes: self.timestamp_writes,
                occlusion_query_set: self.occlusion_query_set,
            }
        }
        pub fn maybe_timestamp_writes<'a>(
            self,
            timestamp_writes: impl Nested<Option<wgpu::RenderPassTimestampWrites<'a>>>,
        ) -> RenderPassDescriptorBuilder<T0, T1, T2, TimestampWritesOptionalValue<'a>, T4>
        where
            T3: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: self.color_attachments,
                depth_stencil_attachment: self.depth_stencil_attachment,
                timestamp_writes: TimestampWritesOptionalValue(timestamp_writes.unnest()),
                occlusion_query_set: self.occlusion_query_set,
            }
        }
        pub fn timestamp_writes<'a>(
            self,
            timestamp_writes: impl Nested<wgpu::RenderPassTimestampWrites<'a>>,
        ) -> RenderPassDescriptorBuilder<T0, T1, T2, TimestampWritesOptionalValue<'a>, T4>
        where
            T3: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: self.color_attachments,
                depth_stencil_attachment: self.depth_stencil_attachment,
                timestamp_writes: TimestampWritesOptionalValue(Some(timestamp_writes.unnest())),
                occlusion_query_set: self.occlusion_query_set,
            }
        }
        pub fn maybe_occlusion_query_set<'a>(
            self,
            occlusion_query_set: Option<&'a wgpu::QuerySet>,
        ) -> RenderPassDescriptorBuilder<T0, T1, T2, T3, OcclusionQuerySetOptionalValue<'a>>
        where
            T4: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: self.color_attachments,
                depth_stencil_attachment: self.depth_stencil_attachment,
                timestamp_writes: self.timestamp_writes,
                occlusion_query_set: OcclusionQuerySetOptionalValue(occlusion_query_set),
            }
        }
        pub fn occlusion_query_set<'a>(
            self,
            occlusion_query_set: &'a wgpu::QuerySet,
        ) -> RenderPassDescriptorBuilder<T0, T1, T2, T3, OcclusionQuerySetOptionalValue<'a>>
        where
            T4: IsUnsetOptional,
        {
            RenderPassDescriptorBuilder {
                label: self.label,
                color_attachments: self.color_attachments,
                depth_stencil_attachment: self.depth_stencil_attachment,
                timestamp_writes: self.timestamp_writes,
                occlusion_query_set: OcclusionQuerySetOptionalValue(Some(occlusion_query_set)),
            }
        }
    }
    impl<RLabel, RColorAttachments, RDepthStencilAttachment, RTimestampWrites, ROcclusionQuerySet>
        RenderPassDescriptorBuilder<
            RLabel,
            RColorAttachments,
            RDepthStencilAttachment,
            RTimestampWrites,
            ROcclusionQuerySet,
        >
    {
        pub fn build<'a>(self) -> wgpu::RenderPassDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RColorAttachments: ResolveOptional<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>,
            RDepthStencilAttachment:
                ResolveOptional<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>,
            RTimestampWrites: ResolveOptional<Option<wgpu::RenderPassTimestampWrites<'a>>>,
            ROcclusionQuerySet: ResolveOptional<Option<&'a wgpu::QuerySet>>,
        {
            wgpu::RenderPassDescriptor {
                label: self.label.resolve(),
                color_attachments: self.color_attachments.resolve(),
                depth_stencil_attachment: self.depth_stencil_attachment.resolve(),
                timestamp_writes: self.timestamp_writes.resolve(),
                occlusion_query_set: self.occlusion_query_set.resolve(),
            }
        }
    }
    impl<
        'a,
        RLabel,
        RColorAttachments,
        RDepthStencilAttachment,
        RTimestampWrites,
        ROcclusionQuerySet,
    > Nested<wgpu::RenderPassDescriptor<'a>>
        for RenderPassDescriptorBuilder<
            RLabel,
            RColorAttachments,
            RDepthStencilAttachment,
            RTimestampWrites,
            ROcclusionQuerySet,
        >
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RColorAttachments: ResolveOptional<&'a [Option<wgpu::RenderPassColorAttachment<'a>>]>,
        RDepthStencilAttachment:
            ResolveOptional<Option<wgpu::RenderPassDepthStencilAttachment<'a>>>,
        RTimestampWrites: ResolveOptional<Option<wgpu::RenderPassTimestampWrites<'a>>>,
        ROcclusionQuerySet: ResolveOptional<Option<&'a wgpu::QuerySet>>,
    {
        fn unnest(self) -> wgpu::RenderPassDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::RenderPassDescriptor<'a>> for wgpu::RenderPassDescriptor<'a> {
        fn unnest(self) -> wgpu::RenderPassDescriptor<'a> {
            self
        }
    }
}

pub use builder_mesh_state::mesh_state;
pub mod builder_mesh_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn mesh_state()
    -> MeshStateBuilder<UnsetModule, UnsetEntryPointOptional, UnsetCompilationOptionsOptional> {
        MeshStateBuilder::new()
    }
    pub struct MeshStateBuilder<T0, T1, T2> {
        module: T0,
        entry_point: T1,
        compilation_options: T2,
    }
    impl MeshStateBuilder<UnsetModule, UnsetEntryPointOptional, UnsetCompilationOptionsOptional> {
        pub fn new() -> Self {
            Self {
                module: UnsetModule,
                entry_point: UnsetEntryPointOptional,
                compilation_options: UnsetCompilationOptionsOptional,
            }
        }
    }
    pub struct UnsetModule;
    impl IsRequired for UnsetModule {}
    impl IsUnset for UnsetModule {}
    pub struct ModuleValue<'a>(pub &'a wgpu::ShaderModule);
    impl<'a> IsRequired for ModuleValue<'a> {}
    pub struct UnsetEntryPointOptional;
    impl IsOptional for UnsetEntryPointOptional {}
    impl IsUnsetOptional for UnsetEntryPointOptional {}
    impl<'a> ResolveOptional<Option<&'a str>> for UnsetEntryPointOptional {
        fn resolve(self) -> Option<&'a str> {
            Default::default()
        }
    }
    pub struct EntryPointOptionalValue<'a>(pub Option<&'a str>);
    impl<'a> IsOptional for EntryPointOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a str>> for EntryPointOptionalValue<'a> {
        fn resolve(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct UnsetCompilationOptionsOptional;
    impl IsOptional for UnsetCompilationOptionsOptional {}
    impl IsUnsetOptional for UnsetCompilationOptionsOptional {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>> for UnsetCompilationOptionsOptional {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            Default::default()
        }
    }
    pub struct CompilationOptionsOptionalValue<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    impl<'a> IsOptional for CompilationOptionsOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>>
        for CompilationOptionsOptionalValue<'a>
    {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional> MeshStateBuilder<T0, T1, T2> {
        pub fn module<'a>(
            self,
            module: &'a wgpu::ShaderModule,
        ) -> MeshStateBuilder<ModuleValue<'a>, T1, T2>
        where
            T0: IsUnset,
        {
            MeshStateBuilder {
                module: ModuleValue(module),
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
            }
        }
        pub fn maybe_entry_point<'a>(
            self,
            entry_point: Option<&'a str>,
        ) -> MeshStateBuilder<T0, EntryPointOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            MeshStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(entry_point),
                compilation_options: self.compilation_options,
            }
        }
        pub fn entry_point<'a>(
            self,
            entry_point: &'a str,
        ) -> MeshStateBuilder<T0, EntryPointOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            MeshStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(Some(entry_point)),
                compilation_options: self.compilation_options,
            }
        }
        pub fn compilation_options<'a>(
            self,
            compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
        ) -> MeshStateBuilder<T0, T1, CompilationOptionsOptionalValue<'a>>
        where
            T2: IsUnsetOptional,
        {
            MeshStateBuilder {
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: CompilationOptionsOptionalValue(compilation_options.unnest()),
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions>
        MeshStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions>
    {
        pub fn build(self) -> wgpu::MeshState<'a>
        where
            REntryPoint: ResolveOptional<Option<&'a str>>,
            RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
        {
            wgpu::MeshState {
                module: self.module.0,
                entry_point: self.entry_point.resolve(),
                compilation_options: self.compilation_options.resolve(),
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions> Nested<wgpu::MeshState<'a>>
        for MeshStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions>
    where
        REntryPoint: ResolveOptional<Option<&'a str>>,
        RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
    {
        fn unnest(self) -> wgpu::MeshState<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::MeshState<'a>> for wgpu::MeshState<'a> {
        fn unnest(self) -> wgpu::MeshState<'a> {
            self
        }
    }
}

pub use builder_stencil_face_state::stencil_face_state;
pub mod builder_stencil_face_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn stencil_face_state() -> StencilFaceStateBuilder<
        UnsetCompareOptional,
        UnsetFailOpOptional,
        UnsetDepthFailOpOptional,
        UnsetPassOpOptional,
    > {
        StencilFaceStateBuilder::new()
    }
    pub struct StencilFaceStateBuilder<T0, T1, T2, T3> {
        compare: T0,
        fail_op: T1,
        depth_fail_op: T2,
        pass_op: T3,
    }
    impl
        StencilFaceStateBuilder<
            UnsetCompareOptional,
            UnsetFailOpOptional,
            UnsetDepthFailOpOptional,
            UnsetPassOpOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                compare: UnsetCompareOptional,
                fail_op: UnsetFailOpOptional,
                depth_fail_op: UnsetDepthFailOpOptional,
                pass_op: UnsetPassOpOptional,
            }
        }
    }
    pub struct UnsetCompareOptional;
    impl IsOptional for UnsetCompareOptional {}
    impl IsUnsetOptional for UnsetCompareOptional {}
    impl ResolveOptional<wgpu::CompareFunction> for UnsetCompareOptional {
        fn resolve(self) -> wgpu::CompareFunction {
            wgpu::CompareFunction::Always
        }
    }
    pub struct CompareOptionalValue(pub wgpu::CompareFunction);
    impl IsOptional for CompareOptionalValue {}
    impl ResolveOptional<wgpu::CompareFunction> for CompareOptionalValue {
        fn resolve(self) -> wgpu::CompareFunction {
            self.0
        }
    }
    pub struct UnsetFailOpOptional;
    impl IsOptional for UnsetFailOpOptional {}
    impl IsUnsetOptional for UnsetFailOpOptional {}
    impl ResolveOptional<wgpu::StencilOperation> for UnsetFailOpOptional {
        fn resolve(self) -> wgpu::StencilOperation {
            Default::default()
        }
    }
    pub struct FailOpOptionalValue(pub wgpu::StencilOperation);
    impl IsOptional for FailOpOptionalValue {}
    impl ResolveOptional<wgpu::StencilOperation> for FailOpOptionalValue {
        fn resolve(self) -> wgpu::StencilOperation {
            self.0
        }
    }
    pub struct UnsetDepthFailOpOptional;
    impl IsOptional for UnsetDepthFailOpOptional {}
    impl IsUnsetOptional for UnsetDepthFailOpOptional {}
    impl ResolveOptional<wgpu::StencilOperation> for UnsetDepthFailOpOptional {
        fn resolve(self) -> wgpu::StencilOperation {
            Default::default()
        }
    }
    pub struct DepthFailOpOptionalValue(pub wgpu::StencilOperation);
    impl IsOptional for DepthFailOpOptionalValue {}
    impl ResolveOptional<wgpu::StencilOperation> for DepthFailOpOptionalValue {
        fn resolve(self) -> wgpu::StencilOperation {
            self.0
        }
    }
    pub struct UnsetPassOpOptional;
    impl IsOptional for UnsetPassOpOptional {}
    impl IsUnsetOptional for UnsetPassOpOptional {}
    impl ResolveOptional<wgpu::StencilOperation> for UnsetPassOpOptional {
        fn resolve(self) -> wgpu::StencilOperation {
            Default::default()
        }
    }
    pub struct PassOpOptionalValue(pub wgpu::StencilOperation);
    impl IsOptional for PassOpOptionalValue {}
    impl ResolveOptional<wgpu::StencilOperation> for PassOpOptionalValue {
        fn resolve(self) -> wgpu::StencilOperation {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional>
        StencilFaceStateBuilder<T0, T1, T2, T3>
    {
        pub fn compare(
            self,
            compare: wgpu::CompareFunction,
        ) -> StencilFaceStateBuilder<CompareOptionalValue, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            StencilFaceStateBuilder {
                compare: CompareOptionalValue(compare),
                fail_op: self.fail_op,
                depth_fail_op: self.depth_fail_op,
                pass_op: self.pass_op,
            }
        }
        pub fn fail_op(
            self,
            fail_op: wgpu::StencilOperation,
        ) -> StencilFaceStateBuilder<T0, FailOpOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            StencilFaceStateBuilder {
                compare: self.compare,
                fail_op: FailOpOptionalValue(fail_op),
                depth_fail_op: self.depth_fail_op,
                pass_op: self.pass_op,
            }
        }
        pub fn depth_fail_op(
            self,
            depth_fail_op: wgpu::StencilOperation,
        ) -> StencilFaceStateBuilder<T0, T1, DepthFailOpOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            StencilFaceStateBuilder {
                compare: self.compare,
                fail_op: self.fail_op,
                depth_fail_op: DepthFailOpOptionalValue(depth_fail_op),
                pass_op: self.pass_op,
            }
        }
        pub fn pass_op(
            self,
            pass_op: wgpu::StencilOperation,
        ) -> StencilFaceStateBuilder<T0, T1, T2, PassOpOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            StencilFaceStateBuilder {
                compare: self.compare,
                fail_op: self.fail_op,
                depth_fail_op: self.depth_fail_op,
                pass_op: PassOpOptionalValue(pass_op),
            }
        }
    }
    impl<RCompare, RFailOp, RDepthFailOp, RPassOp>
        StencilFaceStateBuilder<RCompare, RFailOp, RDepthFailOp, RPassOp>
    {
        pub fn build(self) -> wgpu::StencilFaceState
        where
            RCompare: ResolveOptional<wgpu::CompareFunction>,
            RFailOp: ResolveOptional<wgpu::StencilOperation>,
            RDepthFailOp: ResolveOptional<wgpu::StencilOperation>,
            RPassOp: ResolveOptional<wgpu::StencilOperation>,
        {
            wgpu::StencilFaceState {
                compare: self.compare.resolve(),
                fail_op: self.fail_op.resolve(),
                depth_fail_op: self.depth_fail_op.resolve(),
                pass_op: self.pass_op.resolve(),
            }
        }
    }
    impl<RCompare, RFailOp, RDepthFailOp, RPassOp> Nested<wgpu::StencilFaceState>
        for StencilFaceStateBuilder<RCompare, RFailOp, RDepthFailOp, RPassOp>
    where
        RCompare: ResolveOptional<wgpu::CompareFunction>,
        RFailOp: ResolveOptional<wgpu::StencilOperation>,
        RDepthFailOp: ResolveOptional<wgpu::StencilOperation>,
        RPassOp: ResolveOptional<wgpu::StencilOperation>,
    {
        fn unnest(self) -> wgpu::StencilFaceState {
            self.build()
        }
    }
    impl Nested<wgpu::StencilFaceState> for wgpu::StencilFaceState {
        fn unnest(self) -> wgpu::StencilFaceState {
            self
        }
    }
}

pub use builder_stencil_state::stencil_state;
pub mod builder_stencil_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn stencil_state() -> StencilStateBuilder<
        UnsetFrontOptional,
        UnsetBackOptional,
        UnsetReadMaskOptional,
        UnsetWriteMaskOptional,
    > {
        StencilStateBuilder::new()
    }
    pub struct StencilStateBuilder<T0, T1, T2, T3> {
        front: T0,
        back: T1,
        read_mask: T2,
        write_mask: T3,
    }
    impl
        StencilStateBuilder<
            UnsetFrontOptional,
            UnsetBackOptional,
            UnsetReadMaskOptional,
            UnsetWriteMaskOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                front: UnsetFrontOptional,
                back: UnsetBackOptional,
                read_mask: UnsetReadMaskOptional,
                write_mask: UnsetWriteMaskOptional,
            }
        }
    }
    pub struct UnsetFrontOptional;
    impl IsOptional for UnsetFrontOptional {}
    impl IsUnsetOptional for UnsetFrontOptional {}
    impl ResolveOptional<wgpu::StencilFaceState> for UnsetFrontOptional {
        fn resolve(self) -> wgpu::StencilFaceState {
            Default::default()
        }
    }
    pub struct FrontOptionalValue(pub wgpu::StencilFaceState);
    impl IsOptional for FrontOptionalValue {}
    impl ResolveOptional<wgpu::StencilFaceState> for FrontOptionalValue {
        fn resolve(self) -> wgpu::StencilFaceState {
            self.0
        }
    }
    pub struct UnsetBackOptional;
    impl IsOptional for UnsetBackOptional {}
    impl IsUnsetOptional for UnsetBackOptional {}
    impl ResolveOptional<wgpu::StencilFaceState> for UnsetBackOptional {
        fn resolve(self) -> wgpu::StencilFaceState {
            Default::default()
        }
    }
    pub struct BackOptionalValue(pub wgpu::StencilFaceState);
    impl IsOptional for BackOptionalValue {}
    impl ResolveOptional<wgpu::StencilFaceState> for BackOptionalValue {
        fn resolve(self) -> wgpu::StencilFaceState {
            self.0
        }
    }
    pub struct UnsetReadMaskOptional;
    impl IsOptional for UnsetReadMaskOptional {}
    impl IsUnsetOptional for UnsetReadMaskOptional {}
    impl ResolveOptional<u32> for UnsetReadMaskOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct ReadMaskOptionalValue(pub u32);
    impl IsOptional for ReadMaskOptionalValue {}
    impl ResolveOptional<u32> for ReadMaskOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetWriteMaskOptional;
    impl IsOptional for UnsetWriteMaskOptional {}
    impl IsUnsetOptional for UnsetWriteMaskOptional {}
    impl ResolveOptional<u32> for UnsetWriteMaskOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct WriteMaskOptionalValue(pub u32);
    impl IsOptional for WriteMaskOptionalValue {}
    impl ResolveOptional<u32> for WriteMaskOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional>
        StencilStateBuilder<T0, T1, T2, T3>
    {
        pub fn front(
            self,
            front: impl Nested<wgpu::StencilFaceState>,
        ) -> StencilStateBuilder<FrontOptionalValue, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            StencilStateBuilder {
                front: FrontOptionalValue(front.unnest()),
                back: self.back,
                read_mask: self.read_mask,
                write_mask: self.write_mask,
            }
        }
        pub fn back(
            self,
            back: impl Nested<wgpu::StencilFaceState>,
        ) -> StencilStateBuilder<T0, BackOptionalValue, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            StencilStateBuilder {
                front: self.front,
                back: BackOptionalValue(back.unnest()),
                read_mask: self.read_mask,
                write_mask: self.write_mask,
            }
        }
        pub fn read_mask(
            self,
            read_mask: u32,
        ) -> StencilStateBuilder<T0, T1, ReadMaskOptionalValue, T3>
        where
            T2: IsUnsetOptional,
        {
            StencilStateBuilder {
                front: self.front,
                back: self.back,
                read_mask: ReadMaskOptionalValue(read_mask),
                write_mask: self.write_mask,
            }
        }
        pub fn write_mask(
            self,
            write_mask: u32,
        ) -> StencilStateBuilder<T0, T1, T2, WriteMaskOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            StencilStateBuilder {
                front: self.front,
                back: self.back,
                read_mask: self.read_mask,
                write_mask: WriteMaskOptionalValue(write_mask),
            }
        }
    }
    impl<RFront, RBack, RReadMask, RWriteMask>
        StencilStateBuilder<RFront, RBack, RReadMask, RWriteMask>
    {
        pub fn build(self) -> wgpu::StencilState
        where
            RFront: ResolveOptional<wgpu::StencilFaceState>,
            RBack: ResolveOptional<wgpu::StencilFaceState>,
            RReadMask: ResolveOptional<u32>,
            RWriteMask: ResolveOptional<u32>,
        {
            wgpu::StencilState {
                front: self.front.resolve(),
                back: self.back.resolve(),
                read_mask: self.read_mask.resolve(),
                write_mask: self.write_mask.resolve(),
            }
        }
    }
    impl<RFront, RBack, RReadMask, RWriteMask> Nested<wgpu::StencilState>
        for StencilStateBuilder<RFront, RBack, RReadMask, RWriteMask>
    where
        RFront: ResolveOptional<wgpu::StencilFaceState>,
        RBack: ResolveOptional<wgpu::StencilFaceState>,
        RReadMask: ResolveOptional<u32>,
        RWriteMask: ResolveOptional<u32>,
    {
        fn unnest(self) -> wgpu::StencilState {
            self.build()
        }
    }
    impl Nested<wgpu::StencilState> for wgpu::StencilState {
        fn unnest(self) -> wgpu::StencilState {
            self
        }
    }
}

pub use builder_compute_pass_descriptor::compute_pass_descriptor;
pub mod builder_compute_pass_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn compute_pass_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> ComputePassDescriptorBuilder<LabelOptionalValue<'a>, UnsetTimestampWritesOptional> {
        ComputePassDescriptorBuilder::new().label(label)
    }
    pub struct ComputePassDescriptorBuilder<T0, T1> {
        label: T0,
        timestamp_writes: T1,
    }
    impl ComputePassDescriptorBuilder<UnsetLabelOptional, UnsetTimestampWritesOptional> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                timestamp_writes: UnsetTimestampWritesOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetTimestampWritesOptional;
    impl IsOptional for UnsetTimestampWritesOptional {}
    impl IsUnsetOptional for UnsetTimestampWritesOptional {}
    impl<'a> ResolveOptional<Option<wgpu::ComputePassTimestampWrites<'a>>>
        for UnsetTimestampWritesOptional
    {
        fn resolve(self) -> Option<wgpu::ComputePassTimestampWrites<'a>> {
            Default::default()
        }
    }
    pub struct TimestampWritesOptionalValue<'a>(pub Option<wgpu::ComputePassTimestampWrites<'a>>);
    impl<'a> IsOptional for TimestampWritesOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<wgpu::ComputePassTimestampWrites<'a>>>
        for TimestampWritesOptionalValue<'a>
    {
        fn resolve(self) -> Option<wgpu::ComputePassTimestampWrites<'a>> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional> ComputePassDescriptorBuilder<T0, T1> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> ComputePassDescriptorBuilder<LabelOptionalValue<'a>, T1>
        where
            T0: IsUnsetOptional,
        {
            ComputePassDescriptorBuilder {
                label: LabelOptionalValue(label),
                timestamp_writes: self.timestamp_writes,
            }
        }
        pub fn maybe_timestamp_writes<'a>(
            self,
            timestamp_writes: impl Nested<Option<wgpu::ComputePassTimestampWrites<'a>>>,
        ) -> ComputePassDescriptorBuilder<T0, TimestampWritesOptionalValue<'a>>
        where
            T1: IsUnsetOptional,
        {
            ComputePassDescriptorBuilder {
                label: self.label,
                timestamp_writes: TimestampWritesOptionalValue(timestamp_writes.unnest()),
            }
        }
        pub fn timestamp_writes<'a>(
            self,
            timestamp_writes: impl Nested<wgpu::ComputePassTimestampWrites<'a>>,
        ) -> ComputePassDescriptorBuilder<T0, TimestampWritesOptionalValue<'a>>
        where
            T1: IsUnsetOptional,
        {
            ComputePassDescriptorBuilder {
                label: self.label,
                timestamp_writes: TimestampWritesOptionalValue(Some(timestamp_writes.unnest())),
            }
        }
    }
    impl<RLabel, RTimestampWrites> ComputePassDescriptorBuilder<RLabel, RTimestampWrites> {
        pub fn build<'a>(self) -> wgpu::ComputePassDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RTimestampWrites: ResolveOptional<Option<wgpu::ComputePassTimestampWrites<'a>>>,
        {
            wgpu::ComputePassDescriptor {
                label: self.label.resolve(),
                timestamp_writes: self.timestamp_writes.resolve(),
            }
        }
    }
    impl<'a, RLabel, RTimestampWrites> Nested<wgpu::ComputePassDescriptor<'a>>
        for ComputePassDescriptorBuilder<RLabel, RTimestampWrites>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RTimestampWrites: ResolveOptional<Option<wgpu::ComputePassTimestampWrites<'a>>>,
    {
        fn unnest(self) -> wgpu::ComputePassDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::ComputePassDescriptor<'a>> for wgpu::ComputePassDescriptor<'a> {
        fn unnest(self) -> wgpu::ComputePassDescriptor<'a> {
            self
        }
    }
}

pub use builder_compute_pass_timestamp_writes::compute_pass_timestamp_writes;
pub mod builder_compute_pass_timestamp_writes {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn compute_pass_timestamp_writes() -> ComputePassTimestampWritesBuilder<
        UnsetQuerySet,
        UnsetBeginningOfPassWriteIndexOptional,
        UnsetEndOfPassWriteIndexOptional,
    > {
        ComputePassTimestampWritesBuilder::new()
    }
    pub struct ComputePassTimestampWritesBuilder<T0, T1, T2> {
        query_set: T0,
        beginning_of_pass_write_index: T1,
        end_of_pass_write_index: T2,
    }
    impl
        ComputePassTimestampWritesBuilder<
            UnsetQuerySet,
            UnsetBeginningOfPassWriteIndexOptional,
            UnsetEndOfPassWriteIndexOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                query_set: UnsetQuerySet,
                beginning_of_pass_write_index: UnsetBeginningOfPassWriteIndexOptional,
                end_of_pass_write_index: UnsetEndOfPassWriteIndexOptional,
            }
        }
    }
    pub struct UnsetQuerySet;
    impl IsRequired for UnsetQuerySet {}
    impl IsUnset for UnsetQuerySet {}
    pub struct QuerySetValue<'a>(pub &'a wgpu::QuerySet);
    impl<'a> IsRequired for QuerySetValue<'a> {}
    pub struct UnsetBeginningOfPassWriteIndexOptional;
    impl IsOptional for UnsetBeginningOfPassWriteIndexOptional {}
    impl IsUnsetOptional for UnsetBeginningOfPassWriteIndexOptional {}
    impl ResolveOptional<Option<u32>> for UnsetBeginningOfPassWriteIndexOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct BeginningOfPassWriteIndexOptionalValue(pub Option<u32>);
    impl IsOptional for BeginningOfPassWriteIndexOptionalValue {}
    impl ResolveOptional<Option<u32>> for BeginningOfPassWriteIndexOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetEndOfPassWriteIndexOptional;
    impl IsOptional for UnsetEndOfPassWriteIndexOptional {}
    impl IsUnsetOptional for UnsetEndOfPassWriteIndexOptional {}
    impl ResolveOptional<Option<u32>> for UnsetEndOfPassWriteIndexOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct EndOfPassWriteIndexOptionalValue(pub Option<u32>);
    impl IsOptional for EndOfPassWriteIndexOptionalValue {}
    impl ResolveOptional<Option<u32>> for EndOfPassWriteIndexOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional> ComputePassTimestampWritesBuilder<T0, T1, T2> {
        pub fn query_set<'a>(
            self,
            query_set: &'a wgpu::QuerySet,
        ) -> ComputePassTimestampWritesBuilder<QuerySetValue<'a>, T1, T2>
        where
            T0: IsUnset,
        {
            ComputePassTimestampWritesBuilder {
                query_set: QuerySetValue(query_set),
                beginning_of_pass_write_index: self.beginning_of_pass_write_index,
                end_of_pass_write_index: self.end_of_pass_write_index,
            }
        }
        pub fn maybe_beginning_of_pass_write_index(
            self,
            beginning_of_pass_write_index: Option<u32>,
        ) -> ComputePassTimestampWritesBuilder<T0, BeginningOfPassWriteIndexOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            ComputePassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: BeginningOfPassWriteIndexOptionalValue(
                    beginning_of_pass_write_index,
                ),
                end_of_pass_write_index: self.end_of_pass_write_index,
            }
        }
        pub fn beginning_of_pass_write_index(
            self,
            beginning_of_pass_write_index: u32,
        ) -> ComputePassTimestampWritesBuilder<T0, BeginningOfPassWriteIndexOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            ComputePassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: BeginningOfPassWriteIndexOptionalValue(Some(
                    beginning_of_pass_write_index,
                )),
                end_of_pass_write_index: self.end_of_pass_write_index,
            }
        }
        pub fn maybe_end_of_pass_write_index(
            self,
            end_of_pass_write_index: Option<u32>,
        ) -> ComputePassTimestampWritesBuilder<T0, T1, EndOfPassWriteIndexOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            ComputePassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: self.beginning_of_pass_write_index,
                end_of_pass_write_index: EndOfPassWriteIndexOptionalValue(end_of_pass_write_index),
            }
        }
        pub fn end_of_pass_write_index(
            self,
            end_of_pass_write_index: u32,
        ) -> ComputePassTimestampWritesBuilder<T0, T1, EndOfPassWriteIndexOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            ComputePassTimestampWritesBuilder {
                query_set: self.query_set,
                beginning_of_pass_write_index: self.beginning_of_pass_write_index,
                end_of_pass_write_index: EndOfPassWriteIndexOptionalValue(Some(
                    end_of_pass_write_index,
                )),
            }
        }
    }
    impl<'a, RBeginningOfPassWriteIndex, REndOfPassWriteIndex>
        ComputePassTimestampWritesBuilder<
            QuerySetValue<'a>,
            RBeginningOfPassWriteIndex,
            REndOfPassWriteIndex,
        >
    {
        pub fn build(self) -> wgpu::ComputePassTimestampWrites<'a>
        where
            RBeginningOfPassWriteIndex: ResolveOptional<Option<u32>>,
            REndOfPassWriteIndex: ResolveOptional<Option<u32>>,
        {
            wgpu::ComputePassTimestampWrites {
                query_set: self.query_set.0,
                beginning_of_pass_write_index: self.beginning_of_pass_write_index.resolve(),
                end_of_pass_write_index: self.end_of_pass_write_index.resolve(),
            }
        }
    }
    impl<'a, RBeginningOfPassWriteIndex, REndOfPassWriteIndex>
        Nested<wgpu::ComputePassTimestampWrites<'a>>
        for ComputePassTimestampWritesBuilder<
            QuerySetValue<'a>,
            RBeginningOfPassWriteIndex,
            REndOfPassWriteIndex,
        >
    where
        RBeginningOfPassWriteIndex: ResolveOptional<Option<u32>>,
        REndOfPassWriteIndex: ResolveOptional<Option<u32>>,
    {
        fn unnest(self) -> wgpu::ComputePassTimestampWrites<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::ComputePassTimestampWrites<'a>> for wgpu::ComputePassTimestampWrites<'a> {
        fn unnest(self) -> wgpu::ComputePassTimestampWrites<'a> {
            self
        }
    }
}

pub use builder_downlevel_limits::downlevel_limits;
pub mod builder_downlevel_limits {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn downlevel_limits() -> DownlevelLimitsBuilder {
        DownlevelLimitsBuilder::new()
    }
    pub struct DownlevelLimitsBuilder {}
    impl DownlevelLimitsBuilder {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl DownlevelLimitsBuilder {}
    impl DownlevelLimitsBuilder {
        pub fn build(self) -> wgpu::DownlevelLimits where {
            wgpu::DownlevelLimits {}
        }
    }
    impl Nested<wgpu::DownlevelLimits> for DownlevelLimitsBuilder {
        fn unnest(self) -> wgpu::DownlevelLimits {
            self.build()
        }
    }
    impl Nested<wgpu::DownlevelLimits> for wgpu::DownlevelLimits {
        fn unnest(self) -> wgpu::DownlevelLimits {
            self
        }
    }
}

pub use builder_fragment_state::fragment_state;
pub mod builder_fragment_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn fragment_state() -> FragmentStateBuilder<
        UnsetModule,
        UnsetEntryPointOptional,
        UnsetCompilationOptionsOptional,
        UnsetTargets,
    > {
        FragmentStateBuilder::new()
    }
    pub struct FragmentStateBuilder<T0, T1, T2, T3> {
        module: T0,
        entry_point: T1,
        compilation_options: T2,
        targets: T3,
    }
    impl
        FragmentStateBuilder<
            UnsetModule,
            UnsetEntryPointOptional,
            UnsetCompilationOptionsOptional,
            UnsetTargets,
        >
    {
        pub fn new() -> Self {
            Self {
                module: UnsetModule,
                entry_point: UnsetEntryPointOptional,
                compilation_options: UnsetCompilationOptionsOptional,
                targets: UnsetTargets,
            }
        }
    }
    pub struct UnsetModule;
    impl IsRequired for UnsetModule {}
    impl IsUnset for UnsetModule {}
    pub struct ModuleValue<'a>(pub &'a wgpu::ShaderModule);
    impl<'a> IsRequired for ModuleValue<'a> {}
    pub struct UnsetEntryPointOptional;
    impl IsOptional for UnsetEntryPointOptional {}
    impl IsUnsetOptional for UnsetEntryPointOptional {}
    impl<'a> ResolveOptional<Option<&'a str>> for UnsetEntryPointOptional {
        fn resolve(self) -> Option<&'a str> {
            Default::default()
        }
    }
    pub struct EntryPointOptionalValue<'a>(pub Option<&'a str>);
    impl<'a> IsOptional for EntryPointOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a str>> for EntryPointOptionalValue<'a> {
        fn resolve(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct UnsetCompilationOptionsOptional;
    impl IsOptional for UnsetCompilationOptionsOptional {}
    impl IsUnsetOptional for UnsetCompilationOptionsOptional {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>> for UnsetCompilationOptionsOptional {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            Default::default()
        }
    }
    pub struct CompilationOptionsOptionalValue<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    impl<'a> IsOptional for CompilationOptionsOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>>
        for CompilationOptionsOptionalValue<'a>
    {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    pub struct UnsetTargets;
    impl IsRequired for UnsetTargets {}
    impl IsUnset for UnsetTargets {}
    pub struct TargetsValue<'a>(pub &'a [Option<wgpu::ColorTargetState>]);
    impl<'a> IsRequired for TargetsValue<'a> {}
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional, T3: IsRequired>
        FragmentStateBuilder<T0, T1, T2, T3>
    {
        pub fn module<'a>(
            self,
            module: &'a wgpu::ShaderModule,
        ) -> FragmentStateBuilder<ModuleValue<'a>, T1, T2, T3>
        where
            T0: IsUnset,
        {
            FragmentStateBuilder {
                module: ModuleValue(module),
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                targets: self.targets,
            }
        }
        pub fn maybe_entry_point<'a>(
            self,
            entry_point: Option<&'a str>,
        ) -> FragmentStateBuilder<T0, EntryPointOptionalValue<'a>, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            FragmentStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(entry_point),
                compilation_options: self.compilation_options,
                targets: self.targets,
            }
        }
        pub fn entry_point<'a>(
            self,
            entry_point: &'a str,
        ) -> FragmentStateBuilder<T0, EntryPointOptionalValue<'a>, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            FragmentStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(Some(entry_point)),
                compilation_options: self.compilation_options,
                targets: self.targets,
            }
        }
        pub fn compilation_options<'a>(
            self,
            compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
        ) -> FragmentStateBuilder<T0, T1, CompilationOptionsOptionalValue<'a>, T3>
        where
            T2: IsUnsetOptional,
        {
            FragmentStateBuilder {
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: CompilationOptionsOptionalValue(compilation_options.unnest()),
                targets: self.targets,
            }
        }
        pub fn targets<'a>(
            self,
            targets: &'a [Option<wgpu::ColorTargetState>],
        ) -> FragmentStateBuilder<T0, T1, T2, TargetsValue<'a>>
        where
            T3: IsUnset,
        {
            FragmentStateBuilder {
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                targets: TargetsValue(targets),
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions>
        FragmentStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions, TargetsValue<'a>>
    {
        pub fn build(self) -> wgpu::FragmentState<'a>
        where
            REntryPoint: ResolveOptional<Option<&'a str>>,
            RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
        {
            wgpu::FragmentState {
                module: self.module.0,
                entry_point: self.entry_point.resolve(),
                compilation_options: self.compilation_options.resolve(),
                targets: self.targets.0,
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions> Nested<wgpu::FragmentState<'a>>
        for FragmentStateBuilder<
            ModuleValue<'a>,
            REntryPoint,
            RCompilationOptions,
            TargetsValue<'a>,
        >
    where
        REntryPoint: ResolveOptional<Option<&'a str>>,
        RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
    {
        fn unnest(self) -> wgpu::FragmentState<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::FragmentState<'a>> for wgpu::FragmentState<'a> {
        fn unnest(self) -> wgpu::FragmentState<'a> {
            self
        }
    }
}

pub use builder_buffer_transition::buffer_transition;
pub mod builder_buffer_transition {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn buffer_transition() -> BufferTransitionBuilder<UnsetBuffer, UnsetState> {
        BufferTransitionBuilder::new()
    }
    pub struct BufferTransitionBuilder<T0, T1> {
        buffer: T0,
        state: T1,
    }
    impl BufferTransitionBuilder<UnsetBuffer, UnsetState> {
        pub fn new() -> Self {
            Self {
                buffer: UnsetBuffer,
                state: UnsetState,
            }
        }
    }
    pub struct UnsetBuffer;
    impl IsRequired for UnsetBuffer {}
    impl IsUnset for UnsetBuffer {}
    pub struct BufferValue<T>(pub T);
    impl<T> IsRequired for BufferValue<T> {}
    pub struct UnsetState;
    impl IsRequired for UnsetState {}
    impl IsUnset for UnsetState {}
    pub struct StateValue(pub wgpu::BufferUses);
    impl IsRequired for StateValue {}
    impl<T0: IsRequired, T1: IsRequired> BufferTransitionBuilder<T0, T1> {
        pub fn buffer<T>(self, buffer: T) -> BufferTransitionBuilder<BufferValue<T>, T1>
        where
            T0: IsUnset,
        {
            BufferTransitionBuilder {
                buffer: BufferValue(buffer),
                state: self.state,
            }
        }
        pub fn state(self, state: wgpu::BufferUses) -> BufferTransitionBuilder<T0, StateValue>
        where
            T1: IsUnset,
        {
            BufferTransitionBuilder {
                buffer: self.buffer,
                state: StateValue(state),
            }
        }
    }
    impl<T> BufferTransitionBuilder<BufferValue<T>, StateValue> {
        pub fn build(self) -> wgpu::BufferTransition<T> where {
            wgpu::BufferTransition {
                buffer: self.buffer.0,
                state: self.state.0,
            }
        }
    }
    impl<T> Nested<wgpu::BufferTransition<T>> for BufferTransitionBuilder<BufferValue<T>, StateValue> {
        fn unnest(self) -> wgpu::BufferTransition<T> {
            self.build()
        }
    }
    impl<T> Nested<wgpu::BufferTransition<T>> for wgpu::BufferTransition<T> {
        fn unnest(self) -> wgpu::BufferTransition<T> {
            self
        }
    }
}

pub use builder_bind_group_layout_entry::bind_group_layout_entry;
pub mod builder_bind_group_layout_entry {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn bind_group_layout_entry()
    -> BindGroupLayoutEntryBuilder<UnsetBinding, UnsetVisibility, UnsetTy, UnsetCountOptional> {
        BindGroupLayoutEntryBuilder::new()
    }
    pub struct BindGroupLayoutEntryBuilder<T0, T1, T2, T3> {
        binding: T0,
        visibility: T1,
        ty: T2,
        count: T3,
    }
    impl BindGroupLayoutEntryBuilder<UnsetBinding, UnsetVisibility, UnsetTy, UnsetCountOptional> {
        pub fn new() -> Self {
            Self {
                binding: UnsetBinding,
                visibility: UnsetVisibility,
                ty: UnsetTy,
                count: UnsetCountOptional,
            }
        }
    }
    pub struct UnsetBinding;
    impl IsRequired for UnsetBinding {}
    impl IsUnset for UnsetBinding {}
    pub struct BindingValue(pub u32);
    impl IsRequired for BindingValue {}
    pub struct UnsetVisibility;
    impl IsRequired for UnsetVisibility {}
    impl IsUnset for UnsetVisibility {}
    pub struct VisibilityValue(pub wgpu::ShaderStages);
    impl IsRequired for VisibilityValue {}
    pub struct UnsetTy;
    impl IsRequired for UnsetTy {}
    impl IsUnset for UnsetTy {}
    pub struct TyValue(pub wgpu::BindingType);
    impl IsRequired for TyValue {}
    pub struct UnsetCountOptional;
    impl IsOptional for UnsetCountOptional {}
    impl IsUnsetOptional for UnsetCountOptional {}
    impl ResolveOptional<Option<NonZeroU32>> for UnsetCountOptional {
        fn resolve(self) -> Option<NonZeroU32> {
            Default::default()
        }
    }
    pub struct CountOptionalValue(pub Option<NonZeroU32>);
    impl IsOptional for CountOptionalValue {}
    impl ResolveOptional<Option<NonZeroU32>> for CountOptionalValue {
        fn resolve(self) -> Option<NonZeroU32> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsRequired, T2: IsRequired, T3: IsOptional>
        BindGroupLayoutEntryBuilder<T0, T1, T2, T3>
    {
        pub fn binding(self, binding: u32) -> BindGroupLayoutEntryBuilder<BindingValue, T1, T2, T3>
        where
            T0: IsUnset,
        {
            BindGroupLayoutEntryBuilder {
                binding: BindingValue(binding),
                visibility: self.visibility,
                ty: self.ty,
                count: self.count,
            }
        }
        pub fn visibility(
            self,
            visibility: wgpu::ShaderStages,
        ) -> BindGroupLayoutEntryBuilder<T0, VisibilityValue, T2, T3>
        where
            T1: IsUnset,
        {
            BindGroupLayoutEntryBuilder {
                binding: self.binding,
                visibility: VisibilityValue(visibility),
                ty: self.ty,
                count: self.count,
            }
        }
        pub fn ty(self, ty: wgpu::BindingType) -> BindGroupLayoutEntryBuilder<T0, T1, TyValue, T3>
        where
            T2: IsUnset,
        {
            BindGroupLayoutEntryBuilder {
                binding: self.binding,
                visibility: self.visibility,
                ty: TyValue(ty),
                count: self.count,
            }
        }
        pub fn maybe_count(
            self,
            count: Option<NonZeroU32>,
        ) -> BindGroupLayoutEntryBuilder<T0, T1, T2, CountOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            BindGroupLayoutEntryBuilder {
                binding: self.binding,
                visibility: self.visibility,
                ty: self.ty,
                count: CountOptionalValue(count),
            }
        }
        pub fn count(
            self,
            count: NonZeroU32,
        ) -> BindGroupLayoutEntryBuilder<T0, T1, T2, CountOptionalValue>
        where
            T3: IsUnsetOptional,
        {
            BindGroupLayoutEntryBuilder {
                binding: self.binding,
                visibility: self.visibility,
                ty: self.ty,
                count: CountOptionalValue(Some(count)),
            }
        }
    }
    impl<RCount> BindGroupLayoutEntryBuilder<BindingValue, VisibilityValue, TyValue, RCount> {
        pub fn build(self) -> wgpu::BindGroupLayoutEntry
        where
            RCount: ResolveOptional<Option<NonZeroU32>>,
        {
            wgpu::BindGroupLayoutEntry {
                binding: self.binding.0,
                visibility: self.visibility.0,
                ty: self.ty.0,
                count: self.count.resolve(),
            }
        }
    }
    impl<RCount> Nested<wgpu::BindGroupLayoutEntry>
        for BindGroupLayoutEntryBuilder<BindingValue, VisibilityValue, TyValue, RCount>
    where
        RCount: ResolveOptional<Option<NonZeroU32>>,
    {
        fn unnest(self) -> wgpu::BindGroupLayoutEntry {
            self.build()
        }
    }
    impl Nested<wgpu::BindGroupLayoutEntry> for wgpu::BindGroupLayoutEntry {
        fn unnest(self) -> wgpu::BindGroupLayoutEntry {
            self
        }
    }
}

pub use builder_buffer_binding::buffer_binding;
pub mod builder_buffer_binding {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn buffer_binding() -> BufferBindingBuilder<UnsetBuffer, UnsetOffset, UnsetSizeOptional> {
        BufferBindingBuilder::new()
    }
    pub struct BufferBindingBuilder<T0, T1, T2> {
        buffer: T0,
        offset: T1,
        size: T2,
    }
    impl BufferBindingBuilder<UnsetBuffer, UnsetOffset, UnsetSizeOptional> {
        pub fn new() -> Self {
            Self {
                buffer: UnsetBuffer,
                offset: UnsetOffset,
                size: UnsetSizeOptional,
            }
        }
    }
    pub struct UnsetBuffer;
    impl IsRequired for UnsetBuffer {}
    impl IsUnset for UnsetBuffer {}
    pub struct BufferValue<'a>(pub &'a wgpu::Buffer);
    impl<'a> IsRequired for BufferValue<'a> {}
    pub struct UnsetOffset;
    impl IsRequired for UnsetOffset {}
    impl IsUnset for UnsetOffset {}
    pub struct OffsetValue(pub wgpu::BufferAddress);
    impl IsRequired for OffsetValue {}
    pub struct UnsetSizeOptional;
    impl IsOptional for UnsetSizeOptional {}
    impl IsUnsetOptional for UnsetSizeOptional {}
    impl ResolveOptional<Option<wgpu::BufferSize>> for UnsetSizeOptional {
        fn resolve(self) -> Option<wgpu::BufferSize> {
            Default::default()
        }
    }
    pub struct SizeOptionalValue(pub Option<wgpu::BufferSize>);
    impl IsOptional for SizeOptionalValue {}
    impl ResolveOptional<Option<wgpu::BufferSize>> for SizeOptionalValue {
        fn resolve(self) -> Option<wgpu::BufferSize> {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsRequired, T2: IsOptional> BufferBindingBuilder<T0, T1, T2> {
        pub fn buffer<'a>(
            self,
            buffer: &'a wgpu::Buffer,
        ) -> BufferBindingBuilder<BufferValue<'a>, T1, T2>
        where
            T0: IsUnset,
        {
            BufferBindingBuilder {
                buffer: BufferValue(buffer),
                offset: self.offset,
                size: self.size,
            }
        }
        pub fn offset(
            self,
            offset: wgpu::BufferAddress,
        ) -> BufferBindingBuilder<T0, OffsetValue, T2>
        where
            T1: IsUnset,
        {
            BufferBindingBuilder {
                buffer: self.buffer,
                offset: OffsetValue(offset),
                size: self.size,
            }
        }
        pub fn maybe_size(
            self,
            size: Option<wgpu::BufferSize>,
        ) -> BufferBindingBuilder<T0, T1, SizeOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            BufferBindingBuilder {
                buffer: self.buffer,
                offset: self.offset,
                size: SizeOptionalValue(size),
            }
        }
        pub fn size(self, size: wgpu::BufferSize) -> BufferBindingBuilder<T0, T1, SizeOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            BufferBindingBuilder {
                buffer: self.buffer,
                offset: self.offset,
                size: SizeOptionalValue(Some(size)),
            }
        }
    }
    impl<'a, RSize> BufferBindingBuilder<BufferValue<'a>, OffsetValue, RSize> {
        pub fn build(self) -> wgpu::BufferBinding<'a>
        where
            RSize: ResolveOptional<Option<wgpu::BufferSize>>,
        {
            wgpu::BufferBinding {
                buffer: self.buffer.0,
                offset: self.offset.0,
                size: self.size.resolve(),
            }
        }
    }
    impl<'a, RSize> Nested<wgpu::BufferBinding<'a>>
        for BufferBindingBuilder<BufferValue<'a>, OffsetValue, RSize>
    where
        RSize: ResolveOptional<Option<wgpu::BufferSize>>,
    {
        fn unnest(self) -> wgpu::BufferBinding<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::BufferBinding<'a>> for wgpu::BufferBinding<'a> {
        fn unnest(self) -> wgpu::BufferBinding<'a> {
            self
        }
    }
}

pub use builder_draw_indexed_indirect_args::draw_indexed_indirect_args;
pub mod builder_draw_indexed_indirect_args {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn draw_indexed_indirect_args() -> DrawIndexedIndirectArgsBuilder<
        UnsetIndexCountOptional,
        UnsetInstanceCountOptional,
        UnsetFirstIndexOptional,
        UnsetBaseVertexOptional,
        UnsetFirstInstanceOptional,
    > {
        DrawIndexedIndirectArgsBuilder::new()
    }
    pub struct DrawIndexedIndirectArgsBuilder<T0, T1, T2, T3, T4> {
        index_count: T0,
        instance_count: T1,
        first_index: T2,
        base_vertex: T3,
        first_instance: T4,
    }
    impl
        DrawIndexedIndirectArgsBuilder<
            UnsetIndexCountOptional,
            UnsetInstanceCountOptional,
            UnsetFirstIndexOptional,
            UnsetBaseVertexOptional,
            UnsetFirstInstanceOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                index_count: UnsetIndexCountOptional,
                instance_count: UnsetInstanceCountOptional,
                first_index: UnsetFirstIndexOptional,
                base_vertex: UnsetBaseVertexOptional,
                first_instance: UnsetFirstInstanceOptional,
            }
        }
    }
    pub struct UnsetIndexCountOptional;
    impl IsOptional for UnsetIndexCountOptional {}
    impl IsUnsetOptional for UnsetIndexCountOptional {}
    impl ResolveOptional<u32> for UnsetIndexCountOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct IndexCountOptionalValue(pub u32);
    impl IsOptional for IndexCountOptionalValue {}
    impl ResolveOptional<u32> for IndexCountOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetInstanceCountOptional;
    impl IsOptional for UnsetInstanceCountOptional {}
    impl IsUnsetOptional for UnsetInstanceCountOptional {}
    impl ResolveOptional<u32> for UnsetInstanceCountOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct InstanceCountOptionalValue(pub u32);
    impl IsOptional for InstanceCountOptionalValue {}
    impl ResolveOptional<u32> for InstanceCountOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetFirstIndexOptional;
    impl IsOptional for UnsetFirstIndexOptional {}
    impl IsUnsetOptional for UnsetFirstIndexOptional {}
    impl ResolveOptional<u32> for UnsetFirstIndexOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct FirstIndexOptionalValue(pub u32);
    impl IsOptional for FirstIndexOptionalValue {}
    impl ResolveOptional<u32> for FirstIndexOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetBaseVertexOptional;
    impl IsOptional for UnsetBaseVertexOptional {}
    impl IsUnsetOptional for UnsetBaseVertexOptional {}
    impl ResolveOptional<i32> for UnsetBaseVertexOptional {
        fn resolve(self) -> i32 {
            Default::default()
        }
    }
    pub struct BaseVertexOptionalValue(pub i32);
    impl IsOptional for BaseVertexOptionalValue {}
    impl ResolveOptional<i32> for BaseVertexOptionalValue {
        fn resolve(self) -> i32 {
            self.0
        }
    }
    pub struct UnsetFirstInstanceOptional;
    impl IsOptional for UnsetFirstInstanceOptional {}
    impl IsUnsetOptional for UnsetFirstInstanceOptional {}
    impl ResolveOptional<u32> for UnsetFirstInstanceOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct FirstInstanceOptionalValue(pub u32);
    impl IsOptional for FirstInstanceOptionalValue {}
    impl ResolveOptional<u32> for FirstInstanceOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional, T4: IsOptional>
        DrawIndexedIndirectArgsBuilder<T0, T1, T2, T3, T4>
    {
        pub fn index_count(
            self,
            index_count: u32,
        ) -> DrawIndexedIndirectArgsBuilder<IndexCountOptionalValue, T1, T2, T3, T4>
        where
            T0: IsUnsetOptional,
        {
            DrawIndexedIndirectArgsBuilder {
                index_count: IndexCountOptionalValue(index_count),
                instance_count: self.instance_count,
                first_index: self.first_index,
                base_vertex: self.base_vertex,
                first_instance: self.first_instance,
            }
        }
        pub fn instance_count(
            self,
            instance_count: u32,
        ) -> DrawIndexedIndirectArgsBuilder<T0, InstanceCountOptionalValue, T2, T3, T4>
        where
            T1: IsUnsetOptional,
        {
            DrawIndexedIndirectArgsBuilder {
                index_count: self.index_count,
                instance_count: InstanceCountOptionalValue(instance_count),
                first_index: self.first_index,
                base_vertex: self.base_vertex,
                first_instance: self.first_instance,
            }
        }
        pub fn first_index(
            self,
            first_index: u32,
        ) -> DrawIndexedIndirectArgsBuilder<T0, T1, FirstIndexOptionalValue, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            DrawIndexedIndirectArgsBuilder {
                index_count: self.index_count,
                instance_count: self.instance_count,
                first_index: FirstIndexOptionalValue(first_index),
                base_vertex: self.base_vertex,
                first_instance: self.first_instance,
            }
        }
        pub fn base_vertex(
            self,
            base_vertex: i32,
        ) -> DrawIndexedIndirectArgsBuilder<T0, T1, T2, BaseVertexOptionalValue, T4>
        where
            T3: IsUnsetOptional,
        {
            DrawIndexedIndirectArgsBuilder {
                index_count: self.index_count,
                instance_count: self.instance_count,
                first_index: self.first_index,
                base_vertex: BaseVertexOptionalValue(base_vertex),
                first_instance: self.first_instance,
            }
        }
        pub fn first_instance(
            self,
            first_instance: u32,
        ) -> DrawIndexedIndirectArgsBuilder<T0, T1, T2, T3, FirstInstanceOptionalValue>
        where
            T4: IsUnsetOptional,
        {
            DrawIndexedIndirectArgsBuilder {
                index_count: self.index_count,
                instance_count: self.instance_count,
                first_index: self.first_index,
                base_vertex: self.base_vertex,
                first_instance: FirstInstanceOptionalValue(first_instance),
            }
        }
    }
    impl<RIndexCount, RInstanceCount, RFirstIndex, RBaseVertex, RFirstInstance>
        DrawIndexedIndirectArgsBuilder<
            RIndexCount,
            RInstanceCount,
            RFirstIndex,
            RBaseVertex,
            RFirstInstance,
        >
    {
        pub fn build(self) -> wgpu::util::DrawIndexedIndirectArgs
        where
            RIndexCount: ResolveOptional<u32>,
            RInstanceCount: ResolveOptional<u32>,
            RFirstIndex: ResolveOptional<u32>,
            RBaseVertex: ResolveOptional<i32>,
            RFirstInstance: ResolveOptional<u32>,
        {
            wgpu::util::DrawIndexedIndirectArgs {
                index_count: self.index_count.resolve(),
                instance_count: self.instance_count.resolve(),
                first_index: self.first_index.resolve(),
                base_vertex: self.base_vertex.resolve(),
                first_instance: self.first_instance.resolve(),
            }
        }
    }
    impl<RIndexCount, RInstanceCount, RFirstIndex, RBaseVertex, RFirstInstance>
        Nested<wgpu::util::DrawIndexedIndirectArgs>
        for DrawIndexedIndirectArgsBuilder<
            RIndexCount,
            RInstanceCount,
            RFirstIndex,
            RBaseVertex,
            RFirstInstance,
        >
    where
        RIndexCount: ResolveOptional<u32>,
        RInstanceCount: ResolveOptional<u32>,
        RFirstIndex: ResolveOptional<u32>,
        RBaseVertex: ResolveOptional<i32>,
        RFirstInstance: ResolveOptional<u32>,
    {
        fn unnest(self) -> wgpu::util::DrawIndexedIndirectArgs {
            self.build()
        }
    }
    impl Nested<wgpu::util::DrawIndexedIndirectArgs> for wgpu::util::DrawIndexedIndirectArgs {
        fn unnest(self) -> wgpu::util::DrawIndexedIndirectArgs {
            self
        }
    }
}

pub use builder_render_bundle_descriptor::render_bundle_descriptor;
pub mod builder_render_bundle_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_bundle_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> RenderBundleDescriptorBuilder<LabelOptionalValue<'a>> {
        RenderBundleDescriptorBuilder::new().label(label)
    }
    pub struct RenderBundleDescriptorBuilder<T0> {
        label: T0,
    }
    impl RenderBundleDescriptorBuilder<UnsetLabelOptional> {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    impl<T0: IsOptional> RenderBundleDescriptorBuilder<T0> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> RenderBundleDescriptorBuilder<LabelOptionalValue<'a>>
        where
            T0: IsUnsetOptional,
        {
            RenderBundleDescriptorBuilder {
                label: LabelOptionalValue(label),
            }
        }
    }
    impl<RLabel> RenderBundleDescriptorBuilder<RLabel> {
        pub fn build<'a>(self) -> wgpu::RenderBundleDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::RenderBundleDescriptor {
                label: self.label.resolve(),
            }
        }
    }
}

pub use builder_vertex_state::vertex_state;
pub mod builder_vertex_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn vertex_state() -> VertexStateBuilder<
        UnsetModule,
        UnsetEntryPointOptional,
        UnsetCompilationOptionsOptional,
        UnsetBuffers,
    > {
        VertexStateBuilder::new()
    }
    pub struct VertexStateBuilder<T0, T1, T2, T3> {
        module: T0,
        entry_point: T1,
        compilation_options: T2,
        buffers: T3,
    }
    impl
        VertexStateBuilder<
            UnsetModule,
            UnsetEntryPointOptional,
            UnsetCompilationOptionsOptional,
            UnsetBuffers,
        >
    {
        pub fn new() -> Self {
            Self {
                module: UnsetModule,
                entry_point: UnsetEntryPointOptional,
                compilation_options: UnsetCompilationOptionsOptional,
                buffers: UnsetBuffers,
            }
        }
    }
    pub struct UnsetModule;
    impl IsRequired for UnsetModule {}
    impl IsUnset for UnsetModule {}
    pub struct ModuleValue<'a>(pub &'a wgpu::ShaderModule);
    impl<'a> IsRequired for ModuleValue<'a> {}
    pub struct UnsetEntryPointOptional;
    impl IsOptional for UnsetEntryPointOptional {}
    impl IsUnsetOptional for UnsetEntryPointOptional {}
    impl<'a> ResolveOptional<Option<&'a str>> for UnsetEntryPointOptional {
        fn resolve(self) -> Option<&'a str> {
            Default::default()
        }
    }
    pub struct EntryPointOptionalValue<'a>(pub Option<&'a str>);
    impl<'a> IsOptional for EntryPointOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a str>> for EntryPointOptionalValue<'a> {
        fn resolve(self) -> Option<&'a str> {
            self.0
        }
    }
    pub struct UnsetCompilationOptionsOptional;
    impl IsOptional for UnsetCompilationOptionsOptional {}
    impl IsUnsetOptional for UnsetCompilationOptionsOptional {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>> for UnsetCompilationOptionsOptional {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            Default::default()
        }
    }
    pub struct CompilationOptionsOptionalValue<'a>(pub wgpu::PipelineCompilationOptions<'a>);
    impl<'a> IsOptional for CompilationOptionsOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::PipelineCompilationOptions<'a>>
        for CompilationOptionsOptionalValue<'a>
    {
        fn resolve(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.0
        }
    }
    pub struct UnsetBuffers;
    impl IsRequired for UnsetBuffers {}
    impl IsUnset for UnsetBuffers {}
    pub struct BuffersValue<'a>(pub &'a [wgpu::VertexBufferLayout<'a>]);
    impl<'a> IsRequired for BuffersValue<'a> {}
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional, T3: IsRequired>
        VertexStateBuilder<T0, T1, T2, T3>
    {
        pub fn module<'a>(
            self,
            module: &'a wgpu::ShaderModule,
        ) -> VertexStateBuilder<ModuleValue<'a>, T1, T2, T3>
        where
            T0: IsUnset,
        {
            VertexStateBuilder {
                module: ModuleValue(module),
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                buffers: self.buffers,
            }
        }
        pub fn maybe_entry_point<'a>(
            self,
            entry_point: Option<&'a str>,
        ) -> VertexStateBuilder<T0, EntryPointOptionalValue<'a>, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            VertexStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(entry_point),
                compilation_options: self.compilation_options,
                buffers: self.buffers,
            }
        }
        pub fn entry_point<'a>(
            self,
            entry_point: &'a str,
        ) -> VertexStateBuilder<T0, EntryPointOptionalValue<'a>, T2, T3>
        where
            T1: IsUnsetOptional,
        {
            VertexStateBuilder {
                module: self.module,
                entry_point: EntryPointOptionalValue(Some(entry_point)),
                compilation_options: self.compilation_options,
                buffers: self.buffers,
            }
        }
        pub fn compilation_options<'a>(
            self,
            compilation_options: impl Nested<wgpu::PipelineCompilationOptions<'a>>,
        ) -> VertexStateBuilder<T0, T1, CompilationOptionsOptionalValue<'a>, T3>
        where
            T2: IsUnsetOptional,
        {
            VertexStateBuilder {
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: CompilationOptionsOptionalValue(compilation_options.unnest()),
                buffers: self.buffers,
            }
        }
        pub fn buffers<'a>(
            self,
            buffers: &'a [wgpu::VertexBufferLayout<'a>],
        ) -> VertexStateBuilder<T0, T1, T2, BuffersValue<'a>>
        where
            T3: IsUnset,
        {
            VertexStateBuilder {
                module: self.module,
                entry_point: self.entry_point,
                compilation_options: self.compilation_options,
                buffers: BuffersValue(buffers),
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions>
        VertexStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions, BuffersValue<'a>>
    {
        pub fn build(self) -> wgpu::VertexState<'a>
        where
            REntryPoint: ResolveOptional<Option<&'a str>>,
            RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
        {
            wgpu::VertexState {
                module: self.module.0,
                entry_point: self.entry_point.resolve(),
                compilation_options: self.compilation_options.resolve(),
                buffers: self.buffers.0,
            }
        }
    }
    impl<'a, REntryPoint, RCompilationOptions> Nested<wgpu::VertexState<'a>>
        for VertexStateBuilder<ModuleValue<'a>, REntryPoint, RCompilationOptions, BuffersValue<'a>>
    where
        REntryPoint: ResolveOptional<Option<&'a str>>,
        RCompilationOptions: ResolveOptional<wgpu::PipelineCompilationOptions<'a>>,
    {
        fn unnest(self) -> wgpu::VertexState<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::VertexState<'a>> for wgpu::VertexState<'a> {
        fn unnest(self) -> wgpu::VertexState<'a> {
            self
        }
    }
}

pub use builder_create_tlas_descriptor::create_tlas_descriptor;
pub mod builder_create_tlas_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn create_tlas_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> CreateTlasDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetMaxInstances,
        UnsetFlags,
        UnsetUpdateMode,
    > {
        CreateTlasDescriptorBuilder::new().label(label)
    }
    pub struct CreateTlasDescriptorBuilder<T0, T1, T2, T3> {
        label: T0,
        max_instances: T1,
        flags: T2,
        update_mode: T3,
    }
    impl
        CreateTlasDescriptorBuilder<
            UnsetLabelOptional,
            UnsetMaxInstances,
            UnsetFlags,
            UnsetUpdateMode,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                max_instances: UnsetMaxInstances,
                flags: UnsetFlags,
                update_mode: UnsetUpdateMode,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetMaxInstances;
    impl IsRequired for UnsetMaxInstances {}
    impl IsUnset for UnsetMaxInstances {}
    pub struct MaxInstancesValue(pub u32);
    impl IsRequired for MaxInstancesValue {}
    pub struct UnsetFlags;
    impl IsRequired for UnsetFlags {}
    impl IsUnset for UnsetFlags {}
    pub struct FlagsValue(pub wgpu::wgt::AccelerationStructureFlags);
    impl IsRequired for FlagsValue {}
    pub struct UnsetUpdateMode;
    impl IsRequired for UnsetUpdateMode {}
    impl IsUnset for UnsetUpdateMode {}
    pub struct UpdateModeValue(pub wgpu::wgt::AccelerationStructureUpdateMode);
    impl IsRequired for UpdateModeValue {}
    impl<T0: IsOptional, T1: IsRequired, T2: IsRequired, T3: IsRequired>
        CreateTlasDescriptorBuilder<T0, T1, T2, T3>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> CreateTlasDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3>
        where
            T0: IsUnsetOptional,
        {
            CreateTlasDescriptorBuilder {
                label: LabelOptionalValue(label),
                max_instances: self.max_instances,
                flags: self.flags,
                update_mode: self.update_mode,
            }
        }
        pub fn max_instances(
            self,
            max_instances: u32,
        ) -> CreateTlasDescriptorBuilder<T0, MaxInstancesValue, T2, T3>
        where
            T1: IsUnset,
        {
            CreateTlasDescriptorBuilder {
                label: self.label,
                max_instances: MaxInstancesValue(max_instances),
                flags: self.flags,
                update_mode: self.update_mode,
            }
        }
        pub fn flags(
            self,
            flags: wgpu::wgt::AccelerationStructureFlags,
        ) -> CreateTlasDescriptorBuilder<T0, T1, FlagsValue, T3>
        where
            T2: IsUnset,
        {
            CreateTlasDescriptorBuilder {
                label: self.label,
                max_instances: self.max_instances,
                flags: FlagsValue(flags),
                update_mode: self.update_mode,
            }
        }
        pub fn update_mode(
            self,
            update_mode: wgpu::wgt::AccelerationStructureUpdateMode,
        ) -> CreateTlasDescriptorBuilder<T0, T1, T2, UpdateModeValue>
        where
            T3: IsUnset,
        {
            CreateTlasDescriptorBuilder {
                label: self.label,
                max_instances: self.max_instances,
                flags: self.flags,
                update_mode: UpdateModeValue(update_mode),
            }
        }
    }
    impl<RLabel> CreateTlasDescriptorBuilder<RLabel, MaxInstancesValue, FlagsValue, UpdateModeValue> {
        pub fn build<'a>(self) -> wgpu::CreateTlasDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
        {
            wgpu::CreateTlasDescriptor {
                label: self.label.resolve(),
                max_instances: self.max_instances.0,
                flags: self.flags.0,
                update_mode: self.update_mode.0,
            }
        }
    }
}

pub use builder_copy_external_image_dest_info::copy_external_image_dest_info;
pub mod builder_copy_external_image_dest_info {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn copy_external_image_dest_info() -> CopyExternalImageDestInfoBuilder<
        UnsetTexture,
        UnsetMipLevel,
        UnsetOriginOptional,
        UnsetAspectOptional,
        UnsetColorSpace,
        UnsetPremultipliedAlpha,
    > {
        CopyExternalImageDestInfoBuilder::new()
    }
    pub struct CopyExternalImageDestInfoBuilder<T0, T1, T2, T3, T4, T5> {
        texture: T0,
        mip_level: T1,
        origin: T2,
        aspect: T3,
        color_space: T4,
        premultiplied_alpha: T5,
    }
    impl
        CopyExternalImageDestInfoBuilder<
            UnsetTexture,
            UnsetMipLevel,
            UnsetOriginOptional,
            UnsetAspectOptional,
            UnsetColorSpace,
            UnsetPremultipliedAlpha,
        >
    {
        pub fn new() -> Self {
            Self {
                texture: UnsetTexture,
                mip_level: UnsetMipLevel,
                origin: UnsetOriginOptional,
                aspect: UnsetAspectOptional,
                color_space: UnsetColorSpace,
                premultiplied_alpha: UnsetPremultipliedAlpha,
            }
        }
    }
    pub struct UnsetTexture;
    impl IsRequired for UnsetTexture {}
    impl IsUnset for UnsetTexture {}
    pub struct TextureValue<T>(pub T);
    impl<T> IsRequired for TextureValue<T> {}
    pub struct UnsetMipLevel;
    impl IsRequired for UnsetMipLevel {}
    impl IsUnset for UnsetMipLevel {}
    pub struct MipLevelValue(pub u32);
    impl IsRequired for MipLevelValue {}
    pub struct UnsetOriginOptional;
    impl IsOptional for UnsetOriginOptional {}
    impl IsUnsetOptional for UnsetOriginOptional {}
    impl ResolveOptional<wgpu::Origin3d> for UnsetOriginOptional {
        fn resolve(self) -> wgpu::Origin3d {
            Default::default()
        }
    }
    pub struct OriginOptionalValue(pub wgpu::Origin3d);
    impl IsOptional for OriginOptionalValue {}
    impl ResolveOptional<wgpu::Origin3d> for OriginOptionalValue {
        fn resolve(self) -> wgpu::Origin3d {
            self.0
        }
    }
    pub struct UnsetAspectOptional;
    impl IsOptional for UnsetAspectOptional {}
    impl IsUnsetOptional for UnsetAspectOptional {}
    impl ResolveOptional<wgpu::TextureAspect> for UnsetAspectOptional {
        fn resolve(self) -> wgpu::TextureAspect {
            Default::default()
        }
    }
    pub struct AspectOptionalValue(pub wgpu::TextureAspect);
    impl IsOptional for AspectOptionalValue {}
    impl ResolveOptional<wgpu::TextureAspect> for AspectOptionalValue {
        fn resolve(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    pub struct UnsetColorSpace;
    impl IsRequired for UnsetColorSpace {}
    impl IsUnset for UnsetColorSpace {}
    pub struct ColorSpaceValue(pub wgpu::PredefinedColorSpace);
    impl IsRequired for ColorSpaceValue {}
    pub struct UnsetPremultipliedAlpha;
    impl IsRequired for UnsetPremultipliedAlpha {}
    impl IsUnset for UnsetPremultipliedAlpha {}
    pub struct PremultipliedAlphaValue(pub bool);
    impl IsRequired for PremultipliedAlphaValue {}
    impl<
        T0: IsRequired,
        T1: IsRequired,
        T2: IsOptional,
        T3: IsOptional,
        T4: IsRequired,
        T5: IsRequired,
    > CopyExternalImageDestInfoBuilder<T0, T1, T2, T3, T4, T5>
    {
        pub fn texture<T>(
            self,
            texture: T,
        ) -> CopyExternalImageDestInfoBuilder<TextureValue<T>, T1, T2, T3, T4, T5>
        where
            T0: IsUnset,
        {
            CopyExternalImageDestInfoBuilder {
                texture: TextureValue(texture),
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: self.aspect,
                color_space: self.color_space,
                premultiplied_alpha: self.premultiplied_alpha,
            }
        }
        pub fn mip_level(
            self,
            mip_level: u32,
        ) -> CopyExternalImageDestInfoBuilder<T0, MipLevelValue, T2, T3, T4, T5>
        where
            T1: IsUnset,
        {
            CopyExternalImageDestInfoBuilder {
                texture: self.texture,
                mip_level: MipLevelValue(mip_level),
                origin: self.origin,
                aspect: self.aspect,
                color_space: self.color_space,
                premultiplied_alpha: self.premultiplied_alpha,
            }
        }
        pub fn origin(
            self,
            origin: impl Nested<wgpu::Origin3d>,
        ) -> CopyExternalImageDestInfoBuilder<T0, T1, OriginOptionalValue, T3, T4, T5>
        where
            T2: IsUnsetOptional,
        {
            CopyExternalImageDestInfoBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: OriginOptionalValue(origin.unnest()),
                aspect: self.aspect,
                color_space: self.color_space,
                premultiplied_alpha: self.premultiplied_alpha,
            }
        }
        pub fn aspect(
            self,
            aspect: wgpu::TextureAspect,
        ) -> CopyExternalImageDestInfoBuilder<T0, T1, T2, AspectOptionalValue, T4, T5>
        where
            T3: IsUnsetOptional,
        {
            CopyExternalImageDestInfoBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: AspectOptionalValue(aspect),
                color_space: self.color_space,
                premultiplied_alpha: self.premultiplied_alpha,
            }
        }
        pub fn color_space(
            self,
            color_space: wgpu::PredefinedColorSpace,
        ) -> CopyExternalImageDestInfoBuilder<T0, T1, T2, T3, ColorSpaceValue, T5>
        where
            T4: IsUnset,
        {
            CopyExternalImageDestInfoBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: self.aspect,
                color_space: ColorSpaceValue(color_space),
                premultiplied_alpha: self.premultiplied_alpha,
            }
        }
        pub fn premultiplied_alpha(
            self,
            premultiplied_alpha: bool,
        ) -> CopyExternalImageDestInfoBuilder<T0, T1, T2, T3, T4, PremultipliedAlphaValue>
        where
            T5: IsUnset,
        {
            CopyExternalImageDestInfoBuilder {
                texture: self.texture,
                mip_level: self.mip_level,
                origin: self.origin,
                aspect: self.aspect,
                color_space: self.color_space,
                premultiplied_alpha: PremultipliedAlphaValue(premultiplied_alpha),
            }
        }
    }
    impl<ROrigin, RAspect, T>
        CopyExternalImageDestInfoBuilder<
            TextureValue<T>,
            MipLevelValue,
            ROrigin,
            RAspect,
            ColorSpaceValue,
            PremultipliedAlphaValue,
        >
    {
        pub fn build(self) -> wgpu::CopyExternalImageDestInfo<T>
        where
            ROrigin: ResolveOptional<wgpu::Origin3d>,
            RAspect: ResolveOptional<wgpu::TextureAspect>,
        {
            wgpu::CopyExternalImageDestInfo {
                texture: self.texture.0,
                mip_level: self.mip_level.0,
                origin: self.origin.resolve(),
                aspect: self.aspect.resolve(),
                color_space: self.color_space.0,
                premultiplied_alpha: self.premultiplied_alpha.0,
            }
        }
    }
    impl<ROrigin, RAspect, T> Nested<wgpu::CopyExternalImageDestInfo<T>>
        for CopyExternalImageDestInfoBuilder<
            TextureValue<T>,
            MipLevelValue,
            ROrigin,
            RAspect,
            ColorSpaceValue,
            PremultipliedAlphaValue,
        >
    where
        ROrigin: ResolveOptional<wgpu::Origin3d>,
        RAspect: ResolveOptional<wgpu::TextureAspect>,
    {
        fn unnest(self) -> wgpu::CopyExternalImageDestInfo<T> {
            self.build()
        }
    }
    impl<T> Nested<wgpu::CopyExternalImageDestInfo<T>> for wgpu::CopyExternalImageDestInfo<T> {
        fn unnest(self) -> wgpu::CopyExternalImageDestInfo<T> {
            self
        }
    }
}

pub use builder_texture_view_descriptor::texture_view_descriptor;
pub mod builder_texture_view_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn texture_view_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> TextureViewDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetFormatOptional,
        UnsetDimensionOptional,
        UnsetUsageOptional,
        UnsetAspectOptional,
        UnsetBaseMipLevelOptional,
        UnsetMipLevelCountOptional,
        UnsetBaseArrayLayerOptional,
        UnsetArrayLayerCountOptional,
    > {
        TextureViewDescriptorBuilder::new().label(label)
    }
    pub struct TextureViewDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
        label: T0,
        format: T1,
        dimension: T2,
        usage: T3,
        aspect: T4,
        base_mip_level: T5,
        mip_level_count: T6,
        base_array_layer: T7,
        array_layer_count: T8,
    }
    impl
        TextureViewDescriptorBuilder<
            UnsetLabelOptional,
            UnsetFormatOptional,
            UnsetDimensionOptional,
            UnsetUsageOptional,
            UnsetAspectOptional,
            UnsetBaseMipLevelOptional,
            UnsetMipLevelCountOptional,
            UnsetBaseArrayLayerOptional,
            UnsetArrayLayerCountOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                format: UnsetFormatOptional,
                dimension: UnsetDimensionOptional,
                usage: UnsetUsageOptional,
                aspect: UnsetAspectOptional,
                base_mip_level: UnsetBaseMipLevelOptional,
                mip_level_count: UnsetMipLevelCountOptional,
                base_array_layer: UnsetBaseArrayLayerOptional,
                array_layer_count: UnsetArrayLayerCountOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetFormatOptional;
    impl IsOptional for UnsetFormatOptional {}
    impl IsUnsetOptional for UnsetFormatOptional {}
    impl ResolveOptional<Option<wgpu::TextureFormat>> for UnsetFormatOptional {
        fn resolve(self) -> Option<wgpu::TextureFormat> {
            Default::default()
        }
    }
    pub struct FormatOptionalValue(pub Option<wgpu::TextureFormat>);
    impl IsOptional for FormatOptionalValue {}
    impl ResolveOptional<Option<wgpu::TextureFormat>> for FormatOptionalValue {
        fn resolve(self) -> Option<wgpu::TextureFormat> {
            self.0
        }
    }
    pub struct UnsetDimensionOptional;
    impl IsOptional for UnsetDimensionOptional {}
    impl IsUnsetOptional for UnsetDimensionOptional {}
    impl ResolveOptional<Option<wgpu::TextureViewDimension>> for UnsetDimensionOptional {
        fn resolve(self) -> Option<wgpu::TextureViewDimension> {
            Default::default()
        }
    }
    pub struct DimensionOptionalValue(pub Option<wgpu::TextureViewDimension>);
    impl IsOptional for DimensionOptionalValue {}
    impl ResolveOptional<Option<wgpu::TextureViewDimension>> for DimensionOptionalValue {
        fn resolve(self) -> Option<wgpu::TextureViewDimension> {
            self.0
        }
    }
    pub struct UnsetUsageOptional;
    impl IsOptional for UnsetUsageOptional {}
    impl IsUnsetOptional for UnsetUsageOptional {}
    impl ResolveOptional<Option<wgpu::TextureUsages>> for UnsetUsageOptional {
        fn resolve(self) -> Option<wgpu::TextureUsages> {
            Default::default()
        }
    }
    pub struct UsageOptionalValue(pub Option<wgpu::TextureUsages>);
    impl IsOptional for UsageOptionalValue {}
    impl ResolveOptional<Option<wgpu::TextureUsages>> for UsageOptionalValue {
        fn resolve(self) -> Option<wgpu::TextureUsages> {
            self.0
        }
    }
    pub struct UnsetAspectOptional;
    impl IsOptional for UnsetAspectOptional {}
    impl IsUnsetOptional for UnsetAspectOptional {}
    impl ResolveOptional<wgpu::TextureAspect> for UnsetAspectOptional {
        fn resolve(self) -> wgpu::TextureAspect {
            Default::default()
        }
    }
    pub struct AspectOptionalValue(pub wgpu::TextureAspect);
    impl IsOptional for AspectOptionalValue {}
    impl ResolveOptional<wgpu::TextureAspect> for AspectOptionalValue {
        fn resolve(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    pub struct UnsetBaseMipLevelOptional;
    impl IsOptional for UnsetBaseMipLevelOptional {}
    impl IsUnsetOptional for UnsetBaseMipLevelOptional {}
    impl ResolveOptional<u32> for UnsetBaseMipLevelOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct BaseMipLevelOptionalValue(pub u32);
    impl IsOptional for BaseMipLevelOptionalValue {}
    impl ResolveOptional<u32> for BaseMipLevelOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetMipLevelCountOptional;
    impl IsOptional for UnsetMipLevelCountOptional {}
    impl IsUnsetOptional for UnsetMipLevelCountOptional {}
    impl ResolveOptional<Option<u32>> for UnsetMipLevelCountOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct MipLevelCountOptionalValue(pub Option<u32>);
    impl IsOptional for MipLevelCountOptionalValue {}
    impl ResolveOptional<Option<u32>> for MipLevelCountOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetBaseArrayLayerOptional;
    impl IsOptional for UnsetBaseArrayLayerOptional {}
    impl IsUnsetOptional for UnsetBaseArrayLayerOptional {}
    impl ResolveOptional<u32> for UnsetBaseArrayLayerOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct BaseArrayLayerOptionalValue(pub u32);
    impl IsOptional for BaseArrayLayerOptionalValue {}
    impl ResolveOptional<u32> for BaseArrayLayerOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetArrayLayerCountOptional;
    impl IsOptional for UnsetArrayLayerCountOptional {}
    impl IsUnsetOptional for UnsetArrayLayerCountOptional {}
    impl ResolveOptional<Option<u32>> for UnsetArrayLayerCountOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct ArrayLayerCountOptionalValue(pub Option<u32>);
    impl IsOptional for ArrayLayerCountOptionalValue {}
    impl ResolveOptional<Option<u32>> for ArrayLayerCountOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    impl<
        T0: IsOptional,
        T1: IsOptional,
        T2: IsOptional,
        T3: IsOptional,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
        T7: IsOptional,
        T8: IsOptional,
    > TextureViewDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, T7, T8>
    {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> TextureViewDescriptorBuilder<LabelOptionalValue<'a>, T1, T2, T3, T4, T5, T6, T7, T8>
        where
            T0: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: LabelOptionalValue(label),
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_format(
            self,
            format: Option<wgpu::TextureFormat>,
        ) -> TextureViewDescriptorBuilder<T0, FormatOptionalValue, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: FormatOptionalValue(format),
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn format(
            self,
            format: wgpu::TextureFormat,
        ) -> TextureViewDescriptorBuilder<T0, FormatOptionalValue, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: FormatOptionalValue(Some(format)),
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_dimension(
            self,
            dimension: Option<wgpu::TextureViewDimension>,
        ) -> TextureViewDescriptorBuilder<T0, T1, DimensionOptionalValue, T3, T4, T5, T6, T7, T8>
        where
            T2: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: DimensionOptionalValue(dimension),
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn dimension(
            self,
            dimension: wgpu::TextureViewDimension,
        ) -> TextureViewDescriptorBuilder<T0, T1, DimensionOptionalValue, T3, T4, T5, T6, T7, T8>
        where
            T2: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: DimensionOptionalValue(Some(dimension)),
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_usage(
            self,
            usage: Option<wgpu::TextureUsages>,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, UsageOptionalValue, T4, T5, T6, T7, T8>
        where
            T3: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: UsageOptionalValue(usage),
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn usage(
            self,
            usage: wgpu::TextureUsages,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, UsageOptionalValue, T4, T5, T6, T7, T8>
        where
            T3: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: UsageOptionalValue(Some(usage)),
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn aspect(
            self,
            aspect: wgpu::TextureAspect,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, T3, AspectOptionalValue, T5, T6, T7, T8>
        where
            T4: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: AspectOptionalValue(aspect),
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn base_mip_level(
            self,
            base_mip_level: u32,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, T3, T4, BaseMipLevelOptionalValue, T6, T7, T8>
        where
            T5: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: BaseMipLevelOptionalValue(base_mip_level),
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_mip_level_count(
            self,
            mip_level_count: Option<u32>,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, T3, T4, T5, MipLevelCountOptionalValue, T7, T8>
        where
            T6: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: MipLevelCountOptionalValue(mip_level_count),
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn mip_level_count(
            self,
            mip_level_count: u32,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, T3, T4, T5, MipLevelCountOptionalValue, T7, T8>
        where
            T6: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: MipLevelCountOptionalValue(Some(mip_level_count)),
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn base_array_layer(
            self,
            base_array_layer: u32,
        ) -> TextureViewDescriptorBuilder<T0, T1, T2, T3, T4, T5, T6, BaseArrayLayerOptionalValue, T8>
        where
            T7: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: BaseArrayLayerOptionalValue(base_array_layer),
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_array_layer_count(
            self,
            array_layer_count: Option<u32>,
        ) -> TextureViewDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            ArrayLayerCountOptionalValue,
        >
        where
            T8: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: ArrayLayerCountOptionalValue(array_layer_count),
            }
        }
        pub fn array_layer_count(
            self,
            array_layer_count: u32,
        ) -> TextureViewDescriptorBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            ArrayLayerCountOptionalValue,
        >
        where
            T8: IsUnsetOptional,
        {
            TextureViewDescriptorBuilder {
                label: self.label,
                format: self.format,
                dimension: self.dimension,
                usage: self.usage,
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: ArrayLayerCountOptionalValue(Some(array_layer_count)),
            }
        }
    }
    impl<
        RLabel,
        RFormat,
        RDimension,
        RUsage,
        RAspect,
        RBaseMipLevel,
        RMipLevelCount,
        RBaseArrayLayer,
        RArrayLayerCount,
    >
        TextureViewDescriptorBuilder<
            RLabel,
            RFormat,
            RDimension,
            RUsage,
            RAspect,
            RBaseMipLevel,
            RMipLevelCount,
            RBaseArrayLayer,
            RArrayLayerCount,
        >
    {
        pub fn build<'a>(self) -> wgpu::TextureViewDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RFormat: ResolveOptional<Option<wgpu::TextureFormat>>,
            RDimension: ResolveOptional<Option<wgpu::TextureViewDimension>>,
            RUsage: ResolveOptional<Option<wgpu::TextureUsages>>,
            RAspect: ResolveOptional<wgpu::TextureAspect>,
            RBaseMipLevel: ResolveOptional<u32>,
            RMipLevelCount: ResolveOptional<Option<u32>>,
            RBaseArrayLayer: ResolveOptional<u32>,
            RArrayLayerCount: ResolveOptional<Option<u32>>,
        {
            wgpu::TextureViewDescriptor {
                label: self.label.resolve(),
                format: self.format.resolve(),
                dimension: self.dimension.resolve(),
                usage: self.usage.resolve(),
                aspect: self.aspect.resolve(),
                base_mip_level: self.base_mip_level.resolve(),
                mip_level_count: self.mip_level_count.resolve(),
                base_array_layer: self.base_array_layer.resolve(),
                array_layer_count: self.array_layer_count.resolve(),
            }
        }
    }
}

pub use builder_depth_bias_state::depth_bias_state;
pub mod builder_depth_bias_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn depth_bias_state()
    -> DepthBiasStateBuilder<UnsetConstantOptional, UnsetSlopeScaleOptional, UnsetClampOptional>
    {
        DepthBiasStateBuilder::new()
    }
    pub struct DepthBiasStateBuilder<T0, T1, T2> {
        constant: T0,
        slope_scale: T1,
        clamp: T2,
    }
    impl DepthBiasStateBuilder<UnsetConstantOptional, UnsetSlopeScaleOptional, UnsetClampOptional> {
        pub fn new() -> Self {
            Self {
                constant: UnsetConstantOptional,
                slope_scale: UnsetSlopeScaleOptional,
                clamp: UnsetClampOptional,
            }
        }
    }
    pub struct UnsetConstantOptional;
    impl IsOptional for UnsetConstantOptional {}
    impl IsUnsetOptional for UnsetConstantOptional {}
    impl ResolveOptional<i32> for UnsetConstantOptional {
        fn resolve(self) -> i32 {
            Default::default()
        }
    }
    pub struct ConstantOptionalValue(pub i32);
    impl IsOptional for ConstantOptionalValue {}
    impl ResolveOptional<i32> for ConstantOptionalValue {
        fn resolve(self) -> i32 {
            self.0
        }
    }
    pub struct UnsetSlopeScaleOptional;
    impl IsOptional for UnsetSlopeScaleOptional {}
    impl IsUnsetOptional for UnsetSlopeScaleOptional {}
    impl ResolveOptional<f32> for UnsetSlopeScaleOptional {
        fn resolve(self) -> f32 {
            Default::default()
        }
    }
    pub struct SlopeScaleOptionalValue(pub f32);
    impl IsOptional for SlopeScaleOptionalValue {}
    impl ResolveOptional<f32> for SlopeScaleOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    pub struct UnsetClampOptional;
    impl IsOptional for UnsetClampOptional {}
    impl IsUnsetOptional for UnsetClampOptional {}
    impl ResolveOptional<f32> for UnsetClampOptional {
        fn resolve(self) -> f32 {
            Default::default()
        }
    }
    pub struct ClampOptionalValue(pub f32);
    impl IsOptional for ClampOptionalValue {}
    impl ResolveOptional<f32> for ClampOptionalValue {
        fn resolve(self) -> f32 {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> DepthBiasStateBuilder<T0, T1, T2> {
        pub fn constant(self, constant: i32) -> DepthBiasStateBuilder<ConstantOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            DepthBiasStateBuilder {
                constant: ConstantOptionalValue(constant),
                slope_scale: self.slope_scale,
                clamp: self.clamp,
            }
        }
        pub fn slope_scale(
            self,
            slope_scale: f32,
        ) -> DepthBiasStateBuilder<T0, SlopeScaleOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            DepthBiasStateBuilder {
                constant: self.constant,
                slope_scale: SlopeScaleOptionalValue(slope_scale),
                clamp: self.clamp,
            }
        }
        pub fn clamp(self, clamp: f32) -> DepthBiasStateBuilder<T0, T1, ClampOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            DepthBiasStateBuilder {
                constant: self.constant,
                slope_scale: self.slope_scale,
                clamp: ClampOptionalValue(clamp),
            }
        }
    }
    impl<RConstant, RSlopeScale, RClamp> DepthBiasStateBuilder<RConstant, RSlopeScale, RClamp> {
        pub fn build(self) -> wgpu::DepthBiasState
        where
            RConstant: ResolveOptional<i32>,
            RSlopeScale: ResolveOptional<f32>,
            RClamp: ResolveOptional<f32>,
        {
            wgpu::DepthBiasState {
                constant: self.constant.resolve(),
                slope_scale: self.slope_scale.resolve(),
                clamp: self.clamp.resolve(),
            }
        }
    }
    impl<RConstant, RSlopeScale, RClamp> Nested<wgpu::DepthBiasState>
        for DepthBiasStateBuilder<RConstant, RSlopeScale, RClamp>
    where
        RConstant: ResolveOptional<i32>,
        RSlopeScale: ResolveOptional<f32>,
        RClamp: ResolveOptional<f32>,
    {
        fn unnest(self) -> wgpu::DepthBiasState {
            self.build()
        }
    }
    impl Nested<wgpu::DepthBiasState> for wgpu::DepthBiasState {
        fn unnest(self) -> wgpu::DepthBiasState {
            self
        }
    }
}

pub use builder_color_target_state::color_target_state;
pub mod builder_color_target_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn color_target_state()
    -> ColorTargetStateBuilder<UnsetFormat, UnsetBlendOptional, UnsetWriteMaskOptional> {
        ColorTargetStateBuilder::new()
    }
    pub struct ColorTargetStateBuilder<T0, T1, T2> {
        format: T0,
        blend: T1,
        write_mask: T2,
    }
    impl ColorTargetStateBuilder<UnsetFormat, UnsetBlendOptional, UnsetWriteMaskOptional> {
        pub fn new() -> Self {
            Self {
                format: UnsetFormat,
                blend: UnsetBlendOptional,
                write_mask: UnsetWriteMaskOptional,
            }
        }
    }
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::TextureFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetBlendOptional;
    impl IsOptional for UnsetBlendOptional {}
    impl IsUnsetOptional for UnsetBlendOptional {}
    impl ResolveOptional<Option<wgpu::BlendState>> for UnsetBlendOptional {
        fn resolve(self) -> Option<wgpu::BlendState> {
            Default::default()
        }
    }
    pub struct BlendOptionalValue(pub Option<wgpu::BlendState>);
    impl IsOptional for BlendOptionalValue {}
    impl ResolveOptional<Option<wgpu::BlendState>> for BlendOptionalValue {
        fn resolve(self) -> Option<wgpu::BlendState> {
            self.0
        }
    }
    pub struct UnsetWriteMaskOptional;
    impl IsOptional for UnsetWriteMaskOptional {}
    impl IsUnsetOptional for UnsetWriteMaskOptional {}
    impl ResolveOptional<wgpu::ColorWrites> for UnsetWriteMaskOptional {
        fn resolve(self) -> wgpu::ColorWrites {
            Default::default()
        }
    }
    pub struct WriteMaskOptionalValue(pub wgpu::ColorWrites);
    impl IsOptional for WriteMaskOptionalValue {}
    impl ResolveOptional<wgpu::ColorWrites> for WriteMaskOptionalValue {
        fn resolve(self) -> wgpu::ColorWrites {
            self.0
        }
    }
    impl<T0: IsRequired, T1: IsOptional, T2: IsOptional> ColorTargetStateBuilder<T0, T1, T2> {
        pub fn format(
            self,
            format: wgpu::TextureFormat,
        ) -> ColorTargetStateBuilder<FormatValue, T1, T2>
        where
            T0: IsUnset,
        {
            ColorTargetStateBuilder {
                format: FormatValue(format),
                blend: self.blend,
                write_mask: self.write_mask,
            }
        }
        pub fn maybe_blend(
            self,
            blend: impl Nested<Option<wgpu::BlendState>>,
        ) -> ColorTargetStateBuilder<T0, BlendOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            ColorTargetStateBuilder {
                format: self.format,
                blend: BlendOptionalValue(blend.unnest()),
                write_mask: self.write_mask,
            }
        }
        pub fn blend(
            self,
            blend: impl Nested<wgpu::BlendState>,
        ) -> ColorTargetStateBuilder<T0, BlendOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            ColorTargetStateBuilder {
                format: self.format,
                blend: BlendOptionalValue(Some(blend.unnest())),
                write_mask: self.write_mask,
            }
        }
        pub fn write_mask(
            self,
            write_mask: wgpu::ColorWrites,
        ) -> ColorTargetStateBuilder<T0, T1, WriteMaskOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            ColorTargetStateBuilder {
                format: self.format,
                blend: self.blend,
                write_mask: WriteMaskOptionalValue(write_mask),
            }
        }
    }
    impl<RBlend, RWriteMask> ColorTargetStateBuilder<FormatValue, RBlend, RWriteMask> {
        pub fn build(self) -> wgpu::ColorTargetState
        where
            RBlend: ResolveOptional<Option<wgpu::BlendState>>,
            RWriteMask: ResolveOptional<wgpu::ColorWrites>,
        {
            wgpu::ColorTargetState {
                format: self.format.0,
                blend: self.blend.resolve(),
                write_mask: self.write_mask.resolve(),
            }
        }
    }
    impl<RBlend, RWriteMask> Nested<wgpu::ColorTargetState>
        for ColorTargetStateBuilder<FormatValue, RBlend, RWriteMask>
    where
        RBlend: ResolveOptional<Option<wgpu::BlendState>>,
        RWriteMask: ResolveOptional<wgpu::ColorWrites>,
    {
        fn unnest(self) -> wgpu::ColorTargetState {
            self.build()
        }
    }
    impl Nested<wgpu::ColorTargetState> for wgpu::ColorTargetState {
        fn unnest(self) -> wgpu::ColorTargetState {
            self
        }
    }
}

pub use builder_pipeline_layout_descriptor::pipeline_layout_descriptor;
pub mod builder_pipeline_layout_descriptor {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn pipeline_layout_descriptor<'a>(
        label: wgpu::Label<'a>,
    ) -> PipelineLayoutDescriptorBuilder<
        LabelOptionalValue<'a>,
        UnsetBindGroupLayoutsOptional,
        UnsetPushConstantRangesOptional,
    > {
        PipelineLayoutDescriptorBuilder::new().label(label)
    }
    pub struct PipelineLayoutDescriptorBuilder<T0, T1, T2> {
        label: T0,
        bind_group_layouts: T1,
        push_constant_ranges: T2,
    }
    impl
        PipelineLayoutDescriptorBuilder<
            UnsetLabelOptional,
            UnsetBindGroupLayoutsOptional,
            UnsetPushConstantRangesOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                label: UnsetLabelOptional,
                bind_group_layouts: UnsetBindGroupLayoutsOptional,
                push_constant_ranges: UnsetPushConstantRangesOptional,
            }
        }
    }
    pub struct UnsetLabelOptional;
    impl IsOptional for UnsetLabelOptional {}
    impl IsUnsetOptional for UnsetLabelOptional {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for UnsetLabelOptional {
        fn resolve(self) -> wgpu::Label<'a> {
            Default::default()
        }
    }
    pub struct LabelOptionalValue<'a>(pub wgpu::Label<'a>);
    impl<'a> IsOptional for LabelOptionalValue<'a> {}
    impl<'a> ResolveOptional<wgpu::Label<'a>> for LabelOptionalValue<'a> {
        fn resolve(self) -> wgpu::Label<'a> {
            self.0
        }
    }
    pub struct UnsetBindGroupLayoutsOptional;
    impl IsOptional for UnsetBindGroupLayoutsOptional {}
    impl IsUnsetOptional for UnsetBindGroupLayoutsOptional {}
    impl<'a> ResolveOptional<&'a [&'a wgpu::BindGroupLayout]> for UnsetBindGroupLayoutsOptional {
        fn resolve(self) -> &'a [&'a wgpu::BindGroupLayout] {
            Default::default()
        }
    }
    pub struct BindGroupLayoutsOptionalValue<'a>(pub &'a [&'a wgpu::BindGroupLayout]);
    impl<'a> IsOptional for BindGroupLayoutsOptionalValue<'a> {}
    impl<'a> ResolveOptional<&'a [&'a wgpu::BindGroupLayout]> for BindGroupLayoutsOptionalValue<'a> {
        fn resolve(self) -> &'a [&'a wgpu::BindGroupLayout] {
            self.0
        }
    }
    pub struct UnsetPushConstantRangesOptional;
    impl IsOptional for UnsetPushConstantRangesOptional {}
    impl IsUnsetOptional for UnsetPushConstantRangesOptional {}
    impl<'a> ResolveOptional<&'a [wgpu::PushConstantRange]> for UnsetPushConstantRangesOptional {
        fn resolve(self) -> &'a [wgpu::PushConstantRange] {
            Default::default()
        }
    }
    pub struct PushConstantRangesOptionalValue<'a>(pub &'a [wgpu::PushConstantRange]);
    impl<'a> IsOptional for PushConstantRangesOptionalValue<'a> {}
    impl<'a> ResolveOptional<&'a [wgpu::PushConstantRange]> for PushConstantRangesOptionalValue<'a> {
        fn resolve(self) -> &'a [wgpu::PushConstantRange] {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> PipelineLayoutDescriptorBuilder<T0, T1, T2> {
        pub fn label<'a>(
            self,
            label: wgpu::Label<'a>,
        ) -> PipelineLayoutDescriptorBuilder<LabelOptionalValue<'a>, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            PipelineLayoutDescriptorBuilder {
                label: LabelOptionalValue(label),
                bind_group_layouts: self.bind_group_layouts,
                push_constant_ranges: self.push_constant_ranges,
            }
        }
        pub fn bind_group_layouts<'a>(
            self,
            bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
        ) -> PipelineLayoutDescriptorBuilder<T0, BindGroupLayoutsOptionalValue<'a>, T2>
        where
            T1: IsUnsetOptional,
        {
            PipelineLayoutDescriptorBuilder {
                label: self.label,
                bind_group_layouts: BindGroupLayoutsOptionalValue(bind_group_layouts),
                push_constant_ranges: self.push_constant_ranges,
            }
        }
        pub fn push_constant_ranges<'a>(
            self,
            push_constant_ranges: &'a [wgpu::PushConstantRange],
        ) -> PipelineLayoutDescriptorBuilder<T0, T1, PushConstantRangesOptionalValue<'a>>
        where
            T2: IsUnsetOptional,
        {
            PipelineLayoutDescriptorBuilder {
                label: self.label,
                bind_group_layouts: self.bind_group_layouts,
                push_constant_ranges: PushConstantRangesOptionalValue(push_constant_ranges),
            }
        }
    }
    impl<RLabel, RBindGroupLayouts, RPushConstantRanges>
        PipelineLayoutDescriptorBuilder<RLabel, RBindGroupLayouts, RPushConstantRanges>
    {
        pub fn build<'a>(self) -> wgpu::PipelineLayoutDescriptor<'a>
        where
            RLabel: ResolveOptional<wgpu::Label<'a>>,
            RBindGroupLayouts: ResolveOptional<&'a [&'a wgpu::BindGroupLayout]>,
            RPushConstantRanges: ResolveOptional<&'a [wgpu::PushConstantRange]>,
        {
            wgpu::PipelineLayoutDescriptor {
                label: self.label.resolve(),
                bind_group_layouts: self.bind_group_layouts.resolve(),
                push_constant_ranges: self.push_constant_ranges.resolve(),
            }
        }
    }
    impl<'a, RLabel, RBindGroupLayouts, RPushConstantRanges>
        Nested<wgpu::PipelineLayoutDescriptor<'a>>
        for PipelineLayoutDescriptorBuilder<RLabel, RBindGroupLayouts, RPushConstantRanges>
    where
        RLabel: ResolveOptional<wgpu::Label<'a>>,
        RBindGroupLayouts: ResolveOptional<&'a [&'a wgpu::BindGroupLayout]>,
        RPushConstantRanges: ResolveOptional<&'a [wgpu::PushConstantRange]>,
    {
        fn unnest(self) -> wgpu::PipelineLayoutDescriptor<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::PipelineLayoutDescriptor<'a>> for wgpu::PipelineLayoutDescriptor<'a> {
        fn unnest(self) -> wgpu::PipelineLayoutDescriptor<'a> {
            self
        }
    }
}

pub use builder_push_constant_range::push_constant_range;
pub mod builder_push_constant_range {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn push_constant_range() -> PushConstantRangeBuilder<UnsetStages, UnsetRange> {
        PushConstantRangeBuilder::new()
    }
    pub struct PushConstantRangeBuilder<T0, T1> {
        stages: T0,
        range: T1,
    }
    impl PushConstantRangeBuilder<UnsetStages, UnsetRange> {
        pub fn new() -> Self {
            Self {
                stages: UnsetStages,
                range: UnsetRange,
            }
        }
    }
    pub struct UnsetStages;
    impl IsRequired for UnsetStages {}
    impl IsUnset for UnsetStages {}
    pub struct StagesValue(pub wgpu::ShaderStages);
    impl IsRequired for StagesValue {}
    pub struct UnsetRange;
    impl IsRequired for UnsetRange {}
    impl IsUnset for UnsetRange {}
    pub struct RangeValue(pub Range<u32>);
    impl IsRequired for RangeValue {}
    impl<T0: IsRequired, T1: IsRequired> PushConstantRangeBuilder<T0, T1> {
        pub fn stages(self, stages: wgpu::ShaderStages) -> PushConstantRangeBuilder<StagesValue, T1>
        where
            T0: IsUnset,
        {
            PushConstantRangeBuilder {
                stages: StagesValue(stages),
                range: self.range,
            }
        }
        pub fn range(self, range: Range<u32>) -> PushConstantRangeBuilder<T0, RangeValue>
        where
            T1: IsUnset,
        {
            PushConstantRangeBuilder {
                stages: self.stages,
                range: RangeValue(range),
            }
        }
    }
    impl PushConstantRangeBuilder<StagesValue, RangeValue> {
        pub fn build(self) -> wgpu::PushConstantRange where {
            wgpu::PushConstantRange {
                stages: self.stages.0,
                range: self.range.0,
            }
        }
    }
    impl Nested<wgpu::PushConstantRange> for PushConstantRangeBuilder<StagesValue, RangeValue> {
        fn unnest(self) -> wgpu::PushConstantRange {
            self.build()
        }
    }
    impl Nested<wgpu::PushConstantRange> for wgpu::PushConstantRange {
        fn unnest(self) -> wgpu::PushConstantRange {
            self
        }
    }
}

pub use builder_blas_triangle_geometry::blas_triangle_geometry;
pub mod builder_blas_triangle_geometry {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn blas_triangle_geometry() -> BlasTriangleGeometryBuilder<
        UnsetSize,
        UnsetVertexBuffer,
        UnsetFirstVertex,
        UnsetVertexStride,
        UnsetIndexBufferOptional,
        UnsetFirstIndexOptional,
        UnsetTransformBufferOptional,
        UnsetTransformBufferOffsetOptional,
    > {
        BlasTriangleGeometryBuilder::new()
    }
    pub struct BlasTriangleGeometryBuilder<T0, T1, T2, T3, T4, T5, T6, T7> {
        size: T0,
        vertex_buffer: T1,
        first_vertex: T2,
        vertex_stride: T3,
        index_buffer: T4,
        first_index: T5,
        transform_buffer: T6,
        transform_buffer_offset: T7,
    }
    impl
        BlasTriangleGeometryBuilder<
            UnsetSize,
            UnsetVertexBuffer,
            UnsetFirstVertex,
            UnsetVertexStride,
            UnsetIndexBufferOptional,
            UnsetFirstIndexOptional,
            UnsetTransformBufferOptional,
            UnsetTransformBufferOffsetOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                size: UnsetSize,
                vertex_buffer: UnsetVertexBuffer,
                first_vertex: UnsetFirstVertex,
                vertex_stride: UnsetVertexStride,
                index_buffer: UnsetIndexBufferOptional,
                first_index: UnsetFirstIndexOptional,
                transform_buffer: UnsetTransformBufferOptional,
                transform_buffer_offset: UnsetTransformBufferOffsetOptional,
            }
        }
    }
    pub struct UnsetSize;
    impl IsRequired for UnsetSize {}
    impl IsUnset for UnsetSize {}
    pub struct SizeValue<'a>(pub &'a wgpu::BlasTriangleGeometrySizeDescriptor);
    impl<'a> IsRequired for SizeValue<'a> {}
    pub struct UnsetVertexBuffer;
    impl IsRequired for UnsetVertexBuffer {}
    impl IsUnset for UnsetVertexBuffer {}
    pub struct VertexBufferValue<'a>(pub &'a wgpu::Buffer);
    impl<'a> IsRequired for VertexBufferValue<'a> {}
    pub struct UnsetFirstVertex;
    impl IsRequired for UnsetFirstVertex {}
    impl IsUnset for UnsetFirstVertex {}
    pub struct FirstVertexValue(pub u32);
    impl IsRequired for FirstVertexValue {}
    pub struct UnsetVertexStride;
    impl IsRequired for UnsetVertexStride {}
    impl IsUnset for UnsetVertexStride {}
    pub struct VertexStrideValue(pub wgpu::BufferAddress);
    impl IsRequired for VertexStrideValue {}
    pub struct UnsetIndexBufferOptional;
    impl IsOptional for UnsetIndexBufferOptional {}
    impl IsUnsetOptional for UnsetIndexBufferOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::Buffer>> for UnsetIndexBufferOptional {
        fn resolve(self) -> Option<&'a wgpu::Buffer> {
            Default::default()
        }
    }
    pub struct IndexBufferOptionalValue<'a>(pub Option<&'a wgpu::Buffer>);
    impl<'a> IsOptional for IndexBufferOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::Buffer>> for IndexBufferOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::Buffer> {
            self.0
        }
    }
    pub struct UnsetFirstIndexOptional;
    impl IsOptional for UnsetFirstIndexOptional {}
    impl IsUnsetOptional for UnsetFirstIndexOptional {}
    impl ResolveOptional<Option<u32>> for UnsetFirstIndexOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct FirstIndexOptionalValue(pub Option<u32>);
    impl IsOptional for FirstIndexOptionalValue {}
    impl ResolveOptional<Option<u32>> for FirstIndexOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetTransformBufferOptional;
    impl IsOptional for UnsetTransformBufferOptional {}
    impl IsUnsetOptional for UnsetTransformBufferOptional {}
    impl<'a> ResolveOptional<Option<&'a wgpu::Buffer>> for UnsetTransformBufferOptional {
        fn resolve(self) -> Option<&'a wgpu::Buffer> {
            Default::default()
        }
    }
    pub struct TransformBufferOptionalValue<'a>(pub Option<&'a wgpu::Buffer>);
    impl<'a> IsOptional for TransformBufferOptionalValue<'a> {}
    impl<'a> ResolveOptional<Option<&'a wgpu::Buffer>> for TransformBufferOptionalValue<'a> {
        fn resolve(self) -> Option<&'a wgpu::Buffer> {
            self.0
        }
    }
    pub struct UnsetTransformBufferOffsetOptional;
    impl IsOptional for UnsetTransformBufferOffsetOptional {}
    impl IsUnsetOptional for UnsetTransformBufferOffsetOptional {}
    impl ResolveOptional<Option<wgpu::BufferAddress>> for UnsetTransformBufferOffsetOptional {
        fn resolve(self) -> Option<wgpu::BufferAddress> {
            Default::default()
        }
    }
    pub struct TransformBufferOffsetOptionalValue(pub Option<wgpu::BufferAddress>);
    impl IsOptional for TransformBufferOffsetOptionalValue {}
    impl ResolveOptional<Option<wgpu::BufferAddress>> for TransformBufferOffsetOptionalValue {
        fn resolve(self) -> Option<wgpu::BufferAddress> {
            self.0
        }
    }
    impl<
        T0: IsRequired,
        T1: IsRequired,
        T2: IsRequired,
        T3: IsRequired,
        T4: IsOptional,
        T5: IsOptional,
        T6: IsOptional,
        T7: IsOptional,
    > BlasTriangleGeometryBuilder<T0, T1, T2, T3, T4, T5, T6, T7>
    {
        pub fn size<'a>(
            self,
            size: &'a wgpu::BlasTriangleGeometrySizeDescriptor,
        ) -> BlasTriangleGeometryBuilder<SizeValue<'a>, T1, T2, T3, T4, T5, T6, T7>
        where
            T0: IsUnset,
        {
            BlasTriangleGeometryBuilder {
                size: SizeValue(size),
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn vertex_buffer<'a>(
            self,
            vertex_buffer: &'a wgpu::Buffer,
        ) -> BlasTriangleGeometryBuilder<T0, VertexBufferValue<'a>, T2, T3, T4, T5, T6, T7>
        where
            T1: IsUnset,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: VertexBufferValue(vertex_buffer),
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn first_vertex(
            self,
            first_vertex: u32,
        ) -> BlasTriangleGeometryBuilder<T0, T1, FirstVertexValue, T3, T4, T5, T6, T7>
        where
            T2: IsUnset,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: FirstVertexValue(first_vertex),
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn vertex_stride(
            self,
            vertex_stride: wgpu::BufferAddress,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, VertexStrideValue, T4, T5, T6, T7>
        where
            T3: IsUnset,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: VertexStrideValue(vertex_stride),
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn maybe_index_buffer<'a>(
            self,
            index_buffer: Option<&'a wgpu::Buffer>,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, T3, IndexBufferOptionalValue<'a>, T5, T6, T7>
        where
            T4: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: IndexBufferOptionalValue(index_buffer),
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn index_buffer<'a>(
            self,
            index_buffer: &'a wgpu::Buffer,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, T3, IndexBufferOptionalValue<'a>, T5, T6, T7>
        where
            T4: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: IndexBufferOptionalValue(Some(index_buffer)),
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn maybe_first_index(
            self,
            first_index: Option<u32>,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, T3, T4, FirstIndexOptionalValue, T6, T7>
        where
            T5: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: FirstIndexOptionalValue(first_index),
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn first_index(
            self,
            first_index: u32,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, T3, T4, FirstIndexOptionalValue, T6, T7>
        where
            T5: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: FirstIndexOptionalValue(Some(first_index)),
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn maybe_transform_buffer<'a>(
            self,
            transform_buffer: Option<&'a wgpu::Buffer>,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, T3, T4, T5, TransformBufferOptionalValue<'a>, T7>
        where
            T6: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: TransformBufferOptionalValue(transform_buffer),
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn transform_buffer<'a>(
            self,
            transform_buffer: &'a wgpu::Buffer,
        ) -> BlasTriangleGeometryBuilder<T0, T1, T2, T3, T4, T5, TransformBufferOptionalValue<'a>, T7>
        where
            T6: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: TransformBufferOptionalValue(Some(transform_buffer)),
                transform_buffer_offset: self.transform_buffer_offset,
            }
        }
        pub fn maybe_transform_buffer_offset(
            self,
            transform_buffer_offset: Option<wgpu::BufferAddress>,
        ) -> BlasTriangleGeometryBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            TransformBufferOffsetOptionalValue,
        >
        where
            T7: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: TransformBufferOffsetOptionalValue(
                    transform_buffer_offset,
                ),
            }
        }
        pub fn transform_buffer_offset(
            self,
            transform_buffer_offset: wgpu::BufferAddress,
        ) -> BlasTriangleGeometryBuilder<
            T0,
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            TransformBufferOffsetOptionalValue,
        >
        where
            T7: IsUnsetOptional,
        {
            BlasTriangleGeometryBuilder {
                size: self.size,
                vertex_buffer: self.vertex_buffer,
                first_vertex: self.first_vertex,
                vertex_stride: self.vertex_stride,
                index_buffer: self.index_buffer,
                first_index: self.first_index,
                transform_buffer: self.transform_buffer,
                transform_buffer_offset: TransformBufferOffsetOptionalValue(Some(
                    transform_buffer_offset,
                )),
            }
        }
    }
    impl<'a, RIndexBuffer, RFirstIndex, RTransformBuffer, RTransformBufferOffset>
        BlasTriangleGeometryBuilder<
            SizeValue<'a>,
            VertexBufferValue<'a>,
            FirstVertexValue,
            VertexStrideValue,
            RIndexBuffer,
            RFirstIndex,
            RTransformBuffer,
            RTransformBufferOffset,
        >
    {
        pub fn build(self) -> wgpu::BlasTriangleGeometry<'a>
        where
            RIndexBuffer: ResolveOptional<Option<&'a wgpu::Buffer>>,
            RFirstIndex: ResolveOptional<Option<u32>>,
            RTransformBuffer: ResolveOptional<Option<&'a wgpu::Buffer>>,
            RTransformBufferOffset: ResolveOptional<Option<wgpu::BufferAddress>>,
        {
            wgpu::BlasTriangleGeometry {
                size: self.size.0,
                vertex_buffer: self.vertex_buffer.0,
                first_vertex: self.first_vertex.0,
                vertex_stride: self.vertex_stride.0,
                index_buffer: self.index_buffer.resolve(),
                first_index: self.first_index.resolve(),
                transform_buffer: self.transform_buffer.resolve(),
                transform_buffer_offset: self.transform_buffer_offset.resolve(),
            }
        }
    }
    impl<'a, RIndexBuffer, RFirstIndex, RTransformBuffer, RTransformBufferOffset>
        Nested<wgpu::BlasTriangleGeometry<'a>>
        for BlasTriangleGeometryBuilder<
            SizeValue<'a>,
            VertexBufferValue<'a>,
            FirstVertexValue,
            VertexStrideValue,
            RIndexBuffer,
            RFirstIndex,
            RTransformBuffer,
            RTransformBufferOffset,
        >
    where
        RIndexBuffer: ResolveOptional<Option<&'a wgpu::Buffer>>,
        RFirstIndex: ResolveOptional<Option<u32>>,
        RTransformBuffer: ResolveOptional<Option<&'a wgpu::Buffer>>,
        RTransformBufferOffset: ResolveOptional<Option<wgpu::BufferAddress>>,
    {
        fn unnest(self) -> wgpu::BlasTriangleGeometry<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::BlasTriangleGeometry<'a>> for wgpu::BlasTriangleGeometry<'a> {
        fn unnest(self) -> wgpu::BlasTriangleGeometry<'a> {
            self
        }
    }
}

pub use builder_render_bundle_depth_stencil::render_bundle_depth_stencil;
pub mod builder_render_bundle_depth_stencil {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn render_bundle_depth_stencil()
    -> RenderBundleDepthStencilBuilder<UnsetFormat, UnsetDepthReadOnly, UnsetStencilReadOnly> {
        RenderBundleDepthStencilBuilder::new()
    }
    pub struct RenderBundleDepthStencilBuilder<T0, T1, T2> {
        format: T0,
        depth_read_only: T1,
        stencil_read_only: T2,
    }
    impl RenderBundleDepthStencilBuilder<UnsetFormat, UnsetDepthReadOnly, UnsetStencilReadOnly> {
        pub fn new() -> Self {
            Self {
                format: UnsetFormat,
                depth_read_only: UnsetDepthReadOnly,
                stencil_read_only: UnsetStencilReadOnly,
            }
        }
    }
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::TextureFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetDepthReadOnly;
    impl IsRequired for UnsetDepthReadOnly {}
    impl IsUnset for UnsetDepthReadOnly {}
    pub struct DepthReadOnlyValue(pub bool);
    impl IsRequired for DepthReadOnlyValue {}
    pub struct UnsetStencilReadOnly;
    impl IsRequired for UnsetStencilReadOnly {}
    impl IsUnset for UnsetStencilReadOnly {}
    pub struct StencilReadOnlyValue(pub bool);
    impl IsRequired for StencilReadOnlyValue {}
    impl<T0: IsRequired, T1: IsRequired, T2: IsRequired> RenderBundleDepthStencilBuilder<T0, T1, T2> {
        pub fn format(
            self,
            format: wgpu::TextureFormat,
        ) -> RenderBundleDepthStencilBuilder<FormatValue, T1, T2>
        where
            T0: IsUnset,
        {
            RenderBundleDepthStencilBuilder {
                format: FormatValue(format),
                depth_read_only: self.depth_read_only,
                stencil_read_only: self.stencil_read_only,
            }
        }
        pub fn depth_read_only(
            self,
            depth_read_only: bool,
        ) -> RenderBundleDepthStencilBuilder<T0, DepthReadOnlyValue, T2>
        where
            T1: IsUnset,
        {
            RenderBundleDepthStencilBuilder {
                format: self.format,
                depth_read_only: DepthReadOnlyValue(depth_read_only),
                stencil_read_only: self.stencil_read_only,
            }
        }
        pub fn stencil_read_only(
            self,
            stencil_read_only: bool,
        ) -> RenderBundleDepthStencilBuilder<T0, T1, StencilReadOnlyValue>
        where
            T2: IsUnset,
        {
            RenderBundleDepthStencilBuilder {
                format: self.format,
                depth_read_only: self.depth_read_only,
                stencil_read_only: StencilReadOnlyValue(stencil_read_only),
            }
        }
    }
    impl RenderBundleDepthStencilBuilder<FormatValue, DepthReadOnlyValue, StencilReadOnlyValue> {
        pub fn build(self) -> wgpu::RenderBundleDepthStencil where {
            wgpu::RenderBundleDepthStencil {
                format: self.format.0,
                depth_read_only: self.depth_read_only.0,
                stencil_read_only: self.stencil_read_only.0,
            }
        }
    }
    impl Nested<wgpu::RenderBundleDepthStencil>
        for RenderBundleDepthStencilBuilder<FormatValue, DepthReadOnlyValue, StencilReadOnlyValue>
    {
        fn unnest(self) -> wgpu::RenderBundleDepthStencil {
            self.build()
        }
    }
    impl Nested<wgpu::RenderBundleDepthStencil> for wgpu::RenderBundleDepthStencil {
        fn unnest(self) -> wgpu::RenderBundleDepthStencil {
            self
        }
    }
}

pub use builder_gl_backend_options::gl_backend_options;
pub mod builder_gl_backend_options {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn gl_backend_options()
    -> GlBackendOptionsBuilder<UnsetGlesMinorVersionOptional, UnsetFenceBehaviorOptional> {
        GlBackendOptionsBuilder::new()
    }
    pub struct GlBackendOptionsBuilder<T0, T1> {
        gles_minor_version: T0,
        fence_behavior: T1,
    }
    impl GlBackendOptionsBuilder<UnsetGlesMinorVersionOptional, UnsetFenceBehaviorOptional> {
        pub fn new() -> Self {
            Self {
                gles_minor_version: UnsetGlesMinorVersionOptional,
                fence_behavior: UnsetFenceBehaviorOptional,
            }
        }
    }
    pub struct UnsetGlesMinorVersionOptional;
    impl IsOptional for UnsetGlesMinorVersionOptional {}
    impl IsUnsetOptional for UnsetGlesMinorVersionOptional {}
    impl ResolveOptional<wgpu::Gles3MinorVersion> for UnsetGlesMinorVersionOptional {
        fn resolve(self) -> wgpu::Gles3MinorVersion {
            Default::default()
        }
    }
    pub struct GlesMinorVersionOptionalValue(pub wgpu::Gles3MinorVersion);
    impl IsOptional for GlesMinorVersionOptionalValue {}
    impl ResolveOptional<wgpu::Gles3MinorVersion> for GlesMinorVersionOptionalValue {
        fn resolve(self) -> wgpu::Gles3MinorVersion {
            self.0
        }
    }
    pub struct UnsetFenceBehaviorOptional;
    impl IsOptional for UnsetFenceBehaviorOptional {}
    impl IsUnsetOptional for UnsetFenceBehaviorOptional {}
    impl ResolveOptional<wgpu::GlFenceBehavior> for UnsetFenceBehaviorOptional {
        fn resolve(self) -> wgpu::GlFenceBehavior {
            Default::default()
        }
    }
    pub struct FenceBehaviorOptionalValue(pub wgpu::GlFenceBehavior);
    impl IsOptional for FenceBehaviorOptionalValue {}
    impl ResolveOptional<wgpu::GlFenceBehavior> for FenceBehaviorOptionalValue {
        fn resolve(self) -> wgpu::GlFenceBehavior {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional> GlBackendOptionsBuilder<T0, T1> {
        pub fn gles_minor_version(
            self,
            gles_minor_version: wgpu::Gles3MinorVersion,
        ) -> GlBackendOptionsBuilder<GlesMinorVersionOptionalValue, T1>
        where
            T0: IsUnsetOptional,
        {
            GlBackendOptionsBuilder {
                gles_minor_version: GlesMinorVersionOptionalValue(gles_minor_version),
                fence_behavior: self.fence_behavior,
            }
        }
        pub fn fence_behavior(
            self,
            fence_behavior: wgpu::GlFenceBehavior,
        ) -> GlBackendOptionsBuilder<T0, FenceBehaviorOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            GlBackendOptionsBuilder {
                gles_minor_version: self.gles_minor_version,
                fence_behavior: FenceBehaviorOptionalValue(fence_behavior),
            }
        }
    }
    impl<RGlesMinorVersion, RFenceBehavior> GlBackendOptionsBuilder<RGlesMinorVersion, RFenceBehavior> {
        pub fn build(self) -> wgpu::GlBackendOptions
        where
            RGlesMinorVersion: ResolveOptional<wgpu::Gles3MinorVersion>,
            RFenceBehavior: ResolveOptional<wgpu::GlFenceBehavior>,
        {
            wgpu::GlBackendOptions {
                gles_minor_version: self.gles_minor_version.resolve(),
                fence_behavior: self.fence_behavior.resolve(),
            }
        }
    }
    impl<RGlesMinorVersion, RFenceBehavior> Nested<wgpu::GlBackendOptions>
        for GlBackendOptionsBuilder<RGlesMinorVersion, RFenceBehavior>
    where
        RGlesMinorVersion: ResolveOptional<wgpu::Gles3MinorVersion>,
        RFenceBehavior: ResolveOptional<wgpu::GlFenceBehavior>,
    {
        fn unnest(self) -> wgpu::GlBackendOptions {
            self.build()
        }
    }
    impl Nested<wgpu::GlBackendOptions> for wgpu::GlBackendOptions {
        fn unnest(self) -> wgpu::GlBackendOptions {
            self
        }
    }
}

pub use builder_vertex_attribute::vertex_attribute;
pub mod builder_vertex_attribute {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn vertex_attribute()
    -> VertexAttributeBuilder<UnsetFormat, UnsetOffset, UnsetShaderLocation> {
        VertexAttributeBuilder::new()
    }
    pub struct VertexAttributeBuilder<T0, T1, T2> {
        format: T0,
        offset: T1,
        shader_location: T2,
    }
    impl VertexAttributeBuilder<UnsetFormat, UnsetOffset, UnsetShaderLocation> {
        pub fn new() -> Self {
            Self {
                format: UnsetFormat,
                offset: UnsetOffset,
                shader_location: UnsetShaderLocation,
            }
        }
    }
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::VertexFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetOffset;
    impl IsRequired for UnsetOffset {}
    impl IsUnset for UnsetOffset {}
    pub struct OffsetValue(pub wgpu::BufferAddress);
    impl IsRequired for OffsetValue {}
    pub struct UnsetShaderLocation;
    impl IsRequired for UnsetShaderLocation {}
    impl IsUnset for UnsetShaderLocation {}
    pub struct ShaderLocationValue(pub wgpu::ShaderLocation);
    impl IsRequired for ShaderLocationValue {}
    impl<T0: IsRequired, T1: IsRequired, T2: IsRequired> VertexAttributeBuilder<T0, T1, T2> {
        pub fn format(
            self,
            format: wgpu::VertexFormat,
        ) -> VertexAttributeBuilder<FormatValue, T1, T2>
        where
            T0: IsUnset,
        {
            VertexAttributeBuilder {
                format: FormatValue(format),
                offset: self.offset,
                shader_location: self.shader_location,
            }
        }
        pub fn offset(
            self,
            offset: wgpu::BufferAddress,
        ) -> VertexAttributeBuilder<T0, OffsetValue, T2>
        where
            T1: IsUnset,
        {
            VertexAttributeBuilder {
                format: self.format,
                offset: OffsetValue(offset),
                shader_location: self.shader_location,
            }
        }
        pub fn shader_location(
            self,
            shader_location: wgpu::ShaderLocation,
        ) -> VertexAttributeBuilder<T0, T1, ShaderLocationValue>
        where
            T2: IsUnset,
        {
            VertexAttributeBuilder {
                format: self.format,
                offset: self.offset,
                shader_location: ShaderLocationValue(shader_location),
            }
        }
    }
    impl VertexAttributeBuilder<FormatValue, OffsetValue, ShaderLocationValue> {
        pub fn build(self) -> wgpu::VertexAttribute where {
            wgpu::VertexAttribute {
                format: self.format.0,
                offset: self.offset.0,
                shader_location: self.shader_location.0,
            }
        }
    }
    impl Nested<wgpu::VertexAttribute>
        for VertexAttributeBuilder<FormatValue, OffsetValue, ShaderLocationValue>
    {
        fn unnest(self) -> wgpu::VertexAttribute {
            self.build()
        }
    }
    impl Nested<wgpu::VertexAttribute> for wgpu::VertexAttribute {
        fn unnest(self) -> wgpu::VertexAttribute {
            self
        }
    }
}

pub use builder_surface_configuration::surface_configuration;
pub mod builder_surface_configuration {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn surface_configuration() -> SurfaceConfigurationBuilder<
        UnsetUsage,
        UnsetFormat,
        UnsetWidth,
        UnsetHeight,
        UnsetPresentModeOptional,
        UnsetDesiredMaximumFrameLatency,
        UnsetAlphaModeOptional,
        UnsetViewFormats,
    > {
        SurfaceConfigurationBuilder::new()
    }
    pub struct SurfaceConfigurationBuilder<T0, T1, T2, T3, T4, T5, T6, T7> {
        usage: T0,
        format: T1,
        width: T2,
        height: T3,
        present_mode: T4,
        desired_maximum_frame_latency: T5,
        alpha_mode: T6,
        view_formats: T7,
    }
    impl
        SurfaceConfigurationBuilder<
            UnsetUsage,
            UnsetFormat,
            UnsetWidth,
            UnsetHeight,
            UnsetPresentModeOptional,
            UnsetDesiredMaximumFrameLatency,
            UnsetAlphaModeOptional,
            UnsetViewFormats,
        >
    {
        pub fn new() -> Self {
            Self {
                usage: UnsetUsage,
                format: UnsetFormat,
                width: UnsetWidth,
                height: UnsetHeight,
                present_mode: UnsetPresentModeOptional,
                desired_maximum_frame_latency: UnsetDesiredMaximumFrameLatency,
                alpha_mode: UnsetAlphaModeOptional,
                view_formats: UnsetViewFormats,
            }
        }
    }
    pub struct UnsetUsage;
    impl IsRequired for UnsetUsage {}
    impl IsUnset for UnsetUsage {}
    pub struct UsageValue(pub wgpu::TextureUsages);
    impl IsRequired for UsageValue {}
    pub struct UnsetFormat;
    impl IsRequired for UnsetFormat {}
    impl IsUnset for UnsetFormat {}
    pub struct FormatValue(pub wgpu::TextureFormat);
    impl IsRequired for FormatValue {}
    pub struct UnsetWidth;
    impl IsRequired for UnsetWidth {}
    impl IsUnset for UnsetWidth {}
    pub struct WidthValue(pub u32);
    impl IsRequired for WidthValue {}
    pub struct UnsetHeight;
    impl IsRequired for UnsetHeight {}
    impl IsUnset for UnsetHeight {}
    pub struct HeightValue(pub u32);
    impl IsRequired for HeightValue {}
    pub struct UnsetPresentModeOptional;
    impl IsOptional for UnsetPresentModeOptional {}
    impl IsUnsetOptional for UnsetPresentModeOptional {}
    impl ResolveOptional<wgpu::PresentMode> for UnsetPresentModeOptional {
        fn resolve(self) -> wgpu::PresentMode {
            Default::default()
        }
    }
    pub struct PresentModeOptionalValue(pub wgpu::PresentMode);
    impl IsOptional for PresentModeOptionalValue {}
    impl ResolveOptional<wgpu::PresentMode> for PresentModeOptionalValue {
        fn resolve(self) -> wgpu::PresentMode {
            self.0
        }
    }
    pub struct UnsetDesiredMaximumFrameLatency;
    impl IsRequired for UnsetDesiredMaximumFrameLatency {}
    impl IsUnset for UnsetDesiredMaximumFrameLatency {}
    pub struct DesiredMaximumFrameLatencyValue(pub u32);
    impl IsRequired for DesiredMaximumFrameLatencyValue {}
    pub struct UnsetAlphaModeOptional;
    impl IsOptional for UnsetAlphaModeOptional {}
    impl IsUnsetOptional for UnsetAlphaModeOptional {}
    impl ResolveOptional<wgpu::CompositeAlphaMode> for UnsetAlphaModeOptional {
        fn resolve(self) -> wgpu::CompositeAlphaMode {
            Default::default()
        }
    }
    pub struct AlphaModeOptionalValue(pub wgpu::CompositeAlphaMode);
    impl IsOptional for AlphaModeOptionalValue {}
    impl ResolveOptional<wgpu::CompositeAlphaMode> for AlphaModeOptionalValue {
        fn resolve(self) -> wgpu::CompositeAlphaMode {
            self.0
        }
    }
    pub struct UnsetViewFormats;
    impl IsRequired for UnsetViewFormats {}
    impl IsUnset for UnsetViewFormats {}
    pub struct ViewFormatsValue(pub Vec<wgpu::TextureFormat>);
    impl IsRequired for ViewFormatsValue {}
    impl<
        T0: IsRequired,
        T1: IsRequired,
        T2: IsRequired,
        T3: IsRequired,
        T4: IsOptional,
        T5: IsRequired,
        T6: IsOptional,
        T7: IsRequired,
    > SurfaceConfigurationBuilder<T0, T1, T2, T3, T4, T5, T6, T7>
    {
        pub fn usage(
            self,
            usage: wgpu::TextureUsages,
        ) -> SurfaceConfigurationBuilder<UsageValue, T1, T2, T3, T4, T5, T6, T7>
        where
            T0: IsUnset,
        {
            SurfaceConfigurationBuilder {
                usage: UsageValue(usage),
                format: self.format,
                width: self.width,
                height: self.height,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: self.alpha_mode,
                view_formats: self.view_formats,
            }
        }
        pub fn format(
            self,
            format: wgpu::TextureFormat,
        ) -> SurfaceConfigurationBuilder<T0, FormatValue, T2, T3, T4, T5, T6, T7>
        where
            T1: IsUnset,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: FormatValue(format),
                width: self.width,
                height: self.height,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: self.alpha_mode,
                view_formats: self.view_formats,
            }
        }
        pub fn width(
            self,
            width: u32,
        ) -> SurfaceConfigurationBuilder<T0, T1, WidthValue, T3, T4, T5, T6, T7>
        where
            T2: IsUnset,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: self.format,
                width: WidthValue(width),
                height: self.height,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: self.alpha_mode,
                view_formats: self.view_formats,
            }
        }
        pub fn height(
            self,
            height: u32,
        ) -> SurfaceConfigurationBuilder<T0, T1, T2, HeightValue, T4, T5, T6, T7>
        where
            T3: IsUnset,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: self.format,
                width: self.width,
                height: HeightValue(height),
                present_mode: self.present_mode,
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: self.alpha_mode,
                view_formats: self.view_formats,
            }
        }
        pub fn present_mode(
            self,
            present_mode: wgpu::PresentMode,
        ) -> SurfaceConfigurationBuilder<T0, T1, T2, T3, PresentModeOptionalValue, T5, T6, T7>
        where
            T4: IsUnsetOptional,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: self.format,
                width: self.width,
                height: self.height,
                present_mode: PresentModeOptionalValue(present_mode),
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: self.alpha_mode,
                view_formats: self.view_formats,
            }
        }
        pub fn desired_maximum_frame_latency(
            self,
            desired_maximum_frame_latency: u32,
        ) -> SurfaceConfigurationBuilder<T0, T1, T2, T3, T4, DesiredMaximumFrameLatencyValue, T6, T7>
        where
            T5: IsUnset,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: self.format,
                width: self.width,
                height: self.height,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: DesiredMaximumFrameLatencyValue(
                    desired_maximum_frame_latency,
                ),
                alpha_mode: self.alpha_mode,
                view_formats: self.view_formats,
            }
        }
        pub fn alpha_mode(
            self,
            alpha_mode: wgpu::CompositeAlphaMode,
        ) -> SurfaceConfigurationBuilder<T0, T1, T2, T3, T4, T5, AlphaModeOptionalValue, T7>
        where
            T6: IsUnsetOptional,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: self.format,
                width: self.width,
                height: self.height,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: AlphaModeOptionalValue(alpha_mode),
                view_formats: self.view_formats,
            }
        }
        pub fn view_formats(
            self,
            view_formats: Vec<wgpu::TextureFormat>,
        ) -> SurfaceConfigurationBuilder<T0, T1, T2, T3, T4, T5, T6, ViewFormatsValue>
        where
            T7: IsUnset,
        {
            SurfaceConfigurationBuilder {
                usage: self.usage,
                format: self.format,
                width: self.width,
                height: self.height,
                present_mode: self.present_mode,
                desired_maximum_frame_latency: self.desired_maximum_frame_latency,
                alpha_mode: self.alpha_mode,
                view_formats: ViewFormatsValue(view_formats),
            }
        }
    }
    impl<RPresentMode, RAlphaMode>
        SurfaceConfigurationBuilder<
            UsageValue,
            FormatValue,
            WidthValue,
            HeightValue,
            RPresentMode,
            DesiredMaximumFrameLatencyValue,
            RAlphaMode,
            ViewFormatsValue,
        >
    {
        pub fn build(self) -> wgpu::SurfaceConfiguration
        where
            RPresentMode: ResolveOptional<wgpu::PresentMode>,
            RAlphaMode: ResolveOptional<wgpu::CompositeAlphaMode>,
        {
            wgpu::SurfaceConfiguration {
                usage: self.usage.0,
                format: self.format.0,
                width: self.width.0,
                height: self.height.0,
                present_mode: self.present_mode.resolve(),
                desired_maximum_frame_latency: self.desired_maximum_frame_latency.0,
                alpha_mode: self.alpha_mode.resolve(),
                view_formats: self.view_formats.0,
            }
        }
    }
}

pub use builder_request_adapter_options_base::request_adapter_options_base;
pub mod builder_request_adapter_options_base {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn request_adapter_options_base() -> RequestAdapterOptionsBaseBuilder<
        UnsetPowerPreferenceOptional,
        UnsetForceFallbackAdapterOptional,
        UnsetCompatibleSurfaceOptional,
    > {
        RequestAdapterOptionsBaseBuilder::new()
    }
    pub struct RequestAdapterOptionsBaseBuilder<T0, T1, T2> {
        power_preference: T0,
        force_fallback_adapter: T1,
        compatible_surface: T2,
    }
    impl
        RequestAdapterOptionsBaseBuilder<
            UnsetPowerPreferenceOptional,
            UnsetForceFallbackAdapterOptional,
            UnsetCompatibleSurfaceOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                power_preference: UnsetPowerPreferenceOptional,
                force_fallback_adapter: UnsetForceFallbackAdapterOptional,
                compatible_surface: UnsetCompatibleSurfaceOptional,
            }
        }
    }
    pub struct UnsetPowerPreferenceOptional;
    impl IsOptional for UnsetPowerPreferenceOptional {}
    impl IsUnsetOptional for UnsetPowerPreferenceOptional {}
    impl ResolveOptional<wgpu::PowerPreference> for UnsetPowerPreferenceOptional {
        fn resolve(self) -> wgpu::PowerPreference {
            Default::default()
        }
    }
    pub struct PowerPreferenceOptionalValue(pub wgpu::PowerPreference);
    impl IsOptional for PowerPreferenceOptionalValue {}
    impl ResolveOptional<wgpu::PowerPreference> for PowerPreferenceOptionalValue {
        fn resolve(self) -> wgpu::PowerPreference {
            self.0
        }
    }
    pub struct UnsetForceFallbackAdapterOptional;
    impl IsOptional for UnsetForceFallbackAdapterOptional {}
    impl IsUnsetOptional for UnsetForceFallbackAdapterOptional {}
    impl ResolveOptional<bool> for UnsetForceFallbackAdapterOptional {
        fn resolve(self) -> bool {
            false
        }
    }
    pub struct ForceFallbackAdapterOptionalValue(pub bool);
    impl IsOptional for ForceFallbackAdapterOptionalValue {}
    impl ResolveOptional<bool> for ForceFallbackAdapterOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    pub struct UnsetCompatibleSurfaceOptional;
    impl IsOptional for UnsetCompatibleSurfaceOptional {}
    impl IsUnsetOptional for UnsetCompatibleSurfaceOptional {}
    impl<S> ResolveOptional<Option<S>> for UnsetCompatibleSurfaceOptional {
        fn resolve(self) -> Option<S> {
            Default::default()
        }
    }
    pub struct CompatibleSurfaceOptionalValue<S>(pub Option<S>);
    impl<S> IsOptional for CompatibleSurfaceOptionalValue<S> {}
    impl<S> ResolveOptional<Option<S>> for CompatibleSurfaceOptionalValue<S> {
        fn resolve(self) -> Option<S> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> RequestAdapterOptionsBaseBuilder<T0, T1, T2> {
        pub fn power_preference(
            self,
            power_preference: wgpu::PowerPreference,
        ) -> RequestAdapterOptionsBaseBuilder<PowerPreferenceOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            RequestAdapterOptionsBaseBuilder {
                power_preference: PowerPreferenceOptionalValue(power_preference),
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: self.compatible_surface,
            }
        }
        pub fn force_fallback_adapter(
            self,
            force_fallback_adapter: bool,
        ) -> RequestAdapterOptionsBaseBuilder<T0, ForceFallbackAdapterOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            RequestAdapterOptionsBaseBuilder {
                power_preference: self.power_preference,
                force_fallback_adapter: ForceFallbackAdapterOptionalValue(force_fallback_adapter),
                compatible_surface: self.compatible_surface,
            }
        }
        pub fn maybe_compatible_surface<S>(
            self,
            compatible_surface: Option<S>,
        ) -> RequestAdapterOptionsBaseBuilder<T0, T1, CompatibleSurfaceOptionalValue<S>>
        where
            T2: IsUnsetOptional,
        {
            RequestAdapterOptionsBaseBuilder {
                power_preference: self.power_preference,
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: CompatibleSurfaceOptionalValue(compatible_surface),
            }
        }
        pub fn compatible_surface<S>(
            self,
            compatible_surface: S,
        ) -> RequestAdapterOptionsBaseBuilder<T0, T1, CompatibleSurfaceOptionalValue<S>>
        where
            T2: IsUnsetOptional,
        {
            RequestAdapterOptionsBaseBuilder {
                power_preference: self.power_preference,
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: CompatibleSurfaceOptionalValue(Some(compatible_surface)),
            }
        }
    }
    impl<RPowerPreference, RForceFallbackAdapter, RCompatibleSurface>
        RequestAdapterOptionsBaseBuilder<
            RPowerPreference,
            RForceFallbackAdapter,
            RCompatibleSurface,
        >
    {
        pub fn build<S>(self) -> wgpu::RequestAdapterOptionsBase<S>
        where
            RPowerPreference: ResolveOptional<wgpu::PowerPreference>,
            RForceFallbackAdapter: ResolveOptional<bool>,
            RCompatibleSurface: ResolveOptional<Option<S>>,
        {
            wgpu::RequestAdapterOptionsBase {
                power_preference: self.power_preference.resolve(),
                force_fallback_adapter: self.force_fallback_adapter.resolve(),
                compatible_surface: self.compatible_surface.resolve(),
            }
        }
    }
    impl<S, RPowerPreference, RForceFallbackAdapter, RCompatibleSurface>
        Nested<wgpu::RequestAdapterOptionsBase<S>>
        for RequestAdapterOptionsBaseBuilder<
            RPowerPreference,
            RForceFallbackAdapter,
            RCompatibleSurface,
        >
    where
        RPowerPreference: ResolveOptional<wgpu::PowerPreference>,
        RForceFallbackAdapter: ResolveOptional<bool>,
        RCompatibleSurface: ResolveOptional<Option<S>>,
    {
        fn unnest(self) -> wgpu::RequestAdapterOptionsBase<S> {
            self.build()
        }
    }
    impl<S> Nested<wgpu::RequestAdapterOptionsBase<S>> for wgpu::RequestAdapterOptionsBase<S> {
        fn unnest(self) -> wgpu::RequestAdapterOptionsBase<S> {
            self
        }
    }
}

pub use builder_image_subresource_range::image_subresource_range;
pub mod builder_image_subresource_range {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn image_subresource_range() -> ImageSubresourceRangeBuilder<
        UnsetAspectOptional,
        UnsetBaseMipLevelOptional,
        UnsetMipLevelCountOptional,
        UnsetBaseArrayLayerOptional,
        UnsetArrayLayerCountOptional,
    > {
        ImageSubresourceRangeBuilder::new()
    }
    pub struct ImageSubresourceRangeBuilder<T0, T1, T2, T3, T4> {
        aspect: T0,
        base_mip_level: T1,
        mip_level_count: T2,
        base_array_layer: T3,
        array_layer_count: T4,
    }
    impl
        ImageSubresourceRangeBuilder<
            UnsetAspectOptional,
            UnsetBaseMipLevelOptional,
            UnsetMipLevelCountOptional,
            UnsetBaseArrayLayerOptional,
            UnsetArrayLayerCountOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                aspect: UnsetAspectOptional,
                base_mip_level: UnsetBaseMipLevelOptional,
                mip_level_count: UnsetMipLevelCountOptional,
                base_array_layer: UnsetBaseArrayLayerOptional,
                array_layer_count: UnsetArrayLayerCountOptional,
            }
        }
    }
    pub struct UnsetAspectOptional;
    impl IsOptional for UnsetAspectOptional {}
    impl IsUnsetOptional for UnsetAspectOptional {}
    impl ResolveOptional<wgpu::TextureAspect> for UnsetAspectOptional {
        fn resolve(self) -> wgpu::TextureAspect {
            Default::default()
        }
    }
    pub struct AspectOptionalValue(pub wgpu::TextureAspect);
    impl IsOptional for AspectOptionalValue {}
    impl ResolveOptional<wgpu::TextureAspect> for AspectOptionalValue {
        fn resolve(self) -> wgpu::TextureAspect {
            self.0
        }
    }
    pub struct UnsetBaseMipLevelOptional;
    impl IsOptional for UnsetBaseMipLevelOptional {}
    impl IsUnsetOptional for UnsetBaseMipLevelOptional {}
    impl ResolveOptional<u32> for UnsetBaseMipLevelOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct BaseMipLevelOptionalValue(pub u32);
    impl IsOptional for BaseMipLevelOptionalValue {}
    impl ResolveOptional<u32> for BaseMipLevelOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetMipLevelCountOptional;
    impl IsOptional for UnsetMipLevelCountOptional {}
    impl IsUnsetOptional for UnsetMipLevelCountOptional {}
    impl ResolveOptional<Option<u32>> for UnsetMipLevelCountOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct MipLevelCountOptionalValue(pub Option<u32>);
    impl IsOptional for MipLevelCountOptionalValue {}
    impl ResolveOptional<Option<u32>> for MipLevelCountOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    pub struct UnsetBaseArrayLayerOptional;
    impl IsOptional for UnsetBaseArrayLayerOptional {}
    impl IsUnsetOptional for UnsetBaseArrayLayerOptional {}
    impl ResolveOptional<u32> for UnsetBaseArrayLayerOptional {
        fn resolve(self) -> u32 {
            Default::default()
        }
    }
    pub struct BaseArrayLayerOptionalValue(pub u32);
    impl IsOptional for BaseArrayLayerOptionalValue {}
    impl ResolveOptional<u32> for BaseArrayLayerOptionalValue {
        fn resolve(self) -> u32 {
            self.0
        }
    }
    pub struct UnsetArrayLayerCountOptional;
    impl IsOptional for UnsetArrayLayerCountOptional {}
    impl IsUnsetOptional for UnsetArrayLayerCountOptional {}
    impl ResolveOptional<Option<u32>> for UnsetArrayLayerCountOptional {
        fn resolve(self) -> Option<u32> {
            Default::default()
        }
    }
    pub struct ArrayLayerCountOptionalValue(pub Option<u32>);
    impl IsOptional for ArrayLayerCountOptionalValue {}
    impl ResolveOptional<Option<u32>> for ArrayLayerCountOptionalValue {
        fn resolve(self) -> Option<u32> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional, T3: IsOptional, T4: IsOptional>
        ImageSubresourceRangeBuilder<T0, T1, T2, T3, T4>
    {
        pub fn aspect(
            self,
            aspect: wgpu::TextureAspect,
        ) -> ImageSubresourceRangeBuilder<AspectOptionalValue, T1, T2, T3, T4>
        where
            T0: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: AspectOptionalValue(aspect),
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn base_mip_level(
            self,
            base_mip_level: u32,
        ) -> ImageSubresourceRangeBuilder<T0, BaseMipLevelOptionalValue, T2, T3, T4>
        where
            T1: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: self.aspect,
                base_mip_level: BaseMipLevelOptionalValue(base_mip_level),
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_mip_level_count(
            self,
            mip_level_count: Option<u32>,
        ) -> ImageSubresourceRangeBuilder<T0, T1, MipLevelCountOptionalValue, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: MipLevelCountOptionalValue(mip_level_count),
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn mip_level_count(
            self,
            mip_level_count: u32,
        ) -> ImageSubresourceRangeBuilder<T0, T1, MipLevelCountOptionalValue, T3, T4>
        where
            T2: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: MipLevelCountOptionalValue(Some(mip_level_count)),
                base_array_layer: self.base_array_layer,
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn base_array_layer(
            self,
            base_array_layer: u32,
        ) -> ImageSubresourceRangeBuilder<T0, T1, T2, BaseArrayLayerOptionalValue, T4>
        where
            T3: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: BaseArrayLayerOptionalValue(base_array_layer),
                array_layer_count: self.array_layer_count,
            }
        }
        pub fn maybe_array_layer_count(
            self,
            array_layer_count: Option<u32>,
        ) -> ImageSubresourceRangeBuilder<T0, T1, T2, T3, ArrayLayerCountOptionalValue>
        where
            T4: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: ArrayLayerCountOptionalValue(array_layer_count),
            }
        }
        pub fn array_layer_count(
            self,
            array_layer_count: u32,
        ) -> ImageSubresourceRangeBuilder<T0, T1, T2, T3, ArrayLayerCountOptionalValue>
        where
            T4: IsUnsetOptional,
        {
            ImageSubresourceRangeBuilder {
                aspect: self.aspect,
                base_mip_level: self.base_mip_level,
                mip_level_count: self.mip_level_count,
                base_array_layer: self.base_array_layer,
                array_layer_count: ArrayLayerCountOptionalValue(Some(array_layer_count)),
            }
        }
    }
    impl<RAspect, RBaseMipLevel, RMipLevelCount, RBaseArrayLayer, RArrayLayerCount>
        ImageSubresourceRangeBuilder<
            RAspect,
            RBaseMipLevel,
            RMipLevelCount,
            RBaseArrayLayer,
            RArrayLayerCount,
        >
    {
        pub fn build(self) -> wgpu::ImageSubresourceRange
        where
            RAspect: ResolveOptional<wgpu::TextureAspect>,
            RBaseMipLevel: ResolveOptional<u32>,
            RMipLevelCount: ResolveOptional<Option<u32>>,
            RBaseArrayLayer: ResolveOptional<u32>,
            RArrayLayerCount: ResolveOptional<Option<u32>>,
        {
            wgpu::ImageSubresourceRange {
                aspect: self.aspect.resolve(),
                base_mip_level: self.base_mip_level.resolve(),
                mip_level_count: self.mip_level_count.resolve(),
                base_array_layer: self.base_array_layer.resolve(),
                array_layer_count: self.array_layer_count.resolve(),
            }
        }
    }
    impl<RAspect, RBaseMipLevel, RMipLevelCount, RBaseArrayLayer, RArrayLayerCount>
        Nested<wgpu::ImageSubresourceRange>
        for ImageSubresourceRangeBuilder<
            RAspect,
            RBaseMipLevel,
            RMipLevelCount,
            RBaseArrayLayer,
            RArrayLayerCount,
        >
    where
        RAspect: ResolveOptional<wgpu::TextureAspect>,
        RBaseMipLevel: ResolveOptional<u32>,
        RMipLevelCount: ResolveOptional<Option<u32>>,
        RBaseArrayLayer: ResolveOptional<u32>,
        RArrayLayerCount: ResolveOptional<Option<u32>>,
    {
        fn unnest(self) -> wgpu::ImageSubresourceRange {
            self.build()
        }
    }
    impl Nested<wgpu::ImageSubresourceRange> for wgpu::ImageSubresourceRange {
        fn unnest(self) -> wgpu::ImageSubresourceRange {
            self
        }
    }
}

pub use builder_request_adapter_options::request_adapter_options;
pub mod builder_request_adapter_options {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn request_adapter_options() -> RequestAdapterOptionsBuilder<
        UnsetPowerPreferenceOptional,
        UnsetForceFallbackAdapterOptional,
        UnsetCompatibleSurfaceOptional,
    > {
        RequestAdapterOptionsBuilder::new()
    }
    pub struct RequestAdapterOptionsBuilder<T0, T1, T2> {
        power_preference: T0,
        force_fallback_adapter: T1,
        compatible_surface: T2,
    }
    impl
        RequestAdapterOptionsBuilder<
            UnsetPowerPreferenceOptional,
            UnsetForceFallbackAdapterOptional,
            UnsetCompatibleSurfaceOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                power_preference: UnsetPowerPreferenceOptional,
                force_fallback_adapter: UnsetForceFallbackAdapterOptional,
                compatible_surface: UnsetCompatibleSurfaceOptional,
            }
        }
    }
    pub struct UnsetPowerPreferenceOptional;
    impl IsOptional for UnsetPowerPreferenceOptional {}
    impl IsUnsetOptional for UnsetPowerPreferenceOptional {}
    impl ResolveOptional<wgpu::PowerPreference> for UnsetPowerPreferenceOptional {
        fn resolve(self) -> wgpu::PowerPreference {
            Default::default()
        }
    }
    pub struct PowerPreferenceOptionalValue(pub wgpu::PowerPreference);
    impl IsOptional for PowerPreferenceOptionalValue {}
    impl ResolveOptional<wgpu::PowerPreference> for PowerPreferenceOptionalValue {
        fn resolve(self) -> wgpu::PowerPreference {
            self.0
        }
    }
    pub struct UnsetForceFallbackAdapterOptional;
    impl IsOptional for UnsetForceFallbackAdapterOptional {}
    impl IsUnsetOptional for UnsetForceFallbackAdapterOptional {}
    impl ResolveOptional<bool> for UnsetForceFallbackAdapterOptional {
        fn resolve(self) -> bool {
            false
        }
    }
    pub struct ForceFallbackAdapterOptionalValue(pub bool);
    impl IsOptional for ForceFallbackAdapterOptionalValue {}
    impl ResolveOptional<bool> for ForceFallbackAdapterOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    pub struct UnsetCompatibleSurfaceOptional;
    impl IsOptional for UnsetCompatibleSurfaceOptional {}
    impl IsUnsetOptional for UnsetCompatibleSurfaceOptional {}
    impl<'a, 'b> ResolveOptional<Option<&'a wgpu::Surface<'b>>> for UnsetCompatibleSurfaceOptional {
        fn resolve(self) -> Option<&'a wgpu::Surface<'b>> {
            Default::default()
        }
    }
    pub struct CompatibleSurfaceOptionalValue<'a, 'b>(pub Option<&'a wgpu::Surface<'b>>);
    impl<'a, 'b> IsOptional for CompatibleSurfaceOptionalValue<'a, 'b> {}
    impl<'a, 'b> ResolveOptional<Option<&'a wgpu::Surface<'b>>>
        for CompatibleSurfaceOptionalValue<'a, 'b>
    {
        fn resolve(self) -> Option<&'a wgpu::Surface<'b>> {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> RequestAdapterOptionsBuilder<T0, T1, T2> {
        pub fn power_preference(
            self,
            power_preference: wgpu::PowerPreference,
        ) -> RequestAdapterOptionsBuilder<PowerPreferenceOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            RequestAdapterOptionsBuilder {
                power_preference: PowerPreferenceOptionalValue(power_preference),
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: self.compatible_surface,
            }
        }
        pub fn force_fallback_adapter(
            self,
            force_fallback_adapter: bool,
        ) -> RequestAdapterOptionsBuilder<T0, ForceFallbackAdapterOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            RequestAdapterOptionsBuilder {
                power_preference: self.power_preference,
                force_fallback_adapter: ForceFallbackAdapterOptionalValue(force_fallback_adapter),
                compatible_surface: self.compatible_surface,
            }
        }
        pub fn maybe_compatible_surface<'a, 'b>(
            self,
            compatible_surface: Option<&'a wgpu::Surface<'b>>,
        ) -> RequestAdapterOptionsBuilder<T0, T1, CompatibleSurfaceOptionalValue<'a, 'b>>
        where
            T2: IsUnsetOptional,
        {
            RequestAdapterOptionsBuilder {
                power_preference: self.power_preference,
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: CompatibleSurfaceOptionalValue(compatible_surface),
            }
        }
        pub fn compatible_surface<'a, 'b>(
            self,
            compatible_surface: &'a wgpu::Surface<'b>,
        ) -> RequestAdapterOptionsBuilder<T0, T1, CompatibleSurfaceOptionalValue<'a, 'b>>
        where
            T2: IsUnsetOptional,
        {
            RequestAdapterOptionsBuilder {
                power_preference: self.power_preference,
                force_fallback_adapter: self.force_fallback_adapter,
                compatible_surface: CompatibleSurfaceOptionalValue(Some(compatible_surface)),
            }
        }
    }
    impl<RPowerPreference, RForceFallbackAdapter, RCompatibleSurface>
        RequestAdapterOptionsBuilder<RPowerPreference, RForceFallbackAdapter, RCompatibleSurface>
    {
        pub fn build<'a, 'b>(self) -> wgpu::RequestAdapterOptions<'a, 'b>
        where
            RPowerPreference: ResolveOptional<wgpu::PowerPreference>,
            RForceFallbackAdapter: ResolveOptional<bool>,
            RCompatibleSurface: ResolveOptional<Option<&'a wgpu::Surface<'b>>>,
        {
            wgpu::RequestAdapterOptions {
                power_preference: self.power_preference.resolve(),
                force_fallback_adapter: self.force_fallback_adapter.resolve(),
                compatible_surface: self.compatible_surface.resolve(),
            }
        }
    }
}

pub use builder_dx_12_backend_options::dx_12_backend_options;
pub mod builder_dx_12_backend_options {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn dx_12_backend_options() -> Dx12BackendOptionsBuilder<
        UnsetShaderCompilerOptional,
        UnsetPresentationSystemOptional,
        UnsetLatencyWaitableObjectOptional,
    > {
        Dx12BackendOptionsBuilder::new()
    }
    pub struct Dx12BackendOptionsBuilder<T0, T1, T2> {
        shader_compiler: T0,
        presentation_system: T1,
        latency_waitable_object: T2,
    }
    impl
        Dx12BackendOptionsBuilder<
            UnsetShaderCompilerOptional,
            UnsetPresentationSystemOptional,
            UnsetLatencyWaitableObjectOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                shader_compiler: UnsetShaderCompilerOptional,
                presentation_system: UnsetPresentationSystemOptional,
                latency_waitable_object: UnsetLatencyWaitableObjectOptional,
            }
        }
    }
    pub struct UnsetShaderCompilerOptional;
    impl IsOptional for UnsetShaderCompilerOptional {}
    impl IsUnsetOptional for UnsetShaderCompilerOptional {}
    impl ResolveOptional<wgpu::Dx12Compiler> for UnsetShaderCompilerOptional {
        fn resolve(self) -> wgpu::Dx12Compiler {
            Default::default()
        }
    }
    pub struct ShaderCompilerOptionalValue(pub wgpu::Dx12Compiler);
    impl IsOptional for ShaderCompilerOptionalValue {}
    impl ResolveOptional<wgpu::Dx12Compiler> for ShaderCompilerOptionalValue {
        fn resolve(self) -> wgpu::Dx12Compiler {
            self.0
        }
    }
    pub struct UnsetPresentationSystemOptional;
    impl IsOptional for UnsetPresentationSystemOptional {}
    impl IsUnsetOptional for UnsetPresentationSystemOptional {}
    impl ResolveOptional<wgpu::wgt::Dx12SwapchainKind> for UnsetPresentationSystemOptional {
        fn resolve(self) -> wgpu::wgt::Dx12SwapchainKind {
            Default::default()
        }
    }
    pub struct PresentationSystemOptionalValue(pub wgpu::wgt::Dx12SwapchainKind);
    impl IsOptional for PresentationSystemOptionalValue {}
    impl ResolveOptional<wgpu::wgt::Dx12SwapchainKind> for PresentationSystemOptionalValue {
        fn resolve(self) -> wgpu::wgt::Dx12SwapchainKind {
            self.0
        }
    }
    pub struct UnsetLatencyWaitableObjectOptional;
    impl IsOptional for UnsetLatencyWaitableObjectOptional {}
    impl IsUnsetOptional for UnsetLatencyWaitableObjectOptional {}
    impl ResolveOptional<wgpu::wgt::Dx12UseFrameLatencyWaitableObject>
        for UnsetLatencyWaitableObjectOptional
    {
        fn resolve(self) -> wgpu::wgt::Dx12UseFrameLatencyWaitableObject {
            Default::default()
        }
    }
    pub struct LatencyWaitableObjectOptionalValue(pub wgpu::wgt::Dx12UseFrameLatencyWaitableObject);
    impl IsOptional for LatencyWaitableObjectOptionalValue {}
    impl ResolveOptional<wgpu::wgt::Dx12UseFrameLatencyWaitableObject>
        for LatencyWaitableObjectOptionalValue
    {
        fn resolve(self) -> wgpu::wgt::Dx12UseFrameLatencyWaitableObject {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional, T2: IsOptional> Dx12BackendOptionsBuilder<T0, T1, T2> {
        pub fn shader_compiler(
            self,
            shader_compiler: wgpu::Dx12Compiler,
        ) -> Dx12BackendOptionsBuilder<ShaderCompilerOptionalValue, T1, T2>
        where
            T0: IsUnsetOptional,
        {
            Dx12BackendOptionsBuilder {
                shader_compiler: ShaderCompilerOptionalValue(shader_compiler),
                presentation_system: self.presentation_system,
                latency_waitable_object: self.latency_waitable_object,
            }
        }
        pub fn presentation_system(
            self,
            presentation_system: wgpu::wgt::Dx12SwapchainKind,
        ) -> Dx12BackendOptionsBuilder<T0, PresentationSystemOptionalValue, T2>
        where
            T1: IsUnsetOptional,
        {
            Dx12BackendOptionsBuilder {
                shader_compiler: self.shader_compiler,
                presentation_system: PresentationSystemOptionalValue(presentation_system),
                latency_waitable_object: self.latency_waitable_object,
            }
        }
        pub fn latency_waitable_object(
            self,
            latency_waitable_object: wgpu::wgt::Dx12UseFrameLatencyWaitableObject,
        ) -> Dx12BackendOptionsBuilder<T0, T1, LatencyWaitableObjectOptionalValue>
        where
            T2: IsUnsetOptional,
        {
            Dx12BackendOptionsBuilder {
                shader_compiler: self.shader_compiler,
                presentation_system: self.presentation_system,
                latency_waitable_object: LatencyWaitableObjectOptionalValue(
                    latency_waitable_object,
                ),
            }
        }
    }
    impl<RShaderCompiler, RPresentationSystem, RLatencyWaitableObject>
        Dx12BackendOptionsBuilder<RShaderCompiler, RPresentationSystem, RLatencyWaitableObject>
    {
        pub fn build(self) -> wgpu::Dx12BackendOptions
        where
            RShaderCompiler: ResolveOptional<wgpu::Dx12Compiler>,
            RPresentationSystem: ResolveOptional<wgpu::wgt::Dx12SwapchainKind>,
            RLatencyWaitableObject: ResolveOptional<wgpu::wgt::Dx12UseFrameLatencyWaitableObject>,
        {
            wgpu::Dx12BackendOptions {
                shader_compiler: self.shader_compiler.resolve(),
                presentation_system: self.presentation_system.resolve(),
                latency_waitable_object: self.latency_waitable_object.resolve(),
            }
        }
    }
    impl<RShaderCompiler, RPresentationSystem, RLatencyWaitableObject>
        Nested<wgpu::Dx12BackendOptions>
        for Dx12BackendOptionsBuilder<RShaderCompiler, RPresentationSystem, RLatencyWaitableObject>
    where
        RShaderCompiler: ResolveOptional<wgpu::Dx12Compiler>,
        RPresentationSystem: ResolveOptional<wgpu::wgt::Dx12SwapchainKind>,
        RLatencyWaitableObject: ResolveOptional<wgpu::wgt::Dx12UseFrameLatencyWaitableObject>,
    {
        fn unnest(self) -> wgpu::Dx12BackendOptions {
            self.build()
        }
    }
    impl Nested<wgpu::Dx12BackendOptions> for wgpu::Dx12BackendOptions {
        fn unnest(self) -> wgpu::Dx12BackendOptions {
            self
        }
    }
}

pub use builder_blend_state::blend_state;
pub mod builder_blend_state {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn blend_state() -> BlendStateBuilder<UnsetColorOptional, UnsetAlphaOptional> {
        BlendStateBuilder::new()
    }
    pub struct BlendStateBuilder<T0, T1> {
        color: T0,
        alpha: T1,
    }
    impl BlendStateBuilder<UnsetColorOptional, UnsetAlphaOptional> {
        pub fn new() -> Self {
            Self {
                color: UnsetColorOptional,
                alpha: UnsetAlphaOptional,
            }
        }
    }
    pub struct UnsetColorOptional;
    impl IsOptional for UnsetColorOptional {}
    impl IsUnsetOptional for UnsetColorOptional {}
    impl ResolveOptional<wgpu::BlendComponent> for UnsetColorOptional {
        fn resolve(self) -> wgpu::BlendComponent {
            Default::default()
        }
    }
    pub struct ColorOptionalValue(pub wgpu::BlendComponent);
    impl IsOptional for ColorOptionalValue {}
    impl ResolveOptional<wgpu::BlendComponent> for ColorOptionalValue {
        fn resolve(self) -> wgpu::BlendComponent {
            self.0
        }
    }
    pub struct UnsetAlphaOptional;
    impl IsOptional for UnsetAlphaOptional {}
    impl IsUnsetOptional for UnsetAlphaOptional {}
    impl ResolveOptional<wgpu::BlendComponent> for UnsetAlphaOptional {
        fn resolve(self) -> wgpu::BlendComponent {
            Default::default()
        }
    }
    pub struct AlphaOptionalValue(pub wgpu::BlendComponent);
    impl IsOptional for AlphaOptionalValue {}
    impl ResolveOptional<wgpu::BlendComponent> for AlphaOptionalValue {
        fn resolve(self) -> wgpu::BlendComponent {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional> BlendStateBuilder<T0, T1> {
        pub fn color(
            self,
            color: impl Nested<wgpu::BlendComponent>,
        ) -> BlendStateBuilder<ColorOptionalValue, T1>
        where
            T0: IsUnsetOptional,
        {
            BlendStateBuilder {
                color: ColorOptionalValue(color.unnest()),
                alpha: self.alpha,
            }
        }
        pub fn alpha(
            self,
            alpha: impl Nested<wgpu::BlendComponent>,
        ) -> BlendStateBuilder<T0, AlphaOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            BlendStateBuilder {
                color: self.color,
                alpha: AlphaOptionalValue(alpha.unnest()),
            }
        }
    }
    impl<RColor, RAlpha> BlendStateBuilder<RColor, RAlpha> {
        pub fn build(self) -> wgpu::BlendState
        where
            RColor: ResolveOptional<wgpu::BlendComponent>,
            RAlpha: ResolveOptional<wgpu::BlendComponent>,
        {
            wgpu::BlendState {
                color: self.color.resolve(),
                alpha: self.alpha.resolve(),
            }
        }
    }
    impl<RColor, RAlpha> Nested<wgpu::BlendState> for BlendStateBuilder<RColor, RAlpha>
    where
        RColor: ResolveOptional<wgpu::BlendComponent>,
        RAlpha: ResolveOptional<wgpu::BlendComponent>,
    {
        fn unnest(self) -> wgpu::BlendState {
            self.build()
        }
    }
    impl Nested<wgpu::BlendState> for wgpu::BlendState {
        fn unnest(self) -> wgpu::BlendState {
            self
        }
    }
}

pub use builder_pipeline_compilation_options::pipeline_compilation_options;
pub mod builder_pipeline_compilation_options {
    #[allow(unused_imports)]
    use super::common::*;
    pub fn pipeline_compilation_options() -> PipelineCompilationOptionsBuilder<
        UnsetConstantsOptional,
        UnsetZeroInitializeWorkgroupMemoryOptional,
    > {
        PipelineCompilationOptionsBuilder::new()
    }
    pub struct PipelineCompilationOptionsBuilder<T0, T1> {
        constants: T0,
        zero_initialize_workgroup_memory: T1,
    }
    impl
        PipelineCompilationOptionsBuilder<
            UnsetConstantsOptional,
            UnsetZeroInitializeWorkgroupMemoryOptional,
        >
    {
        pub fn new() -> Self {
            Self {
                constants: UnsetConstantsOptional,
                zero_initialize_workgroup_memory: UnsetZeroInitializeWorkgroupMemoryOptional,
            }
        }
    }
    pub struct UnsetConstantsOptional;
    impl IsOptional for UnsetConstantsOptional {}
    impl IsUnsetOptional for UnsetConstantsOptional {}
    impl<'a> ResolveOptional<&'a [(&'a str, f64)]> for UnsetConstantsOptional {
        fn resolve(self) -> &'a [(&'a str, f64)] {
            Default::default()
        }
    }
    pub struct ConstantsOptionalValue<'a>(pub &'a [(&'a str, f64)]);
    impl<'a> IsOptional for ConstantsOptionalValue<'a> {}
    impl<'a> ResolveOptional<&'a [(&'a str, f64)]> for ConstantsOptionalValue<'a> {
        fn resolve(self) -> &'a [(&'a str, f64)] {
            self.0
        }
    }
    pub struct UnsetZeroInitializeWorkgroupMemoryOptional;
    impl IsOptional for UnsetZeroInitializeWorkgroupMemoryOptional {}
    impl IsUnsetOptional for UnsetZeroInitializeWorkgroupMemoryOptional {}
    impl ResolveOptional<bool> for UnsetZeroInitializeWorkgroupMemoryOptional {
        fn resolve(self) -> bool {
            true
        }
    }
    pub struct ZeroInitializeWorkgroupMemoryOptionalValue(pub bool);
    impl IsOptional for ZeroInitializeWorkgroupMemoryOptionalValue {}
    impl ResolveOptional<bool> for ZeroInitializeWorkgroupMemoryOptionalValue {
        fn resolve(self) -> bool {
            self.0
        }
    }
    impl<T0: IsOptional, T1: IsOptional> PipelineCompilationOptionsBuilder<T0, T1> {
        pub fn constants<'a>(
            self,
            constants: &'a [(&'a str, f64)],
        ) -> PipelineCompilationOptionsBuilder<ConstantsOptionalValue<'a>, T1>
        where
            T0: IsUnsetOptional,
        {
            PipelineCompilationOptionsBuilder {
                constants: ConstantsOptionalValue(constants),
                zero_initialize_workgroup_memory: self.zero_initialize_workgroup_memory,
            }
        }
        pub fn zero_initialize_workgroup_memory(
            self,
            zero_initialize_workgroup_memory: bool,
        ) -> PipelineCompilationOptionsBuilder<T0, ZeroInitializeWorkgroupMemoryOptionalValue>
        where
            T1: IsUnsetOptional,
        {
            PipelineCompilationOptionsBuilder {
                constants: self.constants,
                zero_initialize_workgroup_memory: ZeroInitializeWorkgroupMemoryOptionalValue(
                    zero_initialize_workgroup_memory,
                ),
            }
        }
    }
    impl<RConstants, RZeroInitializeWorkgroupMemory>
        PipelineCompilationOptionsBuilder<RConstants, RZeroInitializeWorkgroupMemory>
    {
        pub fn build<'a>(self) -> wgpu::PipelineCompilationOptions<'a>
        where
            RConstants: ResolveOptional<&'a [(&'a str, f64)]>,
            RZeroInitializeWorkgroupMemory: ResolveOptional<bool>,
        {
            wgpu::PipelineCompilationOptions {
                constants: self.constants.resolve(),
                zero_initialize_workgroup_memory: self.zero_initialize_workgroup_memory.resolve(),
            }
        }
    }
    impl<'a, RConstants, RZeroInitializeWorkgroupMemory>
        Nested<wgpu::PipelineCompilationOptions<'a>>
        for PipelineCompilationOptionsBuilder<RConstants, RZeroInitializeWorkgroupMemory>
    where
        RConstants: ResolveOptional<&'a [(&'a str, f64)]>,
        RZeroInitializeWorkgroupMemory: ResolveOptional<bool>,
    {
        fn unnest(self) -> wgpu::PipelineCompilationOptions<'a> {
            self.build()
        }
    }
    impl<'a> Nested<wgpu::PipelineCompilationOptions<'a>> for wgpu::PipelineCompilationOptions<'a> {
        fn unnest(self) -> wgpu::PipelineCompilationOptions<'a> {
            self
        }
    }
}
