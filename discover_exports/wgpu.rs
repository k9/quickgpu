#![feature(prelude_import)]
//! `wgpu` is a cross-platform, safe, pure-Rust graphics API. It runs natively on
//! Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on wasm.
//!
//! The API is based on the [WebGPU standard][webgpu]. It serves as the core of the
//! WebGPU integration in Firefox, Servo, and Deno.
//!
//! [webgpu]: https://gpuweb.github.io/gpuweb/
//!
//! ## Getting Started
//!
//! The main entry point to the API is the [`Instance`] type, from which you can create [`Adapter`], [`Device`], and [`Surface`].
//!
//! If you are new to `wgpu` and graphics programming, we recommend reading
//! <https://sotrh.github.io/learn-wgpu/> and <https://webgpufundamentals.org/>. The latter is a WebGPU
//! tutorial, but the concepts are nearly identical to `wgpu`.
//!
//! There are examples for this version [available on GitHub](https://github.com/gfx-rs/wgpu/tree/v27/examples#readme).
//!
//! The API is refcounted, so all handles are cloneable, and if you create a resource which references another,
//! it will automatically keep dependent resources alive.
//!
//! ## Feature flags
/*! ### Backends
* **`dx12`** *(enabled by default)* —  Enables the DX12 backend on Windows.
* **`metal`** *(enabled by default)* —  Enables the Metal backend on macOS & iOS.
* **`vulkan`** *(enabled by default)* —  Enables the Vulkan backend on Windows, Linux, and Android.
* **`gles`** *(enabled by default)* —  Enables the OpenGL/GLES backend on Windows, Linux, Android, and Emscripten.
* **`webgpu`** *(enabled by default)* —  Enables the WebGPU backend on WebAssembly.

 ### Conditional Backends
* **`angle`** —  Enables the GLES backend on macOS only for use with [ANGLE](https://github.com/google/angle).
* **`vulkan-portability`** —  Enables the Vulkan backend on macOS & iOS only for use with [MoltenVK](https://github.com/KhronosGroup/MoltenVK).
* **`webgl`** —  Enables the GLES backend on WebAssembly only.
* **`noop`** —  Enables the noop backend for testing.

  This backend allows creating resources such as buffers and textures,
  but performs no computation.
  Because it lacks basic functionality, it is only actually used if explicitly enabled
  through `NoopBackendOptions`.

 **Note:** In the documentation, if you see that an item depends on a backend,
 it means that the item is only available when that backend is enabled _and_ the backend
 is supported on the current platform.
 ### Shading language support
 These features enable support for that input language on all platforms.
 We will translate the input language to whatever the backend requires.
* **`spirv`** —  Enable accepting SPIR-V shaders as input.
* **`glsl`** —  Enable accepting GLSL shaders as input.
* **`wgsl`** *(enabled by default)* —  Enable accepting WGSL shaders as input.
* **`naga-ir`** —  Enable accepting naga IR shaders as input.

 ### Assertions and Serialization
* **`strict_asserts`** —  Apply run-time checks, even in release builds. These are in addition
  to the validation carried out at public APIs in all builds.
* **`serde`** —  Enables serialization via `serde` on common wgpu types.

 ### External libraries
 The following features facilitate integration with third-party supporting libraries.
* **`static-dxc`** —  Enables statically linking DXC.

  Normally, to use the modern DXC shader compiler with wgpu, the final application
  must be shipped alongside `dxcompiler.dll` (min v1.8.2502) (which can be downloaded from [Microsoft's GitHub][dxc]).
  This feature statically links a version of DXC so that no external binaries are required
  to compile DX12 shaders.

  [dxc]: https://github.com/Microsoft/DirectXShaderCompiler

 ### Other
* **`counters`** —  Internally count resources and events for debugging purposes. If the counters
  feature is disabled, the counting infrastructure is removed from the build and
  the exposed counters always return 0.
* **`fragile-send-sync-non-atomic-wasm`** —  Implement `Send` and `Sync` on Wasm, but only if atomics are not enabled.

  WebGL/WebGPU objects can not be shared between threads.
  However, it can be useful to artificially mark them as `Send` and `Sync`
  anyways to make it easier to write cross-platform code.
  This is technically *very* unsafe in a multithreaded environment,
  but on a wasm binary compiled without atomics is a definitionally single-threaded environment.
* **`web`** *(enabled by default)* —  Use web-specific libraries on WASM

  Those libraties (wasm-bindgen, web-sys, js-sys) can only be used when there is a JavaScript
  context around the WASM VM, e.g., when the WASM binary is used in a browser.
* **`std`** *(enabled by default)* —  Enables use of the standard library within `wgpu` and its dependencies.

  This can allow for better error reporting and for improved multithreading
  support.
* **`parking_lot`** *(enabled by default)* —  Uses `parking_lot` as the implementation for locking primitives.

  This is a recommended feature for most users and should only be disabled when
  required, e.g., for `no_std` support.
  If disabled, either `std::sync::Mutex` or `core::cell::RefCell` will be used,
  based on whether `std` is enabled or not.
*/
//!
//! ### Feature Aliases
//!
//! These features aren't actually features on the crate itself, but a convenient shorthand for
//! complicated cases.
//!
//! - **`wgpu_core`** --- Enabled when there is any non-webgpu backend enabled on the platform.
//! - **`naga`** --- Enabled when target `glsl` or `spirv`` input is enabled, or when `wgpu_core` is enabled.
//!
#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/gfx-rs/wgpu/trunk/logo.png")]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    missing_docs,
    rust_2018_idioms,
    unsafe_op_in_unsafe_fn
)]
#![allow(
    clippy::large_enum_variant,
    clippy::bool_assert_comparison,
    clippy::bool_comparison,
)]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
extern crate alloc;
extern crate std;
pub extern crate wgpu_core as wgc;
pub extern crate wgpu_hal as hal;
pub extern crate wgpu_types as wgt;
mod api {
    //! Types and functions which define our public api and their
    //! helper functionality.
    //!
    //! # Conventions
    //!
    //! Each major type gets its own module. The module is laid out as follows:
    //!
    //! - The type itself
    //! - `impl` block for the type
    //! - `Drop` implementation for the type (if needed)
    //! - Descriptor types and their subtypes.
    //! - Any non-public helper types or functions.
    //!
    //! # Imports
    //!
    //! Because our public api is "flat" (i.e. all types are directly under the `wgpu` module),
    //! we use a single `crate::*` import at the top of each module to bring in all the types in
    //! the public api. This is done to:
    //! - Avoid having to write out a long list of imports for each module.
    //! - Allow docs to be written naturally, without needing to worry about needing dedicated doc imports.
    //! - Treat wgpu-types types and wgpu-core types as a single set.
    mod adapter {
        use core::future::Future;
        use core::ops::Deref;
        use crate::*;
        /// Handle to a physical graphics and/or compute device.
        ///
        /// Adapters can be created using [`Instance::request_adapter`]
        /// or other [`Instance`] methods.
        ///
        /// Adapters can be used to open a connection to the corresponding [`Device`]
        /// on the host system by using [`Adapter::request_device`].
        ///
        /// Does not have to be kept alive.
        ///
        /// Corresponds to [WebGPU `GPUAdapter`](https://gpuweb.github.io/gpuweb/#gpu-adapter).
        pub struct Adapter {
            pub(crate) inner: dispatch::DispatchAdapter,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Adapter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Adapter",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Adapter {
            #[inline]
            fn clone(&self) -> Adapter {
                Adapter {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Adapter>();
        };
        impl PartialEq for Adapter {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Adapter {}
        impl PartialOrd for Adapter {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Adapter {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Adapter {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        pub use wgt::RequestAdapterOptions as RequestAdapterOptionsBase;
        /// Additional information required when requesting an adapter.
        ///
        /// For use with [`Instance::request_adapter`].
        ///
        /// Corresponds to [WebGPU `GPURequestAdapterOptions`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpurequestadapteroptions).
        pub type RequestAdapterOptions<'a, 'b> = RequestAdapterOptionsBase<
            &'a Surface<'b>,
        >;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RequestAdapterOptions<'_, '_>>();
        };
        impl Adapter {
            /// Requests a connection to a physical device, creating a logical device.
            ///
            /// Returns the [`Device`] together with a [`Queue`] that executes command buffers.
            ///
            /// [Per the WebGPU specification], an [`Adapter`] may only be used once to create a device.
            /// If another device is wanted, call [`Instance::request_adapter()`] again to get a fresh
            /// [`Adapter`].
            /// However, `wgpu` does not currently enforce this restriction.
            ///
            /// # Panics
            ///
            /// - `request_device()` was already called on this `Adapter`.
            /// - Features specified by `desc` are not supported by this adapter.
            /// - Unsafe features were requested but not enabled when requesting the adapter.
            /// - Limits requested exceed the values provided by the adapter.
            /// - Adapter does not support all features wgpu requires to safely operate.
            ///
            /// [Per the WebGPU specification]: https://www.w3.org/TR/webgpu/#dom-gpuadapter-requestdevice
            pub fn request_device(
                &self,
                desc: &DeviceDescriptor<'_>,
            ) -> impl Future<
                Output = Result<(Device, Queue), RequestDeviceError>,
            > + WasmNotSend {
                let device = self.inner.request_device(desc);
                async move {
                    device
                        .await
                        .map(|(device, queue)| (
                            Device { inner: device },
                            Queue { inner: queue },
                        ))
                }
            }
            /// Create a wgpu [`Device`] and [`Queue`] from a wgpu-hal [`hal::OpenDevice`].
            ///
            /// # Safety
            ///
            /// - `hal_device` must be created from this adapter internal handle.
            /// - `desc.features` must be a subset of `hal_device`'s supported features.
            pub unsafe fn create_device_from_hal<A: hal::Api>(
                &self,
                hal_device: hal::OpenDevice<A>,
                desc: &DeviceDescriptor<'_>,
            ) -> Result<(Device, Queue), RequestDeviceError> {
                let core_adapter = self.inner.as_core();
                let (device, queue) = unsafe {
                    core_adapter
                        .context
                        .create_device_from_hal(core_adapter, hal_device, desc)
                }?;
                Ok((Device { inner: device.into() }, Queue { inner: queue.into() }))
            }
            /// Get the [`wgpu_hal`] adapter from this `Adapter`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Adapter`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Adapter`
            ///- [`hal::api::Metal`] uses [`hal::metal::Adapter`]
            ///- `hal::api::Dx12` uses `hal::dx12::Adapter`
            ///- `hal::api::Gles` uses `hal::gles::Adapter`
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The adapter is not from the backend specified by `A`.
            /// - The adapter is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Adapter`]: hal::Api::Adapter
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::Adapter> + WasmNotSendSync> {
                let adapter = self.inner.as_core_opt()?;
                unsafe { adapter.context.adapter_as_hal::<A>(adapter) }
            }
            /// Returns whether this adapter may present to the passed surface.
            pub fn is_surface_supported(&self, surface: &Surface<'_>) -> bool {
                self.inner.is_surface_supported(&surface.inner)
            }
            /// The features which can be used to create devices on this adapter.
            pub fn features(&self) -> Features {
                self.inner.features()
            }
            /// The best limits which can be used to create devices on this adapter.
            pub fn limits(&self) -> Limits {
                self.inner.limits()
            }
            /// Get info about the adapter itself.
            pub fn get_info(&self) -> AdapterInfo {
                self.inner.get_info()
            }
            /// Get info about the adapter itself.
            pub fn get_downlevel_capabilities(&self) -> DownlevelCapabilities {
                self.inner.downlevel_capabilities()
            }
            /// Returns the features supported for a given texture format by this adapter.
            ///
            /// Note that the WebGPU spec further restricts the available usages/features.
            /// To disable these restrictions on a device, request the [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] feature.
            pub fn get_texture_format_features(
                &self,
                format: TextureFormat,
            ) -> TextureFormatFeatures {
                self.inner.get_texture_format_features(format)
            }
            /// Generates a timestamp using the clock used by the presentation engine.
            ///
            /// When comparing completely opaque timestamp systems, we need a way of generating timestamps that signal
            /// the exact same time. You can do this by calling your own timestamp function immediately after a call to
            /// this function. This should result in timestamps that are 0.5 to 5 microseconds apart. There are locks
            /// that must be taken during the call, so don't call your function before.
            ///
            /// ```no_run
            /// # let adapter: wgpu::Adapter = panic!();
            /// # let some_code = || wgpu::PresentationTimestamp::INVALID_TIMESTAMP;
            /// use std::time::{Duration, Instant};
            /// let presentation = adapter.get_presentation_timestamp();
            /// let instant = Instant::now();
            ///
            /// // We can now turn a new presentation timestamp into an Instant.
            /// let some_pres_timestamp = some_code();
            /// let duration = Duration::from_nanos((some_pres_timestamp.0 - presentation.0) as u64);
            /// let new_instant: Instant = instant + duration;
            /// ```
            /// [Instant]: std::time::Instant
            pub fn get_presentation_timestamp(&self) -> PresentationTimestamp {
                self.inner.get_presentation_timestamp()
            }
        }
    }
    mod bind_group {
        use crate::*;
        /// Handle to a binding group.
        ///
        /// A `BindGroup` represents the set of resources bound to the bindings described by a
        /// [`BindGroupLayout`]. It can be created with [`Device::create_bind_group`]. A `BindGroup` can
        /// be bound to a particular [`RenderPass`] with [`RenderPass::set_bind_group`], or to a
        /// [`ComputePass`] with [`ComputePass::set_bind_group`].
        ///
        /// Corresponds to [WebGPU `GPUBindGroup`](https://gpuweb.github.io/gpuweb/#gpubindgroup).
        pub struct BindGroup {
            pub(crate) inner: dispatch::DispatchBindGroup,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BindGroup {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "BindGroup",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BindGroup {
            #[inline]
            fn clone(&self) -> BindGroup {
                BindGroup {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BindGroup>();
        };
        impl PartialEq for BindGroup {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for BindGroup {}
        impl PartialOrd for BindGroup {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for BindGroup {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for BindGroup {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl BindGroup {}
        /// Resource to be bound by a [`BindGroup`] for use with a pipeline.
        ///
        /// The pipeline’s [`BindGroupLayout`] must contain a matching [`BindingType`].
        ///
        /// Corresponds to [WebGPU `GPUBindingResource`](
        /// https://gpuweb.github.io/gpuweb/#typedefdef-gpubindingresource).
        #[non_exhaustive]
        pub enum BindingResource<'a> {
            /// Binding is backed by a buffer.
            ///
            /// Corresponds to [`wgt::BufferBindingType::Uniform`] and [`wgt::BufferBindingType::Storage`]
            /// with [`BindGroupLayoutEntry::count`] set to None.
            Buffer(BufferBinding<'a>),
            /// Binding is backed by an array of buffers.
            ///
            /// [`Features::BUFFER_BINDING_ARRAY`] must be supported to use this feature.
            ///
            /// Corresponds to [`wgt::BufferBindingType::Uniform`] and [`wgt::BufferBindingType::Storage`]
            /// with [`BindGroupLayoutEntry::count`] set to Some.
            BufferArray(&'a [BufferBinding<'a>]),
            /// Binding is a sampler.
            ///
            /// Corresponds to [`wgt::BindingType::Sampler`] with [`BindGroupLayoutEntry::count`] set to None.
            Sampler(&'a Sampler),
            /// Binding is backed by an array of samplers.
            ///
            /// [`Features::TEXTURE_BINDING_ARRAY`] must be supported to use this feature.
            ///
            /// Corresponds to [`wgt::BindingType::Sampler`] with [`BindGroupLayoutEntry::count`] set
            /// to Some.
            SamplerArray(&'a [&'a Sampler]),
            /// Binding is backed by a texture.
            ///
            /// Corresponds to [`wgt::BindingType::Texture`] and [`wgt::BindingType::StorageTexture`] with
            /// [`BindGroupLayoutEntry::count`] set to None.
            TextureView(&'a TextureView),
            /// Binding is backed by an array of textures.
            ///
            /// [`Features::TEXTURE_BINDING_ARRAY`] must be supported to use this feature.
            ///
            /// Corresponds to [`wgt::BindingType::Texture`] and [`wgt::BindingType::StorageTexture`] with
            /// [`BindGroupLayoutEntry::count`] set to Some.
            TextureViewArray(&'a [&'a TextureView]),
            /// Binding is backed by a top level acceleration structure
            ///
            /// Corresponds to [`wgt::BindingType::AccelerationStructure`] with [`BindGroupLayoutEntry::count`] set to None.
            ///
            /// # Validation
            /// When using (e.g. with `set_bind_group`) a bind group that has been created with one or more of this binding
            /// resource certain checks take place.
            /// - TLAS must have been built, if not a validation error is generated
            /// - All BLASes that were built into the TLAS must be built before the TLAS, if this was not satisfied and TLAS was
            ///   built using `build_acceleration_structures` a validation error is generated otherwise this is a part of the
            ///   safety section of `build_acceleration_structures_unsafe_tlas` and so undefined behavior occurs.
            AccelerationStructure(&'a Tlas),
            /// Binding is backed by an external texture.
            ///
            /// [`Features::EXTERNAL_TEXTURE`] must be supported to use this feature.
            ///
            /// Corresponds to [`wgt::BindingType::ExternalTexture`].
            ExternalTexture(&'a ExternalTexture),
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BindingResource<'a> {
            #[inline]
            fn clone(&self) -> BindingResource<'a> {
                match self {
                    BindingResource::Buffer(__self_0) => {
                        BindingResource::Buffer(::core::clone::Clone::clone(__self_0))
                    }
                    BindingResource::BufferArray(__self_0) => {
                        BindingResource::BufferArray(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    BindingResource::Sampler(__self_0) => {
                        BindingResource::Sampler(::core::clone::Clone::clone(__self_0))
                    }
                    BindingResource::SamplerArray(__self_0) => {
                        BindingResource::SamplerArray(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    BindingResource::TextureView(__self_0) => {
                        BindingResource::TextureView(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    BindingResource::TextureViewArray(__self_0) => {
                        BindingResource::TextureViewArray(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    BindingResource::AccelerationStructure(__self_0) => {
                        BindingResource::AccelerationStructure(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    BindingResource::ExternalTexture(__self_0) => {
                        BindingResource::ExternalTexture(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BindingResource<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    BindingResource::Buffer(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Buffer",
                            &__self_0,
                        )
                    }
                    BindingResource::BufferArray(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "BufferArray",
                            &__self_0,
                        )
                    }
                    BindingResource::Sampler(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Sampler",
                            &__self_0,
                        )
                    }
                    BindingResource::SamplerArray(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "SamplerArray",
                            &__self_0,
                        )
                    }
                    BindingResource::TextureView(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TextureView",
                            &__self_0,
                        )
                    }
                    BindingResource::TextureViewArray(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TextureViewArray",
                            &__self_0,
                        )
                    }
                    BindingResource::AccelerationStructure(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "AccelerationStructure",
                            &__self_0,
                        )
                    }
                    BindingResource::ExternalTexture(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ExternalTexture",
                            &__self_0,
                        )
                    }
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BindingResource<'_>>();
        };
        /// Describes the segment of a buffer to bind.
        ///
        /// Corresponds to [WebGPU `GPUBufferBinding`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferbinding).
        pub struct BufferBinding<'a> {
            /// The buffer to bind.
            pub buffer: &'a Buffer,
            /// Base offset of the buffer, in bytes.
            ///
            /// If the [`has_dynamic_offset`] field of this buffer's layout entry is
            /// `true`, the offset here will be added to the dynamic offset passed to
            /// [`RenderPass::set_bind_group`] or [`ComputePass::set_bind_group`].
            ///
            /// If the buffer was created with [`BufferUsages::UNIFORM`], then this
            /// offset must be a multiple of
            /// [`Limits::min_uniform_buffer_offset_alignment`].
            ///
            /// If the buffer was created with [`BufferUsages::STORAGE`], then this
            /// offset must be a multiple of
            /// [`Limits::min_storage_buffer_offset_alignment`].
            ///
            /// [`has_dynamic_offset`]: BindingType::Buffer::has_dynamic_offset
            pub offset: BufferAddress,
            /// Size of the binding in bytes, or `None` for using the rest of the buffer.
            pub size: Option<BufferSize>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BufferBinding<'a> {
            #[inline]
            fn clone(&self) -> BufferBinding<'a> {
                BufferBinding {
                    buffer: ::core::clone::Clone::clone(&self.buffer),
                    offset: ::core::clone::Clone::clone(&self.offset),
                    size: ::core::clone::Clone::clone(&self.size),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BufferBinding<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "BufferBinding",
                    "buffer",
                    &self.buffer,
                    "offset",
                    &self.offset,
                    "size",
                    &&self.size,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BufferBinding<'_>>();
        };
        /// An element of a [`BindGroupDescriptor`], consisting of a bindable resource
        /// and the slot to bind it to.
        ///
        /// Corresponds to [WebGPU `GPUBindGroupEntry`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgroupentry).
        pub struct BindGroupEntry<'a> {
            /// Slot for which binding provides resource. Corresponds to an entry of the same
            /// binding index in the [`BindGroupLayoutDescriptor`].
            pub binding: u32,
            /// Resource to attach to the binding
            pub resource: BindingResource<'a>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BindGroupEntry<'a> {
            #[inline]
            fn clone(&self) -> BindGroupEntry<'a> {
                BindGroupEntry {
                    binding: ::core::clone::Clone::clone(&self.binding),
                    resource: ::core::clone::Clone::clone(&self.resource),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BindGroupEntry<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "BindGroupEntry",
                    "binding",
                    &self.binding,
                    "resource",
                    &&self.resource,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BindGroupEntry<'_>>();
        };
        /// Describes a group of bindings and the resources to be bound.
        ///
        /// For use with [`Device::create_bind_group`].
        ///
        /// Corresponds to [WebGPU `GPUBindGroupDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgroupdescriptor).
        pub struct BindGroupDescriptor<'a> {
            /// Debug label of the bind group. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// The [`BindGroupLayout`] that corresponds to this bind group.
            pub layout: &'a BindGroupLayout,
            /// The resources to bind to this bind group.
            pub entries: &'a [BindGroupEntry<'a>],
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BindGroupDescriptor<'a> {
            #[inline]
            fn clone(&self) -> BindGroupDescriptor<'a> {
                BindGroupDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    layout: ::core::clone::Clone::clone(&self.layout),
                    entries: ::core::clone::Clone::clone(&self.entries),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BindGroupDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "BindGroupDescriptor",
                    "label",
                    &self.label,
                    "layout",
                    &self.layout,
                    "entries",
                    &&self.entries,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BindGroupDescriptor<'_>>();
        };
    }
    mod bind_group_layout {
        use crate::*;
        /// Handle to a binding group layout.
        ///
        /// A `BindGroupLayout` is a handle to the GPU-side layout of a binding group. It can be used to
        /// create a [`BindGroupDescriptor`] object, which in turn can be used to create a [`BindGroup`]
        /// object with [`Device::create_bind_group`]. A series of `BindGroupLayout`s can also be used to
        /// create a [`PipelineLayoutDescriptor`], which can be used to create a [`PipelineLayout`].
        ///
        /// It can be created with [`Device::create_bind_group_layout`].
        ///
        /// Corresponds to [WebGPU `GPUBindGroupLayout`](
        /// https://gpuweb.github.io/gpuweb/#gpubindgrouplayout).
        pub struct BindGroupLayout {
            pub(crate) inner: dispatch::DispatchBindGroupLayout,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BindGroupLayout {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "BindGroupLayout",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BindGroupLayout {
            #[inline]
            fn clone(&self) -> BindGroupLayout {
                BindGroupLayout {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BindGroupLayout>();
        };
        impl PartialEq for BindGroupLayout {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for BindGroupLayout {}
        impl PartialOrd for BindGroupLayout {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for BindGroupLayout {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for BindGroupLayout {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl BindGroupLayout {}
        /// Describes a [`BindGroupLayout`].
        ///
        /// For use with [`Device::create_bind_group_layout`].
        ///
        /// Corresponds to [WebGPU `GPUBindGroupLayoutDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgrouplayoutdescriptor).
        pub struct BindGroupLayoutDescriptor<'a> {
            /// Debug label of the bind group layout. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// Array of entries in this BindGroupLayout
            pub entries: &'a [BindGroupLayoutEntry],
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BindGroupLayoutDescriptor<'a> {
            #[inline]
            fn clone(&self) -> BindGroupLayoutDescriptor<'a> {
                BindGroupLayoutDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    entries: ::core::clone::Clone::clone(&self.entries),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BindGroupLayoutDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "BindGroupLayoutDescriptor",
                    "label",
                    &self.label,
                    "entries",
                    &&self.entries,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BindGroupLayoutDescriptor<'_>>();
        };
    }
    mod blas {
        use core::ops::Deref;
        use alloc::{boxed::Box, vec::Vec};
        use wgt::{WasmNotSend, WasmNotSendSync};
        use crate::dispatch;
        use crate::{Buffer, Label};
        /// Descriptor for the size defining attributes of a triangle geometry, for a bottom level acceleration structure.
        pub type BlasTriangleGeometrySizeDescriptor = wgt::BlasTriangleGeometrySizeDescriptor;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BlasTriangleGeometrySizeDescriptor>();
        };
        /// Descriptor for the size defining attributes, for a bottom level acceleration structure.
        pub type BlasGeometrySizeDescriptors = wgt::BlasGeometrySizeDescriptors;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BlasGeometrySizeDescriptors>();
        };
        /// Flags for an acceleration structure.
        pub type AccelerationStructureFlags = wgt::AccelerationStructureFlags;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<AccelerationStructureFlags>();
        };
        /// Flags for a geometry inside a bottom level acceleration structure.
        pub type AccelerationStructureGeometryFlags = wgt::AccelerationStructureGeometryFlags;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<AccelerationStructureGeometryFlags>();
        };
        /// Update mode for acceleration structure builds.
        pub type AccelerationStructureUpdateMode = wgt::AccelerationStructureUpdateMode;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<AccelerationStructureUpdateMode>();
        };
        /// Descriptor to create bottom level acceleration structures.
        pub type CreateBlasDescriptor<'a> = wgt::CreateBlasDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<CreateBlasDescriptor<'_>>();
        };
        /// Safe instance for a [Tlas].
        ///
        /// A TlasInstance may be made invalid, if a TlasInstance is invalid, any attempt to build a [Tlas] containing an
        /// invalid TlasInstance will generate a validation error
        ///
        /// Each one contains:
        /// - A reference to a BLAS, this ***must*** be interacted with using [TlasInstance::new] or [TlasInstance::set_blas], a
        ///   TlasInstance that references a BLAS keeps that BLAS from being dropped
        /// - A user accessible transformation matrix
        /// - A user accessible mask
        /// - A user accessible custom index
        ///
        /// [Tlas]: crate::Tlas
        pub struct TlasInstance {
            pub(crate) blas: dispatch::DispatchBlas,
            /// Affine transform matrix 3x4 (rows x columns, row major order).
            pub transform: [f32; 12],
            /// Custom index for the instance used inside the shader.
            ///
            /// This must only use the lower 24 bits, if any bits are outside that range (byte 4 does not equal 0) the TlasInstance becomes
            /// invalid and generates a validation error when built
            pub custom_data: u32,
            /// Mask for the instance used inside the shader to filter instances.
            /// Reports hit only if `(shader_cull_mask & tlas_instance.mask) != 0u`.
            pub mask: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TlasInstance {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "TlasInstance",
                    "blas",
                    &self.blas,
                    "transform",
                    &self.transform,
                    "custom_data",
                    &self.custom_data,
                    "mask",
                    &&self.mask,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TlasInstance {
            #[inline]
            fn clone(&self) -> TlasInstance {
                TlasInstance {
                    blas: ::core::clone::Clone::clone(&self.blas),
                    transform: ::core::clone::Clone::clone(&self.transform),
                    custom_data: ::core::clone::Clone::clone(&self.custom_data),
                    mask: ::core::clone::Clone::clone(&self.mask),
                }
            }
        }
        impl TlasInstance {
            /// Construct TlasInstance.
            /// - blas: Reference to the bottom level acceleration structure
            /// - transform: Transform buffer offset in bytes (optional, required if transform buffer is present)
            /// - custom_data: Custom index for the instance used inside the shader (max 24 bits)
            /// - mask: Mask for the instance used inside the shader to filter instances
            ///
            /// Note: while one of these contains a reference to a BLAS that BLAS will not be dropped,
            /// but it can still be destroyed. Destroying a BLAS that is referenced by one or more
            /// TlasInstance(s) will immediately make them invalid. If one or more of those invalid
            /// TlasInstances is inside a TlasPackage that is attempted to be built, the build will
            /// generate a validation error.
            pub fn new(
                blas: &Blas,
                transform: [f32; 12],
                custom_data: u32,
                mask: u8,
            ) -> Self {
                Self {
                    blas: blas.inner.clone(),
                    transform,
                    custom_data,
                    mask,
                }
            }
            /// Set the bottom level acceleration structure.
            ///
            /// See the note on [TlasInstance] about the
            /// guarantees of keeping a BLAS alive.
            pub fn set_blas(&mut self, blas: &Blas) {
                self.blas = blas.inner.clone();
            }
        }
        /// Definition for a triangle geometry for a Bottom Level Acceleration Structure (BLAS).
        ///
        /// The size must match the rest of the structures fields, otherwise the build will fail.
        /// (e.g. if index count is present in the size, the index buffer must be present as well.)
        pub struct BlasTriangleGeometry<'a> {
            /// Sub descriptor for the size defining attributes of a triangle geometry.
            pub size: &'a BlasTriangleGeometrySizeDescriptor,
            /// Vertex buffer.
            pub vertex_buffer: &'a Buffer,
            /// Offset into the vertex buffer as a factor of the vertex stride.
            pub first_vertex: u32,
            /// Vertex stride, must be greater than [`wgpu_types::VertexFormat::min_acceleration_structure_vertex_stride`]
            /// of the format and must be a multiple of [`wgpu_types::VertexFormat::acceleration_structure_stride_alignment`].
            pub vertex_stride: wgt::BufferAddress,
            /// Index buffer (optional).
            pub index_buffer: Option<&'a Buffer>,
            /// Number of indexes to skip in the index buffer (optional, required if index buffer is present).
            pub first_index: Option<u32>,
            /// Transform buffer containing 3x4 (rows x columns, row major) affine transform matrices `[f32; 12]` (optional).
            pub transform_buffer: Option<&'a Buffer>,
            /// Transform buffer offset in bytes (optional, required if transform buffer is present).
            pub transform_buffer_offset: Option<wgt::BufferAddress>,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BlasTriangleGeometry<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "size",
                    "vertex_buffer",
                    "first_vertex",
                    "vertex_stride",
                    "index_buffer",
                    "first_index",
                    "transform_buffer",
                    "transform_buffer_offset",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.size,
                    &self.vertex_buffer,
                    &self.first_vertex,
                    &self.vertex_stride,
                    &self.index_buffer,
                    &self.first_index,
                    &self.transform_buffer,
                    &&self.transform_buffer_offset,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "BlasTriangleGeometry",
                    names,
                    values,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + WasmNotSendSync>() {}
            assert_impl_all::<BlasTriangleGeometry<'_>>();
        };
        /// Contains the sets of geometry that go into a [Blas].
        pub enum BlasGeometries<'a> {
            /// Triangle geometry variant.
            TriangleGeometries(Vec<BlasTriangleGeometry<'a>>),
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + WasmNotSendSync>() {}
            assert_impl_all::<BlasGeometries<'_>>();
        };
        /// Builds the given sets of geometry into the given [Blas].
        pub struct BlasBuildEntry<'a> {
            /// Reference to the acceleration structure.
            pub blas: &'a Blas,
            /// Geometries.
            pub geometry: BlasGeometries<'a>,
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + WasmNotSendSync>() {}
            assert_impl_all::<BlasBuildEntry<'_>>();
        };
        /// Bottom Level Acceleration Structure (BLAS).
        ///
        /// A BLAS is a device-specific raytracing acceleration structure that contains geometry data.
        ///
        /// These BLASes are combined with transform in a [TlasInstance] to create a [Tlas].
        ///
        /// [Tlas]: crate::Tlas
        pub struct Blas {
            pub(crate) handle: Option<u64>,
            pub(crate) inner: dispatch::DispatchBlas,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Blas {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Blas",
                    "handle",
                    &self.handle,
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Blas {
            #[inline]
            fn clone(&self) -> Blas {
                Blas {
                    handle: ::core::clone::Clone::clone(&self.handle),
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + WasmNotSendSync>() {}
            assert_impl_all::<Blas>();
        };
        impl PartialEq for Blas {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Blas {}
        impl PartialOrd for Blas {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Blas {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Blas {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Blas {
            /// Raw handle to the acceleration structure, used inside raw instance buffers.
            pub fn handle(&self) -> Option<u64> {
                self.handle
            }
            /// Get the [`wgpu_hal`] acceleration structure from this `Blas`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::AccelerationStructure`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::AccelerationStructure`
            ///- [`hal::api::Metal`] uses [`hal::metal::AccelerationStructure`]
            ///- `hal::api::Dx12` uses `hal::dx12::AccelerationStructure`
            ///- `hal::api::Gles` uses `hal::gles::AccelerationStructure`
            ///
            /// # Deadlocks
            ///
            /// - The returned guard holds a read-lock on a device-local "destruction"
            ///   lock, which will cause all calls to `destroy` to block until the
            ///   guard is released.
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The acceleration structure is not from the backend specified by `A`.
            /// - The acceleration structure is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::AccelerationStructure`]: hal::Api::AccelerationStructure
            pub unsafe fn as_hal<A: hal::Api>(
                &mut self,
            ) -> Option<
                impl Deref<Target = A::AccelerationStructure> + WasmNotSendSync,
            > {
                let blas = self.inner.as_core_opt()?;
                unsafe { blas.context.blas_as_hal::<A>(blas) }
            }
        }
        /// Context version of [BlasTriangleGeometry].
        pub struct ContextBlasTriangleGeometry<'a> {
            #[expect(dead_code)]
            pub(crate) size: &'a BlasTriangleGeometrySizeDescriptor,
            #[expect(dead_code)]
            pub(crate) vertex_buffer: &'a dispatch::DispatchBuffer,
            #[expect(dead_code)]
            pub(crate) index_buffer: Option<&'a dispatch::DispatchBuffer>,
            #[expect(dead_code)]
            pub(crate) transform_buffer: Option<&'a dispatch::DispatchBuffer>,
            #[expect(dead_code)]
            pub(crate) first_vertex: u32,
            #[expect(dead_code)]
            pub(crate) vertex_stride: wgt::BufferAddress,
            #[expect(dead_code)]
            pub(crate) index_buffer_offset: Option<wgt::BufferAddress>,
            #[expect(dead_code)]
            pub(crate) transform_buffer_offset: Option<wgt::BufferAddress>,
        }
        /// Context version of [BlasGeometries].
        pub enum ContextBlasGeometries<'a> {
            /// Triangle geometries.
            TriangleGeometries(
                Box<dyn Iterator<Item = ContextBlasTriangleGeometry<'a>> + 'a>,
            ),
        }
        /// Context version see [BlasBuildEntry].
        pub struct ContextBlasBuildEntry<'a> {
            #[expect(dead_code)]
            pub(crate) blas: &'a dispatch::DispatchBlas,
            #[expect(dead_code)]
            pub(crate) geometries: ContextBlasGeometries<'a>,
        }
        /// Error occurred when trying to asynchronously prepare a blas for compaction.
        pub struct BlasAsyncError;
        #[automatically_derived]
        impl ::core::clone::Clone for BlasAsyncError {
            #[inline]
            fn clone(&self) -> BlasAsyncError {
                BlasAsyncError
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for BlasAsyncError {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for BlasAsyncError {
            #[inline]
            fn eq(&self, other: &BlasAsyncError) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for BlasAsyncError {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BlasAsyncError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "BlasAsyncError")
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BlasAsyncError>();
        };
        impl core::fmt::Display for BlasAsyncError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(
                    format_args!(
                        "Error occurred when trying to asynchronously prepare a blas for compaction",
                    ),
                )
            }
        }
        impl core::error::Error for BlasAsyncError {}
        impl Blas {
            /// Asynchronously prepares this BLAS for compaction. The callback is called once all builds
            /// using this BLAS are finished and the BLAS is compactable. This can be checked using
            /// [`Blas::ready_for_compaction`]. Rebuilding this BLAS will reset its compacted state, and it
            /// will need to be prepared again.
            ///
            /// ### Interaction with other functions
            /// On native, `queue.submit(..)` and polling devices (that is calling `instance.poll_all` or
            /// `device.poll`) with [`PollType::Poll`] may call the callback. On native, polling devices with
            /// [`PollType::Wait`] (optionally with a submission index greater
            /// than the last submit the BLAS was used in) will guarantee callback is called.
            ///
            /// [`PollType::Poll`]: wgpu_types::PollType::Poll
            /// [`PollType::Wait`]: wgpu_types::PollType::Wait
            pub fn prepare_compaction_async(
                &self,
                callback: impl FnOnce(Result<(), BlasAsyncError>) + WasmNotSend + 'static,
            ) {
                self.inner.prepare_compact_async(Box::new(callback));
            }
            /// Checks whether this BLAS is ready for compaction. The returned value is `true` if
            /// [`Blas::prepare_compaction_async`]'s callback was called with a non-error value, otherwise
            /// this is `false`.
            pub fn ready_for_compaction(&self) -> bool {
                self.inner.ready_for_compaction()
            }
        }
    }
    mod buffer {
        use alloc::{boxed::Box, sync::Arc, vec::Vec};
        use core::{error, fmt, ops::{Bound, Deref, DerefMut, Range, RangeBounds}};
        use crate::util::Mutex;
        use crate::*;
        /// Handle to a GPU-accessible buffer.
        ///
        /// A `Buffer` is a memory allocation for use by the GPU, somewhat analogous to
        /// <code>[Box]&lt;[\[u8\]][primitive@slice]&gt;</code> in Rust.
        /// The contents of buffers are untyped bytes; it is up to the application to
        /// specify the interpretation of the bytes when the buffer is used, in ways
        /// such as [`VertexBufferLayout`].
        /// A single buffer can be used to hold multiple independent pieces of data at
        /// different offsets (e.g. both vertices and indices for one or more meshes).
        ///
        /// A `Buffer`'s bytes have "interior mutability": functions like
        /// [`Queue::write_buffer`] or [mapping] a buffer for writing only require a
        /// `&Buffer`, not a `&mut Buffer`, even though they modify its contents. `wgpu`
        /// prevents simultaneous reads and writes of buffer contents using run-time
        /// checks.
        ///
        /// Created with [`Device::create_buffer()`] or
        /// [`DeviceExt::create_buffer_init()`].
        ///
        /// Corresponds to [WebGPU `GPUBuffer`](https://gpuweb.github.io/gpuweb/#buffer-interface).
        ///
        /// [mapping]: Buffer#mapping-buffers
        ///
        /// # How to get your data into a buffer
        ///
        /// Every `Buffer` starts with all bytes zeroed.
        /// There are many ways to load data into a `Buffer`:
        ///
        /// - When creating a buffer, you may set the [`mapped_at_creation`][mac] flag,
        ///   then write to its [`get_mapped_range_mut()`][Buffer::get_mapped_range_mut].
        ///   This only works when the buffer is created and has not yet been used by
        ///   the GPU, but it is all you need for buffers whose contents do not change
        ///   after creation.
        ///   - You may use [`DeviceExt::create_buffer_init()`] as a convenient way to
        ///     do that and copy data from a `&[u8]` you provide.
        /// - After creation, you may use [`Buffer::map_async()`] to map it again;
        ///   however, you then need to wait until the GPU is no longer using the buffer
        ///   before you begin writing.
        /// - You may use [`CommandEncoder::copy_buffer_to_buffer()`] to copy data into
        ///   this buffer from another buffer.
        /// - You may use [`Queue::write_buffer()`] to copy data into the buffer from a
        ///   `&[u8]`. This uses a temporary “staging” buffer managed by `wgpu` to hold
        ///   the data.
        ///   - [`Queue::write_buffer_with()`] allows you to write directly into temporary
        ///     storage instead of providing a slice you already prepared, which may
        ///     allow *your* code to save the allocation of a [`Vec`] or such.
        /// - You may use [`util::StagingBelt`] to manage a set of temporary buffers.
        ///   This may be more efficient than [`Queue::write_buffer_with()`] when you
        ///   have many small copies to perform, but requires more steps to use, and
        ///   tuning of the belt buffer size.
        /// - You may write your own staging buffer management customized to your
        ///   application, based on mapped buffers and
        ///   [`CommandEncoder::copy_buffer_to_buffer()`].
        /// - A GPU computation’s results can be stored in a buffer:
        ///   - A [compute shader][ComputePipeline] may write to a buffer bound as a
        ///     [storage buffer][BufferBindingType::Storage].
        ///   - A render pass may render to a texture which is then copied to a buffer
        ///     using [`CommandEncoder::copy_texture_to_buffer()`].
        ///
        /// # Mapping buffers
        ///
        /// If a `Buffer` is created with the appropriate [`usage`], it can be *mapped*:
        /// you can make its contents accessible to the CPU as an ordinary `&[u8]` or
        /// `&mut [u8]` slice of bytes. Buffers created with the
        /// [`mapped_at_creation`][mac] flag set are also mapped initially.
        ///
        /// Depending on the hardware, the buffer could be memory shared between CPU and
        /// GPU, so that the CPU has direct access to the same bytes the GPU will
        /// consult; or it may be ordinary CPU memory, whose contents the system must
        /// copy to/from the GPU as needed. This crate's API is designed to work the
        /// same way in either case: at any given time, a buffer is either mapped and
        /// available to the CPU, or unmapped and ready for use by the GPU, but never
        /// both. This makes it impossible for either side to observe changes by the
        /// other immediately, and any necessary transfers can be carried out when the
        /// buffer transitions from one state to the other.
        ///
        /// There are two ways to map a buffer:
        ///
        /// - If [`BufferDescriptor::mapped_at_creation`] is `true`, then the entire
        ///   buffer is mapped when it is created. This is the easiest way to initialize
        ///   a new buffer. You can set `mapped_at_creation` on any kind of buffer,
        ///   regardless of its [`usage`] flags.
        ///
        /// - If the buffer's [`usage`] includes the [`MAP_READ`] or [`MAP_WRITE`]
        ///   flags, then you can call `buffer.slice(range).map_async(mode, callback)`
        ///   to map the portion of `buffer` given by `range`. This waits for the GPU to
        ///   finish using the buffer, and invokes `callback` as soon as the buffer is
        ///   safe for the CPU to access.
        ///
        /// Once a buffer is mapped:
        ///
        /// - You can call `buffer.slice(range).get_mapped_range()` to obtain a
        ///   [`BufferView`], which dereferences to a `&[u8]` that you can use to read
        ///   the buffer's contents.
        ///
        /// - Or, you can call `buffer.slice(range).get_mapped_range_mut()` to obtain a
        ///   [`BufferViewMut`], which dereferences to a `&mut [u8]` that you can use to
        ///   read and write the buffer's contents.
        ///
        /// The given `range` must fall within the mapped portion of the buffer. If you
        /// attempt to access overlapping ranges, even for shared access only, these
        /// methods panic.
        ///
        /// While a buffer is mapped, you may not submit any commands to the GPU that
        /// access it. You may record command buffers that use the buffer, but if you
        /// submit them while the buffer is mapped, submission will panic.
        ///
        /// When you are done using the buffer on the CPU, you must call
        /// [`Buffer::unmap`] to make it available for use by the GPU again. All
        /// [`BufferView`] and [`BufferViewMut`] views referring to the buffer must be
        /// dropped before you unmap it; otherwise, [`Buffer::unmap`] will panic.
        ///
        /// # Example
        ///
        /// If `buffer` was created with [`BufferUsages::MAP_WRITE`], we could fill it
        /// with `f32` values like this:
        ///
        /// ```
        /// # #[cfg(feature = "noop")]
        /// # let (device, _queue) = wgpu::Device::noop(&wgpu::DeviceDescriptor::default());
        /// # #[cfg(not(feature = "noop"))]
        /// # let device: wgpu::Device = { return; };
        /// #
        /// # let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        /// #     label: None,
        /// #     size: 400,
        /// #     usage: wgpu::BufferUsages::MAP_WRITE,
        /// #     mapped_at_creation: false,
        /// # });
        /// let capturable = buffer.clone();
        /// buffer.map_async(wgpu::MapMode::Write, .., move |result| {
        ///     if result.is_ok() {
        ///         let mut view = capturable.get_mapped_range_mut(..);
        ///         let floats: &mut [f32] = bytemuck::cast_slice_mut(&mut view);
        ///         floats.fill(42.0);
        ///         drop(view);
        ///         capturable.unmap();
        ///     }
        /// });
        /// ```
        ///
        /// This code takes the following steps:
        ///
        /// - First, it makes a cloned handle to the buffer for capture by
        ///   the callback passed to [`map_async`]. Since a [`map_async`] callback may be
        ///   invoked from another thread, interaction between the callback and the
        ///   thread calling [`map_async`] generally requires some sort of shared heap
        ///   data like this. In real code, there might be an [`Arc`] to some larger
        ///   structure that itself owns `buffer`.
        ///
        /// - Then, it calls [`Buffer::slice`] to make a [`BufferSlice`] referring to
        ///   the buffer's entire contents.
        ///
        /// - Next, it calls [`BufferSlice::map_async`] to request that the bytes to
        ///   which the slice refers be made accessible to the CPU ("mapped"). This may
        ///   entail waiting for previously enqueued operations on `buffer` to finish.
        ///   Although [`map_async`] itself always returns immediately, it saves the
        ///   callback function to be invoked later.
        ///
        /// - When some later call to [`Device::poll`] or [`Instance::poll_all`] (not
        ///   shown in this example) determines that the buffer is mapped and ready for
        ///   the CPU to use, it invokes the callback function.
        ///
        /// - The callback function calls [`Buffer::slice`] and then
        ///   [`BufferSlice::get_mapped_range_mut`] to obtain a [`BufferViewMut`], which
        ///   dereferences to a `&mut [u8]` slice referring to the buffer's bytes.
        ///
        /// - It then uses the [`bytemuck`] crate to turn the `&mut [u8]` into a `&mut
        ///   [f32]`, and calls the slice [`fill`] method to fill the buffer with a
        ///   useful value.
        ///
        /// - Finally, the callback drops the view and calls [`Buffer::unmap`] to unmap
        ///   the buffer. In real code, the callback would also need to do some sort of
        ///   synchronization to let the rest of the program know that it has completed
        ///   its work.
        ///
        /// If using [`map_async`] directly is awkward, you may find it more convenient to
        /// use [`Queue::write_buffer`] and [`util::DownloadBuffer::read_buffer`].
        /// However, those each have their own tradeoffs; the asynchronous nature of GPU
        /// execution makes it hard to avoid friction altogether.
        ///
        /// [`Arc`]: std::sync::Arc
        /// [`map_async`]: BufferSlice::map_async
        /// [`bytemuck`]: https://crates.io/crates/bytemuck
        /// [`fill`]: slice::fill
        ///
        /// ## Mapping buffers on the web
        ///
        /// When compiled to WebAssembly and running in a browser content process,
        /// `wgpu` implements its API in terms of the browser's WebGPU implementation.
        /// In this context, `wgpu` is further isolated from the GPU:
        ///
        /// - Depending on the browser's WebGPU implementation, mapping and unmapping
        ///   buffers probably entails copies between WebAssembly linear memory and the
        ///   graphics driver's buffers.
        ///
        /// - All modern web browsers isolate web content in its own sandboxed process,
        ///   which can only interact with the GPU via interprocess communication (IPC).
        ///   Although most browsers' IPC systems use shared memory for large data
        ///   transfers, there will still probably need to be copies into and out of the
        ///   shared memory buffers.
        ///
        /// All of these copies contribute to the cost of buffer mapping in this
        /// configuration.
        ///
        /// [`usage`]: BufferDescriptor::usage
        /// [mac]: BufferDescriptor::mapped_at_creation
        /// [`MAP_READ`]: BufferUsages::MAP_READ
        /// [`MAP_WRITE`]: BufferUsages::MAP_WRITE
        /// [`DeviceExt::create_buffer_init()`]: util::DeviceExt::create_buffer_init
        pub struct Buffer {
            pub(crate) inner: dispatch::DispatchBuffer,
            pub(crate) map_context: Arc<Mutex<MapContext>>,
            pub(crate) size: wgt::BufferAddress,
            pub(crate) usage: BufferUsages,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Buffer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Buffer",
                    "inner",
                    &self.inner,
                    "map_context",
                    &self.map_context,
                    "size",
                    &self.size,
                    "usage",
                    &&self.usage,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Buffer {
            #[inline]
            fn clone(&self) -> Buffer {
                Buffer {
                    inner: ::core::clone::Clone::clone(&self.inner),
                    map_context: ::core::clone::Clone::clone(&self.map_context),
                    size: ::core::clone::Clone::clone(&self.size),
                    usage: ::core::clone::Clone::clone(&self.usage),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Buffer>();
        };
        impl PartialEq for Buffer {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Buffer {}
        impl PartialOrd for Buffer {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Buffer {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Buffer {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Buffer {
            /// Return the binding view of the entire buffer.
            pub fn as_entire_binding(&self) -> BindingResource<'_> {
                BindingResource::Buffer(self.as_entire_buffer_binding())
            }
            /// Return the binding view of the entire buffer.
            pub fn as_entire_buffer_binding(&self) -> BufferBinding<'_> {
                BufferBinding {
                    buffer: self,
                    offset: 0,
                    size: None,
                }
            }
            /// Get the [`wgpu_hal`] buffer from this `Buffer`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Buffer`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Buffer`
            ///- [`hal::api::Metal`] uses [`hal::metal::Buffer`]
            ///- `hal::api::Dx12` uses `hal::dx12::Buffer`
            ///- `hal::api::Gles` uses `hal::gles::Buffer`
            ///
            /// # Deadlocks
            ///
            /// - The returned guard holds a read-lock on a device-local "destruction"
            ///   lock, which will cause all calls to `destroy` to block until the
            ///   guard is released.
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The buffer is not from the backend specified by `A`.
            /// - The buffer is from the `webgpu` or `custom` backend.
            /// - The buffer has had [`Self::destroy()`] called on it.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Buffer`]: hal::Api::Buffer
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::Buffer> + WasmNotSendSync> {
                let buffer = self.inner.as_core_opt()?;
                unsafe { buffer.context.buffer_as_hal::<A>(buffer) }
            }
            /// Returns a [`BufferSlice`] referring to the portion of `self`'s contents
            /// indicated by `bounds`. Regardless of what sort of data `self` stores,
            /// `bounds` start and end are given in bytes.
            ///
            /// A [`BufferSlice`] can be used to supply vertex and index data, or to map
            /// buffer contents for access from the CPU. See the [`BufferSlice`]
            /// documentation for details.
            ///
            /// The `range` argument can be half or fully unbounded: for example,
            /// `buffer.slice(..)` refers to the entire buffer, and `buffer.slice(n..)`
            /// refers to the portion starting at the `n`th byte and extending to the
            /// end of the buffer.
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside of the bounds of `self`.
            /// - If `bounds` has a length less than 1.
            #[track_caller]
            pub fn slice<S: RangeBounds<BufferAddress>>(
                &self,
                bounds: S,
            ) -> BufferSlice<'_> {
                let (offset, size) = range_to_offset_size(bounds, self.size);
                check_buffer_bounds(self.size, offset, size);
                BufferSlice {
                    buffer: self,
                    offset,
                    size,
                }
            }
            /// Unmaps the buffer from host memory.
            ///
            /// This terminates the effect of all previous [`map_async()`](Self::map_async) operations and
            /// makes the buffer available for use by the GPU again.
            pub fn unmap(&self) {
                self.map_context.lock().reset();
                self.inner.unmap();
            }
            /// Destroy the associated native resources as soon as possible.
            pub fn destroy(&self) {
                self.inner.destroy();
            }
            /// Returns the length of the buffer allocation in bytes.
            ///
            /// This is always equal to the `size` that was specified when creating the buffer.
            pub fn size(&self) -> BufferAddress {
                self.size
            }
            /// Returns the allowed usages for this `Buffer`.
            ///
            /// This is always equal to the `usage` that was specified when creating the buffer.
            pub fn usage(&self) -> BufferUsages {
                self.usage
            }
            /// Map the buffer to host (CPU) memory, making it available for reading or writing via
            /// [`get_mapped_range()`](Self::get_mapped_range). The buffer becomes accessible once the
            /// `callback` is invoked with [`Ok`].
            ///
            /// Use this when you want to map the buffer immediately. If you need to submit GPU work that
            /// uses the buffer before mapping it, use `map_buffer_on_submit` on
            /// [`CommandEncoder`][CEmbos], [`CommandBuffer`][CBmbos], [`RenderPass`][RPmbos], or
            /// [`ComputePass`][CPmbos] to schedule the mapping after submission. This avoids extra calls to
            /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and lets you initiate mapping from a
            /// more convenient place.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated into
            /// an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions after the GPU work
            /// completes. There are no restrictions on the code you can run in the callback; however, on native
            /// the polling call will not return until the callback finishes, so keep callbacks short (set flags,
            /// send messages, etc.).
            ///
            /// While a buffer is mapped, it cannot be used by other commands; at any time, either the GPU or
            /// the CPU has exclusive access to the buffer’s contents.
            ///
            /// This can also be performed using [`BufferSlice::map_async()`].
            ///
            /// # Panics
            ///
            /// - If the buffer is already mapped.
            /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
            /// - If `bounds` is outside of the bounds of `self`.
            /// - If `bounds` has a length less than 1.
            /// - If the start and end of `bounds` are not be aligned to [`MAP_ALIGNMENT`].
            ///
            /// [CEmbos]: CommandEncoder::map_buffer_on_submit
            /// [CBmbos]: CommandBuffer::map_buffer_on_submit
            /// [RPmbos]: RenderPass::map_buffer_on_submit
            /// [CPmbos]: ComputePass::map_buffer_on_submit
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn map_async<S: RangeBounds<BufferAddress>>(
                &self,
                mode: MapMode,
                bounds: S,
                callback: impl FnOnce(
                    Result<(), BufferAsyncError>,
                ) + WasmNotSend + 'static,
            ) {
                self.slice(bounds).map_async(mode, callback)
            }
            /// Gain read-only access to the bytes of a [mapped] [`Buffer`].
            ///
            /// Returns a [`BufferView`] referring to the buffer range represented by
            /// `self`. See the documentation for [`BufferView`] for details.
            ///
            /// `bounds` may be less than the bounds passed to [`Self::map_async()`],
            /// and multiple views may be obtained and used simultaneously as long as they do not overlap.
            ///
            /// This can also be performed using [`BufferSlice::get_mapped_range()`].
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside of the bounds of `self`.
            /// - If `bounds` has a length less than 1.
            /// - If the start and end of `bounds` are not aligned to [`MAP_ALIGNMENT`].
            /// - If the buffer to which `self` refers is not currently [mapped].
            /// - If you try to create a view which overlaps an existing [`BufferViewMut`].
            ///
            /// [mapped]: Buffer#mapping-buffers
            #[track_caller]
            pub fn get_mapped_range<S: RangeBounds<BufferAddress>>(
                &self,
                bounds: S,
            ) -> BufferView {
                self.slice(bounds).get_mapped_range()
            }
            /// Gain write access to the bytes of a [mapped] [`Buffer`].
            ///
            /// Returns a [`BufferViewMut`] referring to the buffer range represented by
            /// `self`. See the documentation for [`BufferViewMut`] for more details.
            ///
            /// `bounds` may be less than the bounds passed to [`Self::map_async()`],
            /// and multiple views may be obtained and used simultaneously as long as they do not overlap.
            ///
            /// This can also be performed using [`BufferSlice::get_mapped_range_mut()`].
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside of the bounds of `self`.
            /// - If `bounds` has a length less than 1.
            /// - If the start and end of `bounds` are not aligned to [`MAP_ALIGNMENT`].
            /// - If the buffer to which `self` refers is not currently [mapped].
            /// - If you try to create a view which overlaps an existing [`BufferView`] or [`BufferViewMut`].
            ///
            /// [mapped]: Buffer#mapping-buffers
            #[track_caller]
            pub fn get_mapped_range_mut<S: RangeBounds<BufferAddress>>(
                &self,
                bounds: S,
            ) -> BufferViewMut {
                self.slice(bounds).get_mapped_range_mut()
            }
        }
        /// A slice of a [`Buffer`], to be mapped, used for vertex or index data, or the like.
        ///
        /// You can create a `BufferSlice` by calling [`Buffer::slice`]:
        ///
        /// ```no_run
        /// # let buffer: wgpu::Buffer = todo!();
        /// let slice = buffer.slice(10..20);
        /// ```
        ///
        /// This returns a slice referring to the second ten bytes of `buffer`. To get a
        /// slice of the entire `Buffer`:
        ///
        /// ```no_run
        /// # let buffer: wgpu::Buffer = todo!();
        /// let whole_buffer_slice = buffer.slice(..);
        /// ```
        ///
        /// You can pass buffer slices to methods like [`RenderPass::set_vertex_buffer`]
        /// and [`RenderPass::set_index_buffer`] to indicate which portion of the buffer
        /// a draw call should consult. You can also convert it to a [`BufferBinding`]
        /// with `.into()`.
        ///
        /// To access the slice's contents on the CPU, you must first [map] the buffer,
        /// and then call [`BufferSlice::get_mapped_range`] or
        /// [`BufferSlice::get_mapped_range_mut`] to obtain a view of the slice's
        /// contents. See the documentation on [mapping][map] for more details,
        /// including example code.
        ///
        /// Unlike a Rust shared slice `&[T]`, whose existence guarantees that
        /// nobody else is modifying the `T` values to which it refers, a
        /// [`BufferSlice`] doesn't guarantee that the buffer's contents aren't
        /// changing. You can still record and submit commands operating on the
        /// buffer while holding a [`BufferSlice`]. A [`BufferSlice`] simply
        /// represents a certain range of the buffer's bytes.
        ///
        /// The `BufferSlice` type is unique to the Rust API of `wgpu`. In the WebGPU
        /// specification, an offset and size are specified as arguments to each call
        /// working with the [`Buffer`], instead.
        ///
        /// [map]: Buffer#mapping-buffers
        pub struct BufferSlice<'a> {
            pub(crate) buffer: &'a Buffer,
            pub(crate) offset: BufferAddress,
            pub(crate) size: BufferSize,
        }
        #[automatically_derived]
        impl<'a> ::core::marker::Copy for BufferSlice<'a> {}
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BufferSlice<'a> {
            #[inline]
            fn clone(&self) -> BufferSlice<'a> {
                let _: ::core::clone::AssertParamIsClone<&'a Buffer>;
                let _: ::core::clone::AssertParamIsClone<BufferAddress>;
                let _: ::core::clone::AssertParamIsClone<BufferSize>;
                *self
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BufferSlice<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "BufferSlice",
                    "buffer",
                    &self.buffer,
                    "offset",
                    &self.offset,
                    "size",
                    &&self.size,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::marker::StructuralPartialEq for BufferSlice<'a> {}
        #[automatically_derived]
        impl<'a> ::core::cmp::PartialEq for BufferSlice<'a> {
            #[inline]
            fn eq(&self, other: &BufferSlice<'a>) -> bool {
                self.buffer == other.buffer && self.offset == other.offset
                    && self.size == other.size
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BufferSlice<'_>>();
        };
        impl<'a> BufferSlice<'a> {
            /// Return another [`BufferSlice`] referring to the portion of `self`'s contents
            /// indicated by `bounds`.
            ///
            /// The `range` argument can be half or fully unbounded: for example,
            /// `buffer.slice(..)` refers to the entire buffer, and `buffer.slice(n..)`
            /// refers to the portion starting at the `n`th byte and extending to the
            /// end of the buffer.
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside of the bounds of `self`.
            /// - If `bounds` has a length less than 1.
            #[track_caller]
            pub fn slice<S: RangeBounds<BufferAddress>>(
                &self,
                bounds: S,
            ) -> BufferSlice<'a> {
                let (offset, size) = range_to_offset_size(bounds, self.size.get());
                check_buffer_bounds(self.size.get(), offset, size);
                BufferSlice {
                    buffer: self.buffer,
                    offset: self.offset + offset,
                    size,
                }
            }
            /// Map the buffer to host (CPU) memory, making it available for reading or writing via
            /// [`get_mapped_range()`](Self::get_mapped_range). The buffer becomes accessible once the
            /// `callback` is invoked with [`Ok`].
            ///
            /// Use this when you want to map the buffer immediately. If you need to submit GPU work that
            /// uses the buffer before mapping it, use `map_buffer_on_submit` on
            /// [`CommandEncoder`][CEmbos], [`CommandBuffer`][CBmbos], [`RenderPass`][RPmbos], or
            /// [`ComputePass`][CPmbos] to schedule the mapping after submission. This avoids extra calls to
            /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and lets you initiate mapping from a
            /// more convenient place.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated into
            /// an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions after the GPU work
            /// completes. There are no restrictions on the code you can run in the callback; however, on native
            /// the polling call will not return until the callback finishes, so keep callbacks short (set flags,
            /// send messages, etc.).
            ///
            /// While a buffer is mapped, it cannot be used by other commands; at any time, either the GPU or
            /// the CPU has exclusive access to the buffer’s contents.
            ///
            /// This can also be performed using [`Buffer::map_async()`].
            ///
            /// # Panics
            ///
            /// - If the buffer is already mapped.
            /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`] within the buffer.
            ///
            /// [CEmbos]: CommandEncoder::map_buffer_on_submit
            /// [CBmbos]: CommandBuffer::map_buffer_on_submit
            /// [RPmbos]: RenderPass::map_buffer_on_submit
            /// [CPmbos]: ComputePass::map_buffer_on_submit
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn map_async(
                &self,
                mode: MapMode,
                callback: impl FnOnce(
                    Result<(), BufferAsyncError>,
                ) + WasmNotSend + 'static,
            ) {
                let mut mc = self.buffer.map_context.lock();
                match (&mc.mapped_range, &(0..0)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::Some(
                                    format_args!("Buffer is already mapped"),
                                ),
                            );
                        }
                    }
                };
                let end = self.offset + self.size.get();
                mc.mapped_range = self.offset..end;
                self.buffer.inner.map_async(mode, self.offset..end, Box::new(callback));
            }
            /// Gain read-only access to the bytes of a [mapped] [`Buffer`].
            ///
            /// Returns a [`BufferView`] referring to the buffer range represented by
            /// `self`. See the documentation for [`BufferView`] for details.
            ///
            /// Multiple views may be obtained and used simultaneously as long as they are from
            /// non-overlapping slices.
            ///
            /// This can also be performed using [`Buffer::get_mapped_range()`].
            ///
            /// # Panics
            ///
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`] within the buffer.
            /// - If the buffer to which `self` refers is not currently [mapped].
            /// - If you try to create a view which overlaps an existing [`BufferViewMut`].
            ///
            /// [mapped]: Buffer#mapping-buffers
            #[track_caller]
            pub fn get_mapped_range(&self) -> BufferView {
                let subrange = Subrange::new(
                    self.offset,
                    self.size,
                    RangeMappingKind::Immutable,
                );
                self.buffer.map_context.lock().validate_and_add(subrange.clone());
                let range = self.buffer.inner.get_mapped_range(subrange.index);
                BufferView {
                    buffer: self.buffer.clone(),
                    size: self.size,
                    offset: self.offset,
                    inner: range,
                }
            }
            /// Gain write access to the bytes of a [mapped] [`Buffer`].
            ///
            /// Returns a [`BufferViewMut`] referring to the buffer range represented by
            /// `self`. See the documentation for [`BufferViewMut`] for more details.
            ///
            /// Multiple views may be obtained and used simultaneously as long as they are from
            /// non-overlapping slices.
            ///
            /// This can also be performed using [`Buffer::get_mapped_range_mut()`].
            ///
            /// # Panics
            ///
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`].
            /// - If the buffer to which `self` refers is not currently [mapped].
            /// - If you try to create a view which overlaps an existing [`BufferView`] or [`BufferViewMut`].
            ///
            /// [mapped]: Buffer#mapping-buffers
            #[track_caller]
            pub fn get_mapped_range_mut(&self) -> BufferViewMut {
                let subrange = Subrange::new(
                    self.offset,
                    self.size,
                    RangeMappingKind::Mutable,
                );
                self.buffer.map_context.lock().validate_and_add(subrange.clone());
                let range = self.buffer.inner.get_mapped_range(subrange.index);
                BufferViewMut {
                    buffer: self.buffer.clone(),
                    size: self.size,
                    offset: self.offset,
                    inner: range,
                    readable: self.buffer.usage.contains(BufferUsages::MAP_READ),
                }
            }
            /// Returns the buffer this is a slice of.
            ///
            /// You should usually not need to call this, and if you received the buffer from code you
            /// do not control, you should refrain from accessing the buffer outside the bounds of the
            /// slice. Nevertheless, it’s possible to get this access, so this method makes it simple.
            pub fn buffer(&self) -> &'a Buffer {
                self.buffer
            }
            /// Returns the offset in [`Self::buffer()`] this slice starts at.
            pub fn offset(&self) -> BufferAddress {
                self.offset
            }
            /// Returns the size of this slice.
            pub fn size(&self) -> BufferSize {
                self.size
            }
        }
        impl<'a> From<BufferSlice<'a>> for crate::BufferBinding<'a> {
            /// Convert a [`BufferSlice`] to an equivalent [`BufferBinding`],
            /// provided that it will be used without a dynamic offset.
            fn from(value: BufferSlice<'a>) -> Self {
                BufferBinding {
                    buffer: value.buffer,
                    offset: value.offset,
                    size: Some(value.size),
                }
            }
        }
        impl<'a> From<BufferSlice<'a>> for crate::BindingResource<'a> {
            /// Convert a [`BufferSlice`] to an equivalent [`BindingResource::Buffer`],
            /// provided that it will be used without a dynamic offset.
            fn from(value: BufferSlice<'a>) -> Self {
                crate::BindingResource::Buffer(crate::BufferBinding::from(value))
            }
        }
        fn range_overlaps(a: &Range<BufferAddress>, b: &Range<BufferAddress>) -> bool {
            a.start < b.end && b.start < a.end
        }
        enum RangeMappingKind {
            Mutable,
            Immutable,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RangeMappingKind {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        RangeMappingKind::Mutable => "Mutable",
                        RangeMappingKind::Immutable => "Immutable",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RangeMappingKind {}
        #[automatically_derived]
        impl ::core::clone::Clone for RangeMappingKind {
            #[inline]
            fn clone(&self) -> RangeMappingKind {
                *self
            }
        }
        impl RangeMappingKind {
            /// Returns true if a range of this kind can touch the same bytes as a range of the other kind.
            ///
            /// This is Rust's Mutable XOR Shared rule.
            fn allowed_concurrently_with(self, other: Self) -> bool {
                #[allow(non_exhaustive_omitted_patterns)]
                match (self, other) {
                    (RangeMappingKind::Immutable, RangeMappingKind::Immutable) => true,
                    _ => false,
                }
            }
        }
        struct Subrange {
            index: Range<BufferAddress>,
            kind: RangeMappingKind,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Subrange {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Subrange",
                    "index",
                    &self.index,
                    "kind",
                    &&self.kind,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Subrange {
            #[inline]
            fn clone(&self) -> Subrange {
                Subrange {
                    index: ::core::clone::Clone::clone(&self.index),
                    kind: ::core::clone::Clone::clone(&self.kind),
                }
            }
        }
        impl Subrange {
            fn new(
                offset: BufferAddress,
                size: BufferSize,
                kind: RangeMappingKind,
            ) -> Self {
                Self {
                    index: offset..(offset + size.get()),
                    kind,
                }
            }
        }
        impl fmt::Display for Subrange {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(
                    format_args!(
                        "{0}..{1} ({2:?})",
                        self.index.start,
                        self.index.end,
                        self.kind,
                    ),
                )
            }
        }
        /// The mapped portion of a buffer, if any, and its outstanding views.
        ///
        /// This ensures that views fall within the mapped range and don't overlap.
        pub(crate) struct MapContext {
            /// The range of the buffer that is mapped.
            ///
            /// This is `0..0` if the buffer is not mapped. This becomes non-empty when
            /// the buffer is mapped at creation time, and when you call `map_async` on
            /// some [`BufferSlice`] (so technically, it indicates the portion that is
            /// *or has been requested to be* mapped.)
            ///
            /// All [`BufferView`]s and [`BufferViewMut`]s must fall within this range.
            mapped_range: Range<BufferAddress>,
            /// The ranges covered by all outstanding [`BufferView`]s and
            /// [`BufferViewMut`]s. These are non-overlapping, and are all contained
            /// within `mapped_range`.
            sub_ranges: Vec<Subrange>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MapContext {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "MapContext",
                    "mapped_range",
                    &self.mapped_range,
                    "sub_ranges",
                    &&self.sub_ranges,
                )
            }
        }
        impl MapContext {
            /// Creates a new `MapContext`.
            ///
            /// For [`mapped_at_creation`] buffers, pass the full buffer range in the
            /// `mapped_range` argument. For other buffers, pass `None`.
            ///
            /// [`mapped_at_creation`]: BufferDescriptor::mapped_at_creation
            pub(crate) fn new(mapped_range: Option<Range<BufferAddress>>) -> Self {
                Self {
                    mapped_range: mapped_range.unwrap_or(0..0),
                    sub_ranges: Vec::new(),
                }
            }
            /// Record that the buffer is no longer mapped.
            fn reset(&mut self) {
                self.mapped_range = 0..0;
                if !self.sub_ranges.is_empty() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "You cannot unmap a buffer that still has accessible mapped views",
                            ),
                        );
                    }
                }
            }
            /// Record that the `size` bytes of the buffer at `offset` are now viewed.
            ///
            /// # Panics
            ///
            /// This panics if the given range is invalid.
            #[track_caller]
            fn validate_and_add(&mut self, new_sub: Subrange) {
                if self.mapped_range.is_empty() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "tried to call get_mapped_range(_mut) on an unmapped buffer",
                            ),
                        );
                    };
                }
                if !range_overlaps(&self.mapped_range, &new_sub.index) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "tried to call get_mapped_range(_mut) on a range that is not entirely mapped. Attempted to get range {0}, but the mapped range is {1}..{2}",
                                new_sub,
                                self.mapped_range.start,
                                self.mapped_range.end,
                            ),
                        );
                    };
                }
                for sub in self.sub_ranges.iter() {
                    if range_overlaps(&sub.index, &new_sub.index)
                        && !sub.kind.allowed_concurrently_with(new_sub.kind)
                    {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "tried to call get_mapped_range(_mut) on a range that has already been mapped and would break Rust memory aliasing rules. Attempted to get range {0}, and the conflicting range is {1}",
                                    new_sub,
                                    sub,
                                ),
                            );
                        };
                    }
                }
                self.sub_ranges.push(new_sub);
            }
            /// Record that the `size` bytes of the buffer at `offset` are no longer viewed.
            ///
            /// # Panics
            ///
            /// This panics if the given range does not exactly match one previously
            /// passed to [`MapContext::validate_and_add`].
            fn remove(&mut self, offset: BufferAddress, size: BufferSize) {
                let end = offset + size.get();
                let index = self
                    .sub_ranges
                    .iter()
                    .position(|r| r.index == (offset..end))
                    .expect("unable to remove range from map context");
                self.sub_ranges.swap_remove(index);
            }
        }
        /// Describes a [`Buffer`].
        ///
        /// For use with [`Device::create_buffer`].
        ///
        /// Corresponds to [WebGPU `GPUBufferDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferdescriptor).
        pub type BufferDescriptor<'a> = wgt::BufferDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BufferDescriptor<'_>>();
        };
        /// Error occurred when trying to async map a buffer.
        pub struct BufferAsyncError;
        #[automatically_derived]
        impl ::core::clone::Clone for BufferAsyncError {
            #[inline]
            fn clone(&self) -> BufferAsyncError {
                BufferAsyncError
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for BufferAsyncError {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for BufferAsyncError {
            #[inline]
            fn eq(&self, other: &BufferAsyncError) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for BufferAsyncError {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BufferAsyncError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "BufferAsyncError")
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<BufferAsyncError>();
        };
        impl fmt::Display for BufferAsyncError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(
                    format_args!("Error occurred when trying to async map a buffer"),
                )
            }
        }
        impl error::Error for BufferAsyncError {}
        /// Type of buffer mapping.
        pub enum MapMode {
            /// Map only for reading
            Read,
            /// Map only for writing
            Write,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MapMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        MapMode::Read => "Read",
                        MapMode::Write => "Write",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MapMode {
            #[inline]
            fn clone(&self) -> MapMode {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for MapMode {}
        #[automatically_derived]
        impl ::core::cmp::Eq for MapMode {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MapMode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MapMode {
            #[inline]
            fn eq(&self, other: &MapMode) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<MapMode>();
        };
        /// A read-only view of a mapped buffer's bytes.
        ///
        /// To get a `BufferView`, first [map] the buffer, and then
        /// call `buffer.slice(range).get_mapped_range()`.
        ///
        /// `BufferView` dereferences to `&[u8]`, so you can use all the usual Rust
        /// slice methods to access the buffer's contents. It also implements
        /// `AsRef<[u8]>`, if that's more convenient.
        ///
        /// Before the buffer can be unmapped, all `BufferView`s observing it
        /// must be dropped. Otherwise, the call to [`Buffer::unmap`] will panic.
        ///
        /// For example code, see the documentation on [mapping buffers][map].
        ///
        /// [map]: Buffer#mapping-buffers
        /// [`map_async`]: BufferSlice::map_async
        pub struct BufferView {
            buffer: Buffer,
            offset: BufferAddress,
            size: BufferSize,
            inner: dispatch::DispatchBufferMappedRange,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BufferView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "BufferView",
                    "buffer",
                    &self.buffer,
                    "offset",
                    &self.offset,
                    "size",
                    &self.size,
                    "inner",
                    &&self.inner,
                )
            }
        }
        impl core::ops::Deref for BufferView {
            type Target = [u8];
            #[inline]
            fn deref(&self) -> &[u8] {
                self.inner.slice()
            }
        }
        impl AsRef<[u8]> for BufferView {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.inner.slice()
            }
        }
        /// A write-only view of a mapped buffer's bytes.
        ///
        /// To get a `BufferViewMut`, first [map] the buffer, and then
        /// call `buffer.slice(range).get_mapped_range_mut()`.
        ///
        /// `BufferViewMut` dereferences to `&mut [u8]`, so you can use all the usual
        /// Rust slice methods to access the buffer's contents. It also implements
        /// `AsMut<[u8]>`, if that's more convenient.
        ///
        /// It is possible to read the buffer using this view, but doing so is not
        /// recommended, as it is likely to be slow.
        ///
        /// Before the buffer can be unmapped, all `BufferViewMut`s observing it
        /// must be dropped. Otherwise, the call to [`Buffer::unmap`] will panic.
        ///
        /// For example code, see the documentation on [mapping buffers][map].
        ///
        /// [map]: Buffer#mapping-buffers
        pub struct BufferViewMut {
            buffer: Buffer,
            offset: BufferAddress,
            size: BufferSize,
            inner: dispatch::DispatchBufferMappedRange,
            readable: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BufferViewMut {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "BufferViewMut",
                    "buffer",
                    &self.buffer,
                    "offset",
                    &self.offset,
                    "size",
                    &self.size,
                    "inner",
                    &self.inner,
                    "readable",
                    &&self.readable,
                )
            }
        }
        impl AsMut<[u8]> for BufferViewMut {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8] {
                self.inner.slice_mut()
            }
        }
        impl Deref for BufferViewMut {
            type Target = [u8];
            fn deref(&self) -> &Self::Target {
                if !self.readable {
                    {
                        {
                            let lvl = ::log::Level::Warn;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!(
                                        "Reading from a BufferViewMut is slow and not recommended.",
                                    ),
                                    lvl,
                                    &(
                                        "wgpu::api::buffer",
                                        "wgpu::api::buffer",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                }
                self.inner.slice()
            }
        }
        impl DerefMut for BufferViewMut {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.inner.slice_mut()
            }
        }
        impl Drop for BufferView {
            fn drop(&mut self) {
                self.buffer.map_context.lock().remove(self.offset, self.size);
            }
        }
        impl Drop for BufferViewMut {
            fn drop(&mut self) {
                self.buffer.map_context.lock().remove(self.offset, self.size);
            }
        }
        #[track_caller]
        fn check_buffer_bounds(
            buffer_size: BufferAddress,
            slice_offset: BufferAddress,
            slice_size: BufferSize,
        ) {
            if slice_offset >= buffer_size {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "slice offset {0} is out of range for buffer of size {1}",
                            slice_offset,
                            buffer_size,
                        ),
                    );
                };
            }
            let end = slice_offset.checked_add(slice_size.get());
            if end.is_none_or(|end| end > buffer_size) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "slice offset {0} size {1} is out of range for buffer of size {2}",
                            slice_offset,
                            slice_size,
                            buffer_size,
                        ),
                    );
                };
            }
        }
        #[track_caller]
        pub(crate) fn range_to_offset_size<S: RangeBounds<BufferAddress>>(
            bounds: S,
            whole_size: BufferAddress,
        ) -> (BufferAddress, BufferSize) {
            let offset = match bounds.start_bound() {
                Bound::Included(&bound) => bound,
                Bound::Excluded(&bound) => bound + 1,
                Bound::Unbounded => 0,
            };
            let size = BufferSize::new(
                    match bounds.end_bound() {
                        Bound::Included(&bound) => bound + 1 - offset,
                        Bound::Excluded(&bound) => bound - offset,
                        Bound::Unbounded => whole_size - offset,
                    },
                )
                .expect("buffer slices can not be empty");
            (offset, size)
        }
    }
    mod command_buffer {
        use crate::{
            api::{
                impl_deferred_command_buffer_actions, SharedDeferredCommandBufferActions,
            },
            *,
        };
        /// Handle to a command buffer on the GPU.
        ///
        /// A `CommandBuffer` represents a complete sequence of commands that may be submitted to a command
        /// queue with [`Queue::submit`]. A `CommandBuffer` is obtained by recording a series of commands to
        /// a [`CommandEncoder`] and then calling [`CommandEncoder::finish`].
        ///
        /// Corresponds to [WebGPU `GPUCommandBuffer`](https://gpuweb.github.io/gpuweb/#command-buffer).
        pub struct CommandBuffer {
            pub(crate) buffer: dispatch::DispatchCommandBuffer,
            /// Deferred actions recorded at encode time, to run at Queue::submit.
            pub(crate) actions: SharedDeferredCommandBufferActions,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CommandBuffer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CommandBuffer",
                    "buffer",
                    &self.buffer,
                    "actions",
                    &&self.actions,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<CommandBuffer>();
        };
        impl CommandBuffer {
            /// On submission, maps the buffer to host (CPU) memory, making it available
            /// for reading or writing via [`get_mapped_range()`](Buffer::get_mapped_range).
            /// The buffer becomes accessible once the `callback` is invoked with [`Ok`].
            ///
            /// Use this when you need to submit work that uses the buffer before mapping it.
            /// Because that submission must happen before calling `map_async`, this method
            /// schedules the mapping for after submission, avoiding extra calls to
            /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and letting you start
            /// the mapping from a more convenient place.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// While a buffer is mapped, it cannot be used by other commands; at any time,
            /// either the GPU or the CPU has exclusive access to the buffer’s contents.
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside the bounds of `buffer`.
            /// - If `bounds` has a length less than 1.
            ///
            /// # Panics During Submit
            ///
            /// - If the buffer is already mapped.
            /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`] within the buffer.
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            /// [CEmbos]: CommandEncoder::map_buffer_on_submit
            /// [CBmbos]: CommandBuffer::map_buffer_on_submit
            /// [RPmbos]: RenderPass::map_buffer_on_submit
            /// [CPmbos]: ComputePass::map_buffer_on_submit
            pub fn map_buffer_on_submit<S: core::ops::RangeBounds<BufferAddress>>(
                &self,
                buffer: &api::Buffer,
                mode: MapMode,
                bounds: S,
                callback: impl FnOnce(
                    Result<(), BufferAsyncError>,
                ) + WasmNotSend + 'static,
            ) {
                let (offset, size) = range_to_offset_size(bounds, buffer.size);
                self.actions
                    .lock()
                    .buffer_mappings
                    .push(crate::api::command_buffer_actions::DeferredBufferMapping {
                        buffer: buffer.clone(),
                        mode,
                        offset,
                        size,
                        callback: alloc::boxed::Box::new(callback),
                    });
            }
            /// Registers a callback that is invoked when this command buffer’s work finishes
            /// executing on the GPU. When this callback runs, all mapped-buffer callbacks
            /// registered for the same submission are guaranteed to have been called.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn on_submitted_work_done(
                &self,
                callback: impl FnOnce() + Send + 'static,
            ) {
                self.actions
                    .lock()
                    .on_submitted_work_done_callbacks
                    .push(alloc::boxed::Box::new(callback));
            }
        }
    }
    /// Not a root type, but common types for command buffer deferral actions.
    mod command_buffer_actions {
        use alloc::{sync::Arc, vec::Vec};
        use core::num::NonZeroU64;
        use crate::{util::Mutex, *};
        /// A deferred buffer mapping request captured during encoding (or a pass)
        /// and executed later when the command buffer is submitted.
        pub(crate) struct DeferredBufferMapping {
            pub buffer: api::Buffer,
            pub mode: MapMode,
            pub offset: u64,
            pub size: NonZeroU64,
            pub callback: dispatch::BufferMapCallback,
        }
        pub(super) type SharedDeferredCommandBufferActions = Arc<
            Mutex<DeferredCommandBufferActions>,
        >;
        /// Set of actions to take when the command buffer is submitted.
        pub(crate) struct DeferredCommandBufferActions {
            pub buffer_mappings: Vec<DeferredBufferMapping>,
            pub on_submitted_work_done_callbacks: Vec<
                dispatch::BoxSubmittedWorkDoneCallback,
            >,
        }
        #[automatically_derived]
        impl ::core::default::Default for DeferredCommandBufferActions {
            #[inline]
            fn default() -> DeferredCommandBufferActions {
                DeferredCommandBufferActions {
                    buffer_mappings: ::core::default::Default::default(),
                    on_submitted_work_done_callbacks: ::core::default::Default::default(),
                }
            }
        }
        impl DeferredCommandBufferActions {
            pub fn append(&mut self, other: &mut Self) {
                self.buffer_mappings.append(&mut other.buffer_mappings);
                self.on_submitted_work_done_callbacks
                    .append(&mut other.on_submitted_work_done_callbacks);
            }
            pub fn execute(self, queue: &dispatch::DispatchQueue) {
                for mapping in self.buffer_mappings {
                    mapping
                        .buffer
                        .map_async(
                            mapping.mode,
                            mapping.offset..mapping.offset + mapping.size.get(),
                            mapping.callback,
                        );
                }
                for callback in self.on_submitted_work_done_callbacks {
                    queue.on_submitted_work_done(callback);
                }
            }
        }
        impl core::fmt::Debug for DeferredCommandBufferActions {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("DeferredCommandBufferActions")
                    .field("buffer_mappings.len()", &self.buffer_mappings.len())
                    .field(
                        "on_submitted_work_done_callbacks.len()",
                        &self.on_submitted_work_done_callbacks.len(),
                    )
                    .finish()
            }
        }
        pub(crate) use impl_deferred_command_buffer_actions;
    }
    mod command_encoder {
        use alloc::sync::Arc;
        use core::ops::Range;
        use crate::{
            api::{
                blas::BlasBuildEntry, impl_deferred_command_buffer_actions, tlas::Tlas,
                SharedDeferredCommandBufferActions,
            },
            *,
        };
        /// Encodes a series of GPU operations.
        ///
        /// A command encoder can record [`RenderPass`]es, [`ComputePass`]es,
        /// and transfer operations between driver-managed resources like [`Buffer`]s and [`Texture`]s.
        ///
        /// When finished recording, call [`CommandEncoder::finish`] to obtain a [`CommandBuffer`] which may
        /// be submitted for execution.
        ///
        /// Corresponds to [WebGPU `GPUCommandEncoder`](https://gpuweb.github.io/gpuweb/#command-encoder).
        pub struct CommandEncoder {
            pub(crate) inner: dispatch::DispatchCommandEncoder,
            pub(crate) actions: SharedDeferredCommandBufferActions,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CommandEncoder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CommandEncoder",
                    "inner",
                    &self.inner,
                    "actions",
                    &&self.actions,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<CommandEncoder>();
        };
        impl PartialEq for CommandEncoder {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for CommandEncoder {}
        impl PartialOrd for CommandEncoder {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CommandEncoder {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for CommandEncoder {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        /// Describes a [`CommandEncoder`].
        ///
        /// For use with [`Device::create_command_encoder`].
        ///
        /// Corresponds to [WebGPU `GPUCommandEncoderDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpucommandencoderdescriptor).
        pub type CommandEncoderDescriptor<'a> = wgt::CommandEncoderDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<CommandEncoderDescriptor<'_>>();
        };
        pub use wgt::TexelCopyBufferInfo as TexelCopyBufferInfoBase;
        /// View of a buffer which can be used to copy to/from a texture.
        ///
        /// Corresponds to [WebGPU `GPUTexelCopyBufferInfo`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopybuffer).
        pub type TexelCopyBufferInfo<'a> = TexelCopyBufferInfoBase<&'a Buffer>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<TexelCopyBufferInfo<'_>>();
        };
        pub use wgt::TexelCopyTextureInfo as TexelCopyTextureInfoBase;
        /// View of a texture which can be used to copy to/from a buffer/texture.
        ///
        /// Corresponds to [WebGPU `GPUTexelCopyTextureInfo`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopytexture).
        pub type TexelCopyTextureInfo<'a> = TexelCopyTextureInfoBase<&'a Texture>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<TexelCopyTextureInfo<'_>>();
        };
        impl CommandEncoder {
            /// Finishes recording and returns a [`CommandBuffer`] that can be submitted for execution.
            pub fn finish(self) -> CommandBuffer {
                let Self { mut inner, actions } = self;
                let buffer = inner.finish();
                CommandBuffer { buffer, actions }
            }
            /// Begins recording of a render pass.
            ///
            /// This function returns a [`RenderPass`] object which records a single render pass.
            ///
            /// As long as the returned  [`RenderPass`] has not ended,
            /// any mutating operation on this command encoder causes an error and invalidates it.
            /// Note that the `'encoder` lifetime relationship protects against this,
            /// but it is possible to opt out of it by calling [`RenderPass::forget_lifetime`].
            /// This can be useful for runtime handling of the encoder->pass
            /// dependency e.g. when pass and encoder are stored in the same data structure.
            pub fn begin_render_pass<'encoder>(
                &'encoder mut self,
                desc: &RenderPassDescriptor<'_>,
            ) -> RenderPass<'encoder> {
                let rpass = self.inner.begin_render_pass(desc);
                RenderPass {
                    inner: rpass,
                    actions: Arc::clone(&self.actions),
                    _encoder_guard: api::PhantomDrop::default(),
                }
            }
            /// Begins recording of a compute pass.
            ///
            /// This function returns a [`ComputePass`] object which records a single compute pass.
            ///
            /// As long as the returned  [`ComputePass`] has not ended,
            /// any mutating operation on this command encoder causes an error and invalidates it.
            /// Note that the `'encoder` lifetime relationship protects against this,
            /// but it is possible to opt out of it by calling [`ComputePass::forget_lifetime`].
            /// This can be useful for runtime handling of the encoder->pass
            /// dependency e.g. when pass and encoder are stored in the same data structure.
            pub fn begin_compute_pass<'encoder>(
                &'encoder mut self,
                desc: &ComputePassDescriptor<'_>,
            ) -> ComputePass<'encoder> {
                let cpass = self.inner.begin_compute_pass(desc);
                ComputePass {
                    inner: cpass,
                    actions: Arc::clone(&self.actions),
                    _encoder_guard: api::PhantomDrop::default(),
                }
            }
            /// Copy data from one buffer to another.
            ///
            /// # Panics
            ///
            /// - Buffer offsets or copy size not a multiple of [`COPY_BUFFER_ALIGNMENT`].
            /// - Copy would overrun buffer.
            /// - Copy within the same buffer.
            pub fn copy_buffer_to_buffer(
                &mut self,
                source: &Buffer,
                source_offset: BufferAddress,
                destination: &Buffer,
                destination_offset: BufferAddress,
                copy_size: impl Into<Option<BufferAddress>>,
            ) {
                self.inner
                    .copy_buffer_to_buffer(
                        &source.inner,
                        source_offset,
                        &destination.inner,
                        destination_offset,
                        copy_size.into(),
                    );
            }
            /// Copy data from a buffer to a texture.
            pub fn copy_buffer_to_texture(
                &mut self,
                source: TexelCopyBufferInfo<'_>,
                destination: TexelCopyTextureInfo<'_>,
                copy_size: Extent3d,
            ) {
                self.inner.copy_buffer_to_texture(source, destination, copy_size);
            }
            /// Copy data from a texture to a buffer.
            pub fn copy_texture_to_buffer(
                &mut self,
                source: TexelCopyTextureInfo<'_>,
                destination: TexelCopyBufferInfo<'_>,
                copy_size: Extent3d,
            ) {
                self.inner.copy_texture_to_buffer(source, destination, copy_size);
            }
            /// Copy data from one texture to another.
            ///
            /// # Panics
            ///
            /// - Textures are not the same type
            /// - If a depth texture, or a multisampled texture, the entire texture must be copied
            /// - Copy would overrun either texture
            pub fn copy_texture_to_texture(
                &mut self,
                source: TexelCopyTextureInfo<'_>,
                destination: TexelCopyTextureInfo<'_>,
                copy_size: Extent3d,
            ) {
                self.inner.copy_texture_to_texture(source, destination, copy_size);
            }
            /// Clears texture to zero.
            ///
            /// Note that unlike with clear_buffer, `COPY_DST` usage is not required.
            ///
            /// # Implementation notes
            ///
            /// - implemented either via buffer copies and render/depth target clear, path depends on texture usages
            /// - behaves like texture zero init, but is performed immediately (clearing is *not* delayed via marking it as uninitialized)
            ///
            /// # Panics
            ///
            /// - `CLEAR_TEXTURE` extension not enabled
            /// - Range is out of bounds
            pub fn clear_texture(
                &mut self,
                texture: &Texture,
                subresource_range: &ImageSubresourceRange,
            ) {
                self.inner.clear_texture(&texture.inner, subresource_range);
            }
            /// Clears buffer to zero.
            ///
            /// # Panics
            ///
            /// - Buffer does not have `COPY_DST` usage.
            /// - Range is out of bounds
            pub fn clear_buffer(
                &mut self,
                buffer: &Buffer,
                offset: BufferAddress,
                size: Option<BufferAddress>,
            ) {
                self.inner.clear_buffer(&buffer.inner, offset, size);
            }
            /// Inserts debug marker.
            pub fn insert_debug_marker(&mut self, label: &str) {
                self.inner.insert_debug_marker(label);
            }
            /// Start record commands and group it into debug marker group.
            pub fn push_debug_group(&mut self, label: &str) {
                self.inner.push_debug_group(label);
            }
            /// Stops command recording and creates debug group.
            pub fn pop_debug_group(&mut self) {
                self.inner.pop_debug_group();
            }
            /// Resolves a query set, writing the results into the supplied destination buffer.
            ///
            /// Occlusion and timestamp queries are 8 bytes each (see [`crate::QUERY_SIZE`]). For pipeline statistics queries,
            /// see [`PipelineStatisticsTypes`] for more information.
            ///
            /// `destination_offset` must be aligned to [`QUERY_RESOLVE_BUFFER_ALIGNMENT`].
            pub fn resolve_query_set(
                &mut self,
                query_set: &QuerySet,
                query_range: Range<u32>,
                destination: &Buffer,
                destination_offset: BufferAddress,
            ) {
                self.inner
                    .resolve_query_set(
                        &query_set.inner,
                        query_range.start,
                        query_range.end - query_range.start,
                        &destination.inner,
                        destination_offset,
                    );
            }
            /// On submission, maps the buffer to host (CPU) memory, making it available
            /// for reading or writing via [`get_mapped_range()`](Buffer::get_mapped_range).
            /// The buffer becomes accessible once the `callback` is invoked with [`Ok`].
            ///
            /// Use this when you need to submit work that uses the buffer before mapping it.
            /// Because that submission must happen before calling `map_async`, this method
            /// schedules the mapping for after submission, avoiding extra calls to
            /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and letting you start
            /// the mapping from a more convenient place.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// While a buffer is mapped, it cannot be used by other commands; at any time,
            /// either the GPU or the CPU has exclusive access to the buffer’s contents.
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside the bounds of `buffer`.
            /// - If `bounds` has a length less than 1.
            ///
            /// # Panics During Submit
            ///
            /// - If the buffer is already mapped.
            /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`] within the buffer.
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            /// [CEmbos]: CommandEncoder::map_buffer_on_submit
            /// [CBmbos]: CommandBuffer::map_buffer_on_submit
            /// [RPmbos]: RenderPass::map_buffer_on_submit
            /// [CPmbos]: ComputePass::map_buffer_on_submit
            pub fn map_buffer_on_submit<S: core::ops::RangeBounds<BufferAddress>>(
                &self,
                buffer: &api::Buffer,
                mode: MapMode,
                bounds: S,
                callback: impl FnOnce(
                    Result<(), BufferAsyncError>,
                ) + WasmNotSend + 'static,
            ) {
                let (offset, size) = range_to_offset_size(bounds, buffer.size);
                self.actions
                    .lock()
                    .buffer_mappings
                    .push(crate::api::command_buffer_actions::DeferredBufferMapping {
                        buffer: buffer.clone(),
                        mode,
                        offset,
                        size,
                        callback: alloc::boxed::Box::new(callback),
                    });
            }
            /// Registers a callback that is invoked when this command buffer’s work finishes
            /// executing on the GPU. When this callback runs, all mapped-buffer callbacks
            /// registered for the same submission are guaranteed to have been called.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn on_submitted_work_done(
                &self,
                callback: impl FnOnce() + Send + 'static,
            ) {
                self.actions
                    .lock()
                    .on_submitted_work_done_callbacks
                    .push(alloc::boxed::Box::new(callback));
            }
            /// Get the [`wgpu_hal`] command encoder from this `CommandEncoder`.
            ///
            /// The returned command encoder will be ready to record onto.
            ///
            /// # Errors
            ///
            /// This method will pass in [`None`] if:
            /// - The encoder is not from the backend specified by `A`.
            /// - The encoder is from the `webgpu` or `custom` backend.
            ///
            /// # Types
            ///
            /// The callback argument depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::CommandEncoder`
            ///- [`hal::api::Metal`] uses [`hal::metal::CommandEncoder`]
            ///- `hal::api::Dx12` uses `hal::dx12::CommandEncoder`
            ///- `hal::api::Gles` uses `hal::gles::CommandEncoder`
            ///
            /// # Safety
            ///
            /// - The raw handle obtained from the `A::CommandEncoder` must not be manually destroyed.
            /// - You must not end the command buffer; wgpu will do it when you call finish.
            /// - The wgpu command encoder must not be interacted with in any way while recording is
            ///   happening to the wgpu_hal or backend command encoder.
            pub unsafe fn as_hal_mut<
                A: hal::Api,
                F: FnOnce(Option<&mut A::CommandEncoder>) -> R,
                R,
            >(&mut self, hal_command_encoder_callback: F) -> R {
                if let Some(encoder) = self.inner.as_core_mut_opt() {
                    unsafe {
                        encoder
                            .context
                            .command_encoder_as_hal_mut::<
                                A,
                                F,
                                R,
                            >(encoder, hal_command_encoder_callback)
                    }
                } else {
                    hal_command_encoder_callback(None)
                }
            }
        }
        /// [`Features::TIMESTAMP_QUERY_INSIDE_ENCODERS`] must be enabled on the device in order to call these functions.
        impl CommandEncoder {
            /// Issue a timestamp command at this point in the queue.
            /// The timestamp will be written to the specified query set, at the specified index.
            ///
            /// Must be multiplied by [`Queue::get_timestamp_period`] to get
            /// the value in nanoseconds. Absolute values have no meaning,
            /// but timestamps can be subtracted to get the time it takes
            /// for a string of operations to complete.
            ///
            /// Attention: Since commands within a command recorder may be reordered,
            /// there is no strict guarantee that timestamps are taken after all commands
            /// recorded so far and all before all commands recorded after.
            /// This may depend both on the backend and the driver.
            pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {
                self.inner.write_timestamp(&query_set.inner, query_index);
            }
        }
        /// [`Features::EXPERIMENTAL_RAY_QUERY`] must be enabled on the device in order to call these functions.
        impl CommandEncoder {
            /// Mark acceleration structures as being built. ***Should only*** be used with wgpu-hal
            /// functions, all wgpu functions already mark acceleration structures as built.
            ///
            /// # Safety
            ///
            /// - All acceleration structures must have been build in this command encoder.
            /// - All BLASes inputted must have been built before all TLASes that were inputted here and
            ///   which use them.
            pub unsafe fn mark_acceleration_structures_built<'a>(
                &self,
                blas: impl IntoIterator<Item = &'a Blas>,
                tlas: impl IntoIterator<Item = &'a Tlas>,
            ) {
                self.inner
                    .mark_acceleration_structures_built(
                        &mut blas.into_iter(),
                        &mut tlas.into_iter(),
                    )
            }
            /// Build bottom and top level acceleration structures.
            ///
            /// Builds the BLASes then the TLASes, but does ***not*** build the BLASes into the TLASes,
            /// that must be done by setting a TLAS instance in the TLAS package to one that contains the BLAS (and with an appropriate transform)
            ///
            /// # Validation
            ///
            /// - blas: Iterator of bottom level acceleration structure entries to build.
            ///   For each entry, the provided size descriptor must be strictly smaller or equal to the descriptor given at BLAS creation, this means:
            ///   - Less or equal number of geometries
            ///   - Same kind of geometry (with index buffer or without) (same vertex/index format)
            ///   - Same flags
            ///   - Less or equal number of vertices
            ///   - Less or equal number of indices (if applicable)
            /// - tlas: iterator of top level acceleration structure packages to build
            ///   For each entry:
            ///   - Each BLAS in each TLAS instance must have been being built in the current call or in a previous call to `build_acceleration_structures` or `build_acceleration_structures_unsafe_tlas`
            ///   - The number of TLAS instances must be less than or equal to the max number of tlas instances when creating (if creating a package with `TlasPackage::new()` this is already satisfied)
            ///
            /// If the device the command encoder is created from does not have [Features::EXPERIMENTAL_RAY_QUERY] enabled then a validation error is generated
            ///
            /// A bottom level acceleration structure may be build and used as a reference in a top level acceleration structure in the same invocation of this function.
            ///
            /// # Bind group usage
            ///
            /// When a top level acceleration structure is used in a bind group, some validation takes place:
            ///    - The top level acceleration structure is valid and has been built.
            ///    - All the bottom level acceleration structures referenced by the top level acceleration structure are valid and have been built prior,
            ///      or at same time as the containing top level acceleration structure.
            ///
            /// [Features::EXPERIMENTAL_RAY_QUERY]: wgt::Features::EXPERIMENTAL_RAY_QUERY
            pub fn build_acceleration_structures<'a>(
                &mut self,
                blas: impl IntoIterator<Item = &'a BlasBuildEntry<'a>>,
                tlas: impl IntoIterator<Item = &'a Tlas>,
            ) {
                self.inner
                    .build_acceleration_structures(
                        &mut blas.into_iter(),
                        &mut tlas.into_iter(),
                    );
            }
            /// Transition resources to an underlying hal resource state.
            ///
            /// This is an advanced, native-only API (no-op on web) that has two main use cases:
            ///
            /// # Batching Barriers
            ///
            /// Wgpu does not have a global view of the frame when recording command buffers. When you submit multiple command buffers in a single queue submission, wgpu may need to record and
            /// insert new command buffers (holding 1 or more barrier commands) in between the user-supplied command buffers in order to ensure that resources are transitioned to the correct state
            /// for the start of the next user-supplied command buffer.
            ///
            /// Wgpu does not currently attempt to batch multiple of these generated command buffers/barriers together, which may lead to suboptimal barrier placement.
            ///
            /// Consider the following scenario, where the user does `queue.submit(&[a, b, c])`:
            /// * CommandBuffer A: Use resource X as a render pass attachment
            /// * CommandBuffer B: Use resource Y as a render pass attachment
            /// * CommandBuffer C: Use resources X and Y in a bind group
            ///
            /// At submission time, wgpu will record and insert some new command buffers, resulting in a submission that looks like `queue.submit(&[0, a, 1, b, 2, c])`:
            /// * CommandBuffer 0: Barrier to transition resource X from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
            /// * CommandBuffer A: Use resource X as a render pass attachment
            /// * CommandBuffer 1: Barrier to transition resource Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
            /// * CommandBuffer B: Use resource Y as a render pass attachment
            /// * CommandBuffer 2: Barrier to transition resources X and Y from TextureUses::COLOR_TARGET to TextureUses::RESOURCE
            /// * CommandBuffer C: Use resources X and Y in a bind group
            ///
            /// To prevent this, after profiling their app, an advanced user might choose to instead do `queue.submit(&[a, b, c])`:
            /// * CommandBuffer A:
            ///     * Use [`CommandEncoder::transition_resources`] to transition resources X and Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
            ///     * Use resource X as a render pass attachment
            /// * CommandBuffer B: Use resource Y as a render pass attachment
            /// * CommandBuffer C:
            ///     * Use [`CommandEncoder::transition_resources`] to transition resources X and Y from TextureUses::COLOR_TARGET to TextureUses::RESOURCE
            ///     * Use resources X and Y in a bind group
            ///
            /// At submission time, wgpu will record and insert some new command buffers, resulting in a submission that looks like `queue.submit(&[0, a, b, 1, c])`:
            /// * CommandBuffer 0: Barrier to transition resources X and Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR_TARGET
            /// * CommandBuffer A: Use resource X as a render pass attachment
            /// * CommandBuffer B: Use resource Y as a render pass attachment
            /// * CommandBuffer 1: Barrier to transition resources X and Y from TextureUses::COLOR_TARGET to TextureUses::RESOURCE
            /// * CommandBuffer C: Use resources X and Y in a bind group
            ///
            /// Which eliminates the extra command buffer and barrier between command buffers A and B.
            ///
            /// # Native Interoperability
            ///
            /// A user wanting to interoperate with the underlying native graphics APIs (Vulkan, DirectX12, Metal, etc) can use this API to generate barriers between wgpu commands and
            /// the native API commands, for synchronization and resource state transition purposes.
            pub fn transition_resources<'a>(
                &mut self,
                buffer_transitions: impl Iterator<
                    Item = wgt::BufferTransition<&'a Buffer>,
                >,
                texture_transitions: impl Iterator<
                    Item = wgt::TextureTransition<&'a Texture>,
                >,
            ) {
                self.inner
                    .transition_resources(
                        &mut buffer_transitions
                            .map(|t| wgt::BufferTransition {
                                buffer: &t.buffer.inner,
                                state: t.state,
                            }),
                        &mut texture_transitions
                            .map(|t| wgt::TextureTransition {
                                texture: &t.texture.inner,
                                selector: t.selector,
                                state: t.state,
                            }),
                    );
            }
        }
    }
    mod common_pipeline {
        use crate::*;
        /// Advanced options for use when a pipeline is compiled
        ///
        /// This implements `Default`, and for most users can be set to `Default::default()`
        pub struct PipelineCompilationOptions<'a> {
            /// Specifies the values of pipeline-overridable constants in the shader module.
            ///
            /// If an `@id` attribute was specified on the declaration,
            /// the key must be the pipeline constant ID as a decimal ASCII number; if not,
            /// the key must be the constant's identifier name.
            ///
            /// If the given constant is specified more than once, the last value specified is used.
            ///
            /// The value may represent any of WGSL's concrete scalar types.
            pub constants: &'a [(&'a str, f64)],
            /// Whether workgroup scoped memory will be initialized with zero values for this stage.
            ///
            /// This is required by the WebGPU spec, but may have overhead which can be avoided
            /// for cross-platform applications
            pub zero_initialize_workgroup_memory: bool,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for PipelineCompilationOptions<'a> {
            #[inline]
            fn clone(&self) -> PipelineCompilationOptions<'a> {
                PipelineCompilationOptions {
                    constants: ::core::clone::Clone::clone(&self.constants),
                    zero_initialize_workgroup_memory: ::core::clone::Clone::clone(
                        &self.zero_initialize_workgroup_memory,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for PipelineCompilationOptions<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PipelineCompilationOptions",
                    "constants",
                    &self.constants,
                    "zero_initialize_workgroup_memory",
                    &&self.zero_initialize_workgroup_memory,
                )
            }
        }
        impl Default for PipelineCompilationOptions<'_> {
            fn default() -> Self {
                Self {
                    constants: Default::default(),
                    zero_initialize_workgroup_memory: true,
                }
            }
        }
        /// Describes a pipeline cache, which allows reusing compilation work
        /// between program runs.
        ///
        /// For use with [`Device::create_pipeline_cache`].
        ///
        /// This type is unique to the Rust API of `wgpu`.
        pub struct PipelineCacheDescriptor<'a> {
            /// Debug label of the pipeline cache. This might show up in some logs from `wgpu`
            pub label: Label<'a>,
            /// The data used to initialise the cache initialise
            ///
            /// # Safety
            ///
            /// This data must have been provided from a previous call to
            /// [`PipelineCache::get_data`], if not `None`
            pub data: Option<&'a [u8]>,
            /// Whether to create a cache without data when the provided data
            /// is invalid.
            ///
            /// Recommended to set to true
            pub fallback: bool,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for PipelineCacheDescriptor<'a> {
            #[inline]
            fn clone(&self) -> PipelineCacheDescriptor<'a> {
                PipelineCacheDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    data: ::core::clone::Clone::clone(&self.data),
                    fallback: ::core::clone::Clone::clone(&self.fallback),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for PipelineCacheDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PipelineCacheDescriptor",
                    "label",
                    &self.label,
                    "data",
                    &self.data,
                    "fallback",
                    &&self.fallback,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<PipelineCacheDescriptor<'_>>();
        };
    }
    mod compute_pass {
        use crate::{
            api::{
                impl_deferred_command_buffer_actions, SharedDeferredCommandBufferActions,
            },
            *,
        };
        /// In-progress recording of a compute pass.
        ///
        /// It can be created with [`CommandEncoder::begin_compute_pass`].
        ///
        /// Corresponds to [WebGPU `GPUComputePassEncoder`](
        /// https://gpuweb.github.io/gpuweb/#compute-pass-encoder).
        pub struct ComputePass<'encoder> {
            pub(crate) inner: dispatch::DispatchComputePass,
            /// Shared with CommandEncoder to enqueue deferred actions from within a pass.
            pub(crate) actions: SharedDeferredCommandBufferActions,
            /// This lifetime is used to protect the [`CommandEncoder`] from being used
            /// while the pass is alive. This needs to be PhantomDrop to prevent the lifetime
            /// from being shortened.
            pub(crate) _encoder_guard: crate::api::PhantomDrop<&'encoder ()>,
        }
        #[automatically_derived]
        impl<'encoder> ::core::fmt::Debug for ComputePass<'encoder> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ComputePass",
                    "inner",
                    &self.inner,
                    "actions",
                    &self.actions,
                    "_encoder_guard",
                    &&self._encoder_guard,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ComputePass<'_>>();
        };
        impl PartialEq for ComputePass<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for ComputePass<'_> {}
        impl PartialOrd for ComputePass<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for ComputePass<'_> {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for ComputePass<'_> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl ComputePass<'_> {
            /// Drops the lifetime relationship to the parent command encoder, making usage of
            /// the encoder while this pass is recorded a run-time error instead.
            ///
            /// Attention: As long as the compute pass has not been ended, any mutating operation on the parent
            /// command encoder will cause a run-time error and invalidate it!
            /// By default, the lifetime constraint prevents this, but it can be useful
            /// to handle this at run time, such as when storing the pass and encoder in the same
            /// data structure.
            ///
            /// This operation has no effect on pass recording.
            /// It's a safe operation, since [`CommandEncoder`] is in a locked state as long as the pass is active
            /// regardless of the lifetime constraint or its absence.
            pub fn forget_lifetime(self) -> ComputePass<'static> {
                ComputePass {
                    inner: self.inner,
                    actions: self.actions,
                    _encoder_guard: crate::api::PhantomDrop::default(),
                }
            }
            /// Sets the active bind group for a given bind group index. The bind group layout
            /// in the active pipeline when the `dispatch()` function is called must match the layout of this bind group.
            ///
            /// If the bind group have dynamic offsets, provide them in the binding order.
            /// These offsets have to be aligned to [`Limits::min_uniform_buffer_offset_alignment`]
            /// or [`Limits::min_storage_buffer_offset_alignment`] appropriately.
            pub fn set_bind_group<'a, BG>(
                &mut self,
                index: u32,
                bind_group: BG,
                offsets: &[DynamicOffset],
            )
            where
                Option<&'a BindGroup>: From<BG>,
            {
                let bg: Option<&BindGroup> = bind_group.into();
                let bg = bg.map(|bg| &bg.inner);
                self.inner.set_bind_group(index, bg, offsets);
            }
            /// Sets the active compute pipeline.
            pub fn set_pipeline(&mut self, pipeline: &ComputePipeline) {
                self.inner.set_pipeline(&pipeline.inner);
            }
            /// Inserts debug marker.
            pub fn insert_debug_marker(&mut self, label: &str) {
                self.inner.insert_debug_marker(label);
            }
            /// Start record commands and group it into debug marker group.
            pub fn push_debug_group(&mut self, label: &str) {
                self.inner.push_debug_group(label);
            }
            /// Stops command recording and creates debug group.
            pub fn pop_debug_group(&mut self) {
                self.inner.pop_debug_group();
            }
            /// Dispatches compute work operations.
            ///
            /// `x`, `y` and `z` denote the number of work groups to dispatch in each dimension.
            pub fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
                self.inner.dispatch_workgroups(x, y, z);
            }
            /// Dispatches compute work operations, based on the contents of the `indirect_buffer`.
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
            pub fn dispatch_workgroups_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
            ) {
                self.inner
                    .dispatch_workgroups_indirect(
                        &indirect_buffer.inner,
                        indirect_offset,
                    );
            }
            /// On submission, maps the buffer to host (CPU) memory, making it available
            /// for reading or writing via [`get_mapped_range()`](Buffer::get_mapped_range).
            /// The buffer becomes accessible once the `callback` is invoked with [`Ok`].
            ///
            /// Use this when you need to submit work that uses the buffer before mapping it.
            /// Because that submission must happen before calling `map_async`, this method
            /// schedules the mapping for after submission, avoiding extra calls to
            /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and letting you start
            /// the mapping from a more convenient place.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// While a buffer is mapped, it cannot be used by other commands; at any time,
            /// either the GPU or the CPU has exclusive access to the buffer’s contents.
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside the bounds of `buffer`.
            /// - If `bounds` has a length less than 1.
            ///
            /// # Panics During Submit
            ///
            /// - If the buffer is already mapped.
            /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`] within the buffer.
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            /// [CEmbos]: CommandEncoder::map_buffer_on_submit
            /// [CBmbos]: CommandBuffer::map_buffer_on_submit
            /// [RPmbos]: RenderPass::map_buffer_on_submit
            /// [CPmbos]: ComputePass::map_buffer_on_submit
            pub fn map_buffer_on_submit<S: core::ops::RangeBounds<BufferAddress>>(
                &self,
                buffer: &api::Buffer,
                mode: MapMode,
                bounds: S,
                callback: impl FnOnce(
                    Result<(), BufferAsyncError>,
                ) + WasmNotSend + 'static,
            ) {
                let (offset, size) = range_to_offset_size(bounds, buffer.size);
                self.actions
                    .lock()
                    .buffer_mappings
                    .push(crate::api::command_buffer_actions::DeferredBufferMapping {
                        buffer: buffer.clone(),
                        mode,
                        offset,
                        size,
                        callback: alloc::boxed::Box::new(callback),
                    });
            }
            /// Registers a callback that is invoked when this command buffer’s work finishes
            /// executing on the GPU. When this callback runs, all mapped-buffer callbacks
            /// registered for the same submission are guaranteed to have been called.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn on_submitted_work_done(
                &self,
                callback: impl FnOnce() + Send + 'static,
            ) {
                self.actions
                    .lock()
                    .on_submitted_work_done_callbacks
                    .push(alloc::boxed::Box::new(callback));
            }
        }
        /// [`Features::PUSH_CONSTANTS`] must be enabled on the device in order to call these functions.
        impl ComputePass<'_> {
            /// Set push constant data for subsequent dispatch calls.
            ///
            /// Write the bytes in `data` at offset `offset` within push constant
            /// storage.  Both `offset` and the length of `data` must be
            /// multiples of [`PUSH_CONSTANT_ALIGNMENT`], which is always 4.
            ///
            /// For example, if `offset` is `4` and `data` is eight bytes long, this
            /// call will write `data` to bytes `4..12` of push constant storage.
            pub fn set_push_constants(&mut self, offset: u32, data: &[u8]) {
                self.inner.set_push_constants(offset, data);
            }
        }
        /// [`Features::TIMESTAMP_QUERY_INSIDE_PASSES`] must be enabled on the device in order to call these functions.
        impl ComputePass<'_> {
            /// Issue a timestamp command at this point in the queue. The timestamp will be written to the specified query set, at the specified index.
            ///
            /// Must be multiplied by [`Queue::get_timestamp_period`] to get
            /// the value in nanoseconds. Absolute values have no meaning,
            /// but timestamps can be subtracted to get the time it takes
            /// for a string of operations to complete.
            pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {
                self.inner.write_timestamp(&query_set.inner, query_index);
            }
        }
        /// [`Features::PIPELINE_STATISTICS_QUERY`] must be enabled on the device in order to call these functions.
        impl ComputePass<'_> {
            /// Start a pipeline statistics query on this compute pass. It can be ended with
            /// `end_pipeline_statistics_query`. Pipeline statistics queries may not be nested.
            pub fn begin_pipeline_statistics_query(
                &mut self,
                query_set: &QuerySet,
                query_index: u32,
            ) {
                self.inner
                    .begin_pipeline_statistics_query(&query_set.inner, query_index);
            }
            /// End the pipeline statistics query on this compute pass. It can be started with
            /// `begin_pipeline_statistics_query`. Pipeline statistics queries may not be nested.
            pub fn end_pipeline_statistics_query(&mut self) {
                self.inner.end_pipeline_statistics_query();
            }
        }
        /// Describes the timestamp writes of a compute pass.
        ///
        /// For use with [`ComputePassDescriptor`].
        /// At least one of `beginning_of_pass_write_index` and `end_of_pass_write_index` must be `Some`.
        ///
        /// Corresponds to [WebGPU `GPUComputePassTimestampWrites`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpucomputepasstimestampwrites).
        pub struct ComputePassTimestampWrites<'a> {
            /// The query set to write to.
            pub query_set: &'a QuerySet,
            /// The index of the query set at which a start timestamp of this pass is written, if any.
            pub beginning_of_pass_write_index: Option<u32>,
            /// The index of the query set at which an end timestamp of this pass is written, if any.
            pub end_of_pass_write_index: Option<u32>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ComputePassTimestampWrites<'a> {
            #[inline]
            fn clone(&self) -> ComputePassTimestampWrites<'a> {
                ComputePassTimestampWrites {
                    query_set: ::core::clone::Clone::clone(&self.query_set),
                    beginning_of_pass_write_index: ::core::clone::Clone::clone(
                        &self.beginning_of_pass_write_index,
                    ),
                    end_of_pass_write_index: ::core::clone::Clone::clone(
                        &self.end_of_pass_write_index,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for ComputePassTimestampWrites<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ComputePassTimestampWrites",
                    "query_set",
                    &self.query_set,
                    "beginning_of_pass_write_index",
                    &self.beginning_of_pass_write_index,
                    "end_of_pass_write_index",
                    &&self.end_of_pass_write_index,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ComputePassTimestampWrites<'_>>();
        };
        /// Describes the attachments of a compute pass.
        ///
        /// For use with [`CommandEncoder::begin_compute_pass`].
        ///
        /// Corresponds to [WebGPU `GPUComputePassDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpucomputepassdescriptor).
        pub struct ComputePassDescriptor<'a> {
            /// Debug label of the compute pass. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// Defines which timestamp values will be written for this pass, and where to write them to.
            ///
            /// Requires [`Features::TIMESTAMP_QUERY`] to be enabled.
            pub timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ComputePassDescriptor<'a> {
            #[inline]
            fn clone(&self) -> ComputePassDescriptor<'a> {
                ComputePassDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    timestamp_writes: ::core::clone::Clone::clone(&self.timestamp_writes),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::default::Default for ComputePassDescriptor<'a> {
            #[inline]
            fn default() -> ComputePassDescriptor<'a> {
                ComputePassDescriptor {
                    label: ::core::default::Default::default(),
                    timestamp_writes: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for ComputePassDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ComputePassDescriptor",
                    "label",
                    &self.label,
                    "timestamp_writes",
                    &&self.timestamp_writes,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ComputePassDescriptor<'_>>();
        };
    }
    mod compute_pipeline {
        use crate::*;
        /// Handle to a compute pipeline.
        ///
        /// A `ComputePipeline` object represents a compute pipeline and its single shader stage.
        /// It can be created with [`Device::create_compute_pipeline`].
        ///
        /// Corresponds to [WebGPU `GPUComputePipeline`](https://gpuweb.github.io/gpuweb/#compute-pipeline).
        pub struct ComputePipeline {
            pub(crate) inner: dispatch::DispatchComputePipeline,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ComputePipeline {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ComputePipeline",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ComputePipeline {
            #[inline]
            fn clone(&self) -> ComputePipeline {
                ComputePipeline {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ComputePipeline>();
        };
        impl PartialEq for ComputePipeline {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for ComputePipeline {}
        impl PartialOrd for ComputePipeline {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for ComputePipeline {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for ComputePipeline {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl ComputePipeline {
            /// Get an object representing the bind group layout at a given index.
            ///
            /// If this pipeline was created with a [default layout][ComputePipelineDescriptor::layout],
            /// then bind groups created with the returned `BindGroupLayout` can only be used with this
            /// pipeline.
            ///
            /// This method will raise a validation error if there is no bind group layout at `index`.
            pub fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout {
                let bind_group = self.inner.get_bind_group_layout(index);
                BindGroupLayout {
                    inner: bind_group,
                }
            }
        }
        /// Describes a compute pipeline.
        ///
        /// For use with [`Device::create_compute_pipeline`].
        ///
        /// Corresponds to [WebGPU `GPUComputePipelineDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpucomputepipelinedescriptor).
        pub struct ComputePipelineDescriptor<'a> {
            /// Debug label of the pipeline. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// The layout of bind groups for this pipeline.
            ///
            /// If this is set, then [`Device::create_compute_pipeline`] will raise a validation error if
            /// the layout doesn't match what the shader module(s) expect.
            ///
            /// Using the same [`PipelineLayout`] for many [`RenderPipeline`] or [`ComputePipeline`]
            /// pipelines guarantees that you don't have to rebind any resources when switching between
            /// those pipelines.
            ///
            /// ## Default pipeline layout
            ///
            /// If `layout` is `None`, then the pipeline has a [default layout] created and used instead.
            /// The default layout is deduced from the shader modules.
            ///
            /// You can use [`ComputePipeline::get_bind_group_layout`] to create bind groups for use with
            /// the default layout. However, these bind groups cannot be used with any other pipelines. This
            /// is convenient for simple pipelines, but using an explicit layout is recommended in most
            /// cases.
            ///
            /// [default layout]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
            pub layout: Option<&'a PipelineLayout>,
            /// The compiled shader module for this stage.
            pub module: &'a ShaderModule,
            /// The name of the entry point in the compiled shader to use.
            ///
            /// If [`Some`], there must be a compute shader entry point with this name in `module`.
            /// Otherwise, expect exactly one compute shader entry point in `module`, which will be
            /// selected.
            pub entry_point: Option<&'a str>,
            /// Advanced options for when this pipeline is compiled
            ///
            /// This implements `Default`, and for most users can be set to `Default::default()`
            pub compilation_options: PipelineCompilationOptions<'a>,
            /// The pipeline cache to use when creating this pipeline.
            pub cache: Option<&'a PipelineCache>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ComputePipelineDescriptor<'a> {
            #[inline]
            fn clone(&self) -> ComputePipelineDescriptor<'a> {
                ComputePipelineDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    layout: ::core::clone::Clone::clone(&self.layout),
                    module: ::core::clone::Clone::clone(&self.module),
                    entry_point: ::core::clone::Clone::clone(&self.entry_point),
                    compilation_options: ::core::clone::Clone::clone(
                        &self.compilation_options,
                    ),
                    cache: ::core::clone::Clone::clone(&self.cache),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for ComputePipelineDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "label",
                    "layout",
                    "module",
                    "entry_point",
                    "compilation_options",
                    "cache",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.label,
                    &self.layout,
                    &self.module,
                    &self.entry_point,
                    &self.compilation_options,
                    &&self.cache,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ComputePipelineDescriptor",
                    names,
                    values,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ComputePipelineDescriptor<'_>>();
        };
    }
    mod device {
        use alloc::{boxed::Box, string::String, sync::Arc, vec};
        use core::ops::Deref;
        use core::{error, fmt, future::Future};
        use crate::api::blas::{Blas, BlasGeometrySizeDescriptors, CreateBlasDescriptor};
        use crate::api::tlas::{CreateTlasDescriptor, Tlas};
        use crate::util::Mutex;
        use crate::*;
        /// Open connection to a graphics and/or compute device.
        ///
        /// Responsible for the creation of most rendering and compute resources.
        /// These are then used in commands, which are submitted to a [`Queue`].
        ///
        /// A device may be requested from an adapter with [`Adapter::request_device`].
        ///
        /// Corresponds to [WebGPU `GPUDevice`](https://gpuweb.github.io/gpuweb/#gpu-device).
        pub struct Device {
            pub(crate) inner: dispatch::DispatchDevice,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Device {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Device",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Device {
            #[inline]
            fn clone(&self) -> Device {
                Device {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Device>();
        };
        impl PartialEq for Device {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Device {}
        impl PartialOrd for Device {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Device {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Device {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        /// Describes a [`Device`].
        ///
        /// For use with [`Adapter::request_device`].
        ///
        /// Corresponds to [WebGPU `GPUDeviceDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpudevicedescriptor).
        pub type DeviceDescriptor<'a> = wgt::DeviceDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<DeviceDescriptor<'_>>();
        };
        impl Device {
            /// Check for resource cleanups and mapping callbacks. Will block if [`PollType::Wait`] is passed.
            ///
            /// Return `true` if the queue is empty, or `false` if there are more queue
            /// submissions still in flight. (Note that, unless access to the [`Queue`] is
            /// coordinated somehow, this information could be out of date by the time
            /// the caller receives it. `Queue`s can be shared between threads, so
            /// other threads could submit new work at any time.)
            ///
            /// When running on WebGPU, this is a no-op. `Device`s are automatically polled.
            pub fn poll(
                &self,
                poll_type: PollType,
            ) -> Result<crate::PollStatus, crate::PollError> {
                self.inner.poll(poll_type.map_index(|s| s.index))
            }
            /// The features which can be used on this device.
            ///
            /// No additional features can be used, even if the underlying adapter can support them.
            #[must_use]
            pub fn features(&self) -> Features {
                self.inner.features()
            }
            /// The limits which can be used on this device.
            ///
            /// No better limits can be used, even if the underlying adapter can support them.
            #[must_use]
            pub fn limits(&self) -> Limits {
                self.inner.limits()
            }
            /// Creates a shader module.
            ///
            /// <div class="warning">
            ///
            /// This function may consume a lot of stack space. Compiler-enforced limits for parsing
            /// recursion exist; if shader compilation runs into them, it will return an error gracefully.
            /// However, on some build profiles and platforms, the default stack size for a thread may be
            /// exceeded before this limit is reached during parsing. Callers should ensure that there is
            /// enough stack space for this, particularly if calls to this method are exposed to user
            /// input.
            ///
            /// </div>
            #[must_use]
            pub fn create_shader_module(
                &self,
                desc: ShaderModuleDescriptor<'_>,
            ) -> ShaderModule {
                let module = self
                    .inner
                    .create_shader_module(desc, wgt::ShaderRuntimeChecks::checked());
                ShaderModule { inner: module }
            }
            /// Deprecated: Use [`create_shader_module_trusted`][csmt] instead.
            ///
            /// # Safety
            ///
            /// See [`create_shader_module_trusted`][csmt].
            ///
            /// [csmt]: Self::create_shader_module_trusted
            #[deprecated(
                since = "24.0.0",
                note = "Use `Device::create_shader_module_trusted(desc, wgpu::ShaderRuntimeChecks::unchecked())` instead."
            )]
            #[must_use]
            pub unsafe fn create_shader_module_unchecked(
                &self,
                desc: ShaderModuleDescriptor<'_>,
            ) -> ShaderModule {
                unsafe {
                    self.create_shader_module_trusted(
                        desc,
                        crate::ShaderRuntimeChecks::unchecked(),
                    )
                }
            }
            /// Creates a shader module with flags to dictate runtime checks.
            ///
            /// When running on WebGPU, this will merely call [`create_shader_module`][csm].
            ///
            /// # Safety
            ///
            /// In contrast with [`create_shader_module`][csm] this function
            /// creates a shader module with user-customizable runtime checks which allows shaders to
            /// perform operations which can lead to undefined behavior like indexing out of bounds,
            /// thus it's the caller responsibility to pass a shader which doesn't perform any of this
            /// operations.
            ///
            /// See the documentation for [`ShaderRuntimeChecks`][src] for more information about specific checks.
            ///
            /// [csm]: Self::create_shader_module
            /// [src]: crate::ShaderRuntimeChecks
            #[must_use]
            pub unsafe fn create_shader_module_trusted(
                &self,
                desc: ShaderModuleDescriptor<'_>,
                runtime_checks: crate::ShaderRuntimeChecks,
            ) -> ShaderModule {
                let module = self.inner.create_shader_module(desc, runtime_checks);
                ShaderModule { inner: module }
            }
            /// Creates a shader module which will bypass wgpu's shader tooling and validation and be used directly by the backend.
            ///
            /// # Safety
            ///
            /// This function passes data to the backend as-is and can potentially result in a
            /// driver crash or bogus behaviour. No attempt is made to ensure that data is valid.
            #[must_use]
            pub unsafe fn create_shader_module_passthrough(
                &self,
                desc: ShaderModuleDescriptorPassthrough<'_>,
            ) -> ShaderModule {
                let module = unsafe {
                    self.inner.create_shader_module_passthrough(&desc)
                };
                ShaderModule { inner: module }
            }
            /// Creates an empty [`CommandEncoder`].
            #[must_use]
            pub fn create_command_encoder(
                &self,
                desc: &CommandEncoderDescriptor<'_>,
            ) -> CommandEncoder {
                let encoder = self.inner.create_command_encoder(desc);
                CommandEncoder {
                    inner: encoder,
                    actions: Default::default(),
                }
            }
            /// Creates an empty [`RenderBundleEncoder`].
            #[must_use]
            pub fn create_render_bundle_encoder<'a>(
                &self,
                desc: &RenderBundleEncoderDescriptor<'_>,
            ) -> RenderBundleEncoder<'a> {
                let encoder = self.inner.create_render_bundle_encoder(desc);
                RenderBundleEncoder {
                    inner: encoder,
                    _p: core::marker::PhantomData,
                }
            }
            /// Creates a new [`BindGroup`].
            #[must_use]
            pub fn create_bind_group(
                &self,
                desc: &BindGroupDescriptor<'_>,
            ) -> BindGroup {
                let group = self.inner.create_bind_group(desc);
                BindGroup { inner: group }
            }
            /// Creates a [`BindGroupLayout`].
            #[must_use]
            pub fn create_bind_group_layout(
                &self,
                desc: &BindGroupLayoutDescriptor<'_>,
            ) -> BindGroupLayout {
                let layout = self.inner.create_bind_group_layout(desc);
                BindGroupLayout { inner: layout }
            }
            /// Creates a [`PipelineLayout`].
            #[must_use]
            pub fn create_pipeline_layout(
                &self,
                desc: &PipelineLayoutDescriptor<'_>,
            ) -> PipelineLayout {
                let layout = self.inner.create_pipeline_layout(desc);
                PipelineLayout { inner: layout }
            }
            /// Creates a [`RenderPipeline`].
            #[must_use]
            pub fn create_render_pipeline(
                &self,
                desc: &RenderPipelineDescriptor<'_>,
            ) -> RenderPipeline {
                let pipeline = self.inner.create_render_pipeline(desc);
                RenderPipeline { inner: pipeline }
            }
            /// Creates a mesh shader based [`RenderPipeline`].
            #[must_use]
            pub fn create_mesh_pipeline(
                &self,
                desc: &MeshPipelineDescriptor<'_>,
            ) -> RenderPipeline {
                let pipeline = self.inner.create_mesh_pipeline(desc);
                RenderPipeline { inner: pipeline }
            }
            /// Creates a [`ComputePipeline`].
            #[must_use]
            pub fn create_compute_pipeline(
                &self,
                desc: &ComputePipelineDescriptor<'_>,
            ) -> ComputePipeline {
                let pipeline = self.inner.create_compute_pipeline(desc);
                ComputePipeline { inner: pipeline }
            }
            /// Creates a [`Buffer`].
            #[must_use]
            pub fn create_buffer(&self, desc: &BufferDescriptor<'_>) -> Buffer {
                let map_context = MapContext::new(
                    desc.mapped_at_creation.then_some(0..desc.size),
                );
                let buffer = self.inner.create_buffer(desc);
                Buffer {
                    inner: buffer,
                    map_context: Arc::new(Mutex::new(map_context)),
                    size: desc.size,
                    usage: desc.usage,
                }
            }
            /// Creates a new [`Texture`].
            ///
            /// `desc` specifies the general format of the texture.
            #[must_use]
            pub fn create_texture(&self, desc: &TextureDescriptor<'_>) -> Texture {
                let texture = self.inner.create_texture(desc);
                Texture {
                    inner: texture,
                    descriptor: TextureDescriptor {
                        label: None,
                        view_formats: &[],
                        ..desc.clone()
                    },
                }
            }
            /// Creates a [`Texture`] from a wgpu-hal Texture.
            ///
            /// # Types
            ///
            /// The type of `A::Texture` depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Texture`
            ///- [`hal::api::Metal`] uses [`hal::metal::Texture`]
            ///- `hal::api::Dx12` uses `hal::dx12::Texture`
            ///- `hal::api::Gles` uses `hal::gles::Texture`
            ///
            /// # Safety
            ///
            /// - `hal_texture` must be created from this device internal handle
            /// - `hal_texture` must be created respecting `desc`
            /// - `hal_texture` must be initialized
            #[must_use]
            pub unsafe fn create_texture_from_hal<A: hal::Api>(
                &self,
                hal_texture: A::Texture,
                desc: &TextureDescriptor<'_>,
            ) -> Texture {
                let texture = unsafe {
                    let core_device = self.inner.as_core();
                    core_device
                        .context
                        .create_texture_from_hal::<A>(hal_texture, core_device, desc)
                };
                Texture {
                    inner: texture.into(),
                    descriptor: TextureDescriptor {
                        label: None,
                        view_formats: &[],
                        ..desc.clone()
                    },
                }
            }
            /// Creates a new [`ExternalTexture`].
            #[must_use]
            pub fn create_external_texture(
                &self,
                desc: &ExternalTextureDescriptor<'_>,
                planes: &[&TextureView],
            ) -> ExternalTexture {
                let external_texture = self.inner.create_external_texture(desc, planes);
                ExternalTexture {
                    inner: external_texture,
                }
            }
            /// Creates a [`Buffer`] from a wgpu-hal Buffer.
            ///
            /// # Types
            ///
            /// The type of `A::Buffer` depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Buffer`
            ///- [`hal::api::Metal`] uses [`hal::metal::Buffer`]
            ///- `hal::api::Dx12` uses `hal::dx12::Buffer`
            ///- `hal::api::Gles` uses `hal::gles::Buffer`
            ///
            /// # Safety
            ///
            /// - `hal_buffer` must be created from this device internal handle
            /// - `hal_buffer` must be created respecting `desc`
            /// - `hal_buffer` must be initialized
            /// - `hal_buffer` must not have zero size
            #[must_use]
            pub unsafe fn create_buffer_from_hal<A: hal::Api>(
                &self,
                hal_buffer: A::Buffer,
                desc: &BufferDescriptor<'_>,
            ) -> Buffer {
                let map_context = MapContext::new(
                    desc.mapped_at_creation.then_some(0..desc.size),
                );
                let buffer = unsafe {
                    let core_device = self.inner.as_core();
                    core_device
                        .context
                        .create_buffer_from_hal::<A>(hal_buffer, core_device, desc)
                };
                Buffer {
                    inner: buffer.into(),
                    map_context: Arc::new(Mutex::new(map_context)),
                    size: desc.size,
                    usage: desc.usage,
                }
            }
            /// Creates a new [`Sampler`].
            ///
            /// `desc` specifies the behavior of the sampler.
            #[must_use]
            pub fn create_sampler(&self, desc: &SamplerDescriptor<'_>) -> Sampler {
                let sampler = self.inner.create_sampler(desc);
                Sampler { inner: sampler }
            }
            /// Creates a new [`QuerySet`].
            #[must_use]
            pub fn create_query_set(&self, desc: &QuerySetDescriptor<'_>) -> QuerySet {
                let query_set = self.inner.create_query_set(desc);
                QuerySet { inner: query_set }
            }
            /// Set a callback which will be called for all errors that are not handled in error scopes.
            pub fn on_uncaptured_error(&self, handler: Arc<dyn UncapturedErrorHandler>) {
                self.inner.on_uncaptured_error(handler)
            }
            /// Push an error scope.
            pub fn push_error_scope(&self, filter: ErrorFilter) {
                self.inner.push_error_scope(filter)
            }
            /// Pop an error scope.
            pub fn pop_error_scope(
                &self,
            ) -> impl Future<Output = Option<Error>> + WasmNotSend {
                self.inner.pop_error_scope()
            }
            /// Starts a capture in the attached graphics debugger.
            ///
            /// This behaves differently depending on which graphics debugger is attached:
            ///
            /// - Renderdoc: Calls [`StartFrameCapture(device, NULL)`][rd].
            /// - Xcode: Creates a capture with [`MTLCaptureManager`][xcode].
            /// - None: No action is taken.
            ///
            /// # Safety
            ///
            /// - There should not be any other captures currently active.
            /// - All other safety rules are defined by the graphics debugger, see the
            ///   documentation for the specific debugger.
            /// - In general, graphics debuggers can easily cause crashes, so this isn't
            ///   ever guaranteed to be sound.
            ///
            /// # Tips
            ///
            /// - Debuggers need to capture both the recording of the commands and the
            ///   submission of the commands to the GPU. Try to wrap all of your
            ///   gpu work in a capture.
            /// - If you encounter issues, try waiting for the GPU to finish all work
            ///   before stopping the capture.
            ///
            /// [rd]: https://renderdoc.org/docs/in_application_api.html#_CPPv417StartFrameCapture23RENDERDOC_DevicePointer22RENDERDOC_WindowHandle
            /// [xcode]: https://developer.apple.com/documentation/metal/mtlcapturemanager
            #[doc(alias = "start_renderdoc_capture")]
            #[doc(alias = "start_xcode_capture")]
            pub unsafe fn start_graphics_debugger_capture(&self) {
                unsafe { self.inner.start_graphics_debugger_capture() }
            }
            /// Stops the current capture in the attached graphics debugger.
            ///
            /// This behaves differently depending on which graphics debugger is attached:
            ///
            /// - Renderdoc: Calls [`EndFrameCapture(device, NULL)`][rd].
            /// - Xcode: Stops the capture with [`MTLCaptureManager`][xcode].
            /// - None: No action is taken.
            ///
            /// # Safety
            ///
            /// - There should be a capture currently active.
            /// - All other safety rules are defined by the graphics debugger, see the
            ///   documentation for the specific debugger.
            /// - In general, graphics debuggers can easily cause crashes, so this isn't
            ///   ever guaranteed to be sound.
            ///
            /// # Tips
            ///
            /// - If you encounter issues, try to submit all work to the GPU, and waiting
            ///   for that work to finish before stopping the capture.
            ///
            /// [rd]: https://renderdoc.org/docs/in_application_api.html#_CPPv415EndFrameCapture23RENDERDOC_DevicePointer22RENDERDOC_WindowHandle
            /// [xcode]: https://developer.apple.com/documentation/metal/mtlcapturemanager
            #[doc(alias = "stop_renderdoc_capture")]
            #[doc(alias = "stop_xcode_capture")]
            pub unsafe fn stop_graphics_debugger_capture(&self) {
                unsafe { self.inner.stop_graphics_debugger_capture() }
            }
            /// Query internal counters from the native backend for debugging purposes.
            ///
            /// Some backends may not set all counters, or may not set any counter at all.
            /// The `counters` cargo feature must be enabled for any counter to be set.
            ///
            /// If a counter is not set, its contains its default value (zero).
            #[must_use]
            pub fn get_internal_counters(&self) -> wgt::InternalCounters {
                self.inner.get_internal_counters()
            }
            /// Generate an GPU memory allocation report if the underlying backend supports it.
            ///
            /// Backends that do not support producing these reports return `None`. A backend may
            /// Support it and still return `None` if it is not using performing sub-allocation,
            /// for example as a workaround for driver issues.
            #[must_use]
            pub fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
                self.inner.generate_allocator_report()
            }
            /// Get the [`wgpu_hal`] device from this `Device`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Device`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Device`
            ///- [`hal::api::Metal`] uses [`hal::metal::Device`]
            ///- `hal::api::Dx12` uses `hal::dx12::Device`
            ///- `hal::api::Gles` uses `hal::gles::Device`
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The device is not from the backend specified by `A`.
            /// - The device is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Device`]: hal::Api::Device
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::Device> + WasmNotSendSync> {
                let device = self.inner.as_core_opt()?;
                unsafe { device.context.device_as_hal::<A>(device) }
            }
            /// Destroy this device.
            pub fn destroy(&self) {
                self.inner.destroy()
            }
            /// Set a DeviceLostCallback on this device.
            pub fn set_device_lost_callback(
                &self,
                callback: impl Fn(DeviceLostReason, String) + Send + 'static,
            ) {
                self.inner.set_device_lost_callback(Box::new(callback))
            }
            /// Create a [`PipelineCache`] with initial data
            ///
            /// This can be passed to [`Device::create_compute_pipeline`]
            /// and [`Device::create_render_pipeline`] to either accelerate these
            /// or add the cache results from those.
            ///
            /// # Safety
            ///
            /// If the `data` field of `desc` is set, it must have previously been returned from a call
            /// to [`PipelineCache::get_data`][^saving]. This `data` will only be used if it came
            /// from an adapter with the same [`util::pipeline_cache_key`].
            /// This *is* compatible across wgpu versions, as any data format change will
            /// be accounted for.
            ///
            /// It is *not* supported to bring caches from previous direct uses of backend APIs
            /// into this method.
            ///
            /// # Errors
            ///
            /// Returns an error value if:
            ///  * the [`PIPELINE_CACHE`](wgt::Features::PIPELINE_CACHE) feature is not enabled
            ///  * this device is invalid; or
            ///  * the device is out of memory
            ///
            /// This method also returns an error value if:
            ///  * The `fallback` field on `desc` is false; and
            ///  * the `data` provided would not be used[^data_not_used]
            ///
            /// If an error value is used in subsequent calls, default caching will be used.
            ///
            /// [^saving]: We do recognise that saving this data to disk means this condition
            /// is impossible to fully prove. Consider the risks for your own application in this case.
            ///
            /// [^data_not_used]: This data may be not used if: the data was produced by a prior
            /// version of wgpu; or was created for an incompatible adapter, or there was a GPU driver
            /// update. In some cases, the data might not be used and a real value is returned,
            /// this is left to the discretion of GPU drivers.
            #[must_use]
            pub unsafe fn create_pipeline_cache(
                &self,
                desc: &PipelineCacheDescriptor<'_>,
            ) -> PipelineCache {
                let cache = unsafe { self.inner.create_pipeline_cache(desc) };
                PipelineCache { inner: cache }
            }
        }
        /// [`Features::EXPERIMENTAL_RAY_QUERY`] must be enabled on the device in order to call these functions.
        impl Device {
            /// Create a bottom level acceleration structure, used inside a top level acceleration structure for ray tracing.
            /// - `desc`: The descriptor of the acceleration structure.
            /// - `sizes`: Size descriptor limiting what can be built into the acceleration structure.
            ///
            /// # Validation
            /// If any of the following is not satisfied a validation error is generated
            ///
            /// The device ***must*** have [`Features::EXPERIMENTAL_RAY_QUERY`] enabled.
            /// if `sizes` is [`BlasGeometrySizeDescriptors::Triangles`] then the following must be satisfied
            /// - For every geometry descriptor (for the purposes this is called `geo_desc`) of `sizes.descriptors` the following must be satisfied:
            ///     - `geo_desc.vertex_format` must be within allowed formats (allowed formats for a given feature set
            ///       may be queried with [`Features::allowed_vertex_formats_for_blas`]).
            ///     - Both or neither of `geo_desc.index_format` and `geo_desc.index_count` must be provided.
            ///
            /// [`Features::EXPERIMENTAL_RAY_QUERY`]: wgt::Features::EXPERIMENTAL_RAY_QUERY
            /// [`Features::allowed_vertex_formats_for_blas`]: wgt::Features::allowed_vertex_formats_for_blas
            #[must_use]
            pub fn create_blas(
                &self,
                desc: &CreateBlasDescriptor<'_>,
                sizes: BlasGeometrySizeDescriptors,
            ) -> Blas {
                let (handle, blas) = self.inner.create_blas(desc, sizes);
                Blas { inner: blas, handle }
            }
            /// Create a top level acceleration structure, used for ray tracing.
            /// - `desc`: The descriptor of the acceleration structure.
            ///
            /// # Validation
            /// If any of the following is not satisfied a validation error is generated
            ///
            /// The device ***must*** have [`Features::EXPERIMENTAL_RAY_QUERY`] enabled.
            ///
            /// [`Features::EXPERIMENTAL_RAY_QUERY`]: wgt::Features::EXPERIMENTAL_RAY_QUERY
            #[must_use]
            pub fn create_tlas(&self, desc: &CreateTlasDescriptor<'_>) -> Tlas {
                let tlas = self.inner.create_tlas(desc);
                Tlas {
                    inner: tlas,
                    instances: ::alloc::vec::from_elem(
                        None,
                        desc.max_instances as usize,
                    ),
                    lowest_unmodified: 0,
                }
            }
        }
        /// Requesting a device from an [`Adapter`] failed.
        pub struct RequestDeviceError {
            pub(crate) inner: RequestDeviceErrorKind,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RequestDeviceError {
            #[inline]
            fn clone(&self) -> RequestDeviceError {
                RequestDeviceError {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RequestDeviceError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "RequestDeviceError",
                    "inner",
                    &&self.inner,
                )
            }
        }
        pub(crate) enum RequestDeviceErrorKind {
            /// Error from [`wgpu_core`].
            Core(wgc::instance::RequestDeviceError),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RequestDeviceErrorKind {
            #[inline]
            fn clone(&self) -> RequestDeviceErrorKind {
                match self {
                    RequestDeviceErrorKind::Core(__self_0) => {
                        RequestDeviceErrorKind::Core(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RequestDeviceErrorKind {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    RequestDeviceErrorKind::Core(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Core",
                            &__self_0,
                        )
                    }
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RequestDeviceError>();
        };
        impl fmt::Display for RequestDeviceError {
            fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self.inner {
                    RequestDeviceErrorKind::Core(error) => error.fmt(_f),
                }
            }
        }
        impl error::Error for RequestDeviceError {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                match &self.inner {
                    RequestDeviceErrorKind::Core(error) => error.source(),
                }
            }
        }
        impl From<wgc::instance::RequestDeviceError> for RequestDeviceError {
            fn from(error: wgc::instance::RequestDeviceError) -> Self {
                Self {
                    inner: RequestDeviceErrorKind::Core(error),
                }
            }
        }
        /// The callback of [`Device::on_uncaptured_error()`].
        ///
        /// It must be a function with this signature.
        pub trait UncapturedErrorHandler: Fn(Error) + Send + Sync + 'static {}
        impl<T> UncapturedErrorHandler for T
        where
            T: Fn(Error) + Send + Sync + 'static,
        {}
        /// Kinds of [`Error`]s a [`Device::push_error_scope()`] may be configured to catch.
        pub enum ErrorFilter {
            /// Catch only out-of-memory errors.
            OutOfMemory,
            /// Catch only validation errors.
            Validation,
            /// Catch only internal errors.
            Internal,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ErrorFilter {
            #[inline]
            fn clone(&self) -> ErrorFilter {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ErrorFilter {}
        #[automatically_derived]
        impl ::core::fmt::Debug for ErrorFilter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ErrorFilter::OutOfMemory => "OutOfMemory",
                        ErrorFilter::Validation => "Validation",
                        ErrorFilter::Internal => "Internal",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ErrorFilter {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ErrorFilter {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ErrorFilter {
            #[inline]
            fn eq(&self, other: &ErrorFilter) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ErrorFilter {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ErrorFilter,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ErrorFilter>();
        };
        /// Lower level source of the error.
        ///
        /// `Send + Sync` varies depending on configuration.
        pub type ErrorSource = Box<dyn error::Error + Send + Sync + 'static>;
        /// Errors resulting from usage of GPU APIs.
        ///
        /// By default, errors translate into panics. Depending on the backend and circumstances,
        /// errors may occur synchronously or asynchronously. When errors need to be handled, use
        /// [`Device::push_error_scope()`] or [`Device::on_uncaptured_error()`].
        pub enum Error {
            /// Out of memory.
            OutOfMemory {
                /// Lower level source of the error.
                source: ErrorSource,
            },
            /// Validation error, signifying a bug in code or data provided to `wgpu`.
            Validation {
                /// Lower level source of the error.
                source: ErrorSource,
                /// Description of the validation error.
                description: String,
            },
            /// Internal error. Used for signalling any failures not explicitly expected by WebGPU.
            ///
            /// These could be due to internal implementation or system limits being reached.
            Internal {
                /// Lower level source of the error.
                source: ErrorSource,
                /// Description of the internal GPU error.
                description: String,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::OutOfMemory { source: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "OutOfMemory",
                            "source",
                            &__self_0,
                        )
                    }
                    Error::Validation { source: __self_0, description: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Validation",
                            "source",
                            __self_0,
                            "description",
                            &__self_1,
                        )
                    }
                    Error::Internal { source: __self_0, description: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Internal",
                            "source",
                            __self_0,
                            "description",
                            &__self_1,
                        )
                    }
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Error>();
        };
        impl error::Error for Error {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                match self {
                    Error::OutOfMemory { source } => Some(source.as_ref()),
                    Error::Validation { source, .. } => Some(source.as_ref()),
                    Error::Internal { source, .. } => Some(source.as_ref()),
                }
            }
        }
        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Error::OutOfMemory { .. } => f.write_str("Out of Memory"),
                    Error::Validation { description, .. } => f.write_str(description),
                    Error::Internal { description, .. } => f.write_str(description),
                }
            }
        }
    }
    mod external_texture {
        use crate::*;
        /// Handle to an external texture on the GPU.
        ///
        /// It can be created with [`Device::create_external_texture`].
        ///
        /// Corresponds to [WebGPU `GPUExternalTexture`](https://gpuweb.github.io/gpuweb/#gpuexternaltexture).
        pub struct ExternalTexture {
            pub(crate) inner: dispatch::DispatchExternalTexture,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExternalTexture {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ExternalTexture",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ExternalTexture {
            #[inline]
            fn clone(&self) -> ExternalTexture {
                ExternalTexture {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ExternalTexture>();
        };
        impl PartialEq for ExternalTexture {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for ExternalTexture {}
        impl PartialOrd for ExternalTexture {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for ExternalTexture {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for ExternalTexture {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl ExternalTexture {
            /// Destroy the associated native resources as soon as possible.
            pub fn destroy(&self) {
                self.inner.destroy();
            }
        }
        /// Describes an [`ExternalTexture`].
        ///
        /// For use with [`Device::create_external_texture`].
        ///
        /// Corresponds to [WebGPU `GPUExternalTextureDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpuexternaltexturedescriptor).
        pub type ExternalTextureDescriptor<'a> = wgt::ExternalTextureDescriptor<
            Label<'a>,
        >;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ExternalTextureDescriptor<'_>>();
        };
    }
    mod instance {
        use alloc::vec::Vec;
        use core::future::Future;
        use crate::{dispatch::InstanceInterface, util::Mutex, *};
        /// WGSL language extensions.
        ///
        /// WGSL spec.: <https://www.w3.org/TR/WGSL/#language-extensions-sec>
        pub struct WgslLanguageFeatures(
            <WgslLanguageFeatures as ::bitflags::__private::PublicFlags>::Internal,
        );
        #[automatically_derived]
        impl ::core::fmt::Debug for WgslLanguageFeatures {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "WgslLanguageFeatures",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WgslLanguageFeatures {
            #[inline]
            fn clone(&self) -> WgslLanguageFeatures {
                WgslLanguageFeatures(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WgslLanguageFeatures {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WgslLanguageFeatures {
            #[inline]
            fn eq(&self, other: &WgslLanguageFeatures) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for WgslLanguageFeatures {
            #[inline]
            fn partial_cmp(
                &self,
                other: &WgslLanguageFeatures,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for WgslLanguageFeatures {
            #[inline]
            fn cmp(&self, other: &WgslLanguageFeatures) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for WgslLanguageFeatures {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <WgslLanguageFeatures as ::bitflags::__private::PublicFlags>::Internal,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WgslLanguageFeatures {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl WgslLanguageFeatures {
            /// <https://www.w3.org/TR/WGSL/#language_extension-readonly_and_readwrite_storage_textures>
            #[allow(deprecated, non_upper_case_globals)]
            pub const ReadOnlyAndReadWriteStorageTextures: Self = Self::from_bits_retain(
                1 << 0,
            );
            /// <https://www.w3.org/TR/WGSL/#language_extension-packed_4x8_integer_dot_product>
            #[allow(deprecated, non_upper_case_globals)]
            pub const Packed4x8IntegerDotProduct: Self = Self::from_bits_retain(1 << 1);
            /// <https://www.w3.org/TR/WGSL/#language_extension-unrestricted_pointer_parameters>
            #[allow(deprecated, non_upper_case_globals)]
            pub const UnrestrictedPointerParameters: Self = Self::from_bits_retain(
                1 << 2,
            );
            /// <https://www.w3.org/TR/WGSL/#language_extension-pointer_composite_access>
            #[allow(deprecated, non_upper_case_globals)]
            pub const PointerCompositeAccess: Self = Self::from_bits_retain(1 << 3);
        }
        impl ::bitflags::Flags for WgslLanguageFeatures {
            const FLAGS: &'static [::bitflags::Flag<WgslLanguageFeatures>] = &[
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new(
                        "ReadOnlyAndReadWriteStorageTextures",
                        WgslLanguageFeatures::ReadOnlyAndReadWriteStorageTextures,
                    )
                },
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new(
                        "Packed4x8IntegerDotProduct",
                        WgslLanguageFeatures::Packed4x8IntegerDotProduct,
                    )
                },
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new(
                        "UnrestrictedPointerParameters",
                        WgslLanguageFeatures::UnrestrictedPointerParameters,
                    )
                },
                {
                    #[allow(deprecated, non_upper_case_globals)]
                    ::bitflags::Flag::new(
                        "PointerCompositeAccess",
                        WgslLanguageFeatures::PointerCompositeAccess,
                    )
                },
            ];
            type Bits = u32;
            fn bits(&self) -> u32 {
                WgslLanguageFeatures::bits(self)
            }
            fn from_bits_retain(bits: u32) -> WgslLanguageFeatures {
                WgslLanguageFeatures::from_bits_retain(bits)
            }
        }
        #[allow(
            dead_code,
            deprecated,
            unused_doc_comments,
            unused_attributes,
            unused_mut,
            unused_imports,
            non_upper_case_globals,
            clippy::assign_op_pattern,
            clippy::indexing_slicing,
            clippy::same_name_method,
            clippy::iter_without_into_iter,
        )]
        const _: () = {
            #[repr(transparent)]
            pub struct InternalBitFlags(u32);
            #[automatically_derived]
            impl ::core::clone::Clone for InternalBitFlags {
                #[inline]
                fn clone(&self) -> InternalBitFlags {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for InternalBitFlags {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for InternalBitFlags {
                #[inline]
                fn eq(&self, other: &InternalBitFlags) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for InternalBitFlags {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for InternalBitFlags {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &InternalBitFlags,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for InternalBitFlags {
                #[inline]
                fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for InternalBitFlags {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.0, state)
                }
            }
            impl ::bitflags::__private::PublicFlags for WgslLanguageFeatures {
                type Primitive = u32;
                type Internal = InternalBitFlags;
            }
            impl ::bitflags::__private::core::default::Default for InternalBitFlags {
                #[inline]
                fn default() -> Self {
                    InternalBitFlags::empty()
                }
            }
            impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                ) -> ::bitflags::__private::core::fmt::Result {
                    if self.is_empty() {
                        f.write_fmt(
                            format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY),
                        )
                    } else {
                        ::bitflags::__private::core::fmt::Display::fmt(self, f)
                    }
                }
            }
            impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
                ) -> ::bitflags::__private::core::fmt::Result {
                    ::bitflags::parser::to_writer(&WgslLanguageFeatures(*self), f)
                }
            }
            impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
                type Err = ::bitflags::parser::ParseError;
                fn from_str(
                    s: &str,
                ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                    ::bitflags::parser::from_str::<WgslLanguageFeatures>(s)
                        .map(|flags| flags.0)
                }
            }
            impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
                fn as_ref(&self) -> &u32 {
                    &self.0
                }
            }
            impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
                fn from(bits: u32) -> Self {
                    Self::from_bits_retain(bits)
                }
            }
            #[allow(dead_code, deprecated, unused_attributes)]
            impl InternalBitFlags {
                /// Get a flags value with all bits unset.
                #[inline]
                pub const fn empty() -> Self {
                    Self(<u32 as ::bitflags::Bits>::EMPTY)
                }
                /// Get a flags value with all known bits set.
                #[inline]
                pub const fn all() -> Self {
                    let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                    let mut i = 0;
                    {
                        {
                            let flag = <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    let _ = i;
                    Self(truncated)
                }
                /// Get the underlying bits value.
                ///
                /// The returned value is exactly the bits set in this flags value.
                #[inline]
                pub const fn bits(&self) -> u32 {
                    self.0
                }
                /// Convert from a bits value.
                ///
                /// This method will return `None` if any unknown bits are set.
                #[inline]
                pub const fn from_bits(
                    bits: u32,
                ) -> ::bitflags::__private::core::option::Option<Self> {
                    let truncated = Self::from_bits_truncate(bits).0;
                    if truncated == bits {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    } else {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
                /// Convert from a bits value, unsetting any unknown bits.
                #[inline]
                pub const fn from_bits_truncate(bits: u32) -> Self {
                    Self(bits & Self::all().0)
                }
                /// Convert from a bits value exactly.
                #[inline]
                pub const fn from_bits_retain(bits: u32) -> Self {
                    Self(bits)
                }
                /// Get a flags value with the bits of a flag with the given name set.
                ///
                /// This method will return `None` if `name` is empty or doesn't
                /// correspond to any named flag.
                #[inline]
                pub fn from_name(
                    name: &str,
                ) -> ::bitflags::__private::core::option::Option<Self> {
                    {
                        if name == "ReadOnlyAndReadWriteStorageTextures" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(
                                    WgslLanguageFeatures::ReadOnlyAndReadWriteStorageTextures
                                        .bits(),
                                ),
                            );
                        }
                    };
                    {
                        if name == "Packed4x8IntegerDotProduct" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(
                                    WgslLanguageFeatures::Packed4x8IntegerDotProduct.bits(),
                                ),
                            );
                        }
                    };
                    {
                        if name == "UnrestrictedPointerParameters" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(
                                    WgslLanguageFeatures::UnrestrictedPointerParameters.bits(),
                                ),
                            );
                        }
                    };
                    {
                        if name == "PointerCompositeAccess" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(WgslLanguageFeatures::PointerCompositeAccess.bits()),
                            );
                        }
                    };
                    let _ = name;
                    ::bitflags::__private::core::option::Option::None
                }
                /// Whether all bits in this flags value are unset.
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    self.0 == <u32 as ::bitflags::Bits>::EMPTY
                }
                /// Whether all known bits in this flags value are set.
                #[inline]
                pub const fn is_all(&self) -> bool {
                    Self::all().0 | self.0 == self.0
                }
                /// Whether any set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn intersects(&self, other: Self) -> bool {
                    self.0 & other.0 != <u32 as ::bitflags::Bits>::EMPTY
                }
                /// Whether all set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn contains(&self, other: Self) -> bool {
                    self.0 & other.0 == other.0
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                pub fn insert(&mut self, other: Self) {
                    *self = Self(self.0).union(other);
                }
                /// The intersection of a source flags value with the complement of a target flags
                /// value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[inline]
                pub fn remove(&mut self, other: Self) {
                    *self = Self(self.0).difference(other);
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                pub fn toggle(&mut self, other: Self) {
                    *self = Self(self.0).symmetric_difference(other);
                }
                /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                #[inline]
                pub fn set(&mut self, other: Self, value: bool) {
                    if value {
                        self.insert(other);
                    } else {
                        self.remove(other);
                    }
                }
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn intersection(self, other: Self) -> Self {
                    Self(self.0 & other.0)
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn union(self, other: Self) -> Self {
                    Self(self.0 | other.0)
                }
                /// The intersection of a source flags value with the complement of a target flags
                /// value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                #[must_use]
                pub const fn difference(self, other: Self) -> Self {
                    Self(self.0 & !other.0)
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn symmetric_difference(self, other: Self) -> Self {
                    Self(self.0 ^ other.0)
                }
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                #[must_use]
                pub const fn complement(self) -> Self {
                    Self::from_bits_truncate(!self.0)
                }
            }
            impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
                type Output = Self;
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor(self, other: InternalBitFlags) -> Self {
                    self.union(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor_assign(&mut self, other: Self) {
                    self.insert(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
                type Output = Self;
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor(self, other: Self) -> Self {
                    self.symmetric_difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor_assign(&mut self, other: Self) {
                    self.toggle(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
                type Output = Self;
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand(self, other: Self) -> Self {
                    self.intersection(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand_assign(&mut self, other: Self) {
                    *self = Self::from_bits_retain(self.bits()).intersection(other);
                }
            }
            impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
                type Output = Self;
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub(self, other: Self) -> Self {
                    self.difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub_assign(&mut self, other: Self) {
                    self.remove(other);
                }
            }
            impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
                type Output = Self;
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                fn not(self) -> Self {
                    self.complement()
                }
            }
            impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
            for InternalBitFlags {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn extend<
                    T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                >(&mut self, iterator: T) {
                    for item in iterator {
                        self.insert(item)
                    }
                }
            }
            impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
            for InternalBitFlags {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn from_iter<
                    T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                >(iterator: T) -> Self {
                    use ::bitflags::__private::core::iter::Extend;
                    let mut result = Self::empty();
                    result.extend(iterator);
                    result
                }
            }
            impl InternalBitFlags {
                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                /// will be yielded together as a final flags value.
                #[inline]
                pub const fn iter(
                    &self,
                ) -> ::bitflags::iter::Iter<WgslLanguageFeatures> {
                    ::bitflags::iter::Iter::__private_const_new(
                        <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS,
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                    )
                }
                /// Yield a set of contained named flags values.
                ///
                /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                #[inline]
                pub const fn iter_names(
                    &self,
                ) -> ::bitflags::iter::IterNames<WgslLanguageFeatures> {
                    ::bitflags::iter::IterNames::__private_const_new(
                        <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS,
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                    )
                }
            }
            impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
                type Item = WgslLanguageFeatures;
                type IntoIter = ::bitflags::iter::Iter<WgslLanguageFeatures>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }
            impl InternalBitFlags {
                /// Returns a mutable reference to the raw value of the flags currently stored.
                #[inline]
                pub fn bits_mut(&mut self) -> &mut u32 {
                    &mut self.0
                }
            }
            #[allow(dead_code, deprecated, unused_attributes)]
            impl WgslLanguageFeatures {
                /// Get a flags value with all bits unset.
                #[inline]
                pub const fn empty() -> Self {
                    Self(InternalBitFlags::empty())
                }
                /// Get a flags value with all known bits set.
                #[inline]
                pub const fn all() -> Self {
                    Self(InternalBitFlags::all())
                }
                /// Get the underlying bits value.
                ///
                /// The returned value is exactly the bits set in this flags value.
                #[inline]
                pub const fn bits(&self) -> u32 {
                    self.0.bits()
                }
                /// Convert from a bits value.
                ///
                /// This method will return `None` if any unknown bits are set.
                #[inline]
                pub const fn from_bits(
                    bits: u32,
                ) -> ::bitflags::__private::core::option::Option<Self> {
                    match InternalBitFlags::from_bits(bits) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        }
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
                /// Convert from a bits value, unsetting any unknown bits.
                #[inline]
                pub const fn from_bits_truncate(bits: u32) -> Self {
                    Self(InternalBitFlags::from_bits_truncate(bits))
                }
                /// Convert from a bits value exactly.
                #[inline]
                pub const fn from_bits_retain(bits: u32) -> Self {
                    Self(InternalBitFlags::from_bits_retain(bits))
                }
                /// Get a flags value with the bits of a flag with the given name set.
                ///
                /// This method will return `None` if `name` is empty or doesn't
                /// correspond to any named flag.
                #[inline]
                pub fn from_name(
                    name: &str,
                ) -> ::bitflags::__private::core::option::Option<Self> {
                    match InternalBitFlags::from_name(name) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        }
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
                /// Whether all bits in this flags value are unset.
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    self.0.is_empty()
                }
                /// Whether all known bits in this flags value are set.
                #[inline]
                pub const fn is_all(&self) -> bool {
                    self.0.is_all()
                }
                /// Whether any set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn intersects(&self, other: Self) -> bool {
                    self.0.intersects(other.0)
                }
                /// Whether all set bits in a source flags value are also set in a target flags value.
                #[inline]
                pub const fn contains(&self, other: Self) -> bool {
                    self.0.contains(other.0)
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                pub fn insert(&mut self, other: Self) {
                    self.0.insert(other.0)
                }
                /// The intersection of a source flags value with the complement of a target flags
                /// value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[inline]
                pub fn remove(&mut self, other: Self) {
                    self.0.remove(other.0)
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                pub fn toggle(&mut self, other: Self) {
                    self.0.toggle(other.0)
                }
                /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
                #[inline]
                pub fn set(&mut self, other: Self, value: bool) {
                    self.0.set(other.0, value)
                }
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn intersection(self, other: Self) -> Self {
                    Self(self.0.intersection(other.0))
                }
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn union(self, other: Self) -> Self {
                    Self(self.0.union(other.0))
                }
                /// The intersection of a source flags value with the complement of a target flags
                /// value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                #[must_use]
                pub const fn difference(self, other: Self) -> Self {
                    Self(self.0.difference(other.0))
                }
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                #[must_use]
                pub const fn symmetric_difference(self, other: Self) -> Self {
                    Self(self.0.symmetric_difference(other.0))
                }
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                #[must_use]
                pub const fn complement(self) -> Self {
                    Self(self.0.complement())
                }
            }
            impl ::bitflags::__private::core::fmt::Binary for WgslLanguageFeatures {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::Octal for WgslLanguageFeatures {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::LowerHex for WgslLanguageFeatures {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::fmt::UpperHex for WgslLanguageFeatures {
                fn fmt(
                    &self,
                    f: &mut ::bitflags::__private::core::fmt::Formatter,
                ) -> ::bitflags::__private::core::fmt::Result {
                    let inner = self.0;
                    ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
                }
            }
            impl ::bitflags::__private::core::ops::BitOr for WgslLanguageFeatures {
                type Output = Self;
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor(self, other: WgslLanguageFeatures) -> Self {
                    self.union(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitOrAssign for WgslLanguageFeatures {
                /// The bitwise or (`|`) of the bits in two flags values.
                #[inline]
                fn bitor_assign(&mut self, other: Self) {
                    self.insert(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitXor for WgslLanguageFeatures {
                type Output = Self;
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor(self, other: Self) -> Self {
                    self.symmetric_difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitXorAssign
            for WgslLanguageFeatures {
                /// The bitwise exclusive-or (`^`) of the bits in two flags values.
                #[inline]
                fn bitxor_assign(&mut self, other: Self) {
                    self.toggle(other);
                }
            }
            impl ::bitflags::__private::core::ops::BitAnd for WgslLanguageFeatures {
                type Output = Self;
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand(self, other: Self) -> Self {
                    self.intersection(other)
                }
            }
            impl ::bitflags::__private::core::ops::BitAndAssign
            for WgslLanguageFeatures {
                /// The bitwise and (`&`) of the bits in two flags values.
                #[inline]
                fn bitand_assign(&mut self, other: Self) {
                    *self = Self::from_bits_retain(self.bits()).intersection(other);
                }
            }
            impl ::bitflags::__private::core::ops::Sub for WgslLanguageFeatures {
                type Output = Self;
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub(self, other: Self) -> Self {
                    self.difference(other)
                }
            }
            impl ::bitflags::__private::core::ops::SubAssign for WgslLanguageFeatures {
                /// The intersection of a source flags value with the complement of a target flags value (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline]
                fn sub_assign(&mut self, other: Self) {
                    self.remove(other);
                }
            }
            impl ::bitflags::__private::core::ops::Not for WgslLanguageFeatures {
                type Output = Self;
                /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
                #[inline]
                fn not(self) -> Self {
                    self.complement()
                }
            }
            impl ::bitflags::__private::core::iter::Extend<WgslLanguageFeatures>
            for WgslLanguageFeatures {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn extend<
                    T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                >(&mut self, iterator: T) {
                    for item in iterator {
                        self.insert(item)
                    }
                }
            }
            impl ::bitflags::__private::core::iter::FromIterator<WgslLanguageFeatures>
            for WgslLanguageFeatures {
                /// The bitwise or (`|`) of the bits in each flags value.
                fn from_iter<
                    T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
                >(iterator: T) -> Self {
                    use ::bitflags::__private::core::iter::Extend;
                    let mut result = Self::empty();
                    result.extend(iterator);
                    result
                }
            }
            impl WgslLanguageFeatures {
                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                /// will be yielded together as a final flags value.
                #[inline]
                pub const fn iter(
                    &self,
                ) -> ::bitflags::iter::Iter<WgslLanguageFeatures> {
                    ::bitflags::iter::Iter::__private_const_new(
                        <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS,
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                    )
                }
                /// Yield a set of contained named flags values.
                ///
                /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
                /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
                #[inline]
                pub const fn iter_names(
                    &self,
                ) -> ::bitflags::iter::IterNames<WgslLanguageFeatures> {
                    ::bitflags::iter::IterNames::__private_const_new(
                        <WgslLanguageFeatures as ::bitflags::Flags>::FLAGS,
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                        WgslLanguageFeatures::from_bits_retain(self.bits()),
                    )
                }
            }
            impl ::bitflags::__private::core::iter::IntoIterator
            for WgslLanguageFeatures {
                type Item = WgslLanguageFeatures;
                type IntoIter = ::bitflags::iter::Iter<WgslLanguageFeatures>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }
        };
        /// Contains the various entry points to start interacting with the system's GPUs.
        ///
        /// This is the first thing you create when using wgpu.
        /// Its primary use is to create [`Adapter`]s and [`Surface`]s.
        ///
        /// Does not have to be kept alive.
        ///
        /// Corresponds to [WebGPU `GPU`](https://gpuweb.github.io/gpuweb/#gpu-interface).
        pub struct Instance {
            inner: dispatch::DispatchInstance,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Instance {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Instance",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Instance {
            #[inline]
            fn clone(&self) -> Instance {
                Instance {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Instance>();
        };
        impl PartialEq for Instance {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Instance {}
        impl PartialOrd for Instance {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Instance {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Instance {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Default for Instance {
            /// Creates a new instance of wgpu with default options.
            ///
            /// Backends are set to `Backends::all()`, and FXC is chosen as the `dx12_shader_compiler`.
            ///
            /// # Panics
            ///
            /// If no backend feature for the active target platform is enabled,
            /// this method will panic, see [`Instance::enabled_backend_features()`].
            fn default() -> Self {
                Self::new(&InstanceDescriptor::default())
            }
        }
        impl Instance {
            /// Create an new instance of wgpu using the given options and enabled backends.
            ///
            /// # Panics
            ///
            /// - If no backend feature for the active target platform is enabled,
            ///   this method will panic; see [`Instance::enabled_backend_features()`].
            #[allow(clippy::allow_attributes, unreachable_code)]
            pub fn new(desc: &InstanceDescriptor) -> Self {
                if Self::enabled_backend_features().is_empty() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "No wgpu backend feature that is implemented for the target platform was enabled. See `wgpu::Instance::enabled_backend_features()` for more information.",
                            ),
                        );
                    };
                }
                {
                    return Self {
                        inner: crate::backend::ContextWgpuCore::new(desc).into(),
                    };
                }
                let _ = desc;
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!(
                                "Earlier check of `enabled_backend_features` should have prevented getting here!",
                            ),
                        ),
                    );
                };
            }
            /// Returns which backends can be picked for the current build configuration.
            ///
            /// The returned set depends on a combination of target platform and enabled features.
            /// This does *not* do any runtime checks and is exclusively based on compile time information.
            ///
            /// `InstanceDescriptor::backends` does not need to be a subset of this,
            /// but any backend that is not in this set, will not be picked.
            pub const fn enabled_backend_features() -> Backends {
                let mut backends = Backends::empty();
                if false {
                    backends = backends.union(Backends::NOOP);
                }
                if false {
                    backends = backends.union(Backends::VULKAN);
                }
                if false {
                    backends = backends.union(Backends::GL);
                }
                if true {
                    backends = backends.union(Backends::METAL);
                }
                if false {
                    backends = backends.union(Backends::DX12);
                }
                if false {
                    backends = backends.union(Backends::BROWSER_WEBGPU);
                }
                backends
            }
            /// Returns the set of [WGSL language extensions] supported by this instance.
            ///
            /// [WGSL language extensions]: https://www.w3.org/TR/webgpu/#gpuwgsllanguagefeatures
            pub fn wgsl_language_features(&self) -> WgslLanguageFeatures {
                self.inner.wgsl_language_features()
            }
            /// Retrieves all available [`Adapter`]s that match the given [`Backends`].
            ///
            /// # Arguments
            ///
            /// - `backends` - Backends from which to enumerate adapters.
            pub fn enumerate_adapters(
                &self,
                backends: Backends,
            ) -> impl Future<Output = Vec<Adapter>> {
                let future = self.inner.enumerate_adapters(backends);
                async move {
                    future
                        .await
                        .iter()
                        .map(|adapter| Adapter { inner: adapter.clone() })
                        .collect()
                }
            }
            /// Retrieves an [`Adapter`] which matches the given [`RequestAdapterOptions`].
            ///
            /// Some options are "soft", so treated as non-mandatory. Others are "hard".
            ///
            /// If no adapters are found that satisfy all the "hard" options, an error is returned.
            ///
            /// When targeting WebGL2, a [`compatible_surface`](RequestAdapterOptions::compatible_surface)
            /// must be specified; using `RequestAdapterOptions::default()` will not succeed.
            pub fn request_adapter(
                &self,
                options: &RequestAdapterOptions<'_, '_>,
            ) -> impl Future<
                Output = Result<Adapter, RequestAdapterError>,
            > + WasmNotSend {
                let future = self.inner.request_adapter(options);
                async move { future.await.map(|adapter| Adapter { inner: adapter }) }
            }
            /// Creates a new surface targeting a given window/canvas/surface/etc..
            ///
            /// Internally, this creates surfaces for all backends that are enabled for this instance.
            ///
            /// See [`SurfaceTarget`] for what targets are supported.
            /// See [`Instance::create_surface_unsafe`] for surface creation with unsafe target variants.
            ///
            /// Most commonly used are window handles (or provider of windows handles)
            /// which can be passed directly as they're automatically converted to [`SurfaceTarget`].
            pub fn create_surface<'window>(
                &self,
                target: impl Into<SurfaceTarget<'window>>,
            ) -> Result<Surface<'window>, CreateSurfaceError> {
                let handle_source;
                let target = target.into();
                let mut surface = match target {
                    SurfaceTarget::Window(window) => {
                        unsafe {
                            let surface = self
                                .create_surface_unsafe(
                                    SurfaceTargetUnsafe::from_window(&window)
                                        .map_err(|e| CreateSurfaceError {
                                            inner: CreateSurfaceErrorKind::RawHandle(e),
                                        })?,
                                );
                            handle_source = Some(window);
                            surface
                        }?
                    }
                };
                surface._handle_source = handle_source;
                Ok(surface)
            }
            /// Creates a new surface targeting a given window/canvas/surface/etc. using an unsafe target.
            ///
            /// Internally, this creates surfaces for all backends that are enabled for this instance.
            ///
            /// See [`SurfaceTargetUnsafe`] for what targets are supported.
            /// See [`Instance::create_surface`] for surface creation with safe target variants.
            ///
            /// # Safety
            ///
            /// - See respective [`SurfaceTargetUnsafe`] variants for safety requirements.
            pub unsafe fn create_surface_unsafe<'window>(
                &self,
                target: SurfaceTargetUnsafe,
            ) -> Result<Surface<'window>, CreateSurfaceError> {
                let surface = unsafe { self.inner.create_surface(target)? };
                Ok(Surface {
                    _handle_source: None,
                    inner: surface,
                    config: Mutex::new(None),
                })
            }
            /// Polls all devices.
            ///
            /// If `force_wait` is true and this is not running on the web, then this
            /// function will block until all in-flight buffers have been mapped and
            /// all submitted commands have finished execution.
            ///
            /// Return `true` if all devices' queues are empty, or `false` if there are
            /// queue submissions still in flight. (Note that, unless access to all
            /// [`Queue`s] associated with this [`Instance`] is coordinated somehow,
            /// this information could be out of date by the time the caller receives
            /// it. `Queue`s can be shared between threads, and other threads could
            /// submit new work at any time.)
            ///
            /// On the web, this is a no-op. `Device`s are automatically polled.
            ///
            /// [`Queue`s]: Queue
            pub fn poll_all(&self, force_wait: bool) -> bool {
                self.inner.poll_all_devices(force_wait)
            }
            /// Generates memory report.
            ///
            /// Returns `None` if the feature is not supported by the backend
            /// which happens only when WebGPU is pre-selected by the instance creation.
            pub fn generate_report(&self) -> Option<wgc::global::GlobalReport> {
                self.inner.as_core_opt().map(|ctx| ctx.generate_report())
            }
        }
        /// Interop with wgpu-hal.
        impl Instance {
            /// Create an new instance of wgpu from a wgpu-hal instance. This is often useful
            /// when you need to do backend specific logic, or interop with an existing backend
            /// instance.
            ///
            /// # Types
            ///
            /// The type of `A::Instance` depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Instance`
            ///- [`hal::api::Metal`] uses [`hal::metal::Instance`]
            ///- `hal::api::Dx12` uses `hal::dx12::Instance`
            ///- `hal::api::Gles` uses `hal::gles::Instance`
            ///
            /// # Safety
            ///
            /// - The `hal_instance` must be a valid and usable instance of the backend specified by `A`.
            /// - wgpu will act like it has complete ownership of this instance, and will destroy it
            ///   when the last reference to the instance, internal or external, is dropped.
            pub unsafe fn from_hal<A: hal::Api>(hal_instance: A::Instance) -> Self {
                Self {
                    inner: unsafe {
                        crate::backend::ContextWgpuCore::from_hal_instance::<
                            A,
                        >(hal_instance)
                            .into()
                    },
                }
            }
            /// Get the [`wgpu_hal`] instance from this `Instance`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Instance`].
            ///
            /// # Types
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Instance`
            ///- [`hal::api::Metal`] uses [`hal::metal::Instance`]
            ///- `hal::api::Dx12` uses `hal::dx12::Instance`
            ///- `hal::api::Gles` uses `hal::gles::Instance`
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The instance is not from the backend specified by `A`.
            /// - The instance is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Instance`]: hal::Api::Instance
            pub unsafe fn as_hal<A: hal::Api>(&self) -> Option<&A::Instance> {
                self.inner
                    .as_core_opt()
                    .and_then(|ctx| unsafe { ctx.instance_as_hal::<A>() })
            }
            /// Converts a wgpu-hal [`hal::ExposedAdapter`] to a wgpu [`Adapter`].
            ///
            /// # Types
            ///
            /// The type of `hal_adapter.adapter` depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Adapter`
            ///- [`hal::api::Metal`] uses [`hal::metal::Adapter`]
            ///- `hal::api::Dx12` uses `hal::dx12::Adapter`
            ///- `hal::api::Gles` uses `hal::gles::Adapter`
            ///
            /// # Safety
            ///
            /// `hal_adapter` must be created from this instance internal handle.
            pub unsafe fn create_adapter_from_hal<A: hal::Api>(
                &self,
                hal_adapter: hal::ExposedAdapter<A>,
            ) -> Adapter {
                let core_instance = self.inner.as_core();
                let adapter = unsafe {
                    core_instance.create_adapter_from_hal(hal_adapter)
                };
                let core = backend::wgpu_core::CoreAdapter {
                    context: core_instance.clone(),
                    id: adapter,
                };
                Adapter { inner: core.into() }
            }
        }
        /// Interop with wgpu-core.
        impl Instance {
            /// Create an new instance of wgpu from a wgpu-core instance.
            ///
            /// # Arguments
            ///
            /// - `core_instance` - wgpu-core instance.
            ///
            /// # Safety
            ///
            /// Refer to the creation of wgpu-core Instance.
            pub unsafe fn from_core(core_instance: wgc::instance::Instance) -> Self {
                Self {
                    inner: unsafe {
                        crate::backend::ContextWgpuCore::from_core_instance(
                                core_instance,
                            )
                            .into()
                    },
                }
            }
        }
    }
    mod pipeline_cache {
        use alloc::vec::Vec;
        use crate::*;
        /// Handle to a pipeline cache, which is used to accelerate
        /// creating [`RenderPipeline`]s and [`ComputePipeline`]s
        /// in subsequent executions
        ///
        /// This reuse is only applicable for the same or similar devices.
        /// See [`util::pipeline_cache_key`] for some details and a suggested workflow.
        ///
        /// Created using [`Device::create_pipeline_cache`].
        ///
        /// # Background
        ///
        /// In most GPU drivers, shader code must be converted into a machine code
        /// which can be executed on the GPU.
        /// Generating this machine code can require a lot of computation.
        /// Pipeline caches allow this computation to be reused between executions
        /// of the program.
        /// This can be very useful for reducing program startup time.
        ///
        /// Note that most desktop GPU drivers will manage their own caches,
        /// meaning that little advantage can be gained from this on those platforms.
        /// However, on some platforms, especially Android, drivers leave this to the
        /// application to implement.
        ///
        /// Unfortunately, drivers do not expose whether they manage their own caches.
        /// Some reasonable policies for applications to use are:
        /// - Manage their own pipeline cache on all platforms
        /// - Only manage pipeline caches on Android
        ///
        /// # Usage
        ///
        /// This is used as [`RenderPipelineDescriptor::cache`] or [`ComputePipelineDescriptor::cache`].
        /// It is valid to use this resource when creating multiple pipelines, in
        /// which case it will likely cache each of those pipelines.
        /// It is also valid to create a new cache for each pipeline.
        ///
        /// This resource is most useful when the data produced from it (using
        /// [`PipelineCache::get_data`]) is persisted.
        /// Care should be taken that pipeline caches are only used for the same device,
        /// as pipeline caches from compatible devices are unlikely to provide any advantage.
        /// `util::pipeline_cache_key` can be used as a file/directory name to help ensure that.
        ///
        /// It is recommended to store pipeline caches atomically. If persisting to disk,
        /// this can usually be achieved by creating a temporary file, then moving/[renaming]
        /// the temporary file over the existing cache
        ///
        /// # Storage Usage
        ///
        /// There is not currently an API available to reduce the size of a cache.
        /// This is due to limitations in the underlying graphics APIs used.
        /// This is especially impactful if your application is being updated, so
        /// previous caches are no longer being used.
        ///
        /// One option to work around this is to regenerate the cache.
        /// That is, creating the pipelines which your program runs using
        /// with the stored cached data, then recreating the *same* pipelines
        /// using a new cache, which your application then store.
        ///
        /// # Implementations
        ///
        /// This resource currently only works on the following backends:
        ///  - Vulkan
        ///
        /// This type is unique to the Rust API of `wgpu`.
        ///
        /// [renaming]: std::fs::rename
        pub struct PipelineCache {
            pub(crate) inner: crate::dispatch::DispatchPipelineCache,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PipelineCache {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PipelineCache",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PipelineCache {
            #[inline]
            fn clone(&self) -> PipelineCache {
                PipelineCache {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<PipelineCache>();
        };
        impl PartialEq for PipelineCache {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for PipelineCache {}
        impl PartialOrd for PipelineCache {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for PipelineCache {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for PipelineCache {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl PipelineCache {
            /// Get the data associated with this pipeline cache.
            /// The data format is an implementation detail of `wgpu`.
            /// The only defined operation on this data setting it as the `data` field
            /// on [`PipelineCacheDescriptor`], then to [`Device::create_pipeline_cache`].
            ///
            /// This function is unique to the Rust API of `wgpu`.
            pub fn get_data(&self) -> Option<Vec<u8>> {
                self.inner.get_data()
            }
        }
    }
    mod pipeline_layout {
        use crate::*;
        /// Handle to a pipeline layout.
        ///
        /// A `PipelineLayout` object describes the available binding groups of a pipeline.
        /// It can be created with [`Device::create_pipeline_layout`].
        ///
        /// Corresponds to [WebGPU `GPUPipelineLayout`](https://gpuweb.github.io/gpuweb/#gpupipelinelayout).
        pub struct PipelineLayout {
            pub(crate) inner: dispatch::DispatchPipelineLayout,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PipelineLayout {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PipelineLayout",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PipelineLayout {
            #[inline]
            fn clone(&self) -> PipelineLayout {
                PipelineLayout {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<PipelineLayout>();
        };
        impl PartialEq for PipelineLayout {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for PipelineLayout {}
        impl PartialOrd for PipelineLayout {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for PipelineLayout {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for PipelineLayout {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl PipelineLayout {}
        /// Describes a [`PipelineLayout`].
        ///
        /// For use with [`Device::create_pipeline_layout`].
        ///
        /// Corresponds to [WebGPU `GPUPipelineLayoutDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpupipelinelayoutdescriptor).
        pub struct PipelineLayoutDescriptor<'a> {
            /// Debug label of the pipeline layout. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// Bind groups that this pipeline uses. The first entry will provide all the bindings for
            /// "set = 0", second entry will provide all the bindings for "set = 1" etc.
            pub bind_group_layouts: &'a [&'a BindGroupLayout],
            /// Set of push constant ranges this pipeline uses. Each shader stage that uses push constants
            /// must define the range in push constant memory that corresponds to its single `var<push_constant>`
            /// buffer.
            ///
            /// If this array is non-empty, the [`Features::PUSH_CONSTANTS`] must be enabled.
            pub push_constant_ranges: &'a [PushConstantRange],
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for PipelineLayoutDescriptor<'a> {
            #[inline]
            fn clone(&self) -> PipelineLayoutDescriptor<'a> {
                PipelineLayoutDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    bind_group_layouts: ::core::clone::Clone::clone(
                        &self.bind_group_layouts,
                    ),
                    push_constant_ranges: ::core::clone::Clone::clone(
                        &self.push_constant_ranges,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for PipelineLayoutDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PipelineLayoutDescriptor",
                    "label",
                    &self.label,
                    "bind_group_layouts",
                    &self.bind_group_layouts,
                    "push_constant_ranges",
                    &&self.push_constant_ranges,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::default::Default for PipelineLayoutDescriptor<'a> {
            #[inline]
            fn default() -> PipelineLayoutDescriptor<'a> {
                PipelineLayoutDescriptor {
                    label: ::core::default::Default::default(),
                    bind_group_layouts: ::core::default::Default::default(),
                    push_constant_ranges: ::core::default::Default::default(),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<PipelineLayoutDescriptor<'_>>();
        };
    }
    mod query_set {
        use crate::*;
        /// Handle to a query set.
        ///
        /// It can be created with [`Device::create_query_set`].
        ///
        /// Corresponds to [WebGPU `GPUQuerySet`](https://gpuweb.github.io/gpuweb/#queryset).
        pub struct QuerySet {
            pub(crate) inner: dispatch::DispatchQuerySet,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for QuerySet {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "QuerySet",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for QuerySet {
            #[inline]
            fn clone(&self) -> QuerySet {
                QuerySet {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<QuerySet>();
        };
        impl PartialEq for QuerySet {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for QuerySet {}
        impl PartialOrd for QuerySet {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for QuerySet {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for QuerySet {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl QuerySet {}
        /// Describes a [`QuerySet`].
        ///
        /// For use with [`Device::create_query_set`].
        ///
        /// Corresponds to [WebGPU `GPUQuerySetDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpuquerysetdescriptor).
        pub type QuerySetDescriptor<'a> = wgt::QuerySetDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<QuerySetDescriptor<'_>>();
        };
    }
    mod queue {
        use alloc::boxed::Box;
        use core::ops::{Deref, DerefMut};
        use crate::{api::DeferredCommandBufferActions, *};
        /// Handle to a command queue on a device.
        ///
        /// A `Queue` executes recorded [`CommandBuffer`] objects and provides convenience methods
        /// for writing to [buffers](Queue::write_buffer) and [textures](Queue::write_texture).
        /// It can be created along with a [`Device`] by calling [`Adapter::request_device`].
        ///
        /// Corresponds to [WebGPU `GPUQueue`](https://gpuweb.github.io/gpuweb/#gpu-queue).
        pub struct Queue {
            pub(crate) inner: dispatch::DispatchQueue,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Queue {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Queue",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Queue {
            #[inline]
            fn clone(&self) -> Queue {
                Queue {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Queue>();
        };
        impl PartialEq for Queue {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Queue {}
        impl PartialOrd for Queue {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Queue {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Queue {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Queue {}
        /// Identifier for a particular call to [`Queue::submit`]. Can be used
        /// as part of an argument to [`Device::poll`] to block for a particular
        /// submission to finish.
        ///
        /// This type is unique to the Rust API of `wgpu`.
        /// There is no analogue in the WebGPU specification.
        pub struct SubmissionIndex {
            pub(crate) index: u64,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SubmissionIndex {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SubmissionIndex",
                    "index",
                    &&self.index,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SubmissionIndex {
            #[inline]
            fn clone(&self) -> SubmissionIndex {
                SubmissionIndex {
                    index: ::core::clone::Clone::clone(&self.index),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<SubmissionIndex>();
        };
        /// Passed to [`Device::poll`] to control how and if it should block.
        pub type PollType = wgt::PollType<SubmissionIndex>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<PollType>();
        };
        /// A write-only view into a staging buffer.
        ///
        /// Reading into this buffer won't yield the contents of the buffer from the
        /// GPU and is likely to be slow. Because of this, although [`AsMut`] is
        /// implemented for this type, [`AsRef`] is not.
        pub struct QueueWriteBufferView {
            queue: Queue,
            buffer: Buffer,
            offset: BufferAddress,
            inner: dispatch::DispatchQueueWriteBuffer,
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<QueueWriteBufferView>();
        };
        impl QueueWriteBufferView {}
        impl Deref for QueueWriteBufferView {
            type Target = [u8];
            fn deref(&self) -> &Self::Target {
                {
                    {
                        let lvl = ::log::Level::Warn;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                { ::log::__private_api::GlobalLogger },
                                format_args!(
                                    "Reading from a QueueWriteBufferView won\'t yield the contents of the buffer and may be slow.",
                                ),
                                lvl,
                                &(
                                    "wgpu::api::queue",
                                    "wgpu::api::queue",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    }
                };
                self.inner.slice()
            }
        }
        impl DerefMut for QueueWriteBufferView {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.inner.slice_mut()
            }
        }
        impl AsMut<[u8]> for QueueWriteBufferView {
            fn as_mut(&mut self) -> &mut [u8] {
                self.inner.slice_mut()
            }
        }
        impl Drop for QueueWriteBufferView {
            fn drop(&mut self) {
                self.queue
                    .inner
                    .write_staging_buffer(&self.buffer.inner, self.offset, &self.inner);
            }
        }
        impl Queue {
            /// Copies the bytes of `data` into `buffer` starting at `offset`.
            ///
            /// The data must be written fully in-bounds, that is, `offset + data.len() <= buffer.len()`.
            ///
            /// # Performance considerations
            ///
            /// * Calls to `write_buffer()` do *not* submit the transfer to the GPU
            ///   immediately. They begin GPU execution only on the next call to
            ///   [`Queue::submit()`], just before the explicitly submitted commands.
            ///   To get a set of scheduled transfers started immediately,
            ///   it's fine to call `submit` with no command buffers at all:
            ///
            ///   ```no_run
            ///   # let queue: wgpu::Queue = todo!();
            ///   # let buffer: wgpu::Buffer = todo!();
            ///   # let data = [0u8];
            ///   queue.write_buffer(&buffer, 0, &data);
            ///   queue.submit([]);
            ///   ```
            ///
            ///   However, `data` will be immediately copied into staging memory, so the
            ///   caller may discard it any time after this call completes.
            ///
            /// * Consider using [`Queue::write_buffer_with()`] instead.
            ///   That method allows you to prepare your data directly within the staging
            ///   memory, rather than first placing it in a separate `[u8]` to be copied.
            ///   That is, `queue.write_buffer(b, offset, data)` is approximately equivalent
            ///   to `queue.write_buffer_with(b, offset, data.len()).copy_from_slice(data)`,
            ///   so use `write_buffer_with()` if you can do something smarter than that
            ///   [`copy_from_slice()`](slice::copy_from_slice). However, for small values
            ///   (e.g. a typical uniform buffer whose contents come from a `struct`),
            ///   there will likely be no difference, since the compiler will be able to
            ///   optimize out unnecessary copies regardless.
            ///
            /// * Currently on native platforms, for both of these methods, the staging
            ///   memory will be a new allocation. This will then be released after the
            ///   next submission finishes. To entirely avoid short-lived allocations, you might
            ///   be able to use [`StagingBelt`](crate::util::StagingBelt),
            ///   or buffers you explicitly create, map, and unmap yourself.
            pub fn write_buffer(
                &self,
                buffer: &Buffer,
                offset: BufferAddress,
                data: &[u8],
            ) {
                self.inner.write_buffer(&buffer.inner, offset, data);
            }
            /// Prepares to write data to a buffer via a mapped staging buffer.
            ///
            /// This operation allocates a temporary buffer and then returns a
            /// [`QueueWriteBufferView`], which
            ///
            /// * dereferences to a `[u8]` of length `size`, and
            /// * when dropped, schedules a copy of its contents into `buffer` at `offset`.
            ///
            /// Therefore, this obtains the same result as [`Queue::write_buffer()`], but may
            /// allow you to skip one allocation and one copy of your data, if you are able to
            /// assemble your data directly into the returned [`QueueWriteBufferView`] instead of
            /// into a separate allocation like a [`Vec`](alloc::vec::Vec) first.
            ///
            /// The data must be written fully in-bounds, that is, `offset + size <= buffer.len()`.
            ///
            /// # Performance considerations
            ///
            /// * For small data not separately heap-allocated, there is no advantage of this
            ///   over [`Queue::write_buffer()`].
            ///
            /// * Reading from the returned view may be slow, and will not yield the current
            ///   contents of `buffer`. You should treat it as “write-only”.
            ///
            /// * Dropping the [`QueueWriteBufferView`] does *not* submit the
            ///   transfer to the GPU immediately. The transfer begins only on the next
            ///   call to [`Queue::submit()`] after the view is dropped, just before the
            ///   explicitly submitted commands. To get a set of scheduled transfers started
            ///   immediately, it's fine to call `queue.submit([])` with no command buffers at all.
            ///
            /// * Currently on native platforms, the staging memory will be a new allocation, which will
            ///   then be released after the next submission finishes. To entirely avoid short-lived
            ///   allocations, you might be able to use [`StagingBelt`](crate::util::StagingBelt),
            ///   or buffers you explicitly create, map, and unmap yourself.
            #[must_use]
            pub fn write_buffer_with(
                &self,
                buffer: &Buffer,
                offset: BufferAddress,
                size: BufferSize,
            ) -> Option<QueueWriteBufferView> {
                self.inner.validate_write_buffer(&buffer.inner, offset, size)?;
                let staging_buffer = self.inner.create_staging_buffer(size)?;
                Some(QueueWriteBufferView {
                    queue: self.clone(),
                    buffer: buffer.clone(),
                    offset,
                    inner: staging_buffer,
                })
            }
            /// Copies the bytes of `data` into into a texture.
            ///
            /// * `data` contains the texels to be written, which must be in
            ///   [the same format as the texture](TextureFormat).
            /// * `data_layout` describes the memory layout of `data`, which does not necessarily
            ///   have to have tightly packed rows.
            /// * `texture` specifies the texture to write into, and the location within the
            ///   texture (coordinate offset, mip level) that will be overwritten.
            /// * `size` is the size, in texels, of the region to be written.
            ///
            /// This method fails if `size` overruns the size of `texture`, or if `data` is too short.
            ///
            /// # Performance considerations
            ///
            /// This operation has the same performance considerations as [`Queue::write_buffer()`];
            /// see its documentation for details.
            ///
            /// However, since there is no “mapped texture” like a mapped buffer,
            /// alternate techniques for writing to textures will generally consist of first copying
            /// the data to a buffer, then using [`CommandEncoder::copy_buffer_to_texture()`], or in
            /// some cases a compute shader, to copy texels from that buffer to the texture.
            pub fn write_texture(
                &self,
                texture: TexelCopyTextureInfo<'_>,
                data: &[u8],
                data_layout: TexelCopyBufferLayout,
                size: Extent3d,
            ) {
                self.inner.write_texture(texture, data, data_layout, size);
            }
            /// Submits a series of finished command buffers for execution.
            pub fn submit<I: IntoIterator<Item = CommandBuffer>>(
                &self,
                command_buffers: I,
            ) -> SubmissionIndex {
                let mut actions = DeferredCommandBufferActions::default();
                let mut command_buffers = command_buffers
                    .into_iter()
                    .map(|comb| {
                        actions.append(&mut comb.actions.lock());
                        comb.buffer
                    });
                let index = self.inner.submit(&mut command_buffers);
                actions.execute(&self.inner);
                SubmissionIndex { index }
            }
            /// Gets the amount of nanoseconds each tick of a timestamp query represents.
            ///
            /// Returns zero if timestamp queries are unsupported.
            ///
            /// Timestamp values are represented in nanosecond values on WebGPU, see `<https://gpuweb.github.io/gpuweb/#timestamp>`
            /// Therefore, this is always 1.0 on the web, but on wgpu-core a manual conversion is required.
            pub fn get_timestamp_period(&self) -> f32 {
                self.inner.get_timestamp_period()
            }
            /// Registers a callback that is invoked when the previous [`Queue::submit`] finishes executing
            /// on the GPU. When this callback runs, all mapped-buffer callbacks registered for the same
            /// submission are guaranteed to have been called.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated into
            /// an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions after the GPU work
            /// completes. There are no restrictions on the code you can run in the callback; however, on native
            /// the polling call will not return until the callback finishes, so keep callbacks short (set flags,
            /// send messages, etc.).
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn on_submitted_work_done(
                &self,
                callback: impl FnOnce() + Send + 'static,
            ) {
                self.inner.on_submitted_work_done(Box::new(callback));
            }
            /// Get the [`wgpu_hal`] device from this `Queue`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Queue`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Queue`
            ///- [`hal::api::Metal`] uses [`hal::metal::Queue`]
            ///- `hal::api::Dx12` uses `hal::dx12::Queue`
            ///- `hal::api::Gles` uses `hal::gles::Queue`
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The queue is not from the backend specified by `A`.
            /// - The queue is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Queue`]: hal::Api::Queue
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::Queue> + WasmNotSendSync> {
                let queue = self.inner.as_core_opt()?;
                unsafe { queue.context.queue_as_hal::<A>(queue) }
            }
            /// Compact a BLAS, it must have had [`Blas::prepare_compaction_async`] called on it and had the
            /// callback provided called.
            ///
            /// The returned BLAS is more restricted than a normal BLAS because it may not be rebuilt or
            /// compacted.
            pub fn compact_blas(&self, blas: &Blas) -> Blas {
                let (handle, dispatch) = self.inner.compact_blas(&blas.inner);
                Blas { handle, inner: dispatch }
            }
        }
    }
    mod render_bundle {
        use crate::*;
        /// Pre-prepared reusable bundle of GPU operations.
        ///
        /// It only supports a handful of render commands, but it makes them reusable. Executing a
        /// [`RenderBundle`] is often more efficient than issuing the underlying commands manually.
        ///
        /// It can be created by use of a [`RenderBundleEncoder`], and executed onto a [`CommandEncoder`]
        /// using [`RenderPass::execute_bundles`].
        ///
        /// Corresponds to [WebGPU `GPURenderBundle`](https://gpuweb.github.io/gpuweb/#render-bundle).
        pub struct RenderBundle {
            pub(crate) inner: dispatch::DispatchRenderBundle,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RenderBundle {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "RenderBundle",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RenderBundle {
            #[inline]
            fn clone(&self) -> RenderBundle {
                RenderBundle {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderBundle>();
        };
        impl PartialEq for RenderBundle {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for RenderBundle {}
        impl PartialOrd for RenderBundle {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for RenderBundle {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for RenderBundle {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl RenderBundle {}
        /// Describes a [`RenderBundle`].
        ///
        /// For use with [`RenderBundleEncoder::finish`].
        ///
        /// Corresponds to [WebGPU `GPURenderBundleDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderbundledescriptor).
        pub type RenderBundleDescriptor<'a> = wgt::RenderBundleDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderBundleDescriptor<'_>>();
        };
    }
    mod render_bundle_encoder {
        use core::{marker::PhantomData, num::NonZeroU32, ops::Range};
        use crate::dispatch::RenderBundleEncoderInterface;
        use crate::*;
        /// Encodes a series of GPU operations into a reusable "render bundle".
        ///
        /// It only supports a handful of render commands, but it makes them reusable.
        /// It can be created with [`Device::create_render_bundle_encoder`].
        /// It can be executed onto a [`CommandEncoder`] using [`RenderPass::execute_bundles`].
        ///
        /// Executing a [`RenderBundle`] is often more efficient than issuing the underlying commands
        /// manually.
        ///
        /// Corresponds to [WebGPU `GPURenderBundleEncoder`](
        /// https://gpuweb.github.io/gpuweb/#gpurenderbundleencoder).
        pub struct RenderBundleEncoder<'a> {
            pub(crate) inner: dispatch::DispatchRenderBundleEncoder,
            /// This type should be !Send !Sync, because it represents an allocation on this thread's
            /// command buffer.
            pub(crate) _p: PhantomData<(*const u8, &'a ())>,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for RenderBundleEncoder<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RenderBundleEncoder",
                    "inner",
                    &self.inner,
                    "_p",
                    &&self._p,
                )
            }
        }
        const _: fn() = || {
            trait AmbiguousIfImpl<A> {
                fn some_item() {}
            }
            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            {
                #[allow(dead_code)]
                struct Invalid;
                impl<T: ?Sized + Send> AmbiguousIfImpl<Invalid> for T {}
            }
            {
                #[allow(dead_code)]
                struct Invalid;
                impl<T: ?Sized + Sync> AmbiguousIfImpl<Invalid> for T {}
            }
            let _ = <RenderBundleEncoder<'_> as AmbiguousIfImpl<_>>::some_item;
        };
        impl PartialEq for RenderBundleEncoder<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for RenderBundleEncoder<'_> {}
        impl PartialOrd for RenderBundleEncoder<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for RenderBundleEncoder<'_> {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for RenderBundleEncoder<'_> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        /// Describes a [`RenderBundleEncoder`].
        ///
        /// For use with [`Device::create_render_bundle_encoder`].
        ///
        /// Corresponds to [WebGPU `GPURenderBundleEncoderDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderbundleencoderdescriptor).
        pub struct RenderBundleEncoderDescriptor<'a> {
            /// Debug label of the render bundle encoder. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// The formats of the color attachments that this render bundle is capable to rendering to. This
            /// must match the formats of the color attachments in the render pass this render bundle is executed in.
            pub color_formats: &'a [Option<TextureFormat>],
            /// Information about the depth attachment that this render bundle is capable to rendering to. This
            /// must match the format of the depth attachments in the render pass this render bundle is executed in.
            pub depth_stencil: Option<RenderBundleDepthStencil>,
            /// Sample count this render bundle is capable of rendering to. This must match the pipelines and
            /// the render passes it is used in.
            pub sample_count: u32,
            /// If this render bundle will rendering to multiple array layers in the attachments at the same time.
            pub multiview: Option<NonZeroU32>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for RenderBundleEncoderDescriptor<'a> {
            #[inline]
            fn clone(&self) -> RenderBundleEncoderDescriptor<'a> {
                RenderBundleEncoderDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    color_formats: ::core::clone::Clone::clone(&self.color_formats),
                    depth_stencil: ::core::clone::Clone::clone(&self.depth_stencil),
                    sample_count: ::core::clone::Clone::clone(&self.sample_count),
                    multiview: ::core::clone::Clone::clone(&self.multiview),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for RenderBundleEncoderDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "RenderBundleEncoderDescriptor",
                    "label",
                    &self.label,
                    "color_formats",
                    &self.color_formats,
                    "depth_stencil",
                    &self.depth_stencil,
                    "sample_count",
                    &self.sample_count,
                    "multiview",
                    &&self.multiview,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::default::Default for RenderBundleEncoderDescriptor<'a> {
            #[inline]
            fn default() -> RenderBundleEncoderDescriptor<'a> {
                RenderBundleEncoderDescriptor {
                    label: ::core::default::Default::default(),
                    color_formats: ::core::default::Default::default(),
                    depth_stencil: ::core::default::Default::default(),
                    sample_count: ::core::default::Default::default(),
                    multiview: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::marker::StructuralPartialEq
        for RenderBundleEncoderDescriptor<'a> {}
        #[automatically_derived]
        impl<'a> ::core::cmp::PartialEq for RenderBundleEncoderDescriptor<'a> {
            #[inline]
            fn eq(&self, other: &RenderBundleEncoderDescriptor<'a>) -> bool {
                self.sample_count == other.sample_count && self.label == other.label
                    && self.color_formats == other.color_formats
                    && self.depth_stencil == other.depth_stencil
                    && self.multiview == other.multiview
            }
        }
        #[automatically_derived]
        impl<'a> ::core::cmp::Eq for RenderBundleEncoderDescriptor<'a> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Label<'a>>;
                let _: ::core::cmp::AssertParamIsEq<&'a [Option<TextureFormat>]>;
                let _: ::core::cmp::AssertParamIsEq<Option<RenderBundleDepthStencil>>;
                let _: ::core::cmp::AssertParamIsEq<u32>;
                let _: ::core::cmp::AssertParamIsEq<Option<NonZeroU32>>;
            }
        }
        #[automatically_derived]
        impl<'a> ::core::hash::Hash for RenderBundleEncoderDescriptor<'a> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.label, state);
                ::core::hash::Hash::hash(&self.color_formats, state);
                ::core::hash::Hash::hash(&self.depth_stencil, state);
                ::core::hash::Hash::hash(&self.sample_count, state);
                ::core::hash::Hash::hash(&self.multiview, state)
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderBundleEncoderDescriptor<'_>>();
        };
        impl<'a> RenderBundleEncoder<'a> {
            /// Finishes recording and returns a [`RenderBundle`] that can be executed in other render passes.
            pub fn finish(self, desc: &RenderBundleDescriptor<'_>) -> RenderBundle {
                let bundle = match self.inner {
                    dispatch::DispatchRenderBundleEncoder::Core(b) => b.finish(desc),
                };
                RenderBundle { inner: bundle }
            }
            /// Sets the active bind group for a given bind group index. The bind group layout
            /// in the active pipeline when any `draw()` function is called must match the layout of this bind group.
            ///
            /// If the bind group have dynamic offsets, provide them in the binding order.
            pub fn set_bind_group<'b, BG>(
                &mut self,
                index: u32,
                bind_group: BG,
                offsets: &[DynamicOffset],
            )
            where
                Option<&'b BindGroup>: From<BG>,
            {
                let bg: Option<&'b BindGroup> = bind_group.into();
                let bg = bg.map(|x| &x.inner);
                self.inner.set_bind_group(index, bg, offsets);
            }
            /// Sets the active render pipeline.
            ///
            /// Subsequent draw calls will exhibit the behavior defined by `pipeline`.
            pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
                self.inner.set_pipeline(&pipeline.inner);
            }
            /// Sets the active index buffer.
            ///
            /// Subsequent calls to [`draw_indexed`](RenderBundleEncoder::draw_indexed) on this [`RenderBundleEncoder`] will
            /// use `buffer` as the source index buffer.
            pub fn set_index_buffer(
                &mut self,
                buffer_slice: BufferSlice<'a>,
                index_format: IndexFormat,
            ) {
                self.inner
                    .set_index_buffer(
                        &buffer_slice.buffer.inner,
                        index_format,
                        buffer_slice.offset,
                        Some(buffer_slice.size),
                    );
            }
            /// Assign a vertex buffer to a slot.
            ///
            /// Subsequent calls to [`draw`] and [`draw_indexed`] on this
            /// [`RenderBundleEncoder`] will use `buffer` as one of the source vertex buffers.
            ///
            /// The `slot` refers to the index of the matching descriptor in
            /// [`VertexState::buffers`].
            ///
            /// [`draw`]: RenderBundleEncoder::draw
            /// [`draw_indexed`]: RenderBundleEncoder::draw_indexed
            pub fn set_vertex_buffer(
                &mut self,
                slot: u32,
                buffer_slice: BufferSlice<'a>,
            ) {
                self.inner
                    .set_vertex_buffer(
                        slot,
                        &buffer_slice.buffer.inner,
                        buffer_slice.offset,
                        Some(buffer_slice.size),
                    );
            }
            /// Draws primitives from the active vertex buffer(s).
            ///
            /// The active vertex buffers can be set with [`RenderBundleEncoder::set_vertex_buffer`].
            /// Does not use an Index Buffer. If you need this see [`RenderBundleEncoder::draw_indexed`]
            ///
            /// Panics if vertices Range is outside of the range of the vertices range of any set vertex buffer.
            ///
            /// vertices: The range of vertices to draw.
            /// instances: Range of Instances to draw. Use 0..1 if instance buffers are not used.
            /// E.g.of how its used internally
            /// ```rust ignore
            /// for instance_id in instance_range {
            ///     for vertex_id in vertex_range {
            ///         let vertex = vertex[vertex_id];
            ///         vertex_shader(vertex, vertex_id, instance_id);
            ///     }
            /// }
            /// ```
            pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
                self.inner.draw(vertices, instances);
            }
            /// Draws indexed primitives using the active index buffer and the active vertex buffer(s).
            ///
            /// The active index buffer can be set with [`RenderBundleEncoder::set_index_buffer`].
            /// The active vertex buffer(s) can be set with [`RenderBundleEncoder::set_vertex_buffer`].
            ///
            /// Panics if indices Range is outside of the range of the indices range of any set index buffer.
            ///
            /// indices: The range of indices to draw.
            /// base_vertex: value added to each index value before indexing into the vertex buffers.
            /// instances: Range of Instances to draw. Use 0..1 if instance buffers are not used.
            /// E.g.of how its used internally
            /// ```rust ignore
            /// for instance_id in instance_range {
            ///     for index_index in index_range {
            ///         let vertex_id = index_buffer[index_index];
            ///         let adjusted_vertex_id = vertex_id + base_vertex;
            ///         let vertex = vertex[adjusted_vertex_id];
            ///         vertex_shader(vertex, adjusted_vertex_id, instance_id);
            ///     }
            /// }
            /// ```
            pub fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            ) {
                self.inner.draw_indexed(indices, base_vertex, instances);
            }
            /// Draws primitives from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
            ///
            /// The active vertex buffers can be set with [`RenderBundleEncoder::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
            pub fn draw_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            ) {
                self.inner.draw_indirect(&indirect_buffer.inner, indirect_offset);
            }
            /// Draws indexed primitives using the active index buffer and the active vertex buffers,
            /// based on the contents of the `indirect_buffer`.
            ///
            /// The active index buffer can be set with [`RenderBundleEncoder::set_index_buffer`], while the active
            /// vertex buffers can be set with [`RenderBundleEncoder::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
            pub fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            ) {
                self.inner
                    .draw_indexed_indirect(&indirect_buffer.inner, indirect_offset);
            }
        }
        /// [`Features::PUSH_CONSTANTS`] must be enabled on the device in order to call these functions.
        impl RenderBundleEncoder<'_> {
            /// Set push constant data.
            ///
            /// Offset is measured in bytes, but must be a multiple of [`PUSH_CONSTANT_ALIGNMENT`].
            ///
            /// Data size must be a multiple of 4 and must have an alignment of 4.
            /// For example, with an offset of 4 and an array of `[u8; 8]`, that will write to the range
            /// of 4..12.
            ///
            /// For each byte in the range of push constant data written, the union of the stages of all push constant
            /// ranges that covers that byte must be exactly `stages`. There's no good way of explaining this simply,
            /// so here are some examples:
            ///
            /// ```text
            /// For the given ranges:
            /// - 0..4 Vertex
            /// - 4..8 Fragment
            /// ```
            ///
            /// You would need to upload this in two set_push_constants calls. First for the `Vertex` range, second for the `Fragment` range.
            ///
            /// ```text
            /// For the given ranges:
            /// - 0..8  Vertex
            /// - 4..12 Fragment
            /// ```
            ///
            /// You would need to upload this in three set_push_constants calls. First for the `Vertex` only range 0..4, second
            /// for the `Vertex | Fragment` range 4..8, third for the `Fragment` range 8..12.
            pub fn set_push_constants(
                &mut self,
                stages: ShaderStages,
                offset: u32,
                data: &[u8],
            ) {
                self.inner.set_push_constants(stages, offset, data);
            }
        }
    }
    mod render_pass {
        use core::ops::Range;
        use crate::{
            api::{
                impl_deferred_command_buffer_actions, SharedDeferredCommandBufferActions,
            },
            *,
        };
        pub use wgt::{LoadOp, Operations, StoreOp};
        /// In-progress recording of a render pass: a list of render commands in a [`CommandEncoder`].
        ///
        /// It can be created with [`CommandEncoder::begin_render_pass()`], whose [`RenderPassDescriptor`]
        /// specifies the attachments (textures) that will be rendered to.
        ///
        /// Most of the methods on `RenderPass` serve one of two purposes, identifiable by their names:
        ///
        /// * `draw_*()`: Drawing (that is, encoding a render command, which, when executed by the GPU, will
        ///   rasterize something and execute shaders).
        /// * `set_*()`: Setting part of the [render state](https://gpuweb.github.io/gpuweb/#renderstate)
        ///   for future drawing commands.
        ///
        /// A render pass may contain any number of drawing commands, and before/between each command the
        /// render state may be updated however you wish; each drawing command will be executed using the
        /// render state that has been set when the `draw_*()` function is called.
        ///
        /// Corresponds to [WebGPU `GPURenderPassEncoder`](
        /// https://gpuweb.github.io/gpuweb/#render-pass-encoder).
        pub struct RenderPass<'encoder> {
            pub(crate) inner: dispatch::DispatchRenderPass,
            pub(crate) actions: SharedDeferredCommandBufferActions,
            /// This lifetime is used to protect the [`CommandEncoder`] from being used
            /// while the pass is alive. This needs to be PhantomDrop to prevent the lifetime
            /// from being shortened.
            pub(crate) _encoder_guard: PhantomDrop<&'encoder ()>,
        }
        #[automatically_derived]
        impl<'encoder> ::core::fmt::Debug for RenderPass<'encoder> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "RenderPass",
                    "inner",
                    &self.inner,
                    "actions",
                    &self.actions,
                    "_encoder_guard",
                    &&self._encoder_guard,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPass<'_>>();
        };
        impl PartialEq for RenderPass<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for RenderPass<'_> {}
        impl PartialOrd for RenderPass<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for RenderPass<'_> {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for RenderPass<'_> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl RenderPass<'_> {
            /// Drops the lifetime relationship to the parent command encoder, making usage of
            /// the encoder while this pass is recorded a run-time error instead.
            ///
            /// Attention: As long as the render pass has not been ended, any mutating operation on the parent
            /// command encoder will cause a run-time error and invalidate it!
            /// By default, the lifetime constraint prevents this, but it can be useful
            /// to handle this at run time, such as when storing the pass and encoder in the same
            /// data structure.
            ///
            /// This operation has no effect on pass recording.
            /// It's a safe operation, since [`CommandEncoder`] is in a locked state as long as the pass is active
            /// regardless of the lifetime constraint or its absence.
            pub fn forget_lifetime(self) -> RenderPass<'static> {
                RenderPass {
                    inner: self.inner,
                    actions: self.actions,
                    _encoder_guard: crate::api::PhantomDrop::default(),
                }
            }
            /// Sets the active bind group for a given bind group index. The bind group layout
            /// in the active pipeline when any `draw_*()` method is called must match the layout of
            /// this bind group.
            ///
            /// If the bind group have dynamic offsets, provide them in binding order.
            /// These offsets have to be aligned to [`Limits::min_uniform_buffer_offset_alignment`]
            /// or [`Limits::min_storage_buffer_offset_alignment`] appropriately.
            ///
            /// Subsequent draw calls’ shader executions will be able to access data in these bind groups.
            pub fn set_bind_group<'a, BG>(
                &mut self,
                index: u32,
                bind_group: BG,
                offsets: &[DynamicOffset],
            )
            where
                Option<&'a BindGroup>: From<BG>,
            {
                let bg: Option<&'a BindGroup> = bind_group.into();
                let bg = bg.map(|bg| &bg.inner);
                self.inner.set_bind_group(index, bg, offsets);
            }
            /// Sets the active render pipeline.
            ///
            /// Subsequent draw calls will exhibit the behavior defined by `pipeline`.
            pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
                self.inner.set_pipeline(&pipeline.inner);
            }
            /// Sets the blend color as used by some of the blending modes.
            ///
            /// Subsequent blending tests will test against this value.
            /// If this method has not been called, the blend constant defaults to [`Color::TRANSPARENT`]
            /// (all components zero).
            pub fn set_blend_constant(&mut self, color: Color) {
                self.inner.set_blend_constant(color);
            }
            /// Sets the active index buffer.
            ///
            /// Subsequent calls to [`draw_indexed`](RenderPass::draw_indexed) on this [`RenderPass`] will
            /// use `buffer` as the source index buffer.
            pub fn set_index_buffer(
                &mut self,
                buffer_slice: BufferSlice<'_>,
                index_format: IndexFormat,
            ) {
                self.inner
                    .set_index_buffer(
                        &buffer_slice.buffer.inner,
                        index_format,
                        buffer_slice.offset,
                        Some(buffer_slice.size),
                    );
            }
            /// Assign a vertex buffer to a slot.
            ///
            /// Subsequent calls to [`draw`] and [`draw_indexed`] on this
            /// [`RenderPass`] will use `buffer` as one of the source vertex buffers.
            /// The format of the data in the buffer is specified by the [`VertexBufferLayout`] in the
            /// pipeline's [`VertexState`].
            ///
            /// The `slot` refers to the index of the matching descriptor in
            /// [`VertexState::buffers`].
            ///
            /// [`draw`]: RenderPass::draw
            /// [`draw_indexed`]: RenderPass::draw_indexed
            pub fn set_vertex_buffer(
                &mut self,
                slot: u32,
                buffer_slice: BufferSlice<'_>,
            ) {
                self.inner
                    .set_vertex_buffer(
                        slot,
                        &buffer_slice.buffer.inner,
                        buffer_slice.offset,
                        Some(buffer_slice.size),
                    );
            }
            /// Sets the scissor rectangle used during the rasterization stage.
            /// After transformation into [viewport coordinates](https://www.w3.org/TR/webgpu/#viewport-coordinates).
            ///
            /// Subsequent draw calls will discard any fragments which fall outside the scissor rectangle.
            /// If this method has not been called, the scissor rectangle defaults to the entire bounds of
            /// the render targets.
            ///
            /// The function of the scissor rectangle resembles [`set_viewport()`](Self::set_viewport),
            /// but it does not affect the coordinate system, only which fragments are discarded.
            pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
                self.inner.set_scissor_rect(x, y, width, height);
            }
            /// Sets the viewport used during the rasterization stage to linearly map
            /// from [normalized device coordinates](https://www.w3.org/TR/webgpu/#ndc) to [viewport coordinates](https://www.w3.org/TR/webgpu/#viewport-coordinates).
            ///
            /// Subsequent draw calls will only draw within this region.
            /// If this method has not been called, the viewport defaults to the entire bounds of the render
            /// targets.
            pub fn set_viewport(
                &mut self,
                x: f32,
                y: f32,
                w: f32,
                h: f32,
                min_depth: f32,
                max_depth: f32,
            ) {
                self.inner.set_viewport(x, y, w, h, min_depth, max_depth);
            }
            /// Sets the stencil reference.
            ///
            /// Subsequent stencil tests will test against this value.
            /// If this method has not been called, the stencil reference value defaults to `0`.
            pub fn set_stencil_reference(&mut self, reference: u32) {
                self.inner.set_stencil_reference(reference);
            }
            /// Inserts debug marker.
            pub fn insert_debug_marker(&mut self, label: &str) {
                self.inner.insert_debug_marker(label);
            }
            /// Start record commands and group it into debug marker group.
            pub fn push_debug_group(&mut self, label: &str) {
                self.inner.push_debug_group(label);
            }
            /// Stops command recording and creates debug group.
            pub fn pop_debug_group(&mut self) {
                self.inner.pop_debug_group();
            }
            /// Draws primitives from the active vertex buffer(s).
            ///
            /// The active vertex buffer(s) can be set with [`RenderPass::set_vertex_buffer`].
            /// This does not use an index buffer. If you need indexed drawing, see [`RenderPass::draw_indexed`]
            ///
            /// Panics if `vertices` range is outside of the range of the vertices range of any set vertex buffer.
            ///
            /// - `vertices`: The range of vertices to draw.
            /// - `instances`: Range of instances to draw. Use `0..1` if instance buffers are not used.
            ///
            /// E.g.of how its used internally
            /// ```rust ignore
            /// for instance_id in instance_range {
            ///     for vertex_id in vertex_range {
            ///         let vertex = vertex[vertex_id];
            ///         vertex_shader(vertex, vertex_id, instance_id);
            ///     }
            /// }
            /// ```
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
                self.inner.draw(vertices, instances);
            }
            /// Draws indexed primitives using the active index buffer and the active vertex buffers.
            ///
            /// The active index buffer can be set with [`RenderPass::set_index_buffer`]
            /// The active vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
            ///
            /// Panics if `indices` range is outside of the range of the indices range of the set index buffer.
            ///
            /// - `indices`: The range of indices to draw.
            /// - `base_vertex`: value added to each index value before indexing into the vertex buffers.
            /// - `instances`: Range of instances to draw. Use `0..1` if instance buffers are not used.
            ///
            /// E.g.of how its used internally
            /// ```rust ignore
            /// for instance_id in instance_range {
            ///     for index_index in index_range {
            ///         let vertex_id = index_buffer[index_index];
            ///         let adjusted_vertex_id = vertex_id + base_vertex;
            ///         let vertex = vertex[adjusted_vertex_id];
            ///         vertex_shader(vertex, adjusted_vertex_id, instance_id);
            ///     }
            /// }
            /// ```
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            ) {
                self.inner.draw_indexed(indices, base_vertex, instances);
            }
            /// Draws using a mesh shader pipeline
            pub fn draw_mesh_tasks(
                &mut self,
                group_count_x: u32,
                group_count_y: u32,
                group_count_z: u32,
            ) {
                self.inner.draw_mesh_tasks(group_count_x, group_count_y, group_count_z);
            }
            /// Draws primitives from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
            ///
            /// This is like calling [`RenderPass::draw`] but the contents of the call are specified in the `indirect_buffer`.
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
            ///
            /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
            pub fn draw_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
            ) {
                self.inner.draw_indirect(&indirect_buffer.inner, indirect_offset);
            }
            /// Draws indexed primitives using the active index buffer and the active vertex buffers,
            /// based on the contents of the `indirect_buffer`.
            ///
            /// This is like calling [`RenderPass::draw_indexed`] but the contents of the call are specified in the `indirect_buffer`.
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
            ///
            /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
            pub fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
            ) {
                self.inner
                    .draw_indexed_indirect(&indirect_buffer.inner, indirect_offset);
            }
            /// Draws using a mesh shader pipeline,
            /// based on the contents of the `indirect_buffer`
            ///
            /// This is like calling [`RenderPass::draw_mesh_tasks`] but the contents of the call are specified in the `indirect_buffer`.
            /// The structure expected in the `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
            ///
            /// Indirect drawing has some caveats depending on the features available. We are not currently able to validate
            /// these and issue an error.
            ///
            /// See details on the individual flags for more information.
            pub fn draw_mesh_tasks_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
            ) {
                self.inner
                    .draw_mesh_tasks_indirect(&indirect_buffer.inner, indirect_offset);
            }
            /// On submission, maps the buffer to host (CPU) memory, making it available
            /// for reading or writing via [`get_mapped_range()`](Buffer::get_mapped_range).
            /// The buffer becomes accessible once the `callback` is invoked with [`Ok`].
            ///
            /// Use this when you need to submit work that uses the buffer before mapping it.
            /// Because that submission must happen before calling `map_async`, this method
            /// schedules the mapping for after submission, avoiding extra calls to
            /// [`Buffer::map_async()`] or [`BufferSlice::map_async()`] and letting you start
            /// the mapping from a more convenient place.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// While a buffer is mapped, it cannot be used by other commands; at any time,
            /// either the GPU or the CPU has exclusive access to the buffer’s contents.
            ///
            /// # Panics
            ///
            /// - If `bounds` is outside the bounds of `buffer`.
            /// - If `bounds` has a length less than 1.
            ///
            /// # Panics During Submit
            ///
            /// - If the buffer is already mapped.
            /// - If the buffer’s [`BufferUsages`] do not allow the requested [`MapMode`].
            /// - If the endpoints of this slice are not aligned to [`MAP_ALIGNMENT`] within the buffer.
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            /// [CEmbos]: CommandEncoder::map_buffer_on_submit
            /// [CBmbos]: CommandBuffer::map_buffer_on_submit
            /// [RPmbos]: RenderPass::map_buffer_on_submit
            /// [CPmbos]: ComputePass::map_buffer_on_submit
            pub fn map_buffer_on_submit<S: core::ops::RangeBounds<BufferAddress>>(
                &self,
                buffer: &api::Buffer,
                mode: MapMode,
                bounds: S,
                callback: impl FnOnce(
                    Result<(), BufferAsyncError>,
                ) + WasmNotSend + 'static,
            ) {
                let (offset, size) = range_to_offset_size(bounds, buffer.size);
                self.actions
                    .lock()
                    .buffer_mappings
                    .push(crate::api::command_buffer_actions::DeferredBufferMapping {
                        buffer: buffer.clone(),
                        mode,
                        offset,
                        size,
                        callback: alloc::boxed::Box::new(callback),
                    });
            }
            /// Registers a callback that is invoked when this command buffer’s work finishes
            /// executing on the GPU. When this callback runs, all mapped-buffer callbacks
            /// registered for the same submission are guaranteed to have been called.
            ///
            /// For the callback to run, either [`queue.submit(..)`][q::s], [`instance.poll_all(..)`][i::p_a],
            /// or [`device.poll(..)`][d::p] must be called elsewhere in the runtime, possibly integrated
            /// into an event loop or run on a separate thread.
            ///
            /// The callback runs on the thread that first calls one of the above functions
            /// after the GPU work completes. There are no restrictions on the code you can run
            /// in the callback; however, on native the polling call will not return until the
            /// callback finishes, so keep callbacks short (set flags, send messages, etc.).
            ///
            /// [q::s]: Queue::submit
            /// [i::p_a]: Instance::poll_all
            /// [d::p]: Device::poll
            pub fn on_submitted_work_done(
                &self,
                callback: impl FnOnce() + Send + 'static,
            ) {
                self.actions
                    .lock()
                    .on_submitted_work_done_callbacks
                    .push(alloc::boxed::Box::new(callback));
            }
            /// Execute a [render bundle][RenderBundle], which is a set of pre-recorded commands
            /// that can be run together.
            ///
            /// Commands in the bundle do not inherit this render pass's current render state, and after the
            /// bundle has executed, the state is **cleared** (reset to defaults, not the previous state).
            pub fn execute_bundles<'a, I: IntoIterator<Item = &'a RenderBundle>>(
                &mut self,
                render_bundles: I,
            ) {
                let mut render_bundles = render_bundles.into_iter().map(|rb| &rb.inner);
                self.inner.execute_bundles(&mut render_bundles);
            }
            /// Dispatches multiple draw calls from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
            /// `count` draw calls are issued.
            ///
            /// The active vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
            /// These draw structures are expected to be tightly packed.
            ///
            /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn multi_draw_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
                count: u32,
            ) {
                self.inner
                    .multi_draw_indirect(&indirect_buffer.inner, indirect_offset, count);
            }
            /// Dispatches multiple draw calls from the active index buffer and the active vertex buffers,
            /// based on the contents of the `indirect_buffer`. `count` draw calls are issued.
            ///
            /// The active index buffer can be set with [`RenderPass::set_index_buffer`], while the active
            /// vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
            /// These draw structures are expected to be tightly packed.
            ///
            /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn multi_draw_indexed_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
                count: u32,
            ) {
                self.inner
                    .multi_draw_indexed_indirect(
                        &indirect_buffer.inner,
                        indirect_offset,
                        count,
                    );
            }
            /// Dispatches multiple draw calls based on the contents of the `indirect_buffer`.
            /// `count` draw calls are issued.
            ///
            /// The structure expected in the `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn multi_draw_mesh_tasks_indirect(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
                count: u32,
            ) {
                self.inner
                    .multi_draw_mesh_tasks_indirect(
                        &indirect_buffer.inner,
                        indirect_offset,
                        count,
                    );
            }
        }
        /// [`Features::MULTI_DRAW_INDIRECT_COUNT`] must be enabled on the device in order to call these functions.
        impl RenderPass<'_> {
            /// Dispatches multiple draw calls from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
            /// The count buffer is read to determine how many draws to issue.
            ///
            /// The indirect buffer must be long enough to account for `max_count` draws, however only `count`
            /// draws will be read. If `count` is greater than `max_count`, `max_count` will be used.
            ///
            /// The active vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
            /// These draw structures are expected to be tightly packed.
            ///
            /// The structure expected in `count_buffer` is the following:
            ///
            /// ```rust
            /// #[repr(C)]
            /// struct DrawIndirectCount {
            ///     count: u32, // Number of draw calls to issue.
            /// }
            /// ```
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn multi_draw_indirect_count(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
                count_buffer: &Buffer,
                count_offset: BufferAddress,
                max_count: u32,
            ) {
                self.inner
                    .multi_draw_indirect_count(
                        &indirect_buffer.inner,
                        indirect_offset,
                        &count_buffer.inner,
                        count_offset,
                        max_count,
                    );
            }
            /// Dispatches multiple draw calls from the active index buffer and the active vertex buffers,
            /// based on the contents of the `indirect_buffer`. The count buffer is read to determine how many draws to issue.
            ///
            /// The indirect buffer must be long enough to account for `max_count` draws, however only `count`
            /// draws will be read. If `count` is greater than `max_count`, `max_count` will be used.
            ///
            /// The active index buffer can be set with [`RenderPass::set_index_buffer`], while the active
            /// vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
            ///
            /// These draw structures are expected to be tightly packed.
            ///
            /// The structure expected in `count_buffer` is the following:
            ///
            /// ```rust
            /// #[repr(C)]
            /// struct DrawIndexedIndirectCount {
            ///     count: u32, // Number of draw calls to issue.
            /// }
            /// ```
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn multi_draw_indexed_indirect_count(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
                count_buffer: &Buffer,
                count_offset: BufferAddress,
                max_count: u32,
            ) {
                self.inner
                    .multi_draw_indexed_indirect_count(
                        &indirect_buffer.inner,
                        indirect_offset,
                        &count_buffer.inner,
                        count_offset,
                        max_count,
                    );
            }
            /// Dispatches multiple draw calls based on the contents of the `indirect_buffer`. The count buffer is read to determine how many draws to issue.
            ///
            /// The indirect buffer must be long enough to account for `max_count` draws, however only `count`
            /// draws will be read. If `count` is greater than `max_count`, `max_count` will be used.
            ///
            /// The structure expected in the `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
            ///
            /// These draw structures are expected to be tightly packed.
            ///
            /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
            /// It is not affected by changes to the state that are performed after it is called.
            pub fn multi_draw_mesh_tasks_indirect_count(
                &mut self,
                indirect_buffer: &Buffer,
                indirect_offset: BufferAddress,
                count_buffer: &Buffer,
                count_offset: BufferAddress,
                max_count: u32,
            ) {
                self.inner
                    .multi_draw_mesh_tasks_indirect_count(
                        &indirect_buffer.inner,
                        indirect_offset,
                        &count_buffer.inner,
                        count_offset,
                        max_count,
                    );
            }
        }
        /// [`Features::PUSH_CONSTANTS`] must be enabled on the device in order to call these functions.
        impl RenderPass<'_> {
            /// Set push constant data for subsequent draw calls.
            ///
            /// Write the bytes in `data` at offset `offset` within push constant
            /// storage, all of which are accessible by all the pipeline stages in
            /// `stages`, and no others.  Both `offset` and the length of `data` must be
            /// multiples of [`PUSH_CONSTANT_ALIGNMENT`], which is always 4.
            ///
            /// For example, if `offset` is `4` and `data` is eight bytes long, this
            /// call will write `data` to bytes `4..12` of push constant storage.
            ///
            /// # Stage matching
            ///
            /// Every byte in the affected range of push constant storage must be
            /// accessible to exactly the same set of pipeline stages, which must match
            /// `stages`. If there are two bytes of storage that are accessible by
            /// different sets of pipeline stages - say, one is accessible by fragment
            /// shaders, and the other is accessible by both fragment shaders and vertex
            /// shaders - then no single `set_push_constants` call may affect both of
            /// them; to write both, you must make multiple calls, each with the
            /// appropriate `stages` value.
            ///
            /// Which pipeline stages may access a given byte is determined by the
            /// pipeline's [`PushConstant`] global variable and (if it is a struct) its
            /// members' offsets.
            ///
            /// For example, suppose you have twelve bytes of push constant storage,
            /// where bytes `0..8` are accessed by the vertex shader, and bytes `4..12`
            /// are accessed by the fragment shader. This means there are three byte
            /// ranges each accessed by a different set of stages:
            ///
            /// - Bytes `0..4` are accessed only by the fragment shader.
            ///
            /// - Bytes `4..8` are accessed by both the fragment shader and the vertex shader.
            ///
            /// - Bytes `8..12` are accessed only by the vertex shader.
            ///
            /// To write all twelve bytes requires three `set_push_constants` calls, one
            /// for each range, each passing the matching `stages` mask.
            ///
            /// [`PushConstant`]: https://docs.rs/naga/latest/naga/enum.StorageClass.html#variant.PushConstant
            pub fn set_push_constants(
                &mut self,
                stages: ShaderStages,
                offset: u32,
                data: &[u8],
            ) {
                self.inner.set_push_constants(stages, offset, data);
            }
        }
        /// [`Features::TIMESTAMP_QUERY_INSIDE_PASSES`] must be enabled on the device in order to call these functions.
        impl RenderPass<'_> {
            /// Issue a timestamp command at this point in the queue. The
            /// timestamp will be written to the specified query set, at the specified index.
            ///
            /// Must be multiplied by [`Queue::get_timestamp_period`] to get
            /// the value in nanoseconds. Absolute values have no meaning,
            /// but timestamps can be subtracted to get the time it takes
            /// for a string of operations to complete.
            pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {
                self.inner.write_timestamp(&query_set.inner, query_index);
            }
        }
        impl RenderPass<'_> {
            /// Start a occlusion query on this render pass. It can be ended with
            /// [`end_occlusion_query`](Self::end_occlusion_query).
            /// Occlusion queries may not be nested.
            pub fn begin_occlusion_query(&mut self, query_index: u32) {
                self.inner.begin_occlusion_query(query_index);
            }
            /// End the occlusion query on this render pass. It can be started with
            /// [`begin_occlusion_query`](Self::begin_occlusion_query).
            /// Occlusion queries may not be nested.
            pub fn end_occlusion_query(&mut self) {
                self.inner.end_occlusion_query();
            }
        }
        /// [`Features::PIPELINE_STATISTICS_QUERY`] must be enabled on the device in order to call these functions.
        impl RenderPass<'_> {
            /// Start a pipeline statistics query on this render pass. It can be ended with
            /// [`end_pipeline_statistics_query`](Self::end_pipeline_statistics_query).
            /// Pipeline statistics queries may not be nested.
            pub fn begin_pipeline_statistics_query(
                &mut self,
                query_set: &QuerySet,
                query_index: u32,
            ) {
                self.inner
                    .begin_pipeline_statistics_query(&query_set.inner, query_index);
            }
            /// End the pipeline statistics query on this render pass. It can be started with
            /// [`begin_pipeline_statistics_query`](Self::begin_pipeline_statistics_query).
            /// Pipeline statistics queries may not be nested.
            pub fn end_pipeline_statistics_query(&mut self) {
                self.inner.end_pipeline_statistics_query();
            }
        }
        /// Describes the timestamp writes of a render pass.
        ///
        /// For use with [`RenderPassDescriptor`].
        /// At least one of [`Self::beginning_of_pass_write_index`] and [`Self::end_of_pass_write_index`]
        /// must be `Some`.
        ///
        /// Corresponds to [WebGPU `GPURenderPassTimestampWrite`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderpasstimestampwrites).
        pub struct RenderPassTimestampWrites<'a> {
            /// The query set to write to.
            pub query_set: &'a QuerySet,
            /// The index of the query set at which a start timestamp of this pass is written, if any.
            pub beginning_of_pass_write_index: Option<u32>,
            /// The index of the query set at which an end timestamp of this pass is written, if any.
            pub end_of_pass_write_index: Option<u32>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for RenderPassTimestampWrites<'a> {
            #[inline]
            fn clone(&self) -> RenderPassTimestampWrites<'a> {
                RenderPassTimestampWrites {
                    query_set: ::core::clone::Clone::clone(&self.query_set),
                    beginning_of_pass_write_index: ::core::clone::Clone::clone(
                        &self.beginning_of_pass_write_index,
                    ),
                    end_of_pass_write_index: ::core::clone::Clone::clone(
                        &self.end_of_pass_write_index,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for RenderPassTimestampWrites<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "RenderPassTimestampWrites",
                    "query_set",
                    &self.query_set,
                    "beginning_of_pass_write_index",
                    &self.beginning_of_pass_write_index,
                    "end_of_pass_write_index",
                    &&self.end_of_pass_write_index,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPassTimestampWrites<'_>>();
        };
        /// Describes a color attachment to a [`RenderPass`].
        ///
        /// For use with [`RenderPassDescriptor`].
        ///
        /// Corresponds to [WebGPU `GPURenderPassColorAttachment`](
        /// https://gpuweb.github.io/gpuweb/#color-attachments).
        pub struct RenderPassColorAttachment<'tex> {
            /// The view to use as an attachment.
            pub view: &'tex TextureView,
            /// The depth slice index of a 3D view. It must not be provided if the view is not 3D.
            pub depth_slice: Option<u32>,
            /// The view that will receive the resolved output if multisampling is used.
            ///
            /// If set, it is always written to, regardless of how [`Self::ops`] is configured.
            pub resolve_target: Option<&'tex TextureView>,
            /// What operations will be performed on this color attachment.
            pub ops: Operations<Color>,
        }
        #[automatically_derived]
        impl<'tex> ::core::clone::Clone for RenderPassColorAttachment<'tex> {
            #[inline]
            fn clone(&self) -> RenderPassColorAttachment<'tex> {
                RenderPassColorAttachment {
                    view: ::core::clone::Clone::clone(&self.view),
                    depth_slice: ::core::clone::Clone::clone(&self.depth_slice),
                    resolve_target: ::core::clone::Clone::clone(&self.resolve_target),
                    ops: ::core::clone::Clone::clone(&self.ops),
                }
            }
        }
        #[automatically_derived]
        impl<'tex> ::core::fmt::Debug for RenderPassColorAttachment<'tex> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "RenderPassColorAttachment",
                    "view",
                    &self.view,
                    "depth_slice",
                    &self.depth_slice,
                    "resolve_target",
                    &self.resolve_target,
                    "ops",
                    &&self.ops,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPassColorAttachment<'_>>();
        };
        /// Describes a depth/stencil attachment to a [`RenderPass`].
        ///
        /// For use with [`RenderPassDescriptor`].
        ///
        /// Corresponds to [WebGPU `GPURenderPassDepthStencilAttachment`](
        /// https://gpuweb.github.io/gpuweb/#depth-stencil-attachments).
        pub struct RenderPassDepthStencilAttachment<'tex> {
            /// The view to use as an attachment.
            pub view: &'tex TextureView,
            /// What operations will be performed on the depth part of the attachment.
            pub depth_ops: Option<Operations<f32>>,
            /// What operations will be performed on the stencil part of the attachment.
            pub stencil_ops: Option<Operations<u32>>,
        }
        #[automatically_derived]
        impl<'tex> ::core::clone::Clone for RenderPassDepthStencilAttachment<'tex> {
            #[inline]
            fn clone(&self) -> RenderPassDepthStencilAttachment<'tex> {
                RenderPassDepthStencilAttachment {
                    view: ::core::clone::Clone::clone(&self.view),
                    depth_ops: ::core::clone::Clone::clone(&self.depth_ops),
                    stencil_ops: ::core::clone::Clone::clone(&self.stencil_ops),
                }
            }
        }
        #[automatically_derived]
        impl<'tex> ::core::fmt::Debug for RenderPassDepthStencilAttachment<'tex> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "RenderPassDepthStencilAttachment",
                    "view",
                    &self.view,
                    "depth_ops",
                    &self.depth_ops,
                    "stencil_ops",
                    &&self.stencil_ops,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPassDepthStencilAttachment<'_>>();
        };
        /// Describes the attachments of a render pass.
        ///
        /// For use with [`CommandEncoder::begin_render_pass`].
        ///
        /// Corresponds to [WebGPU `GPURenderPassDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderpassdescriptor).
        pub struct RenderPassDescriptor<'a> {
            /// Debug label of the render pass. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// The color attachments of the render pass.
            pub color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
            /// The depth and stencil attachment of the render pass, if any.
            pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
            /// Defines which timestamp values will be written for this pass, and where to write them to.
            ///
            /// Requires [`Features::TIMESTAMP_QUERY`] to be enabled.
            pub timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
            /// Defines where the occlusion query results will be stored for this pass.
            pub occlusion_query_set: Option<&'a QuerySet>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for RenderPassDescriptor<'a> {
            #[inline]
            fn clone(&self) -> RenderPassDescriptor<'a> {
                RenderPassDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    color_attachments: ::core::clone::Clone::clone(
                        &self.color_attachments,
                    ),
                    depth_stencil_attachment: ::core::clone::Clone::clone(
                        &self.depth_stencil_attachment,
                    ),
                    timestamp_writes: ::core::clone::Clone::clone(
                        &self.timestamp_writes,
                    ),
                    occlusion_query_set: ::core::clone::Clone::clone(
                        &self.occlusion_query_set,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for RenderPassDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "RenderPassDescriptor",
                    "label",
                    &self.label,
                    "color_attachments",
                    &self.color_attachments,
                    "depth_stencil_attachment",
                    &self.depth_stencil_attachment,
                    "timestamp_writes",
                    &self.timestamp_writes,
                    "occlusion_query_set",
                    &&self.occlusion_query_set,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::default::Default for RenderPassDescriptor<'a> {
            #[inline]
            fn default() -> RenderPassDescriptor<'a> {
                RenderPassDescriptor {
                    label: ::core::default::Default::default(),
                    color_attachments: ::core::default::Default::default(),
                    depth_stencil_attachment: ::core::default::Default::default(),
                    timestamp_writes: ::core::default::Default::default(),
                    occlusion_query_set: ::core::default::Default::default(),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPassDescriptor<'_>>();
        };
    }
    mod render_pipeline {
        use core::num::NonZeroU32;
        use crate::*;
        /// Handle to a rendering (graphics) pipeline.
        ///
        /// A `RenderPipeline` object represents a graphics pipeline and its stages, bindings, vertex
        /// buffers and targets. It can be created with [`Device::create_render_pipeline`].
        ///
        /// Corresponds to [WebGPU `GPURenderPipeline`](https://gpuweb.github.io/gpuweb/#render-pipeline).
        pub struct RenderPipeline {
            pub(crate) inner: dispatch::DispatchRenderPipeline,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RenderPipeline {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "RenderPipeline",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RenderPipeline {
            #[inline]
            fn clone(&self) -> RenderPipeline {
                RenderPipeline {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPipeline>();
        };
        impl PartialEq for RenderPipeline {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for RenderPipeline {}
        impl PartialOrd for RenderPipeline {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for RenderPipeline {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for RenderPipeline {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl RenderPipeline {
            /// Get an object representing the bind group layout at a given index.
            ///
            /// If this pipeline was created with a [default layout][RenderPipelineDescriptor::layout], then
            /// bind groups created with the returned `BindGroupLayout` can only be used with this pipeline.
            ///
            /// This method will raise a validation error if there is no bind group layout at `index`.
            pub fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout {
                let layout = self.inner.get_bind_group_layout(index);
                BindGroupLayout { inner: layout }
            }
        }
        /// Specifies an interpretation of the bytes of a vertex buffer as vertex attributes.
        ///
        /// Use this in a [`RenderPipelineDescriptor`] to describe the format of the vertex buffers that
        /// are passed to [`RenderPass::set_vertex_buffer()`].
        ///
        /// Corresponds to [WebGPU `GPUVertexBufferLayout`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpuvertexbufferlayout).
        ///
        /// # Example
        ///
        /// The following example defines a `struct` with three fields,
        /// and a [`VertexBufferLayout`] that contains [`VertexAttribute`]s for each field,
        /// using the [`vertex_attr_array!`] macro to compute attribute offsets:
        ///
        /// ```
        /// #[repr(C, packed)]
        /// struct Vertex {
        ///     foo: [f32; 2],
        ///     bar: f32,
        ///     baz: [u16; 4],
        /// }
        ///
        /// impl Vertex {
        ///     /// Layout to use with a buffer whose contents are a `[Vertex]`.
        ///     pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        ///         array_stride: size_of::<Self>() as wgpu::BufferAddress,
        ///         step_mode: wgpu::VertexStepMode::Vertex,
        ///         attributes: &wgpu::vertex_attr_array![
        ///             0 => Float32x2,
        ///             1 => Float32,
        ///             2 => Uint16x4,
        ///         ],
        ///     };
        /// }
        ///
        /// # assert_eq!(Vertex::LAYOUT.attributes[2].offset, Vertex::LAYOUT.array_stride - 2 * 4);
        pub struct VertexBufferLayout<'a> {
            /// The stride, in bytes, between elements of this buffer (between vertices).
            ///
            /// This must be a multiple of [`VERTEX_ALIGNMENT`].
            pub array_stride: BufferAddress,
            /// How often this vertex buffer is "stepped" forward.
            pub step_mode: VertexStepMode,
            /// The list of attributes which comprise a single vertex.
            pub attributes: &'a [VertexAttribute],
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for VertexBufferLayout<'a> {
            #[inline]
            fn clone(&self) -> VertexBufferLayout<'a> {
                VertexBufferLayout {
                    array_stride: ::core::clone::Clone::clone(&self.array_stride),
                    step_mode: ::core::clone::Clone::clone(&self.step_mode),
                    attributes: ::core::clone::Clone::clone(&self.attributes),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for VertexBufferLayout<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "VertexBufferLayout",
                    "array_stride",
                    &self.array_stride,
                    "step_mode",
                    &self.step_mode,
                    "attributes",
                    &&self.attributes,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::hash::Hash for VertexBufferLayout<'a> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.array_stride, state);
                ::core::hash::Hash::hash(&self.step_mode, state);
                ::core::hash::Hash::hash(&self.attributes, state)
            }
        }
        #[automatically_derived]
        impl<'a> ::core::cmp::Eq for VertexBufferLayout<'a> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<BufferAddress>;
                let _: ::core::cmp::AssertParamIsEq<VertexStepMode>;
                let _: ::core::cmp::AssertParamIsEq<&'a [VertexAttribute]>;
            }
        }
        #[automatically_derived]
        impl<'a> ::core::marker::StructuralPartialEq for VertexBufferLayout<'a> {}
        #[automatically_derived]
        impl<'a> ::core::cmp::PartialEq for VertexBufferLayout<'a> {
            #[inline]
            fn eq(&self, other: &VertexBufferLayout<'a>) -> bool {
                self.array_stride == other.array_stride
                    && self.step_mode == other.step_mode
                    && self.attributes == other.attributes
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<VertexBufferLayout<'_>>();
        };
        /// Describes the vertex processing in a render pipeline.
        ///
        /// For use in [`RenderPipelineDescriptor`].
        ///
        /// Corresponds to [WebGPU `GPUVertexState`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpuvertexstate).
        pub struct VertexState<'a> {
            /// The compiled shader module for this stage.
            pub module: &'a ShaderModule,
            /// The name of the entry point in the compiled shader to use.
            ///
            /// If [`Some`], there must be a vertex-stage shader entry point with this name in `module`.
            /// Otherwise, expect exactly one vertex-stage entry point in `module`, which will be
            /// selected.
            pub entry_point: Option<&'a str>,
            /// Advanced options for when this pipeline is compiled
            ///
            /// This implements `Default`, and for most users can be set to `Default::default()`
            pub compilation_options: PipelineCompilationOptions<'a>,
            /// The format of any vertex buffers used with this pipeline via
            /// [`RenderPass::set_vertex_buffer()`].
            ///
            /// The attribute locations and types specified in this layout must match the
            /// locations and types of the inputs to the `entry_point` function.
            pub buffers: &'a [VertexBufferLayout<'a>],
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for VertexState<'a> {
            #[inline]
            fn clone(&self) -> VertexState<'a> {
                VertexState {
                    module: ::core::clone::Clone::clone(&self.module),
                    entry_point: ::core::clone::Clone::clone(&self.entry_point),
                    compilation_options: ::core::clone::Clone::clone(
                        &self.compilation_options,
                    ),
                    buffers: ::core::clone::Clone::clone(&self.buffers),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for VertexState<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "VertexState",
                    "module",
                    &self.module,
                    "entry_point",
                    &self.entry_point,
                    "compilation_options",
                    &self.compilation_options,
                    "buffers",
                    &&self.buffers,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<VertexState<'_>>();
        };
        /// Describes the fragment processing in a render pipeline.
        ///
        /// For use in [`RenderPipelineDescriptor`].
        ///
        /// Corresponds to [WebGPU `GPUFragmentState`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpufragmentstate).
        pub struct FragmentState<'a> {
            /// The compiled shader module for this stage.
            pub module: &'a ShaderModule,
            /// The name of the entry point in the compiled shader to use.
            ///
            /// If [`Some`], there must be a `@fragment` shader entry point with this name in `module`.
            /// Otherwise, expect exactly one fragment-stage entry point in `module`, which will be
            /// selected.
            pub entry_point: Option<&'a str>,
            /// Advanced options for when this pipeline is compiled
            ///
            /// This implements `Default`, and for most users can be set to `Default::default()`
            pub compilation_options: PipelineCompilationOptions<'a>,
            /// The color state of the render targets.
            pub targets: &'a [Option<ColorTargetState>],
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for FragmentState<'a> {
            #[inline]
            fn clone(&self) -> FragmentState<'a> {
                FragmentState {
                    module: ::core::clone::Clone::clone(&self.module),
                    entry_point: ::core::clone::Clone::clone(&self.entry_point),
                    compilation_options: ::core::clone::Clone::clone(
                        &self.compilation_options,
                    ),
                    targets: ::core::clone::Clone::clone(&self.targets),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for FragmentState<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "FragmentState",
                    "module",
                    &self.module,
                    "entry_point",
                    &self.entry_point,
                    "compilation_options",
                    &self.compilation_options,
                    "targets",
                    &&self.targets,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<FragmentState<'_>>();
        };
        /// Describes the task shader stage in a mesh shader pipeline.
        ///
        /// For use in [`MeshPipelineDescriptor`]
        pub struct TaskState<'a> {
            /// The compiled shader module for this stage.
            pub module: &'a ShaderModule,
            /// The name of the entry point in the compiled shader to use.
            ///
            /// If [`Some`], there must be a vertex-stage shader entry point with this name in `module`.
            /// Otherwise, expect exactly one vertex-stage entry point in `module`, which will be
            /// selected.
            pub entry_point: Option<&'a str>,
            /// Advanced options for when this pipeline is compiled
            ///
            /// This implements `Default`, and for most users can be set to `Default::default()`
            pub compilation_options: PipelineCompilationOptions<'a>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for TaskState<'a> {
            #[inline]
            fn clone(&self) -> TaskState<'a> {
                TaskState {
                    module: ::core::clone::Clone::clone(&self.module),
                    entry_point: ::core::clone::Clone::clone(&self.entry_point),
                    compilation_options: ::core::clone::Clone::clone(
                        &self.compilation_options,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for TaskState<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TaskState",
                    "module",
                    &self.module,
                    "entry_point",
                    &self.entry_point,
                    "compilation_options",
                    &&self.compilation_options,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<TaskState<'_>>();
        };
        /// Describes the mesh shader stage in a mesh shader pipeline.
        ///
        /// For use in [`MeshPipelineDescriptor`]
        pub struct MeshState<'a> {
            /// The compiled shader module for this stage.
            pub module: &'a ShaderModule,
            /// The name of the entry point in the compiled shader to use.
            ///
            /// If [`Some`], there must be a vertex-stage shader entry point with this name in `module`.
            /// Otherwise, expect exactly one vertex-stage entry point in `module`, which will be
            /// selected.
            pub entry_point: Option<&'a str>,
            /// Advanced options for when this pipeline is compiled
            ///
            /// This implements `Default`, and for most users can be set to `Default::default()`
            pub compilation_options: PipelineCompilationOptions<'a>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for MeshState<'a> {
            #[inline]
            fn clone(&self) -> MeshState<'a> {
                MeshState {
                    module: ::core::clone::Clone::clone(&self.module),
                    entry_point: ::core::clone::Clone::clone(&self.entry_point),
                    compilation_options: ::core::clone::Clone::clone(
                        &self.compilation_options,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for MeshState<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "MeshState",
                    "module",
                    &self.module,
                    "entry_point",
                    &self.entry_point,
                    "compilation_options",
                    &&self.compilation_options,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<MeshState<'_>>();
        };
        /// Describes a render (graphics) pipeline.
        ///
        /// For use with [`Device::create_render_pipeline`].
        ///
        /// Corresponds to [WebGPU `GPURenderPipelineDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderpipelinedescriptor).
        pub struct RenderPipelineDescriptor<'a> {
            /// Debug label of the pipeline. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// The layout of bind groups for this pipeline.
            ///
            /// If this is set, then [`Device::create_render_pipeline`] will raise a validation error if
            /// the layout doesn't match what the shader module(s) expect.
            ///
            /// Using the same [`PipelineLayout`] for many [`RenderPipeline`] or [`ComputePipeline`]
            /// pipelines guarantees that you don't have to rebind any resources when switching between
            /// those pipelines.
            ///
            /// ## Default pipeline layout
            ///
            /// If `layout` is `None`, then the pipeline has a [default layout] created and used instead.
            /// The default layout is deduced from the shader modules.
            ///
            /// You can use [`RenderPipeline::get_bind_group_layout`] to create bind groups for use with the
            /// default layout. However, these bind groups cannot be used with any other pipelines. This is
            /// convenient for simple pipelines, but using an explicit layout is recommended in most cases.
            ///
            /// [default layout]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
            pub layout: Option<&'a PipelineLayout>,
            /// The compiled vertex stage, its entry point, and the input buffers layout.
            pub vertex: VertexState<'a>,
            /// The properties of the pipeline at the primitive assembly and rasterization level.
            pub primitive: PrimitiveState,
            /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
            pub depth_stencil: Option<DepthStencilState>,
            /// The multi-sampling properties of the pipeline.
            pub multisample: MultisampleState,
            /// The compiled fragment stage, its entry point, and the color targets.
            pub fragment: Option<FragmentState<'a>>,
            /// If the pipeline will be used with a multiview render pass, this indicates how many array
            /// layers the attachments will have.
            pub multiview: Option<NonZeroU32>,
            /// The pipeline cache to use when creating this pipeline.
            pub cache: Option<&'a PipelineCache>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for RenderPipelineDescriptor<'a> {
            #[inline]
            fn clone(&self) -> RenderPipelineDescriptor<'a> {
                RenderPipelineDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    layout: ::core::clone::Clone::clone(&self.layout),
                    vertex: ::core::clone::Clone::clone(&self.vertex),
                    primitive: ::core::clone::Clone::clone(&self.primitive),
                    depth_stencil: ::core::clone::Clone::clone(&self.depth_stencil),
                    multisample: ::core::clone::Clone::clone(&self.multisample),
                    fragment: ::core::clone::Clone::clone(&self.fragment),
                    multiview: ::core::clone::Clone::clone(&self.multiview),
                    cache: ::core::clone::Clone::clone(&self.cache),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for RenderPipelineDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "label",
                    "layout",
                    "vertex",
                    "primitive",
                    "depth_stencil",
                    "multisample",
                    "fragment",
                    "multiview",
                    "cache",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.label,
                    &self.layout,
                    &self.vertex,
                    &self.primitive,
                    &self.depth_stencil,
                    &self.multisample,
                    &self.fragment,
                    &self.multiview,
                    &&self.cache,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "RenderPipelineDescriptor",
                    names,
                    values,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<RenderPipelineDescriptor<'_>>();
        };
        /// Describes a mesh shader (graphics) pipeline.
        ///
        /// For use with [`Device::create_mesh_pipeline`].
        pub struct MeshPipelineDescriptor<'a> {
            /// Debug label of the pipeline. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// The layout of bind groups for this pipeline.
            ///
            /// If this is set, then [`Device::create_render_pipeline`] will raise a validation error if
            /// the layout doesn't match what the shader module(s) expect.
            ///
            /// Using the same [`PipelineLayout`] for many [`RenderPipeline`] or [`ComputePipeline`]
            /// pipelines guarantees that you don't have to rebind any resources when switching between
            /// those pipelines.
            ///
            /// ## Default pipeline layout
            ///
            /// If `layout` is `None`, then the pipeline has a [default layout] created and used instead.
            /// The default layout is deduced from the shader modules.
            ///
            /// You can use [`RenderPipeline::get_bind_group_layout`] to create bind groups for use with the
            /// default layout. However, these bind groups cannot be used with any other pipelines. This is
            /// convenient for simple pipelines, but using an explicit layout is recommended in most cases.
            ///
            /// [default layout]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
            pub layout: Option<&'a PipelineLayout>,
            /// The compiled task stage, its entry point, and the color targets.
            pub task: Option<TaskState<'a>>,
            /// The compiled mesh stage and its entry point
            pub mesh: MeshState<'a>,
            /// The properties of the pipeline at the primitive assembly and rasterization level.
            pub primitive: PrimitiveState,
            /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
            pub depth_stencil: Option<DepthStencilState>,
            /// The multi-sampling properties of the pipeline.
            pub multisample: MultisampleState,
            /// The compiled fragment stage, its entry point, and the color targets.
            pub fragment: Option<FragmentState<'a>>,
            /// If the pipeline will be used with a multiview render pass, this indicates how many array
            /// layers the attachments will have.
            pub multiview: Option<NonZeroU32>,
            /// The pipeline cache to use when creating this pipeline.
            pub cache: Option<&'a PipelineCache>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for MeshPipelineDescriptor<'a> {
            #[inline]
            fn clone(&self) -> MeshPipelineDescriptor<'a> {
                MeshPipelineDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    layout: ::core::clone::Clone::clone(&self.layout),
                    task: ::core::clone::Clone::clone(&self.task),
                    mesh: ::core::clone::Clone::clone(&self.mesh),
                    primitive: ::core::clone::Clone::clone(&self.primitive),
                    depth_stencil: ::core::clone::Clone::clone(&self.depth_stencil),
                    multisample: ::core::clone::Clone::clone(&self.multisample),
                    fragment: ::core::clone::Clone::clone(&self.fragment),
                    multiview: ::core::clone::Clone::clone(&self.multiview),
                    cache: ::core::clone::Clone::clone(&self.cache),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for MeshPipelineDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "label",
                    "layout",
                    "task",
                    "mesh",
                    "primitive",
                    "depth_stencil",
                    "multisample",
                    "fragment",
                    "multiview",
                    "cache",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.label,
                    &self.layout,
                    &self.task,
                    &self.mesh,
                    &self.primitive,
                    &self.depth_stencil,
                    &self.multisample,
                    &self.fragment,
                    &self.multiview,
                    &&self.cache,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "MeshPipelineDescriptor",
                    names,
                    values,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<MeshPipelineDescriptor<'_>>();
        };
    }
    mod sampler {
        use crate::*;
        /// Handle to a sampler.
        ///
        /// A `Sampler` object defines how a pipeline will sample from a [`TextureView`]. Samplers define
        /// image filters (including anisotropy) and address (wrapping) modes, among other things. See
        /// the documentation for [`SamplerDescriptor`] for more information.
        ///
        /// It can be created with [`Device::create_sampler`].
        ///
        /// Corresponds to [WebGPU `GPUSampler`](https://gpuweb.github.io/gpuweb/#sampler-interface).
        pub struct Sampler {
            pub(crate) inner: dispatch::DispatchSampler,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Sampler {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Sampler",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Sampler {
            #[inline]
            fn clone(&self) -> Sampler {
                Sampler {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Sampler>();
        };
        impl PartialEq for Sampler {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Sampler {}
        impl PartialOrd for Sampler {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Sampler {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Sampler {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Sampler {}
        /// Describes a [`Sampler`].
        ///
        /// For use with [`Device::create_sampler`].
        ///
        /// Corresponds to [WebGPU `GPUSamplerDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpusamplerdescriptor).
        pub type SamplerDescriptor<'a> = wgt::SamplerDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<SamplerDescriptor<'_>>();
        };
    }
    mod shader_module {
        use alloc::{string::String, vec::Vec};
        use core::{future::Future, marker::PhantomData};
        use crate::*;
        /// Handle to a compiled shader module.
        ///
        /// A `ShaderModule` represents a compiled shader module on the GPU. It can be created by passing
        /// source code to [`Device::create_shader_module`]. MSL shader or SPIR-V binary can also be passed
        /// directly using [`Device::create_shader_module_passthrough`]. Shader modules are used to define
        /// programmable stages of a pipeline.
        ///
        /// Corresponds to [WebGPU `GPUShaderModule`](https://gpuweb.github.io/gpuweb/#shader-module).
        pub struct ShaderModule {
            pub(crate) inner: dispatch::DispatchShaderModule,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ShaderModule {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ShaderModule",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ShaderModule {
            #[inline]
            fn clone(&self) -> ShaderModule {
                ShaderModule {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ShaderModule>();
        };
        impl PartialEq for ShaderModule {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for ShaderModule {}
        impl PartialOrd for ShaderModule {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for ShaderModule {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for ShaderModule {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl ShaderModule {
            /// Get the compilation info for the shader module.
            pub fn get_compilation_info(
                &self,
            ) -> impl Future<Output = CompilationInfo> + WasmNotSend {
                self.inner.get_compilation_info()
            }
        }
        /// Compilation information for a shader module.
        ///
        /// Corresponds to [WebGPU `GPUCompilationInfo`](https://gpuweb.github.io/gpuweb/#gpucompilationinfo).
        /// The source locations use bytes, and index a UTF-8 encoded string.
        pub struct CompilationInfo {
            /// The messages from the shader compilation process.
            pub messages: Vec<CompilationMessage>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CompilationInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CompilationInfo",
                    "messages",
                    &&self.messages,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CompilationInfo {
            #[inline]
            fn clone(&self) -> CompilationInfo {
                CompilationInfo {
                    messages: ::core::clone::Clone::clone(&self.messages),
                }
            }
        }
        /// A single message from the shader compilation process.
        ///
        /// Roughly corresponds to [`GPUCompilationMessage`](https://www.w3.org/TR/webgpu/#gpucompilationmessage),
        /// except that the location uses UTF-8 for all positions.
        pub struct CompilationMessage {
            /// The text of the message.
            pub message: String,
            /// The type of the message.
            pub message_type: CompilationMessageType,
            /// Where in the source code the message points at.
            pub location: Option<SourceLocation>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CompilationMessage {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CompilationMessage",
                    "message",
                    &self.message,
                    "message_type",
                    &self.message_type,
                    "location",
                    &&self.location,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CompilationMessage {
            #[inline]
            fn clone(&self) -> CompilationMessage {
                CompilationMessage {
                    message: ::core::clone::Clone::clone(&self.message),
                    message_type: ::core::clone::Clone::clone(&self.message_type),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        /// The type of a compilation message.
        pub enum CompilationMessageType {
            /// An error message.
            Error,
            /// A warning message.
            Warning,
            /// An informational message.
            Info,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CompilationMessageType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        CompilationMessageType::Error => "Error",
                        CompilationMessageType::Warning => "Warning",
                        CompilationMessageType::Info => "Info",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CompilationMessageType {
            #[inline]
            fn clone(&self) -> CompilationMessageType {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CompilationMessageType {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CompilationMessageType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CompilationMessageType {
            #[inline]
            fn eq(&self, other: &CompilationMessageType) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for CompilationMessageType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        /// A human-readable representation for a span, tailored for text source.
        ///
        /// Roughly corresponds to the positional members of [`GPUCompilationMessage`][gcm] from
        /// the WebGPU specification, except
        /// - `offset` and `length` are in bytes (UTF-8 code units), instead of UTF-16 code units.
        /// - `line_position` is in bytes (UTF-8 code units), and is usually not directly intended for humans.
        ///
        /// [gcm]: https://www.w3.org/TR/webgpu/#gpucompilationmessage
        pub struct SourceLocation {
            /// 1-based line number.
            pub line_number: u32,
            /// 1-based column in code units (in bytes) of the start of the span.
            /// Remember to convert accordingly when displaying to the user.
            pub line_position: u32,
            /// 0-based Offset in code units (in bytes) of the start of the span.
            pub offset: u32,
            /// Length in code units (in bytes) of the span.
            pub length: u32,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for SourceLocation {}
        #[automatically_derived]
        impl ::core::clone::Clone for SourceLocation {
            #[inline]
            fn clone(&self) -> SourceLocation {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SourceLocation {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "SourceLocation",
                    "line_number",
                    &self.line_number,
                    "line_position",
                    &self.line_position,
                    "offset",
                    &self.offset,
                    "length",
                    &&self.length,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SourceLocation {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SourceLocation {
            #[inline]
            fn eq(&self, other: &SourceLocation) -> bool {
                self.line_number == other.line_number
                    && self.line_position == other.line_position
                    && self.offset == other.offset && self.length == other.length
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SourceLocation {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        impl From<crate::naga::error::ShaderError<crate::naga::front::wgsl::ParseError>>
        for CompilationInfo {
            fn from(
                value: crate::naga::error::ShaderError<
                    crate::naga::front::wgsl::ParseError,
                >,
            ) -> Self {
                use alloc::{string::ToString, vec};
                CompilationInfo {
                    messages: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            CompilationMessage {
                                message: value.to_string(),
                                message_type: CompilationMessageType::Error,
                                location: value
                                    .inner
                                    .location(&value.source)
                                    .map(Into::into),
                            },
                        ]),
                    ),
                }
            }
        }
        impl From<
            crate::naga::error::ShaderError<
                crate::naga::WithSpan<crate::naga::valid::ValidationError>,
            >,
        > for CompilationInfo {
            fn from(
                value: crate::naga::error::ShaderError<
                    crate::naga::WithSpan<crate::naga::valid::ValidationError>,
                >,
            ) -> Self {
                use alloc::{string::ToString, vec};
                CompilationInfo {
                    messages: <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            CompilationMessage {
                                message: value.to_string(),
                                message_type: CompilationMessageType::Error,
                                location: value
                                    .inner
                                    .location(&value.source)
                                    .map(Into::into),
                            },
                        ]),
                    ),
                }
            }
        }
        impl From<crate::naga::SourceLocation> for SourceLocation {
            fn from(value: crate::naga::SourceLocation) -> Self {
                SourceLocation {
                    length: value.length,
                    offset: value.offset,
                    line_number: value.line_number,
                    line_position: value.line_position,
                }
            }
        }
        /// Source of a shader module.
        ///
        /// The source will be parsed and validated.
        ///
        /// Any necessary shader translation (e.g. from WGSL to SPIR-V or vice versa)
        /// will be done internally by wgpu.
        ///
        /// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
        /// only WGSL source code strings are accepted.
        #[non_exhaustive]
        pub enum ShaderSource<'a> {
            /// WGSL module as a string slice.
            Wgsl(alloc::borrow::Cow<'a, str>),
            /// Dummy variant because `Naga` doesn't have a lifetime and without enough active features it
            /// could be the last one active.
            #[doc(hidden)]
            Dummy(PhantomData<&'a ()>),
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ShaderSource<'a> {
            #[inline]
            fn clone(&self) -> ShaderSource<'a> {
                match self {
                    ShaderSource::Wgsl(__self_0) => {
                        ShaderSource::Wgsl(::core::clone::Clone::clone(__self_0))
                    }
                    ShaderSource::Dummy(__self_0) => {
                        ShaderSource::Dummy(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for ShaderSource<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ShaderSource::Wgsl(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Wgsl",
                            &__self_0,
                        )
                    }
                    ShaderSource::Dummy(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Dummy",
                            &__self_0,
                        )
                    }
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ShaderSource<'_>>();
        };
        /// Descriptor for use with [`Device::create_shader_module`].
        ///
        /// Corresponds to [WebGPU `GPUShaderModuleDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gpushadermoduledescriptor).
        pub struct ShaderModuleDescriptor<'a> {
            /// Debug label of the shader module. This will show up in graphics debuggers for easy identification.
            pub label: Label<'a>,
            /// Source code for the shader.
            pub source: ShaderSource<'a>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ShaderModuleDescriptor<'a> {
            #[inline]
            fn clone(&self) -> ShaderModuleDescriptor<'a> {
                ShaderModuleDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    source: ::core::clone::Clone::clone(&self.source),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for ShaderModuleDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ShaderModuleDescriptor",
                    "label",
                    &self.label,
                    "source",
                    &&self.source,
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<ShaderModuleDescriptor<'_>>();
        };
        /// Descriptor for a shader module given by any of several sources.
        /// At least one of the shader types that may be used by the backend must be `Some`
        ///
        /// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
        /// only WGSL source code strings are accepted.
        pub type ShaderModuleDescriptorPassthrough<'a> = wgt::CreateShaderModuleDescriptorPassthrough<
            'a,
            Label<'a>,
        >;
    }
    mod surface {
        use alloc::{boxed::Box, string::String, vec, vec::Vec};
        use core::ops::Deref;
        use core::{error, fmt};
        use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
        use crate::util::Mutex;
        use crate::*;
        /// Describes a [`Surface`].
        ///
        /// For use with [`Surface::configure`].
        ///
        /// Corresponds to [WebGPU `GPUCanvasConfiguration`](
        /// https://gpuweb.github.io/gpuweb/#canvas-configuration).
        pub type SurfaceConfiguration = wgt::SurfaceConfiguration<Vec<TextureFormat>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<SurfaceConfiguration>();
        };
        /// Handle to a presentable surface.
        ///
        /// A `Surface` represents a platform-specific surface (e.g. a window) onto which rendered images may
        /// be presented. A `Surface` may be created with the function [`Instance::create_surface`].
        ///
        /// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
        /// [`GPUCanvasContext`](https://gpuweb.github.io/gpuweb/#canvas-context)
        /// serves a similar role.
        pub struct Surface<'window> {
            /// Additional surface data returned by [`DynContext::instance_create_surface`].
            pub(crate) inner: dispatch::DispatchSurface,
            pub(crate) config: Mutex<Option<SurfaceConfiguration>>,
            /// Optionally, keep the source of the handle used for the surface alive.
            ///
            /// This is useful for platforms where the surface is created from a window and the surface
            /// would become invalid when the window is dropped.
            ///
            /// SAFETY: This field must be dropped *after* all other fields to ensure proper cleanup.
            pub(crate) _handle_source: Option<Box<dyn WindowHandle + 'window>>,
        }
        impl Surface<'_> {
            /// Returns the capabilities of the surface when used with the given adapter.
            ///
            /// Returns specified values (see [`SurfaceCapabilities`]) if surface is incompatible with the adapter.
            pub fn get_capabilities(&self, adapter: &Adapter) -> SurfaceCapabilities {
                self.inner.get_capabilities(&adapter.inner)
            }
            /// Return a default `SurfaceConfiguration` from width and height to use for the [`Surface`] with this adapter.
            ///
            /// Returns None if the surface isn't supported by this adapter
            pub fn get_default_config(
                &self,
                adapter: &Adapter,
                width: u32,
                height: u32,
            ) -> Option<SurfaceConfiguration> {
                let caps = self.get_capabilities(adapter);
                Some(SurfaceConfiguration {
                    usage: wgt::TextureUsages::RENDER_ATTACHMENT,
                    format: *caps.formats.first()?,
                    width,
                    height,
                    desired_maximum_frame_latency: 2,
                    present_mode: *caps.present_modes.first()?,
                    alpha_mode: wgt::CompositeAlphaMode::Auto,
                    view_formats: ::alloc::vec::Vec::new(),
                })
            }
            /// Initializes [`Surface`] for presentation.
            ///
            /// If the surface is already configured, this will wait for the GPU to come idle
            /// before recreating the swapchain to prevent race conditions.
            ///
            /// # Validation Errors
            /// - Submissions that happen _during_ the configure may cause the
            ///   internal wait-for-idle to fail, raising a validation error.
            ///
            /// # Panics
            ///
            /// - A old [`SurfaceTexture`] is still alive referencing an old surface.
            /// - Texture format requested is unsupported on the surface.
            /// - `config.width` or `config.height` is zero.
            pub fn configure(&self, device: &Device, config: &SurfaceConfiguration) {
                self.inner.configure(&device.inner, config);
                let mut conf = self.config.lock();
                *conf = Some(config.clone());
            }
            /// Returns the next texture to be presented by the swapchain for drawing.
            ///
            /// In order to present the [`SurfaceTexture`] returned by this method,
            /// first a [`Queue::submit`] needs to be done with some work rendering to this texture.
            /// Then [`SurfaceTexture::present`] needs to be called.
            ///
            /// If a SurfaceTexture referencing this surface is alive when the swapchain is recreated,
            /// recreating the swapchain will panic.
            pub fn get_current_texture(&self) -> Result<SurfaceTexture, SurfaceError> {
                let (texture, status, detail) = self.inner.get_current_texture();
                let suboptimal = match status {
                    SurfaceStatus::Good => false,
                    SurfaceStatus::Suboptimal => true,
                    SurfaceStatus::Timeout => return Err(SurfaceError::Timeout),
                    SurfaceStatus::Outdated => return Err(SurfaceError::Outdated),
                    SurfaceStatus::Lost => return Err(SurfaceError::Lost),
                    SurfaceStatus::Unknown => return Err(SurfaceError::Other),
                };
                let guard = self.config.lock();
                let config = guard
                    .as_ref()
                    .expect("This surface has not been configured yet.");
                let descriptor = TextureDescriptor {
                    label: None,
                    size: Extent3d {
                        width: config.width,
                        height: config.height,
                        depth_or_array_layers: 1,
                    },
                    format: config.format,
                    usage: config.usage,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    view_formats: &[],
                };
                texture
                    .map(|texture| SurfaceTexture {
                        texture: Texture {
                            inner: texture,
                            descriptor,
                        },
                        suboptimal,
                        presented: false,
                        detail,
                    })
                    .ok_or(SurfaceError::Lost)
            }
            /// Get the [`wgpu_hal`] surface from this `Surface`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Surface`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Surface`
            ///- [`hal::api::Metal`] uses [`hal::metal::Surface`]
            ///- `hal::api::Dx12` uses `hal::dx12::Surface`
            ///- `hal::api::Gles` uses `hal::gles::Surface`
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The surface is not from the backend specified by `A`.
            /// - The surface is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Surface`]: hal::Api::Surface
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::Surface> + WasmNotSendSync> {
                let core_surface = self.inner.as_core_opt()?;
                unsafe { core_surface.context.surface_as_hal::<A>(core_surface) }
            }
        }
        impl fmt::Debug for Surface<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Surface")
                    .field(
                        "_handle_source",
                        &if self._handle_source.is_some() { "Some" } else { "None" },
                    )
                    .field("inner", &self.inner)
                    .field("config", &self.config)
                    .finish()
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Surface<'_>>();
        };
        impl PartialEq for Surface<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Surface<'_> {}
        impl PartialOrd for Surface<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Surface<'_> {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Surface<'_> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        /// Super trait for window handles as used in [`SurfaceTarget`].
        pub trait WindowHandle: HasWindowHandle + HasDisplayHandle + WasmNotSendSync {}
        impl<T> WindowHandle for T
        where
            T: HasWindowHandle + HasDisplayHandle + WasmNotSendSync,
        {}
        /// The window/canvas/surface/swap-chain/etc. a surface is attached to, for use with safe surface creation.
        ///
        /// This is either a window or an actual web canvas depending on the platform and
        /// enabled features.
        /// Refer to the individual variants for more information.
        ///
        /// See also [`SurfaceTargetUnsafe`] for unsafe variants.
        #[non_exhaustive]
        pub enum SurfaceTarget<'window> {
            /// Window handle producer.
            ///
            /// If the specified display and window handle are not supported by any of the backends, then the surface
            /// will not be supported by any adapters.
            ///
            /// # Errors
            ///
            /// - On WebGL2: surface creation returns an error if the browser does not support WebGL2,
            ///   or declines to provide GPU access (such as due to a resource shortage).
            ///
            /// # Panics
            ///
            /// - On macOS/Metal: will panic if not called on the main thread.
            /// - On web: will panic if the `raw_window_handle` does not properly refer to a
            ///   canvas element.
            Window(Box<dyn WindowHandle + 'window>),
        }
        impl<'a, T> From<T> for SurfaceTarget<'a>
        where
            T: WindowHandle + 'a,
        {
            fn from(window: T) -> Self {
                Self::Window(Box::new(window))
            }
        }
        /// The window/canvas/surface/swap-chain/etc. a surface is attached to, for use with unsafe surface creation.
        ///
        /// This is either a window or an actual web canvas depending on the platform and
        /// enabled features.
        /// Refer to the individual variants for more information.
        ///
        /// See also [`SurfaceTarget`] for safe variants.
        #[non_exhaustive]
        pub enum SurfaceTargetUnsafe {
            /// Raw window & display handle.
            ///
            /// If the specified display and window handle are not supported by any of the backends, then the surface
            /// will not be supported by any adapters.
            ///
            /// # Safety
            ///
            /// - `raw_window_handle` & `raw_display_handle` must be valid objects to create a surface upon.
            /// - `raw_window_handle` & `raw_display_handle` must remain valid until after the returned
            ///   [`Surface`] is  dropped.
            RawHandle {
                /// Raw display handle, underlying display must outlive the surface created from this.
                raw_display_handle: raw_window_handle::RawDisplayHandle,
                /// Raw display handle, underlying window must outlive the surface created from this.
                raw_window_handle: raw_window_handle::RawWindowHandle,
            },
            /// Surface from `CoreAnimationLayer`.
            ///
            /// # Safety
            ///
            /// - layer must be a valid object to create a surface upon.
            CoreAnimationLayer(*mut core::ffi::c_void),
        }
        impl SurfaceTargetUnsafe {
            /// Creates a [`SurfaceTargetUnsafe::RawHandle`] from a window.
            ///
            /// # Safety
            ///
            /// - `window` must outlive the resulting surface target
            ///   (and subsequently the surface created for this target).
            pub unsafe fn from_window<T>(
                window: &T,
            ) -> Result<Self, raw_window_handle::HandleError>
            where
                T: HasDisplayHandle + HasWindowHandle,
            {
                Ok(Self::RawHandle {
                    raw_display_handle: window.display_handle()?.as_raw(),
                    raw_window_handle: window.window_handle()?.as_raw(),
                })
            }
        }
        /// [`Instance::create_surface()`] or a related function failed.
        #[non_exhaustive]
        pub struct CreateSurfaceError {
            pub(crate) inner: CreateSurfaceErrorKind,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateSurfaceError {
            #[inline]
            fn clone(&self) -> CreateSurfaceError {
                CreateSurfaceError {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateSurfaceError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CreateSurfaceError",
                    "inner",
                    &&self.inner,
                )
            }
        }
        pub(crate) enum CreateSurfaceErrorKind {
            /// Error from [`wgpu_hal`].
            Hal(wgc::instance::CreateSurfaceError),
            /// Error from WebGPU surface creation.
            #[expect(dead_code)]
            Web(String),
            /// Error when trying to get a [`DisplayHandle`] or a [`WindowHandle`] from
            /// `raw_window_handle`.
            RawHandle(raw_window_handle::HandleError),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateSurfaceErrorKind {
            #[inline]
            fn clone(&self) -> CreateSurfaceErrorKind {
                match self {
                    CreateSurfaceErrorKind::Hal(__self_0) => {
                        CreateSurfaceErrorKind::Hal(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    CreateSurfaceErrorKind::Web(__self_0) => {
                        CreateSurfaceErrorKind::Web(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    CreateSurfaceErrorKind::RawHandle(__self_0) => {
                        CreateSurfaceErrorKind::RawHandle(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreateSurfaceErrorKind {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CreateSurfaceErrorKind::Hal(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Hal",
                            &__self_0,
                        )
                    }
                    CreateSurfaceErrorKind::Web(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Web",
                            &__self_0,
                        )
                    }
                    CreateSurfaceErrorKind::RawHandle(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "RawHandle",
                            &__self_0,
                        )
                    }
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<CreateSurfaceError>();
        };
        impl fmt::Display for CreateSurfaceError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self.inner {
                    CreateSurfaceErrorKind::Hal(e) => e.fmt(f),
                    CreateSurfaceErrorKind::Web(e) => e.fmt(f),
                    CreateSurfaceErrorKind::RawHandle(e) => e.fmt(f),
                }
            }
        }
        impl error::Error for CreateSurfaceError {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                match &self.inner {
                    CreateSurfaceErrorKind::Hal(e) => e.source(),
                    CreateSurfaceErrorKind::Web(_) => None,
                    CreateSurfaceErrorKind::RawHandle(e) => e.source(),
                }
            }
        }
        impl From<wgc::instance::CreateSurfaceError> for CreateSurfaceError {
            fn from(e: wgc::instance::CreateSurfaceError) -> Self {
                Self {
                    inner: CreateSurfaceErrorKind::Hal(e),
                }
            }
        }
    }
    mod surface_texture {
        use core::{error, fmt};
        use crate::*;
        /// Surface texture that can be rendered to.
        /// Result of a successful call to [`Surface::get_current_texture`].
        ///
        /// This type is unique to the Rust API of `wgpu`. In the WebGPU specification,
        /// the [`GPUCanvasContext`](https://gpuweb.github.io/gpuweb/#canvas-context) provides
        /// a texture without any additional information.
        pub struct SurfaceTexture {
            /// Accessible view of the frame.
            pub texture: Texture,
            /// `true` if the acquired buffer can still be used for rendering,
            /// but should be recreated for maximum performance.
            pub suboptimal: bool,
            pub(crate) presented: bool,
            pub(crate) detail: dispatch::DispatchSurfaceOutputDetail,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SurfaceTexture {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "SurfaceTexture",
                    "texture",
                    &self.texture,
                    "suboptimal",
                    &self.suboptimal,
                    "presented",
                    &self.presented,
                    "detail",
                    &&self.detail,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SurfaceTexture {
            #[inline]
            fn clone(&self) -> SurfaceTexture {
                SurfaceTexture {
                    texture: ::core::clone::Clone::clone(&self.texture),
                    suboptimal: ::core::clone::Clone::clone(&self.suboptimal),
                    presented: ::core::clone::Clone::clone(&self.presented),
                    detail: ::core::clone::Clone::clone(&self.detail),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<SurfaceTexture>();
        };
        impl PartialEq for SurfaceTexture {
            fn eq(&self, other: &Self) -> bool {
                self.texture.inner == other.texture.inner
            }
        }
        impl Eq for SurfaceTexture {}
        impl PartialOrd for SurfaceTexture {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for SurfaceTexture {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.texture.inner.cmp(&other.texture.inner)
            }
        }
        impl core::hash::Hash for SurfaceTexture {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.texture.inner.hash(state)
            }
        }
        impl SurfaceTexture {
            /// Schedule this texture to be presented on the owning surface.
            ///
            /// Needs to be called after any work on the texture is scheduled via [`Queue::submit`].
            ///
            /// # Platform dependent behavior
            ///
            /// On Wayland, `present` will attach a `wl_buffer` to the underlying `wl_surface` and commit the new surface
            /// state. If it is desired to do things such as request a frame callback, scale the surface using the viewporter
            /// or synchronize other double buffered state, then these operations should be done before the call to `present`.
            pub fn present(mut self) {
                self.presented = true;
                self.detail.present();
            }
        }
        impl Drop for SurfaceTexture {
            fn drop(&mut self) {
                if !self.presented && !thread_panicking() {
                    self.detail.texture_discard();
                }
            }
        }
        /// Result of an unsuccessful call to [`Surface::get_current_texture`].
        pub enum SurfaceError {
            /// A timeout was encountered while trying to acquire the next frame.
            Timeout,
            /// The underlying surface has changed, and therefore the swap chain must be updated.
            Outdated,
            /// The swap chain has been lost and needs to be recreated.
            Lost,
            /// There is no more memory left to allocate a new frame.
            OutOfMemory,
            /// Acquiring a texture failed with a generic error. Check error callbacks for more information.
            Other,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SurfaceError {
            #[inline]
            fn clone(&self) -> SurfaceError {
                match self {
                    SurfaceError::Timeout => SurfaceError::Timeout,
                    SurfaceError::Outdated => SurfaceError::Outdated,
                    SurfaceError::Lost => SurfaceError::Lost,
                    SurfaceError::OutOfMemory => SurfaceError::OutOfMemory,
                    SurfaceError::Other => SurfaceError::Other,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SurfaceError {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SurfaceError {
            #[inline]
            fn eq(&self, other: &SurfaceError) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SurfaceError {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SurfaceError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        SurfaceError::Timeout => "Timeout",
                        SurfaceError::Outdated => "Outdated",
                        SurfaceError::Lost => "Lost",
                        SurfaceError::OutOfMemory => "OutOfMemory",
                        SurfaceError::Other => "Other",
                    },
                )
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<SurfaceError>();
        };
        impl fmt::Display for SurfaceError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(
                    format_args!(
                        "{0}",
                        match self {
                            Self::Timeout => {
                                "A timeout was encountered while trying to acquire the next frame"
                            }
                            Self::Outdated => {
                                "The underlying surface has changed, and therefore the swap chain must be updated"
                            }
                            Self::Lost => {
                                "The swap chain has been lost and needs to be recreated"
                            }
                            Self::OutOfMemory => {
                                "There is no more memory left to allocate a new frame"
                            }
                            Self::Other => {
                                "Acquiring a texture failed with a generic error. Check error callbacks for more information"
                            }
                        },
                    ),
                )
            }
        }
        impl error::Error for SurfaceError {}
        fn thread_panicking() -> bool {
            std::thread::panicking()
        }
    }
    mod texture {
        use core::ops::Deref;
        use crate::*;
        /// Handle to a texture on the GPU.
        ///
        /// It can be created with [`Device::create_texture`].
        ///
        /// Corresponds to [WebGPU `GPUTexture`](https://gpuweb.github.io/gpuweb/#texture-interface).
        pub struct Texture {
            pub(crate) inner: dispatch::DispatchTexture,
            pub(crate) descriptor: TextureDescriptor<'static>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Texture {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Texture",
                    "inner",
                    &self.inner,
                    "descriptor",
                    &&self.descriptor,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Texture {
            #[inline]
            fn clone(&self) -> Texture {
                Texture {
                    inner: ::core::clone::Clone::clone(&self.inner),
                    descriptor: ::core::clone::Clone::clone(&self.descriptor),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<Texture>();
        };
        impl PartialEq for Texture {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Texture {}
        impl PartialOrd for Texture {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Texture {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Texture {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Texture {
            /// Get the [`wgpu_hal`] texture from this `Texture`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::Texture`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::Texture`
            ///- [`hal::api::Metal`] uses [`hal::metal::Texture`]
            ///- `hal::api::Dx12` uses `hal::dx12::Texture`
            ///- `hal::api::Gles` uses `hal::gles::Texture`
            ///
            /// # Deadlocks
            ///
            /// - The returned guard holds a read-lock on a device-local "destruction"
            ///   lock, which will cause all calls to `destroy` to block until the
            ///   guard is released.
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The texture is not from the backend specified by `A`.
            /// - The texture is from the `webgpu` or `custom` backend.
            /// - The texture has had [`Self::destroy()`] called on it.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::Texture`]: hal::Api::Texture
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::Texture>> {
                let texture = self.inner.as_core_opt()?;
                unsafe { texture.context.texture_as_hal::<A>(texture) }
            }
            /// Creates a view of this texture, specifying an interpretation of its texels and
            /// possibly a subset of its layers and mip levels.
            ///
            /// Texture views are needed to use a texture as a binding in a [`BindGroup`]
            /// or as an attachment in a [`RenderPass`].
            pub fn create_view(&self, desc: &TextureViewDescriptor<'_>) -> TextureView {
                let view = self.inner.create_view(desc);
                TextureView {
                    inner: view,
                    texture: self.clone(),
                }
            }
            /// Destroy the associated native resources as soon as possible.
            pub fn destroy(&self) {
                self.inner.destroy();
            }
            /// Make an `TexelCopyTextureInfo` representing the whole texture.
            pub fn as_image_copy(&self) -> TexelCopyTextureInfo<'_> {
                TexelCopyTextureInfo {
                    texture: self,
                    mip_level: 0,
                    origin: Origin3d::ZERO,
                    aspect: TextureAspect::All,
                }
            }
            /// Returns the size of this `Texture`.
            ///
            /// This is always equal to the `size` that was specified when creating the texture.
            pub fn size(&self) -> Extent3d {
                self.descriptor.size
            }
            /// Returns the width of this `Texture`.
            ///
            /// This is always equal to the `size.width` that was specified when creating the texture.
            pub fn width(&self) -> u32 {
                self.descriptor.size.width
            }
            /// Returns the height of this `Texture`.
            ///
            /// This is always equal to the `size.height` that was specified when creating the texture.
            pub fn height(&self) -> u32 {
                self.descriptor.size.height
            }
            /// Returns the depth or layer count of this `Texture`.
            ///
            /// This is always equal to the `size.depth_or_array_layers` that was specified when creating the texture.
            pub fn depth_or_array_layers(&self) -> u32 {
                self.descriptor.size.depth_or_array_layers
            }
            /// Returns the mip_level_count of this `Texture`.
            ///
            /// This is always equal to the `mip_level_count` that was specified when creating the texture.
            pub fn mip_level_count(&self) -> u32 {
                self.descriptor.mip_level_count
            }
            /// Returns the sample_count of this `Texture`.
            ///
            /// This is always equal to the `sample_count` that was specified when creating the texture.
            pub fn sample_count(&self) -> u32 {
                self.descriptor.sample_count
            }
            /// Returns the dimension of this `Texture`.
            ///
            /// This is always equal to the `dimension` that was specified when creating the texture.
            pub fn dimension(&self) -> TextureDimension {
                self.descriptor.dimension
            }
            /// Returns the format of this `Texture`.
            ///
            /// This is always equal to the `format` that was specified when creating the texture.
            pub fn format(&self) -> TextureFormat {
                self.descriptor.format
            }
            /// Returns the allowed usages of this `Texture`.
            ///
            /// This is always equal to the `usage` that was specified when creating the texture.
            pub fn usage(&self) -> TextureUsages {
                self.descriptor.usage
            }
        }
        /// Describes a [`Texture`].
        ///
        /// For use with [`Device::create_texture`].
        ///
        /// Corresponds to [WebGPU `GPUTextureDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor).
        pub type TextureDescriptor<'a> = wgt::TextureDescriptor<
            Label<'a>,
            &'a [TextureFormat],
        >;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<TextureDescriptor<'_>>();
        };
    }
    mod texture_view {
        use core::ops::Deref;
        use crate::*;
        /// Handle to a texture view.
        ///
        /// A `TextureView` object refers to a [`Texture`], or a subset of its layers and mip levels, and
        /// specifies an interpretation of the texture’s texels, which is needed to use a texture as a
        /// binding in a [`BindGroup`] or as an attachment in a [`RenderPass`].
        /// It can be created using [`Texture::create_view()`], which accepts a [`TextureViewDescriptor`]
        /// specifying the properties of the view.
        ///
        /// Corresponds to [WebGPU `GPUTextureView`](https://gpuweb.github.io/gpuweb/#gputextureview).
        pub struct TextureView {
            pub(crate) inner: dispatch::DispatchTextureView,
            pub(crate) texture: Texture,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TextureView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TextureView",
                    "inner",
                    &self.inner,
                    "texture",
                    &&self.texture,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TextureView {
            #[inline]
            fn clone(&self) -> TextureView {
                TextureView {
                    inner: ::core::clone::Clone::clone(&self.inner),
                    texture: ::core::clone::Clone::clone(&self.texture),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<TextureView>();
        };
        impl PartialEq for TextureView {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for TextureView {}
        impl PartialOrd for TextureView {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for TextureView {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for TextureView {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl TextureView {
            /// Returns the [`Texture`] that this `TextureView` refers to.
            ///
            /// All wgpu resources are refcounted, so you can own the returned [`Texture`]
            /// by cloning it.
            pub fn texture(&self) -> &Texture {
                &self.texture
            }
            /// Get the [`wgpu_hal`] texture view from this `TextureView`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::TextureView`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::TextureView`
            ///- [`hal::api::Metal`] uses [`hal::metal::TextureView`]
            ///- `hal::api::Dx12` uses `hal::dx12::TextureView`
            ///- `hal::api::Gles` uses `hal::gles::TextureView`
            ///
            /// # Deadlocks
            ///
            /// - The returned guard holds a read-lock on a device-local "destruction"
            ///   lock, which will cause all calls to `destroy` to block until the
            ///   guard is released.
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The texture view is not from the backend specified by `A`.
            /// - The texture view is from the `webgpu` or `custom` backend.
            /// - The texture this view points to has had [`Texture::destroy()`] called on it.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::TextureView`]: hal::Api::TextureView
            pub unsafe fn as_hal<A: hal::Api>(
                &self,
            ) -> Option<impl Deref<Target = A::TextureView>> {
                let view = self.inner.as_core_opt()?;
                unsafe { view.context.texture_view_as_hal::<A>(view) }
            }
        }
        /// Describes a [`TextureView`].
        ///
        /// For use with [`Texture::create_view`].
        ///
        /// Corresponds to [WebGPU `GPUTextureViewDescriptor`](
        /// https://gpuweb.github.io/gpuweb/#dictdef-gputextureviewdescriptor).
        pub type TextureViewDescriptor<'a> = wgt::TextureViewDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<TextureViewDescriptor<'_>>();
        };
    }
    mod tlas {
        use crate::{api::blas::TlasInstance, dispatch};
        use crate::{BindingResource, Label};
        use alloc::vec::Vec;
        use core::ops::Deref;
        use core::ops::{Index, IndexMut, Range};
        use wgt::WasmNotSendSync;
        /// Descriptor to create top level acceleration structures.
        pub type CreateTlasDescriptor<'a> = wgt::CreateTlasDescriptor<Label<'a>>;
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + Send + Sync>() {}
            assert_impl_all::<CreateTlasDescriptor<'_>>();
        };
        /// Top Level Acceleration Structure (TLAS).
        ///
        /// A TLAS contains a series of [TLAS instances], which are a reference to
        /// a BLAS and a transformation matrix placing the geometry in the world.
        ///
        /// A TLAS also contains an extra set of TLAS instances in a device readable form, you cant interact
        /// directly with these, instead you have to build the TLAS with [TLAS instances].
        ///
        /// [TLAS instances]: TlasInstance
        pub struct Tlas {
            pub(crate) inner: dispatch::DispatchTlas,
            pub(crate) instances: Vec<Option<TlasInstance>>,
            pub(crate) lowest_unmodified: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Tlas {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Tlas",
                    "inner",
                    &self.inner,
                    "instances",
                    &self.instances,
                    "lowest_unmodified",
                    &&self.lowest_unmodified,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Tlas {
            #[inline]
            fn clone(&self) -> Tlas {
                Tlas {
                    inner: ::core::clone::Clone::clone(&self.inner),
                    instances: ::core::clone::Clone::clone(&self.instances),
                    lowest_unmodified: ::core::clone::Clone::clone(
                        &self.lowest_unmodified,
                    ),
                }
            }
        }
        const _: fn() = || {
            fn assert_impl_all<T: ?Sized + WasmNotSendSync>() {}
            assert_impl_all::<Tlas>();
        };
        impl PartialEq for Tlas {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for Tlas {}
        impl PartialOrd for Tlas {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Tlas {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.inner.cmp(&other.inner)
            }
        }
        impl core::hash::Hash for Tlas {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.inner.hash(state)
            }
        }
        impl Tlas {
            /// Get the [`wgpu_hal`] acceleration structure from this `Tlas`.
            ///
            /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
            /// and pass that struct to the to the `A` type parameter.
            ///
            /// Returns a guard that dereferences to the type of the hal backend
            /// which implements [`A::AccelerationStructure`].
            ///
            /// # Types
            ///
            /// The returned type depends on the backend:
            ///
            ///- `hal::api::Vulkan` uses `hal::vulkan::AccelerationStructure`
            ///- [`hal::api::Metal`] uses [`hal::metal::AccelerationStructure`]
            ///- `hal::api::Dx12` uses `hal::dx12::AccelerationStructure`
            ///- `hal::api::Gles` uses `hal::gles::AccelerationStructure`
            ///
            /// # Deadlocks
            ///
            /// - The returned guard holds a read-lock on a device-local "destruction"
            ///   lock, which will cause all calls to `destroy` to block until the
            ///   guard is released.
            ///
            /// # Errors
            ///
            /// This method will return None if:
            /// - The acceleration structure is not from the backend specified by `A`.
            /// - The acceleration structure is from the `webgpu` or `custom` backend.
            ///
            /// # Safety
            ///
            /// - The returned resource must not be destroyed unless the guard
            ///   is the last reference to it and it is not in use by the GPU.
            ///   The guard and handle may be dropped at any time however.
            /// - All the safety requirements of wgpu-hal must be upheld.
            ///
            /// [`A::AccelerationStructure`]: hal::Api::AccelerationStructure
            pub unsafe fn as_hal<A: hal::Api>(
                &mut self,
            ) -> Option<impl Deref<Target = A::AccelerationStructure>> {
                let tlas = self.inner.as_core_opt()?;
                unsafe { tlas.context.tlas_as_hal::<A>(tlas) }
            }
            /// Get a reference to all instances.
            pub fn get(&self) -> &[Option<TlasInstance>] {
                &self.instances
            }
            /// Get a mutable slice to a range of instances.
            /// Returns None if the range is out of bounds.
            /// All elements from the lowest accessed index up are marked as modified.
            /// For best performance it is recommended to prefer access to low elements and modify higher elements as little as possible.
            /// This can be done by ordering instances from the most to the least used. It is recommended
            /// to use [`Self::index_mut`] unless the option if out of bounds is required
            pub fn get_mut_slice(
                &mut self,
                range: Range<usize>,
            ) -> Option<&mut [Option<TlasInstance>]> {
                if range.end > self.instances.len() {
                    return None;
                }
                if range.end as u32 > self.lowest_unmodified {
                    self.lowest_unmodified = range.end as u32;
                }
                Some(&mut self.instances[range])
            }
            /// Get a single mutable reference to an instance.
            /// Returns None if the range is out of bounds.
            /// All elements from the lowest accessed index up are marked as modified.
            /// For best performance it is recommended to prefer access to low elements and modify higher elements as little as possible.
            /// This can be done by ordering instances from the most to the least used. It is recommended
            /// to use [`Self::index_mut`] unless the option if out of bounds is required
            pub fn get_mut_single(
                &mut self,
                index: usize,
            ) -> Option<&mut Option<TlasInstance>> {
                if index >= self.instances.len() {
                    return None;
                }
                if index as u32 + 1 > self.lowest_unmodified {
                    self.lowest_unmodified = index as u32 + 1;
                }
                Some(&mut self.instances[index])
            }
            /// Get the binding resource for the underling acceleration structure, to be used when creating a [`BindGroup`]
            ///
            /// [`BindGroup`]: super::BindGroup
            pub fn as_binding(&self) -> BindingResource<'_> {
                BindingResource::AccelerationStructure(self)
            }
        }
        impl Index<usize> for Tlas {
            type Output = Option<TlasInstance>;
            fn index(&self, index: usize) -> &Self::Output {
                self.instances.index(index)
            }
        }
        impl Index<Range<usize>> for Tlas {
            type Output = [Option<TlasInstance>];
            fn index(&self, index: Range<usize>) -> &Self::Output {
                self.instances.index(index)
            }
        }
        impl IndexMut<usize> for Tlas {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let idx = self.instances.index_mut(index);
                if index as u32 + 1 > self.lowest_unmodified {
                    self.lowest_unmodified = index as u32 + 1;
                }
                idx
            }
        }
        impl IndexMut<Range<usize>> for Tlas {
            fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
                let idx = self.instances.index_mut(index.clone());
                if index.end > self.lowest_unmodified as usize {
                    self.lowest_unmodified = index.end as u32;
                }
                idx
            }
        }
    }
    pub use adapter::*;
    pub use bind_group::*;
    pub use bind_group_layout::*;
    pub use blas::*;
    pub use buffer::*;
    pub use command_buffer::*;
    use command_buffer_actions::*;
    pub use command_encoder::*;
    pub use common_pipeline::*;
    pub use compute_pass::*;
    pub use compute_pipeline::*;
    pub use device::*;
    pub use external_texture::*;
    pub use instance::*;
    pub use pipeline_cache::*;
    pub use pipeline_layout::*;
    pub use query_set::*;
    pub use queue::*;
    pub use render_bundle::*;
    pub use render_bundle_encoder::*;
    pub use render_pass::*;
    pub use render_pipeline::*;
    pub use sampler::*;
    pub use shader_module::*;
    pub use surface::*;
    pub use surface_texture::*;
    pub use texture::*;
    pub use texture_view::*;
    pub use tlas::*;
    /// Object debugging label.
    pub type Label<'a> = Option<&'a str>;
    /// A cute utility type that works just like `PhantomData`, but also
    /// implements `Drop`. This forces any lifetimes that are associated
    /// with the type to be used until the `Drop` impl is ran. This prevents
    /// lifetimes from being shortened.
    pub(crate) struct PhantomDrop<T>(core::marker::PhantomData<T>);
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for PhantomDrop<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PhantomDrop", &&self.0)
        }
    }
    impl<T> Default for PhantomDrop<T> {
        fn default() -> Self {
            Self(core::marker::PhantomData)
        }
    }
    impl<T> Drop for PhantomDrop<T> {
        fn drop(&mut self) {}
    }
}
mod backend {
    pub mod wgpu_core {
        use alloc::{
            borrow::Cow::{self, Borrowed},
            boxed::Box, format, string::{String, ToString as _},
            sync::Arc, vec, vec::Vec,
        };
        use core::{
            error::Error, fmt, future::ready, ops::{Deref, Range},
            pin::Pin, ptr::NonNull, slice,
        };
        use arrayvec::ArrayVec;
        use smallvec::SmallVec;
        use wgc::{
            command::bundle_ffi::*, error::ContextErrorSource,
            pipeline::CreateShaderModuleError, resource::BlasPrepareCompactResult,
        };
        use wgt::{
            error::{ErrorType, WebGpuError},
            WasmNotSendSync,
        };
        use crate::{
            api, dispatch::{self, BlasCompactCallback, BufferMappedRangeInterface},
            BindingResource, Blas, BufferBinding, BufferDescriptor, CompilationInfo,
            CompilationMessage, CompilationMessageType, ErrorSource, Features, Label,
            LoadOp, MapMode, Operations, ShaderSource, SurfaceTargetUnsafe,
            TextureDescriptor, Tlas,
        };
        use crate::{dispatch::DispatchAdapter, util::Mutex};
        pub struct ContextWgpuCore(Arc<wgc::global::Global>);
        #[automatically_derived]
        impl ::core::clone::Clone for ContextWgpuCore {
            #[inline]
            fn clone(&self) -> ContextWgpuCore {
                ContextWgpuCore(::core::clone::Clone::clone(&self.0))
            }
        }
        impl Drop for ContextWgpuCore {
            fn drop(&mut self) {}
        }
        impl fmt::Debug for ContextWgpuCore {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("ContextWgpuCore").field("type", &"Native").finish()
            }
        }
        impl ContextWgpuCore {
            pub unsafe fn from_hal_instance<A: hal::Api>(
                hal_instance: A::Instance,
            ) -> Self {
                Self(unsafe {
                    Arc::new(
                        wgc::global::Global::from_hal_instance::<A>("wgpu", hal_instance),
                    )
                })
            }
            /// # Safety
            ///
            /// - The raw instance handle returned must not be manually destroyed.
            pub unsafe fn instance_as_hal<A: hal::Api>(&self) -> Option<&A::Instance> {
                unsafe { self.0.instance_as_hal::<A>() }
            }
            pub unsafe fn from_core_instance(
                core_instance: wgc::instance::Instance,
            ) -> Self {
                Self(unsafe {
                    Arc::new(wgc::global::Global::from_instance(core_instance))
                })
            }
            pub fn enumerate_adapters(
                &self,
                backends: wgt::Backends,
            ) -> Vec<wgc::id::AdapterId> {
                self.0.enumerate_adapters(backends)
            }
            pub unsafe fn create_adapter_from_hal<A: hal::Api>(
                &self,
                hal_adapter: hal::ExposedAdapter<A>,
            ) -> wgc::id::AdapterId {
                unsafe { self.0.create_adapter_from_hal(hal_adapter.into(), None) }
            }
            pub unsafe fn adapter_as_hal<A: hal::Api>(
                &self,
                adapter: &CoreAdapter,
            ) -> Option<impl Deref<Target = A::Adapter> + WasmNotSendSync> {
                unsafe { self.0.adapter_as_hal::<A>(adapter.id) }
            }
            pub unsafe fn buffer_as_hal<A: hal::Api>(
                &self,
                buffer: &CoreBuffer,
            ) -> Option<impl Deref<Target = A::Buffer>> {
                unsafe { self.0.buffer_as_hal::<A>(buffer.id) }
            }
            pub unsafe fn create_device_from_hal<A: hal::Api>(
                &self,
                adapter: &CoreAdapter,
                hal_device: hal::OpenDevice<A>,
                desc: &crate::DeviceDescriptor<'_>,
            ) -> Result<(CoreDevice, CoreQueue), crate::RequestDeviceError> {
                if !#[allow(non_exhaustive_omitted_patterns)]
                match desc.trace {
                    wgt::Trace::Off => true,
                    _ => false,
                } {
                    {
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!(
                                        "\n                Feature \'trace\' has been removed temporarily; see https://github.com/gfx-rs/wgpu/issues/5974. The `trace` parameter will have no effect.",
                                    ),
                                    lvl,
                                    &(
                                        "wgpu::backend::wgpu_core",
                                        "wgpu::backend::wgpu_core",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                }
                let (device_id, queue_id) = unsafe {
                    self.0
                        .create_device_from_hal(
                            adapter.id,
                            hal_device.into(),
                            &desc.map_label(|l| l.map(Borrowed)),
                            None,
                            None,
                        )
                }?;
                let error_sink = Arc::new(Mutex::new(ErrorSinkRaw::new()));
                let device = CoreDevice {
                    context: self.clone(),
                    id: device_id,
                    error_sink: error_sink.clone(),
                    features: desc.required_features,
                };
                let queue = CoreQueue {
                    context: self.clone(),
                    id: queue_id,
                    error_sink,
                };
                Ok((device, queue))
            }
            pub unsafe fn create_texture_from_hal<A: hal::Api>(
                &self,
                hal_texture: A::Texture,
                device: &CoreDevice,
                desc: &TextureDescriptor<'_>,
            ) -> CoreTexture {
                let descriptor = desc
                    .map_label_and_view_formats(|l| l.map(Borrowed), |v| v.to_vec());
                let (id, error) = unsafe {
                    self.0
                        .create_texture_from_hal(
                            Box::new(hal_texture),
                            device.id,
                            &descriptor,
                            None,
                        )
                };
                if let Some(cause) = error {
                    self.handle_error(
                        &device.error_sink,
                        cause,
                        desc.label,
                        "Device::create_texture_from_hal",
                    );
                }
                CoreTexture {
                    context: self.clone(),
                    id,
                    error_sink: Arc::clone(&device.error_sink),
                }
            }
            /// # Safety
            ///
            /// - `hal_buffer` must be created from `device`.
            /// - `hal_buffer` must be created respecting `desc`
            /// - `hal_buffer` must be initialized
            /// - `hal_buffer` must not have zero size.
            pub unsafe fn create_buffer_from_hal<A: hal::Api>(
                &self,
                hal_buffer: A::Buffer,
                device: &CoreDevice,
                desc: &BufferDescriptor<'_>,
            ) -> CoreBuffer {
                let (id, error) = unsafe {
                    self.0
                        .create_buffer_from_hal::<
                            A,
                        >(
                            hal_buffer,
                            device.id,
                            &desc.map_label(|l| l.map(Borrowed)),
                            None,
                        )
                };
                if let Some(cause) = error {
                    self.handle_error(
                        &device.error_sink,
                        cause,
                        desc.label,
                        "Device::create_buffer_from_hal",
                    );
                }
                CoreBuffer {
                    context: self.clone(),
                    id,
                    error_sink: Arc::clone(&device.error_sink),
                }
            }
            pub unsafe fn device_as_hal<A: hal::Api>(
                &self,
                device: &CoreDevice,
            ) -> Option<impl Deref<Target = A::Device>> {
                unsafe { self.0.device_as_hal::<A>(device.id) }
            }
            pub unsafe fn surface_as_hal<A: hal::Api>(
                &self,
                surface: &CoreSurface,
            ) -> Option<impl Deref<Target = A::Surface>> {
                unsafe { self.0.surface_as_hal::<A>(surface.id) }
            }
            pub unsafe fn texture_as_hal<A: hal::Api>(
                &self,
                texture: &CoreTexture,
            ) -> Option<impl Deref<Target = A::Texture>> {
                unsafe { self.0.texture_as_hal::<A>(texture.id) }
            }
            pub unsafe fn texture_view_as_hal<A: hal::Api>(
                &self,
                texture_view: &CoreTextureView,
            ) -> Option<impl Deref<Target = A::TextureView>> {
                unsafe { self.0.texture_view_as_hal::<A>(texture_view.id) }
            }
            /// This method will start the wgpu_core level command recording.
            pub unsafe fn command_encoder_as_hal_mut<
                A: hal::Api,
                F: FnOnce(Option<&mut A::CommandEncoder>) -> R,
                R,
            >(
                &self,
                command_encoder: &CoreCommandEncoder,
                hal_command_encoder_callback: F,
            ) -> R {
                unsafe {
                    self.0
                        .command_encoder_as_hal_mut::<
                            A,
                            F,
                            R,
                        >(command_encoder.id, hal_command_encoder_callback)
                }
            }
            pub unsafe fn blas_as_hal<A: hal::Api>(
                &self,
                blas: &CoreBlas,
            ) -> Option<impl Deref<Target = A::AccelerationStructure>> {
                unsafe { self.0.blas_as_hal::<A>(blas.id) }
            }
            pub unsafe fn tlas_as_hal<A: hal::Api>(
                &self,
                tlas: &CoreTlas,
            ) -> Option<impl Deref<Target = A::AccelerationStructure>> {
                unsafe { self.0.tlas_as_hal::<A>(tlas.id) }
            }
            pub fn generate_report(&self) -> wgc::global::GlobalReport {
                self.0.generate_report()
            }
            #[cold]
            #[track_caller]
            #[inline(never)]
            fn handle_error_inner(
                &self,
                sink_mutex: &Mutex<ErrorSinkRaw>,
                error_type: ErrorType,
                source: ContextErrorSource,
                label: Label<'_>,
                fn_ident: &'static str,
            ) {
                let source: ErrorSource = Box::new(wgc::error::ContextError {
                    fn_ident,
                    source,
                    label: label.unwrap_or_default().to_string(),
                });
                let final_error_handling = {
                    let mut sink = sink_mutex.lock();
                    let description = || self.format_error(&*source);
                    let error = match error_type {
                        ErrorType::Internal => {
                            let description = description();
                            crate::Error::Internal {
                                source,
                                description,
                            }
                        }
                        ErrorType::OutOfMemory => {
                            crate::Error::OutOfMemory {
                                source,
                            }
                        }
                        ErrorType::Validation => {
                            let description = description();
                            crate::Error::Validation {
                                source,
                                description,
                            }
                        }
                        ErrorType::DeviceLost => return,
                    };
                    sink.handle_error_or_return_handler(error)
                };
                if let Some(f) = final_error_handling {
                    f();
                }
            }
            #[inline]
            #[track_caller]
            fn handle_error(
                &self,
                sink_mutex: &Mutex<ErrorSinkRaw>,
                source: impl WebGpuError + WasmNotSendSync + 'static,
                label: Label<'_>,
                fn_ident: &'static str,
            ) {
                let error_type = source.webgpu_error_type();
                self.handle_error_inner(
                    sink_mutex,
                    error_type,
                    Box::new(source),
                    label,
                    fn_ident,
                )
            }
            #[inline]
            #[track_caller]
            fn handle_error_nolabel(
                &self,
                sink_mutex: &Mutex<ErrorSinkRaw>,
                source: impl WebGpuError + WasmNotSendSync + 'static,
                fn_ident: &'static str,
            ) {
                let error_type = source.webgpu_error_type();
                self.handle_error_inner(
                    sink_mutex,
                    error_type,
                    Box::new(source),
                    None,
                    fn_ident,
                )
            }
            #[track_caller]
            #[cold]
            fn handle_error_fatal(
                &self,
                cause: impl Error + WasmNotSendSync + 'static,
                operation: &'static str,
            ) -> ! {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Error in {1}: {0}",
                            self.format_error(&cause),
                            operation,
                        ),
                    );
                };
            }
            #[inline(never)]
            fn format_error(&self, err: &(dyn Error + 'static)) -> String {
                let mut output = String::new();
                let mut level = 1;
                fn print_tree(
                    output: &mut String,
                    level: &mut usize,
                    e: &(dyn Error + 'static),
                ) {
                    let mut print = |e: &(dyn Error + 'static)| {
                        use core::fmt::Write;
                        output
                            .write_fmt(
                                format_args!("{0}{1}\n", " ".repeat(*level * 2), e),
                            )
                            .unwrap();
                        if let Some(e) = e.source() {
                            *level += 1;
                            print_tree(output, level, e);
                            *level -= 1;
                        }
                    };
                    if let Some(multi) = e.downcast_ref::<wgc::error::MultiError>() {
                        for e in multi.errors() {
                            print(e);
                        }
                    } else {
                        print(e);
                    }
                }
                print_tree(&mut output, &mut level, err);
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("Validation Error\n\nCaused by:\n{0}", output),
                    )
                })
            }
            pub unsafe fn queue_as_hal<A: hal::Api>(
                &self,
                queue: &CoreQueue,
            ) -> Option<impl Deref<Target = A::Queue> + WasmNotSendSync> {
                unsafe { self.0.queue_as_hal::<A>(queue.id) }
            }
        }
        fn map_buffer_copy_view(
            view: crate::TexelCopyBufferInfo<'_>,
        ) -> wgt::TexelCopyBufferInfo<wgc::id::BufferId> {
            wgt::TexelCopyBufferInfo {
                buffer: view.buffer.inner.as_core().id,
                layout: view.layout,
            }
        }
        fn map_texture_copy_view(
            view: crate::TexelCopyTextureInfo<'_>,
        ) -> wgt::TexelCopyTextureInfo<wgc::id::TextureId> {
            wgt::TexelCopyTextureInfo {
                texture: view.texture.inner.as_core().id,
                mip_level: view.mip_level,
                origin: view.origin,
                aspect: view.aspect,
            }
        }
        #[expect(unused)]
        fn map_texture_tagged_copy_view(
            view: crate::CopyExternalImageDestInfo<&api::Texture>,
        ) -> wgt::CopyExternalImageDestInfo<wgc::id::TextureId> {
            wgt::CopyExternalImageDestInfo {
                texture: view.texture.inner.as_core().id,
                mip_level: view.mip_level,
                origin: view.origin,
                aspect: view.aspect,
                color_space: view.color_space,
                premultiplied_alpha: view.premultiplied_alpha,
            }
        }
        fn map_load_op<V: Copy>(load: &LoadOp<V>) -> LoadOp<Option<V>> {
            match load {
                LoadOp::Clear(clear_value) => LoadOp::Clear(Some(*clear_value)),
                LoadOp::Load => LoadOp::Load,
            }
        }
        fn map_pass_channel<V: Copy>(
            ops: Option<&Operations<V>>,
        ) -> wgc::command::PassChannel<Option<V>> {
            match ops {
                Some(&Operations { load, store }) => {
                    wgc::command::PassChannel {
                        load_op: Some(map_load_op(&load)),
                        store_op: Some(store),
                        read_only: false,
                    }
                }
                None => {
                    wgc::command::PassChannel {
                        load_op: None,
                        store_op: None,
                        read_only: true,
                    }
                }
            }
        }
        pub struct CoreSurface {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::SurfaceId,
            /// Configured device is needed to know which backend
            /// code to execute when acquiring a new frame.
            configured_device: Mutex<Option<wgc::id::DeviceId>>,
            /// The error sink with which to report errors.
            /// `None` if the surface has not been configured.
            error_sink: Mutex<Option<ErrorSink>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreSurface {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "CoreSurface",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "configured_device",
                    &self.configured_device,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreAdapter {
            pub(crate) context: ContextWgpuCore,
            pub(crate) id: wgc::id::AdapterId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreAdapter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreAdapter",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreDevice {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::DeviceId,
            error_sink: ErrorSink,
            features: Features,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreDevice {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "CoreDevice",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &self.error_sink,
                    "features",
                    &&self.features,
                )
            }
        }
        pub struct CoreBuffer {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::BufferId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreBuffer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreBuffer",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreShaderModule {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::ShaderModuleId,
            compilation_info: CompilationInfo,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreShaderModule {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreShaderModule",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "compilation_info",
                    &&self.compilation_info,
                )
            }
        }
        pub struct CoreBindGroupLayout {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::BindGroupLayoutId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreBindGroupLayout {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreBindGroupLayout",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreBindGroup {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::BindGroupId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreBindGroup {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreBindGroup",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreTexture {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::TextureId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreTexture {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreTexture",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreTextureView {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::TextureViewId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreTextureView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreTextureView",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreExternalTexture {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::ExternalTextureId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreExternalTexture {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreExternalTexture",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreSampler {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::SamplerId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreSampler {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreSampler",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreQuerySet {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::QuerySetId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreQuerySet {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreQuerySet",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CorePipelineLayout {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::PipelineLayoutId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CorePipelineLayout {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CorePipelineLayout",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CorePipelineCache {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::PipelineCacheId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CorePipelineCache {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CorePipelineCache",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreCommandBuffer {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::CommandBufferId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreCommandBuffer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreCommandBuffer",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreRenderBundleEncoder {
            pub(crate) context: ContextWgpuCore,
            encoder: wgc::command::RenderBundleEncoder,
            id: crate::cmp::Identifier,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreRenderBundleEncoder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreRenderBundleEncoder",
                    "context",
                    &self.context,
                    "encoder",
                    &self.encoder,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreRenderBundle {
            id: wgc::id::RenderBundleId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreRenderBundle {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CoreRenderBundle",
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreQueue {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::QueueId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreQueue {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreQueue",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreComputePipeline {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::ComputePipelineId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreComputePipeline {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreComputePipeline",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreRenderPipeline {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::RenderPipelineId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreRenderPipeline {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreRenderPipeline",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreComputePass {
            pub(crate) context: ContextWgpuCore,
            pass: wgc::command::ComputePass,
            error_sink: ErrorSink,
            id: crate::cmp::Identifier,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreComputePass {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "CoreComputePass",
                    "context",
                    &self.context,
                    "pass",
                    &self.pass,
                    "error_sink",
                    &self.error_sink,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreRenderPass {
            pub(crate) context: ContextWgpuCore,
            pass: wgc::command::RenderPass,
            error_sink: ErrorSink,
            id: crate::cmp::Identifier,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreRenderPass {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "CoreRenderPass",
                    "context",
                    &self.context,
                    "pass",
                    &self.pass,
                    "error_sink",
                    &self.error_sink,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreCommandEncoder {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::CommandEncoderId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreCommandEncoder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreCommandEncoder",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreBlas {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::BlasId,
            error_sink: ErrorSink,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreBlas {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "CoreBlas",
                    "context",
                    &self.context,
                    "id",
                    &self.id,
                    "error_sink",
                    &&self.error_sink,
                )
            }
        }
        pub struct CoreTlas {
            pub(crate) context: ContextWgpuCore,
            id: wgc::id::TlasId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreTlas {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreTlas",
                    "context",
                    &self.context,
                    "id",
                    &&self.id,
                )
            }
        }
        pub struct CoreSurfaceOutputDetail {
            context: ContextWgpuCore,
            surface_id: wgc::id::SurfaceId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreSurfaceOutputDetail {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreSurfaceOutputDetail",
                    "context",
                    &self.context,
                    "surface_id",
                    &&self.surface_id,
                )
            }
        }
        type ErrorSink = Arc<Mutex<ErrorSinkRaw>>;
        struct ErrorScope {
            error: Option<crate::Error>,
            filter: crate::ErrorFilter,
        }
        struct ErrorSinkRaw {
            scopes: Vec<ErrorScope>,
            uncaptured_handler: Option<Arc<dyn crate::UncapturedErrorHandler>>,
        }
        impl ErrorSinkRaw {
            fn new() -> ErrorSinkRaw {
                ErrorSinkRaw {
                    scopes: Vec::new(),
                    uncaptured_handler: None,
                }
            }
            /// Deliver the error to
            ///
            /// * the innermost error scope, if any, or
            /// * the uncaptured error handler, if there is one, or
            /// * [`default_error_handler()`].
            ///
            /// If a closure is returned, the caller should call it immediately after dropping the
            /// [`ErrorSink`] mutex guard. This makes sure that the user callback is not called with
            /// a wgpu mutex held.
            #[track_caller]
            #[must_use]
            fn handle_error_or_return_handler(
                &mut self,
                err: crate::Error,
            ) -> Option<impl FnOnce()> {
                let filter = match err {
                    crate::Error::OutOfMemory { .. } => crate::ErrorFilter::OutOfMemory,
                    crate::Error::Validation { .. } => crate::ErrorFilter::Validation,
                    crate::Error::Internal { .. } => crate::ErrorFilter::Internal,
                };
                match self.scopes.iter_mut().rev().find(|scope| scope.filter == filter) {
                    Some(scope) => {
                        if scope.error.is_none() {
                            scope.error = Some(err);
                        }
                        None
                    }
                    None => {
                        if let Some(custom_handler) = &self.uncaptured_handler {
                            let custom_handler = Arc::clone(custom_handler);
                            Some(move || (custom_handler)(err))
                        } else {
                            default_error_handler(err)
                        }
                    }
                }
            }
        }
        impl fmt::Debug for ErrorSinkRaw {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(format_args!("ErrorSink"))
            }
        }
        #[track_caller]
        fn default_error_handler(err: crate::Error) -> ! {
            {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            { ::log::__private_api::GlobalLogger },
                            format_args!("Handling wgpu errors as fatal by default"),
                            lvl,
                            &(
                                "wgpu::backend::wgpu_core",
                                "wgpu::backend::wgpu_core",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                }
            };
            {
                ::core::panicking::panic_fmt(format_args!("wgpu error: {0}\n", err));
            };
        }
        impl From<CreateShaderModuleError> for CompilationInfo {
            fn from(value: CreateShaderModuleError) -> Self {
                match value {
                    CreateShaderModuleError::Parsing(v) => v.into(),
                    CreateShaderModuleError::Validation(v) => v.into(),
                    CreateShaderModuleError::Device(_)
                    | CreateShaderModuleError::Generation => {
                        CompilationInfo {
                            messages: Vec::new(),
                        }
                    }
                    _ => {
                        CompilationInfo {
                            messages: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    CompilationMessage {
                                        message: value.to_string(),
                                        message_type: CompilationMessageType::Error,
                                        location: None,
                                    },
                                ]),
                            ),
                        }
                    }
                }
            }
        }
        pub struct CoreQueueWriteBuffer {
            buffer_id: wgc::id::StagingBufferId,
            mapping: CoreBufferMappedRange,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreQueueWriteBuffer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreQueueWriteBuffer",
                    "buffer_id",
                    &self.buffer_id,
                    "mapping",
                    &&self.mapping,
                )
            }
        }
        pub struct CoreBufferMappedRange {
            ptr: NonNull<u8>,
            size: usize,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CoreBufferMappedRange {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CoreBufferMappedRange",
                    "ptr",
                    &self.ptr,
                    "size",
                    &&self.size,
                )
            }
        }
        unsafe impl Send for CoreBufferMappedRange {}
        unsafe impl Sync for CoreBufferMappedRange {}
        impl Drop for CoreBufferMappedRange {
            fn drop(&mut self) {}
        }
        impl PartialEq for ContextWgpuCore {
            fn eq(&self, other: &Self) -> bool {
                let address_left = alloc::sync::Arc::as_ptr(&self.0);
                let address_right = alloc::sync::Arc::as_ptr(&other.0);
                address_left == address_right
            }
        }
        impl Eq for ContextWgpuCore {}
        impl PartialOrd for ContextWgpuCore {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for ContextWgpuCore {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                let address_left = alloc::sync::Arc::as_ptr(&self.0);
                let address_right = alloc::sync::Arc::as_ptr(&other.0);
                address_left.cmp(&address_right)
            }
        }
        impl core::hash::Hash for ContextWgpuCore {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let address = alloc::sync::Arc::as_ptr(&self.0);
                address.hash(state)
            }
        }
        impl PartialEq for CoreAdapter {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreAdapter {}
        impl PartialOrd for CoreAdapter {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreAdapter {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreAdapter {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreDevice {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreDevice {}
        impl PartialOrd for CoreDevice {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreDevice {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreDevice {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreQueue {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreQueue {}
        impl PartialOrd for CoreQueue {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreQueue {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreQueue {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreShaderModule {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreShaderModule {}
        impl PartialOrd for CoreShaderModule {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreShaderModule {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreShaderModule {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreBindGroupLayout {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreBindGroupLayout {}
        impl PartialOrd for CoreBindGroupLayout {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreBindGroupLayout {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreBindGroupLayout {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreBindGroup {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreBindGroup {}
        impl PartialOrd for CoreBindGroup {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreBindGroup {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreBindGroup {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreTextureView {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreTextureView {}
        impl PartialOrd for CoreTextureView {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreTextureView {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreTextureView {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreSampler {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreSampler {}
        impl PartialOrd for CoreSampler {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreSampler {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreSampler {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreBuffer {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreBuffer {}
        impl PartialOrd for CoreBuffer {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreBuffer {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreBuffer {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreTexture {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreTexture {}
        impl PartialOrd for CoreTexture {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreTexture {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreTexture {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreExternalTexture {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreExternalTexture {}
        impl PartialOrd for CoreExternalTexture {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreExternalTexture {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreExternalTexture {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreBlas {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreBlas {}
        impl PartialOrd for CoreBlas {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreBlas {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreBlas {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreTlas {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreTlas {}
        impl PartialOrd for CoreTlas {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreTlas {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreTlas {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreQuerySet {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreQuerySet {}
        impl PartialOrd for CoreQuerySet {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreQuerySet {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreQuerySet {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CorePipelineLayout {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CorePipelineLayout {}
        impl PartialOrd for CorePipelineLayout {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CorePipelineLayout {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CorePipelineLayout {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreRenderPipeline {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreRenderPipeline {}
        impl PartialOrd for CoreRenderPipeline {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreRenderPipeline {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreRenderPipeline {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreComputePipeline {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreComputePipeline {}
        impl PartialOrd for CoreComputePipeline {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreComputePipeline {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreComputePipeline {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CorePipelineCache {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CorePipelineCache {}
        impl PartialOrd for CorePipelineCache {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CorePipelineCache {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CorePipelineCache {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreCommandEncoder {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreCommandEncoder {}
        impl PartialOrd for CoreCommandEncoder {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreCommandEncoder {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreCommandEncoder {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreComputePass {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreComputePass {}
        impl PartialOrd for CoreComputePass {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreComputePass {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreComputePass {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreRenderPass {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreRenderPass {}
        impl PartialOrd for CoreRenderPass {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreRenderPass {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreRenderPass {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreCommandBuffer {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreCommandBuffer {}
        impl PartialOrd for CoreCommandBuffer {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreCommandBuffer {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreCommandBuffer {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreRenderBundleEncoder {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreRenderBundleEncoder {}
        impl PartialOrd for CoreRenderBundleEncoder {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreRenderBundleEncoder {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreRenderBundleEncoder {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreRenderBundle {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreRenderBundle {}
        impl PartialOrd for CoreRenderBundle {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreRenderBundle {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreRenderBundle {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreSurface {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for CoreSurface {}
        impl PartialOrd for CoreSurface {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreSurface {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.id.cmp(&other.id)
            }
        }
        impl core::hash::Hash for CoreSurface {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state)
            }
        }
        impl PartialEq for CoreSurfaceOutputDetail {
            fn eq(&self, other: &Self) -> bool {
                self.surface_id == other.surface_id
            }
        }
        impl Eq for CoreSurfaceOutputDetail {}
        impl PartialOrd for CoreSurfaceOutputDetail {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreSurfaceOutputDetail {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.surface_id.cmp(&other.surface_id)
            }
        }
        impl core::hash::Hash for CoreSurfaceOutputDetail {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.surface_id.hash(state)
            }
        }
        impl PartialEq for CoreQueueWriteBuffer {
            fn eq(&self, other: &Self) -> bool {
                self.mapping.ptr == other.mapping.ptr
            }
        }
        impl Eq for CoreQueueWriteBuffer {}
        impl PartialOrd for CoreQueueWriteBuffer {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreQueueWriteBuffer {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.mapping.ptr.cmp(&other.mapping.ptr)
            }
        }
        impl core::hash::Hash for CoreQueueWriteBuffer {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.mapping.ptr.hash(state)
            }
        }
        impl PartialEq for CoreBufferMappedRange {
            fn eq(&self, other: &Self) -> bool {
                self.ptr == other.ptr
            }
        }
        impl Eq for CoreBufferMappedRange {}
        impl PartialOrd for CoreBufferMappedRange {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for CoreBufferMappedRange {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.ptr.cmp(&other.ptr)
            }
        }
        impl core::hash::Hash for CoreBufferMappedRange {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.ptr.hash(state)
            }
        }
        impl dispatch::InstanceInterface for ContextWgpuCore {
            fn new(desc: &wgt::InstanceDescriptor) -> Self
            where
                Self: Sized,
            {
                Self(Arc::new(wgc::global::Global::new("wgpu", desc)))
            }
            unsafe fn create_surface(
                &self,
                target: crate::api::SurfaceTargetUnsafe,
            ) -> Result<dispatch::DispatchSurface, crate::CreateSurfaceError> {
                let id = match target {
                    SurfaceTargetUnsafe::RawHandle {
                        raw_display_handle,
                        raw_window_handle,
                    } => {
                        unsafe {
                            self.0
                                .instance_create_surface(
                                    raw_display_handle,
                                    raw_window_handle,
                                    None,
                                )
                        }
                    }
                    SurfaceTargetUnsafe::CoreAnimationLayer(layer) => {
                        unsafe { self.0.instance_create_surface_metal(layer, None) }
                    }
                }?;
                Ok(
                    CoreSurface {
                        context: self.clone(),
                        id,
                        configured_device: Mutex::default(),
                        error_sink: Mutex::default(),
                    }
                        .into(),
                )
            }
            fn request_adapter(
                &self,
                options: &crate::api::RequestAdapterOptions<'_, '_>,
            ) -> Pin<Box<dyn dispatch::RequestAdapterFuture>> {
                let id = self
                    .0
                    .request_adapter(
                        &wgc::instance::RequestAdapterOptions {
                            power_preference: options.power_preference,
                            force_fallback_adapter: options.force_fallback_adapter,
                            compatible_surface: options
                                .compatible_surface
                                .map(|surface| surface.inner.as_core().id),
                        },
                        wgt::Backends::all(),
                        None,
                    );
                let adapter = id
                    .map(|id| {
                        let core = CoreAdapter {
                            context: self.clone(),
                            id,
                        };
                        let generic: dispatch::DispatchAdapter = core.into();
                        generic
                    });
                Box::pin(ready(adapter))
            }
            fn poll_all_devices(&self, force_wait: bool) -> bool {
                match self.0.poll_all_devices(force_wait) {
                    Ok(all_queue_empty) => all_queue_empty,
                    Err(err) => {
                        self.handle_error_fatal(err, "Instance::poll_all_devices")
                    }
                }
            }
            fn wgsl_language_features(&self) -> crate::WgslLanguageFeatures {
                use wgc::naga::front::wgsl::ImplementedLanguageExtension;
                ImplementedLanguageExtension::all()
                    .iter()
                    .copied()
                    .fold(
                        crate::WgslLanguageFeatures::empty(),
                        |acc, wle| {
                            acc
                                | match wle {
                                    ImplementedLanguageExtension::ReadOnlyAndReadWriteStorageTextures => {
                                        crate::WgslLanguageFeatures::ReadOnlyAndReadWriteStorageTextures
                                    }
                                    ImplementedLanguageExtension::Packed4x8IntegerDotProduct => {
                                        crate::WgslLanguageFeatures::Packed4x8IntegerDotProduct
                                    }
                                    ImplementedLanguageExtension::PointerCompositeAccess => {
                                        crate::WgslLanguageFeatures::PointerCompositeAccess
                                    }
                                }
                        },
                    )
            }
            fn enumerate_adapters(
                &self,
                backends: crate::Backends,
            ) -> Pin<Box<dyn dispatch::EnumerateAdapterFuture>> {
                let adapters: Vec<DispatchAdapter> = self
                    .enumerate_adapters(backends)
                    .into_iter()
                    .map(|adapter| {
                        let core = crate::backend::wgpu_core::CoreAdapter {
                            context: self.clone(),
                            id: adapter,
                        };
                        core.into()
                    })
                    .collect();
                Box::pin(ready(adapters))
            }
        }
        impl dispatch::AdapterInterface for CoreAdapter {
            fn request_device(
                &self,
                desc: &crate::DeviceDescriptor<'_>,
            ) -> Pin<Box<dyn dispatch::RequestDeviceFuture>> {
                if !#[allow(non_exhaustive_omitted_patterns)]
                match desc.trace {
                    wgt::Trace::Off => true,
                    _ => false,
                } {
                    {
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    { ::log::__private_api::GlobalLogger },
                                    format_args!(
                                        "\n                Feature \'trace\' has been removed temporarily; see https://github.com/gfx-rs/wgpu/issues/5974. The `trace` parameter will have no effect.",
                                    ),
                                    lvl,
                                    &(
                                        "wgpu::backend::wgpu_core",
                                        "wgpu::backend::wgpu_core",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                    };
                }
                let res = self
                    .context
                    .0
                    .adapter_request_device(
                        self.id,
                        &desc.map_label(|l| l.map(Borrowed)),
                        None,
                        None,
                    );
                let (device_id, queue_id) = match res {
                    Ok(ids) => ids,
                    Err(err) => {
                        return Box::pin(ready(Err(err.into())));
                    }
                };
                let error_sink = Arc::new(Mutex::new(ErrorSinkRaw::new()));
                let device = CoreDevice {
                    context: self.context.clone(),
                    id: device_id,
                    error_sink: error_sink.clone(),
                    features: desc.required_features,
                };
                let queue = CoreQueue {
                    context: self.context.clone(),
                    id: queue_id,
                    error_sink,
                };
                Box::pin(ready(Ok((device.into(), queue.into()))))
            }
            fn is_surface_supported(&self, surface: &dispatch::DispatchSurface) -> bool {
                let surface = surface.as_core();
                self.context.0.adapter_is_surface_supported(self.id, surface.id)
            }
            fn features(&self) -> crate::Features {
                self.context.0.adapter_features(self.id)
            }
            fn limits(&self) -> crate::Limits {
                self.context.0.adapter_limits(self.id)
            }
            fn downlevel_capabilities(&self) -> crate::DownlevelCapabilities {
                self.context.0.adapter_downlevel_capabilities(self.id)
            }
            fn get_info(&self) -> crate::AdapterInfo {
                self.context.0.adapter_get_info(self.id)
            }
            fn get_texture_format_features(
                &self,
                format: crate::TextureFormat,
            ) -> crate::TextureFormatFeatures {
                self.context.0.adapter_get_texture_format_features(self.id, format)
            }
            fn get_presentation_timestamp(&self) -> crate::PresentationTimestamp {
                self.context.0.adapter_get_presentation_timestamp(self.id)
            }
        }
        impl Drop for CoreAdapter {
            fn drop(&mut self) {
                self.context.0.adapter_drop(self.id)
            }
        }
        impl dispatch::DeviceInterface for CoreDevice {
            fn features(&self) -> crate::Features {
                self.context.0.device_features(self.id)
            }
            fn limits(&self) -> crate::Limits {
                self.context.0.device_limits(self.id)
            }
            fn create_shader_module(
                &self,
                desc: crate::ShaderModuleDescriptor<'_>,
                shader_bound_checks: wgt::ShaderRuntimeChecks,
            ) -> dispatch::DispatchShaderModule {
                let descriptor = wgc::pipeline::ShaderModuleDescriptor {
                    label: desc.label.map(Borrowed),
                    runtime_checks: shader_bound_checks,
                };
                let source = match desc.source {
                    ShaderSource::Wgsl(ref code) => {
                        wgc::pipeline::ShaderModuleSource::Wgsl(Borrowed(code))
                    }
                    ShaderSource::Dummy(_) => {
                        ::core::panicking::panic_fmt(
                            format_args!("found `ShaderSource::Dummy`"),
                        );
                    }
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_shader_module(self.id, &descriptor, source, None);
                let compilation_info = match error {
                    Some(cause) => {
                        self.context
                            .handle_error(
                                &self.error_sink,
                                cause.clone(),
                                desc.label,
                                "Device::create_shader_module",
                            );
                        CompilationInfo::from(cause)
                    }
                    None => {
                        CompilationInfo {
                            messages: ::alloc::vec::Vec::new(),
                        }
                    }
                };
                CoreShaderModule {
                    context: self.context.clone(),
                    id,
                    compilation_info,
                }
                    .into()
            }
            unsafe fn create_shader_module_passthrough(
                &self,
                desc: &crate::ShaderModuleDescriptorPassthrough<'_>,
            ) -> dispatch::DispatchShaderModule {
                let desc = desc.map_label(|l| l.map(Cow::from));
                let (id, error) = unsafe {
                    self.context
                        .0
                        .device_create_shader_module_passthrough(self.id, &desc, None)
                };
                let compilation_info = match error {
                    Some(cause) => {
                        self.context
                            .handle_error(
                                &self.error_sink,
                                cause.clone(),
                                desc.label.as_deref(),
                                "Device::create_shader_module_passthrough",
                            );
                        CompilationInfo::from(cause)
                    }
                    None => {
                        CompilationInfo {
                            messages: ::alloc::vec::Vec::new(),
                        }
                    }
                };
                CoreShaderModule {
                    context: self.context.clone(),
                    id,
                    compilation_info,
                }
                    .into()
            }
            fn create_bind_group_layout(
                &self,
                desc: &crate::BindGroupLayoutDescriptor<'_>,
            ) -> dispatch::DispatchBindGroupLayout {
                let descriptor = wgc::binding_model::BindGroupLayoutDescriptor {
                    label: desc.label.map(Borrowed),
                    entries: Borrowed(desc.entries),
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_bind_group_layout(self.id, &descriptor, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_bind_group_layout",
                        );
                }
                CoreBindGroupLayout {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_bind_group(
                &self,
                desc: &crate::BindGroupDescriptor<'_>,
            ) -> dispatch::DispatchBindGroup {
                use wgc::binding_model as bm;
                let mut arrayed_texture_views = Vec::new();
                let mut arrayed_samplers = Vec::new();
                if self.features.contains(Features::TEXTURE_BINDING_ARRAY) {
                    for entry in desc.entries.iter() {
                        if let BindingResource::TextureViewArray(array) = entry.resource
                        {
                            arrayed_texture_views
                                .extend(array.iter().map(|view| view.inner.as_core().id));
                        }
                        if let BindingResource::SamplerArray(array) = entry.resource {
                            arrayed_samplers
                                .extend(
                                    array.iter().map(|sampler| sampler.inner.as_core().id),
                                );
                        }
                    }
                }
                let mut remaining_arrayed_texture_views = &arrayed_texture_views[..];
                let mut remaining_arrayed_samplers = &arrayed_samplers[..];
                let mut arrayed_buffer_bindings = Vec::new();
                if self.features.contains(Features::BUFFER_BINDING_ARRAY) {
                    for entry in desc.entries.iter() {
                        if let BindingResource::BufferArray(array) = entry.resource {
                            arrayed_buffer_bindings
                                .extend(
                                    array
                                        .iter()
                                        .map(|binding| bm::BufferBinding {
                                            buffer: binding.buffer.inner.as_core().id,
                                            offset: binding.offset,
                                            size: binding.size,
                                        }),
                                );
                        }
                    }
                }
                let mut remaining_arrayed_buffer_bindings = &arrayed_buffer_bindings[..];
                let entries = desc
                    .entries
                    .iter()
                    .map(|entry| bm::BindGroupEntry {
                        binding: entry.binding,
                        resource: match entry.resource {
                            BindingResource::Buffer(
                                BufferBinding { buffer, offset, size },
                            ) => {
                                bm::BindingResource::Buffer(bm::BufferBinding {
                                    buffer: buffer.inner.as_core().id,
                                    offset,
                                    size,
                                })
                            }
                            BindingResource::BufferArray(array) => {
                                let slice = &remaining_arrayed_buffer_bindings[..array
                                    .len()];
                                remaining_arrayed_buffer_bindings = &remaining_arrayed_buffer_bindings[array
                                    .len()..];
                                bm::BindingResource::BufferArray(Borrowed(slice))
                            }
                            BindingResource::Sampler(sampler) => {
                                bm::BindingResource::Sampler(sampler.inner.as_core().id)
                            }
                            BindingResource::SamplerArray(array) => {
                                let slice = &remaining_arrayed_samplers[..array.len()];
                                remaining_arrayed_samplers = &remaining_arrayed_samplers[array
                                    .len()..];
                                bm::BindingResource::SamplerArray(Borrowed(slice))
                            }
                            BindingResource::TextureView(texture_view) => {
                                bm::BindingResource::TextureView(
                                    texture_view.inner.as_core().id,
                                )
                            }
                            BindingResource::TextureViewArray(array) => {
                                let slice = &remaining_arrayed_texture_views[..array.len()];
                                remaining_arrayed_texture_views = &remaining_arrayed_texture_views[array
                                    .len()..];
                                bm::BindingResource::TextureViewArray(Borrowed(slice))
                            }
                            BindingResource::AccelerationStructure(
                                acceleration_structure,
                            ) => {
                                bm::BindingResource::AccelerationStructure(
                                    acceleration_structure.inner.as_core().id,
                                )
                            }
                            BindingResource::ExternalTexture(external_texture) => {
                                bm::BindingResource::ExternalTexture(
                                    external_texture.inner.as_core().id,
                                )
                            }
                        },
                    })
                    .collect::<Vec<_>>();
                let descriptor = bm::BindGroupDescriptor {
                    label: desc.label.as_ref().map(|label| Borrowed(&label[..])),
                    layout: desc.layout.inner.as_core().id,
                    entries: Borrowed(&entries),
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_bind_group(self.id, &descriptor, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_bind_group",
                        );
                }
                CoreBindGroup {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_pipeline_layout(
                &self,
                desc: &crate::PipelineLayoutDescriptor<'_>,
            ) -> dispatch::DispatchPipelineLayout {
                if !(desc.bind_group_layouts.len() <= wgc::MAX_BIND_GROUPS) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Bind group layout count {0} exceeds device bind group limit {1}",
                                desc.bind_group_layouts.len(),
                                wgc::MAX_BIND_GROUPS,
                            ),
                        );
                    }
                }
                let temp_layouts = desc
                    .bind_group_layouts
                    .iter()
                    .map(|bgl| bgl.inner.as_core().id)
                    .collect::<ArrayVec<_, { wgc::MAX_BIND_GROUPS }>>();
                let descriptor = wgc::binding_model::PipelineLayoutDescriptor {
                    label: desc.label.map(Borrowed),
                    bind_group_layouts: Borrowed(&temp_layouts),
                    push_constant_ranges: Borrowed(desc.push_constant_ranges),
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_pipeline_layout(self.id, &descriptor, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_pipeline_layout",
                        );
                }
                CorePipelineLayout {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_render_pipeline(
                &self,
                desc: &crate::RenderPipelineDescriptor<'_>,
            ) -> dispatch::DispatchRenderPipeline {
                use wgc::pipeline as pipe;
                let vertex_buffers: ArrayVec<_, { wgc::MAX_VERTEX_BUFFERS }> = desc
                    .vertex
                    .buffers
                    .iter()
                    .map(|vbuf| pipe::VertexBufferLayout {
                        array_stride: vbuf.array_stride,
                        step_mode: vbuf.step_mode,
                        attributes: Borrowed(vbuf.attributes),
                    })
                    .collect();
                let vert_constants = desc
                    .vertex
                    .compilation_options
                    .constants
                    .iter()
                    .map(|&(key, value)| (String::from(key), value))
                    .collect();
                let descriptor = pipe::RenderPipelineDescriptor {
                    label: desc.label.map(Borrowed),
                    layout: desc.layout.map(|layout| layout.inner.as_core().id),
                    vertex: pipe::VertexState {
                        stage: pipe::ProgrammableStageDescriptor {
                            module: desc.vertex.module.inner.as_core().id,
                            entry_point: desc.vertex.entry_point.map(Borrowed),
                            constants: vert_constants,
                            zero_initialize_workgroup_memory: desc
                                .vertex
                                .compilation_options
                                .zero_initialize_workgroup_memory,
                        },
                        buffers: Borrowed(&vertex_buffers),
                    },
                    primitive: desc.primitive,
                    depth_stencil: desc.depth_stencil.clone(),
                    multisample: desc.multisample,
                    fragment: desc
                        .fragment
                        .as_ref()
                        .map(|frag| {
                            let frag_constants = frag
                                .compilation_options
                                .constants
                                .iter()
                                .map(|&(key, value)| (String::from(key), value))
                                .collect();
                            pipe::FragmentState {
                                stage: pipe::ProgrammableStageDescriptor {
                                    module: frag.module.inner.as_core().id,
                                    entry_point: frag.entry_point.map(Borrowed),
                                    constants: frag_constants,
                                    zero_initialize_workgroup_memory: frag
                                        .compilation_options
                                        .zero_initialize_workgroup_memory,
                                },
                                targets: Borrowed(frag.targets),
                            }
                        }),
                    multiview: desc.multiview,
                    cache: desc.cache.map(|cache| cache.inner.as_core().id),
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_render_pipeline(self.id, &descriptor, None);
                if let Some(cause) = error {
                    if let wgc::pipeline::CreateRenderPipelineError::Internal {
                        stage,
                        ref error,
                    } = cause {
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Shader translation error for stage {0:?}: {1}",
                                            stage,
                                            error,
                                        ),
                                        lvl,
                                        &(
                                            "wgpu::backend::wgpu_core",
                                            "wgpu::backend::wgpu_core",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Please report it to https://github.com/gfx-rs/wgpu",
                                        ),
                                        lvl,
                                        &(
                                            "wgpu::backend::wgpu_core",
                                            "wgpu::backend::wgpu_core",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                    }
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_render_pipeline",
                        );
                }
                CoreRenderPipeline {
                    context: self.context.clone(),
                    id,
                    error_sink: Arc::clone(&self.error_sink),
                }
                    .into()
            }
            fn create_mesh_pipeline(
                &self,
                desc: &crate::MeshPipelineDescriptor<'_>,
            ) -> dispatch::DispatchRenderPipeline {
                use wgc::pipeline as pipe;
                let mesh_constants = desc
                    .mesh
                    .compilation_options
                    .constants
                    .iter()
                    .map(|&(key, value)| (String::from(key), value))
                    .collect();
                let descriptor = pipe::MeshPipelineDescriptor {
                    label: desc.label.map(Borrowed),
                    task: desc
                        .task
                        .as_ref()
                        .map(|task| {
                            let task_constants = task
                                .compilation_options
                                .constants
                                .iter()
                                .map(|&(key, value)| (String::from(key), value))
                                .collect();
                            pipe::TaskState {
                                stage: pipe::ProgrammableStageDescriptor {
                                    module: task.module.inner.as_core().id,
                                    entry_point: task.entry_point.map(Borrowed),
                                    constants: task_constants,
                                    zero_initialize_workgroup_memory: desc
                                        .mesh
                                        .compilation_options
                                        .zero_initialize_workgroup_memory,
                                },
                            }
                        }),
                    mesh: pipe::MeshState {
                        stage: pipe::ProgrammableStageDescriptor {
                            module: desc.mesh.module.inner.as_core().id,
                            entry_point: desc.mesh.entry_point.map(Borrowed),
                            constants: mesh_constants,
                            zero_initialize_workgroup_memory: desc
                                .mesh
                                .compilation_options
                                .zero_initialize_workgroup_memory,
                        },
                    },
                    layout: desc.layout.map(|layout| layout.inner.as_core().id),
                    primitive: desc.primitive,
                    depth_stencil: desc.depth_stencil.clone(),
                    multisample: desc.multisample,
                    fragment: desc
                        .fragment
                        .as_ref()
                        .map(|frag| {
                            let frag_constants = frag
                                .compilation_options
                                .constants
                                .iter()
                                .map(|&(key, value)| (String::from(key), value))
                                .collect();
                            pipe::FragmentState {
                                stage: pipe::ProgrammableStageDescriptor {
                                    module: frag.module.inner.as_core().id,
                                    entry_point: frag.entry_point.map(Borrowed),
                                    constants: frag_constants,
                                    zero_initialize_workgroup_memory: frag
                                        .compilation_options
                                        .zero_initialize_workgroup_memory,
                                },
                                targets: Borrowed(frag.targets),
                            }
                        }),
                    multiview: desc.multiview,
                    cache: desc.cache.map(|cache| cache.inner.as_core().id),
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_mesh_pipeline(self.id, &descriptor, None);
                if let Some(cause) = error {
                    if let wgc::pipeline::CreateRenderPipelineError::Internal {
                        stage,
                        ref error,
                    } = cause {
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Shader translation error for stage {0:?}: {1}",
                                            stage,
                                            error,
                                        ),
                                        lvl,
                                        &(
                                            "wgpu::backend::wgpu_core",
                                            "wgpu::backend::wgpu_core",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Please report it to https://github.com/gfx-rs/wgpu",
                                        ),
                                        lvl,
                                        &(
                                            "wgpu::backend::wgpu_core",
                                            "wgpu::backend::wgpu_core",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                    }
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_render_pipeline",
                        );
                }
                CoreRenderPipeline {
                    context: self.context.clone(),
                    id,
                    error_sink: Arc::clone(&self.error_sink),
                }
                    .into()
            }
            fn create_compute_pipeline(
                &self,
                desc: &crate::ComputePipelineDescriptor<'_>,
            ) -> dispatch::DispatchComputePipeline {
                use wgc::pipeline as pipe;
                let constants = desc
                    .compilation_options
                    .constants
                    .iter()
                    .map(|&(key, value)| (String::from(key), value))
                    .collect();
                let descriptor = pipe::ComputePipelineDescriptor {
                    label: desc.label.map(Borrowed),
                    layout: desc.layout.map(|pll| pll.inner.as_core().id),
                    stage: pipe::ProgrammableStageDescriptor {
                        module: desc.module.inner.as_core().id,
                        entry_point: desc.entry_point.map(Borrowed),
                        constants,
                        zero_initialize_workgroup_memory: desc
                            .compilation_options
                            .zero_initialize_workgroup_memory,
                    },
                    cache: desc.cache.map(|cache| cache.inner.as_core().id),
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_compute_pipeline(self.id, &descriptor, None);
                if let Some(cause) = error {
                    if let wgc::pipeline::CreateComputePipelineError::Internal(
                        ref error,
                    ) = cause {
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Shader translation error for stage {0:?}: {1}",
                                            wgt::ShaderStages::COMPUTE,
                                            error,
                                        ),
                                        lvl,
                                        &(
                                            "wgpu::backend::wgpu_core",
                                            "wgpu::backend::wgpu_core",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                        {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        { ::log::__private_api::GlobalLogger },
                                        format_args!(
                                            "Please report it to https://github.com/gfx-rs/wgpu",
                                        ),
                                        lvl,
                                        &(
                                            "wgpu::backend::wgpu_core",
                                            "wgpu::backend::wgpu_core",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            }
                        };
                    }
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_compute_pipeline",
                        );
                }
                CoreComputePipeline {
                    context: self.context.clone(),
                    id,
                    error_sink: Arc::clone(&self.error_sink),
                }
                    .into()
            }
            unsafe fn create_pipeline_cache(
                &self,
                desc: &crate::PipelineCacheDescriptor<'_>,
            ) -> dispatch::DispatchPipelineCache {
                use wgc::pipeline as pipe;
                let descriptor = pipe::PipelineCacheDescriptor {
                    label: desc.label.map(Borrowed),
                    data: desc.data.map(Borrowed),
                    fallback: desc.fallback,
                };
                let (id, error) = unsafe {
                    self.context
                        .0
                        .device_create_pipeline_cache(self.id, &descriptor, None)
                };
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::device_create_pipeline_cache_init",
                        );
                }
                CorePipelineCache {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_buffer(
                &self,
                desc: &crate::BufferDescriptor<'_>,
            ) -> dispatch::DispatchBuffer {
                let (id, error) = self
                    .context
                    .0
                    .device_create_buffer(
                        self.id,
                        &desc.map_label(|l| l.map(Borrowed)),
                        None,
                    );
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_buffer",
                        );
                }
                CoreBuffer {
                    context: self.context.clone(),
                    id,
                    error_sink: Arc::clone(&self.error_sink),
                }
                    .into()
            }
            fn create_texture(
                &self,
                desc: &crate::TextureDescriptor<'_>,
            ) -> dispatch::DispatchTexture {
                let wgt_desc = desc
                    .map_label_and_view_formats(|l| l.map(Borrowed), |v| v.to_vec());
                let (id, error) = self
                    .context
                    .0
                    .device_create_texture(self.id, &wgt_desc, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_texture",
                        );
                }
                CoreTexture {
                    context: self.context.clone(),
                    id,
                    error_sink: Arc::clone(&self.error_sink),
                }
                    .into()
            }
            fn create_external_texture(
                &self,
                desc: &crate::ExternalTextureDescriptor<'_>,
                planes: &[&crate::TextureView],
            ) -> dispatch::DispatchExternalTexture {
                let wgt_desc = desc.map_label(|l| l.map(Borrowed));
                let planes = planes
                    .iter()
                    .map(|plane| plane.inner.as_core().id)
                    .collect::<Vec<_>>();
                let (id, error) = self
                    .context
                    .0
                    .device_create_external_texture(self.id, &wgt_desc, &planes, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_external_texture",
                        );
                }
                CoreExternalTexture {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_blas(
                &self,
                desc: &crate::CreateBlasDescriptor<'_>,
                sizes: crate::BlasGeometrySizeDescriptors,
            ) -> (Option<u64>, dispatch::DispatchBlas) {
                let global = &self.context.0;
                let (id, handle, error) = global
                    .device_create_blas(
                        self.id,
                        &desc.map_label(|l| l.map(Borrowed)),
                        sizes,
                        None,
                    );
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_blas",
                        );
                }
                (
                    handle,
                    CoreBlas {
                        context: self.context.clone(),
                        id,
                        error_sink: Arc::clone(&self.error_sink),
                    }
                        .into(),
                )
            }
            fn create_tlas(
                &self,
                desc: &crate::CreateTlasDescriptor<'_>,
            ) -> dispatch::DispatchTlas {
                let global = &self.context.0;
                let (id, error) = global
                    .device_create_tlas(
                        self.id,
                        &desc.map_label(|l| l.map(Borrowed)),
                        None,
                    );
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_tlas",
                        );
                }
                CoreTlas {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_sampler(
                &self,
                desc: &crate::SamplerDescriptor<'_>,
            ) -> dispatch::DispatchSampler {
                let descriptor = wgc::resource::SamplerDescriptor {
                    label: desc.label.map(Borrowed),
                    address_modes: [
                        desc.address_mode_u,
                        desc.address_mode_v,
                        desc.address_mode_w,
                    ],
                    mag_filter: desc.mag_filter,
                    min_filter: desc.min_filter,
                    mipmap_filter: desc.mipmap_filter,
                    lod_min_clamp: desc.lod_min_clamp,
                    lod_max_clamp: desc.lod_max_clamp,
                    compare: desc.compare,
                    anisotropy_clamp: desc.anisotropy_clamp,
                    border_color: desc.border_color,
                };
                let (id, error) = self
                    .context
                    .0
                    .device_create_sampler(self.id, &descriptor, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_sampler",
                        );
                }
                CoreSampler {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_query_set(
                &self,
                desc: &crate::QuerySetDescriptor<'_>,
            ) -> dispatch::DispatchQuerySet {
                let (id, error) = self
                    .context
                    .0
                    .device_create_query_set(
                        self.id,
                        &desc.map_label(|l| l.map(Borrowed)),
                        None,
                    );
                if let Some(cause) = error {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "Device::create_query_set",
                        );
                }
                CoreQuerySet {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn create_command_encoder(
                &self,
                desc: &crate::CommandEncoderDescriptor<'_>,
            ) -> dispatch::DispatchCommandEncoder {
                let (id, error) = self
                    .context
                    .0
                    .device_create_command_encoder(
                        self.id,
                        &desc.map_label(|l| l.map(Borrowed)),
                        None,
                    );
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Device::create_command_encoder",
                        );
                }
                CoreCommandEncoder {
                    context: self.context.clone(),
                    id,
                    error_sink: Arc::clone(&self.error_sink),
                }
                    .into()
            }
            fn create_render_bundle_encoder(
                &self,
                desc: &crate::RenderBundleEncoderDescriptor<'_>,
            ) -> dispatch::DispatchRenderBundleEncoder {
                let descriptor = wgc::command::RenderBundleEncoderDescriptor {
                    label: desc.label.map(Borrowed),
                    color_formats: Borrowed(desc.color_formats),
                    depth_stencil: desc.depth_stencil,
                    sample_count: desc.sample_count,
                    multiview: desc.multiview,
                };
                let encoder = match wgc::command::RenderBundleEncoder::new(
                    &descriptor,
                    self.id,
                    None,
                ) {
                    Ok(encoder) => encoder,
                    Err(e) => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Error in Device::create_render_bundle_encoder: {0}",
                                e,
                            ),
                        );
                    }
                };
                CoreRenderBundleEncoder {
                    context: self.context.clone(),
                    encoder,
                    id: crate::cmp::Identifier::create(),
                }
                    .into()
            }
            fn set_device_lost_callback(
                &self,
                device_lost_callback: dispatch::BoxDeviceLostCallback,
            ) {
                self.context
                    .0
                    .device_set_device_lost_closure(self.id, device_lost_callback);
            }
            fn on_uncaptured_error(
                &self,
                handler: Arc<dyn crate::UncapturedErrorHandler>,
            ) {
                let mut error_sink = self.error_sink.lock();
                error_sink.uncaptured_handler = Some(handler);
            }
            fn push_error_scope(&self, filter: crate::ErrorFilter) {
                let mut error_sink = self.error_sink.lock();
                error_sink.scopes.push(ErrorScope { error: None, filter });
            }
            fn pop_error_scope(&self) -> Pin<Box<dyn dispatch::PopErrorScopeFuture>> {
                let mut error_sink = self.error_sink.lock();
                let scope = error_sink.scopes.pop().unwrap();
                Box::pin(ready(scope.error))
            }
            unsafe fn start_graphics_debugger_capture(&self) {
                unsafe {
                    self.context.0.device_start_graphics_debugger_capture(self.id)
                };
            }
            unsafe fn stop_graphics_debugger_capture(&self) {
                unsafe { self.context.0.device_stop_graphics_debugger_capture(self.id) };
            }
            fn poll(
                &self,
                poll_type: wgt::PollType<u64>,
            ) -> Result<crate::PollStatus, crate::PollError> {
                match self.context.0.device_poll(self.id, poll_type) {
                    Ok(status) => Ok(status),
                    Err(err) => {
                        if let Some(poll_error) = err.to_poll_error() {
                            return Err(poll_error);
                        }
                        self.context.handle_error_fatal(err, "Device::poll")
                    }
                }
            }
            fn get_internal_counters(&self) -> crate::InternalCounters {
                self.context.0.device_get_internal_counters(self.id)
            }
            fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
                self.context.0.device_generate_allocator_report(self.id)
            }
            fn destroy(&self) {
                self.context.0.device_destroy(self.id);
            }
        }
        impl Drop for CoreDevice {
            fn drop(&mut self) {
                self.context.0.device_drop(self.id)
            }
        }
        impl dispatch::QueueInterface for CoreQueue {
            fn write_buffer(
                &self,
                buffer: &dispatch::DispatchBuffer,
                offset: crate::BufferAddress,
                data: &[u8],
            ) {
                let buffer = buffer.as_core();
                match self.context.0.queue_write_buffer(self.id, buffer.id, offset, data)
                {
                    Ok(()) => {}
                    Err(err) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                err,
                                "Queue::write_buffer",
                            )
                    }
                }
            }
            fn create_staging_buffer(
                &self,
                size: crate::BufferSize,
            ) -> Option<dispatch::DispatchQueueWriteBuffer> {
                match self.context.0.queue_create_staging_buffer(self.id, size, None) {
                    Ok((buffer_id, ptr)) => {
                        Some(
                            CoreQueueWriteBuffer {
                                buffer_id,
                                mapping: CoreBufferMappedRange {
                                    ptr,
                                    size: size.get() as usize,
                                },
                            }
                                .into(),
                        )
                    }
                    Err(err) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                err,
                                "Queue::write_buffer_with",
                            );
                        None
                    }
                }
            }
            fn validate_write_buffer(
                &self,
                buffer: &dispatch::DispatchBuffer,
                offset: wgt::BufferAddress,
                size: wgt::BufferSize,
            ) -> Option<()> {
                let buffer = buffer.as_core();
                match self
                    .context
                    .0
                    .queue_validate_write_buffer(self.id, buffer.id, offset, size)
                {
                    Ok(()) => Some(()),
                    Err(err) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                err,
                                "Queue::write_buffer_with",
                            );
                        None
                    }
                }
            }
            fn write_staging_buffer(
                &self,
                buffer: &dispatch::DispatchBuffer,
                offset: crate::BufferAddress,
                staging_buffer: &dispatch::DispatchQueueWriteBuffer,
            ) {
                let buffer = buffer.as_core();
                let staging_buffer = staging_buffer.as_core();
                match self
                    .context
                    .0
                    .queue_write_staging_buffer(
                        self.id,
                        buffer.id,
                        offset,
                        staging_buffer.buffer_id,
                    )
                {
                    Ok(()) => {}
                    Err(err) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                err,
                                "Queue::write_buffer_with",
                            );
                    }
                }
            }
            fn write_texture(
                &self,
                texture: crate::TexelCopyTextureInfo<'_>,
                data: &[u8],
                data_layout: crate::TexelCopyBufferLayout,
                size: crate::Extent3d,
            ) {
                match self
                    .context
                    .0
                    .queue_write_texture(
                        self.id,
                        &map_texture_copy_view(texture),
                        data,
                        &data_layout,
                        &size,
                    )
                {
                    Ok(()) => {}
                    Err(err) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                err,
                                "Queue::write_texture",
                            )
                    }
                }
            }
            fn submit(
                &self,
                command_buffers: &mut dyn Iterator<
                    Item = dispatch::DispatchCommandBuffer,
                >,
            ) -> u64 {
                let temp_command_buffers = command_buffers.collect::<SmallVec<[_; 4]>>();
                let command_buffer_ids = temp_command_buffers
                    .iter()
                    .map(|cmdbuf| cmdbuf.as_core().id)
                    .collect::<SmallVec<[_; 4]>>();
                let index = match self
                    .context
                    .0
                    .queue_submit(self.id, &command_buffer_ids)
                {
                    Ok(index) => index,
                    Err((index, err)) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                err,
                                "Queue::submit",
                            );
                        index
                    }
                };
                drop(temp_command_buffers);
                index
            }
            fn get_timestamp_period(&self) -> f32 {
                self.context.0.queue_get_timestamp_period(self.id)
            }
            fn on_submitted_work_done(
                &self,
                callback: dispatch::BoxSubmittedWorkDoneCallback,
            ) {
                self.context.0.queue_on_submitted_work_done(self.id, callback);
            }
            fn compact_blas(
                &self,
                blas: &dispatch::DispatchBlas,
            ) -> (Option<u64>, dispatch::DispatchBlas) {
                let (id, handle, error) = self
                    .context
                    .0
                    .queue_compact_blas(self.id, blas.as_core().id, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "Queue::compact_blas",
                        );
                }
                (
                    handle,
                    CoreBlas {
                        context: self.context.clone(),
                        id,
                        error_sink: Arc::clone(&self.error_sink),
                    }
                        .into(),
                )
            }
        }
        impl Drop for CoreQueue {
            fn drop(&mut self) {
                self.context.0.queue_drop(self.id)
            }
        }
        impl dispatch::ShaderModuleInterface for CoreShaderModule {
            fn get_compilation_info(
                &self,
            ) -> Pin<Box<dyn dispatch::ShaderCompilationInfoFuture>> {
                Box::pin(ready(self.compilation_info.clone()))
            }
        }
        impl Drop for CoreShaderModule {
            fn drop(&mut self) {
                self.context.0.shader_module_drop(self.id)
            }
        }
        impl dispatch::BindGroupLayoutInterface for CoreBindGroupLayout {}
        impl Drop for CoreBindGroupLayout {
            fn drop(&mut self) {
                self.context.0.bind_group_layout_drop(self.id)
            }
        }
        impl dispatch::BindGroupInterface for CoreBindGroup {}
        impl Drop for CoreBindGroup {
            fn drop(&mut self) {
                self.context.0.bind_group_drop(self.id)
            }
        }
        impl dispatch::TextureViewInterface for CoreTextureView {}
        impl Drop for CoreTextureView {
            fn drop(&mut self) {
                let _ = self.context.0.texture_view_drop(self.id);
            }
        }
        impl dispatch::ExternalTextureInterface for CoreExternalTexture {
            fn destroy(&self) {
                self.context.0.external_texture_destroy(self.id);
            }
        }
        impl Drop for CoreExternalTexture {
            fn drop(&mut self) {
                self.context.0.external_texture_drop(self.id);
            }
        }
        impl dispatch::SamplerInterface for CoreSampler {}
        impl Drop for CoreSampler {
            fn drop(&mut self) {
                self.context.0.sampler_drop(self.id)
            }
        }
        impl dispatch::BufferInterface for CoreBuffer {
            fn map_async(
                &self,
                mode: crate::MapMode,
                range: Range<crate::BufferAddress>,
                callback: dispatch::BufferMapCallback,
            ) {
                let operation = wgc::resource::BufferMapOperation {
                    host: match mode {
                        MapMode::Read => wgc::device::HostMap::Read,
                        MapMode::Write => wgc::device::HostMap::Write,
                    },
                    callback: Some(
                        Box::new(|status| {
                            let res = status.map_err(|_| crate::BufferAsyncError);
                            callback(res);
                        }),
                    ),
                };
                match self
                    .context
                    .0
                    .buffer_map_async(
                        self.id,
                        range.start,
                        Some(range.end - range.start),
                        operation,
                    )
                {
                    Ok(_) => {}
                    Err(cause) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                cause,
                                "Buffer::map_async",
                            )
                    }
                }
            }
            fn get_mapped_range(
                &self,
                sub_range: Range<crate::BufferAddress>,
            ) -> dispatch::DispatchBufferMappedRange {
                let size = sub_range.end - sub_range.start;
                match self
                    .context
                    .0
                    .buffer_get_mapped_range(self.id, sub_range.start, Some(size))
                {
                    Ok((ptr, size)) => {
                        CoreBufferMappedRange {
                            ptr,
                            size: size as usize,
                        }
                            .into()
                    }
                    Err(err) => {
                        self.context.handle_error_fatal(err, "Buffer::get_mapped_range")
                    }
                }
            }
            fn unmap(&self) {
                match self.context.0.buffer_unmap(self.id) {
                    Ok(()) => {}
                    Err(cause) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                cause,
                                "Buffer::buffer_unmap",
                            )
                    }
                }
            }
            fn destroy(&self) {
                self.context.0.buffer_destroy(self.id);
            }
        }
        impl Drop for CoreBuffer {
            fn drop(&mut self) {
                self.context.0.buffer_drop(self.id)
            }
        }
        impl dispatch::TextureInterface for CoreTexture {
            fn create_view(
                &self,
                desc: &crate::TextureViewDescriptor<'_>,
            ) -> dispatch::DispatchTextureView {
                let descriptor = wgc::resource::TextureViewDescriptor {
                    label: desc.label.map(Borrowed),
                    format: desc.format,
                    dimension: desc.dimension,
                    usage: desc.usage,
                    range: wgt::ImageSubresourceRange {
                        aspect: desc.aspect,
                        base_mip_level: desc.base_mip_level,
                        mip_level_count: desc.mip_level_count,
                        base_array_layer: desc.base_array_layer,
                        array_layer_count: desc.array_layer_count,
                    },
                };
                let (id, error) = self
                    .context
                    .0
                    .texture_create_view(self.id, &descriptor, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "Texture::create_view",
                        );
                }
                CoreTextureView {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn destroy(&self) {
                self.context.0.texture_destroy(self.id);
            }
        }
        impl Drop for CoreTexture {
            fn drop(&mut self) {
                self.context.0.texture_drop(self.id)
            }
        }
        impl dispatch::BlasInterface for CoreBlas {
            fn prepare_compact_async(&self, callback: BlasCompactCallback) {
                let callback: Option<wgc::resource::BlasCompactCallback> = Some(
                    Box::new(|status: BlasPrepareCompactResult| {
                        let res = status.map_err(|_| crate::BlasAsyncError);
                        callback(res);
                    }),
                );
                match self.context.0.blas_prepare_compact_async(self.id, callback) {
                    Ok(_) => {}
                    Err(cause) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                cause,
                                "Blas::prepare_compact_async",
                            )
                    }
                }
            }
            fn ready_for_compaction(&self) -> bool {
                match self.context.0.ready_for_compaction(self.id) {
                    Ok(ready) => ready,
                    Err(cause) => {
                        self.context
                            .handle_error_nolabel(
                                &self.error_sink,
                                cause,
                                "Blas::ready_for_compaction",
                            );
                        false
                    }
                }
            }
        }
        impl Drop for CoreBlas {
            fn drop(&mut self) {
                self.context.0.blas_drop(self.id)
            }
        }
        impl dispatch::TlasInterface for CoreTlas {}
        impl Drop for CoreTlas {
            fn drop(&mut self) {
                self.context.0.tlas_drop(self.id)
            }
        }
        impl dispatch::QuerySetInterface for CoreQuerySet {}
        impl Drop for CoreQuerySet {
            fn drop(&mut self) {
                self.context.0.query_set_drop(self.id)
            }
        }
        impl dispatch::PipelineLayoutInterface for CorePipelineLayout {}
        impl Drop for CorePipelineLayout {
            fn drop(&mut self) {
                self.context.0.pipeline_layout_drop(self.id)
            }
        }
        impl dispatch::RenderPipelineInterface for CoreRenderPipeline {
            fn get_bind_group_layout(
                &self,
                index: u32,
            ) -> dispatch::DispatchBindGroupLayout {
                let (id, error) = self
                    .context
                    .0
                    .render_pipeline_get_bind_group_layout(self.id, index, None);
                if let Some(err) = error {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            err,
                            "RenderPipeline::get_bind_group_layout",
                        )
                }
                CoreBindGroupLayout {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
        }
        impl Drop for CoreRenderPipeline {
            fn drop(&mut self) {
                self.context.0.render_pipeline_drop(self.id)
            }
        }
        impl dispatch::ComputePipelineInterface for CoreComputePipeline {
            fn get_bind_group_layout(
                &self,
                index: u32,
            ) -> dispatch::DispatchBindGroupLayout {
                let (id, error) = self
                    .context
                    .0
                    .compute_pipeline_get_bind_group_layout(self.id, index, None);
                if let Some(err) = error {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            err,
                            "ComputePipeline::get_bind_group_layout",
                        )
                }
                CoreBindGroupLayout {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
        }
        impl Drop for CoreComputePipeline {
            fn drop(&mut self) {
                self.context.0.compute_pipeline_drop(self.id)
            }
        }
        impl dispatch::PipelineCacheInterface for CorePipelineCache {
            fn get_data(&self) -> Option<Vec<u8>> {
                self.context.0.pipeline_cache_get_data(self.id)
            }
        }
        impl Drop for CorePipelineCache {
            fn drop(&mut self) {
                self.context.0.pipeline_cache_drop(self.id)
            }
        }
        impl dispatch::CommandEncoderInterface for CoreCommandEncoder {
            fn copy_buffer_to_buffer(
                &self,
                source: &dispatch::DispatchBuffer,
                source_offset: crate::BufferAddress,
                destination: &dispatch::DispatchBuffer,
                destination_offset: crate::BufferAddress,
                copy_size: Option<crate::BufferAddress>,
            ) {
                let source = source.as_core();
                let destination = destination.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_copy_buffer_to_buffer(
                        self.id,
                        source.id,
                        source_offset,
                        destination.id,
                        destination_offset,
                        copy_size,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::copy_buffer_to_buffer",
                        );
                }
            }
            fn copy_buffer_to_texture(
                &self,
                source: crate::TexelCopyBufferInfo<'_>,
                destination: crate::TexelCopyTextureInfo<'_>,
                copy_size: crate::Extent3d,
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_copy_buffer_to_texture(
                        self.id,
                        &map_buffer_copy_view(source),
                        &map_texture_copy_view(destination),
                        &copy_size,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::copy_buffer_to_texture",
                        );
                }
            }
            fn copy_texture_to_buffer(
                &self,
                source: crate::TexelCopyTextureInfo<'_>,
                destination: crate::TexelCopyBufferInfo<'_>,
                copy_size: crate::Extent3d,
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_copy_texture_to_buffer(
                        self.id,
                        &map_texture_copy_view(source),
                        &map_buffer_copy_view(destination),
                        &copy_size,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::copy_texture_to_buffer",
                        );
                }
            }
            fn copy_texture_to_texture(
                &self,
                source: crate::TexelCopyTextureInfo<'_>,
                destination: crate::TexelCopyTextureInfo<'_>,
                copy_size: crate::Extent3d,
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_copy_texture_to_texture(
                        self.id,
                        &map_texture_copy_view(source),
                        &map_texture_copy_view(destination),
                        &copy_size,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::copy_texture_to_texture",
                        );
                }
            }
            fn begin_compute_pass(
                &self,
                desc: &crate::ComputePassDescriptor<'_>,
            ) -> dispatch::DispatchComputePass {
                let timestamp_writes = desc
                    .timestamp_writes
                    .as_ref()
                    .map(|tw| wgc::command::PassTimestampWrites {
                        query_set: tw.query_set.inner.as_core().id,
                        beginning_of_pass_write_index: tw.beginning_of_pass_write_index,
                        end_of_pass_write_index: tw.end_of_pass_write_index,
                    });
                let (pass, err) = self
                    .context
                    .0
                    .command_encoder_begin_compute_pass(
                        self.id,
                        &wgc::command::ComputePassDescriptor {
                            label: desc.label.map(Borrowed),
                            timestamp_writes,
                        },
                    );
                if let Some(cause) = err {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "CommandEncoder::begin_compute_pass",
                        );
                }
                CoreComputePass {
                    context: self.context.clone(),
                    pass,
                    error_sink: self.error_sink.clone(),
                    id: crate::cmp::Identifier::create(),
                }
                    .into()
            }
            fn begin_render_pass(
                &self,
                desc: &crate::RenderPassDescriptor<'_>,
            ) -> dispatch::DispatchRenderPass {
                let colors = desc
                    .color_attachments
                    .iter()
                    .map(|ca| {
                        ca.as_ref()
                            .map(|at| wgc::command::RenderPassColorAttachment {
                                view: at.view.inner.as_core().id,
                                depth_slice: at.depth_slice,
                                resolve_target: at
                                    .resolve_target
                                    .map(|view| view.inner.as_core().id),
                                load_op: at.ops.load,
                                store_op: at.ops.store,
                            })
                    })
                    .collect::<Vec<_>>();
                let depth_stencil = desc
                    .depth_stencil_attachment
                    .as_ref()
                    .map(|dsa| {
                        wgc::command::RenderPassDepthStencilAttachment {
                            view: dsa.view.inner.as_core().id,
                            depth: map_pass_channel(dsa.depth_ops.as_ref()),
                            stencil: map_pass_channel(dsa.stencil_ops.as_ref()),
                        }
                    });
                let timestamp_writes = desc
                    .timestamp_writes
                    .as_ref()
                    .map(|tw| wgc::command::PassTimestampWrites {
                        query_set: tw.query_set.inner.as_core().id,
                        beginning_of_pass_write_index: tw.beginning_of_pass_write_index,
                        end_of_pass_write_index: tw.end_of_pass_write_index,
                    });
                let (pass, err) = self
                    .context
                    .0
                    .command_encoder_begin_render_pass(
                        self.id,
                        &wgc::command::RenderPassDescriptor {
                            label: desc.label.map(Borrowed),
                            timestamp_writes: timestamp_writes.as_ref(),
                            color_attachments: Borrowed(&colors),
                            depth_stencil_attachment: depth_stencil.as_ref(),
                            occlusion_query_set: desc
                                .occlusion_query_set
                                .map(|qs| qs.inner.as_core().id),
                        },
                    );
                if let Some(cause) = err {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            desc.label,
                            "CommandEncoder::begin_render_pass",
                        );
                }
                CoreRenderPass {
                    context: self.context.clone(),
                    pass,
                    error_sink: self.error_sink.clone(),
                    id: crate::cmp::Identifier::create(),
                }
                    .into()
            }
            fn finish(&mut self) -> dispatch::DispatchCommandBuffer {
                let descriptor = wgt::CommandBufferDescriptor::default();
                let (id, error) = self
                    .context
                    .0
                    .command_encoder_finish(self.id, &descriptor, None);
                if let Some(cause) = error {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "a CommandEncoder",
                        );
                }
                CoreCommandBuffer {
                    context: self.context.clone(),
                    id,
                }
                    .into()
            }
            fn clear_texture(
                &self,
                texture: &dispatch::DispatchTexture,
                subresource_range: &crate::ImageSubresourceRange,
            ) {
                let texture = texture.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_clear_texture(
                        self.id,
                        texture.id,
                        subresource_range,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::clear_texture",
                        );
                }
            }
            fn clear_buffer(
                &self,
                buffer: &dispatch::DispatchBuffer,
                offset: crate::BufferAddress,
                size: Option<crate::BufferAddress>,
            ) {
                let buffer = buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_clear_buffer(self.id, buffer.id, offset, size)
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::fill_buffer",
                        );
                }
            }
            fn insert_debug_marker(&self, label: &str) {
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_insert_debug_marker(self.id, label)
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::insert_debug_marker",
                        );
                }
            }
            fn push_debug_group(&self, label: &str) {
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_push_debug_group(self.id, label)
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::push_debug_group",
                        );
                }
            }
            fn pop_debug_group(&self) {
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_pop_debug_group(self.id)
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::pop_debug_group",
                        );
                }
            }
            fn write_timestamp(
                &self,
                query_set: &dispatch::DispatchQuerySet,
                query_index: u32,
            ) {
                let query_set = query_set.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_write_timestamp(self.id, query_set.id, query_index)
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::write_timestamp",
                        );
                }
            }
            fn resolve_query_set(
                &self,
                query_set: &dispatch::DispatchQuerySet,
                first_query: u32,
                query_count: u32,
                destination: &dispatch::DispatchBuffer,
                destination_offset: crate::BufferAddress,
            ) {
                let query_set = query_set.as_core();
                let destination = destination.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_resolve_query_set(
                        self.id,
                        query_set.id,
                        first_query,
                        query_count,
                        destination.id,
                        destination_offset,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::resolve_query_set",
                        );
                }
            }
            fn mark_acceleration_structures_built<'a>(
                &self,
                blas: &mut dyn Iterator<Item = &'a Blas>,
                tlas: &mut dyn Iterator<Item = &'a Tlas>,
            ) {
                let blas = blas
                    .map(|b| b.inner.as_core().id)
                    .collect::<SmallVec<[_; 4]>>();
                let tlas = tlas
                    .map(|t| t.inner.as_core().id)
                    .collect::<SmallVec<[_; 4]>>();
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_mark_acceleration_structures_built(
                        self.id,
                        &blas,
                        &tlas,
                    )
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::build_acceleration_structures_unsafe_tlas",
                        );
                }
            }
            fn build_acceleration_structures<'a>(
                &self,
                blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
                tlas: &mut dyn Iterator<Item = &'a crate::Tlas>,
            ) {
                let blas = blas
                    .map(|e: &crate::BlasBuildEntry<'_>| {
                        let geometries = match e.geometry {
                            crate::BlasGeometries::TriangleGeometries(
                                ref triangle_geometries,
                            ) => {
                                let iter = triangle_geometries
                                    .iter()
                                    .map(|tg| {
                                        wgc::ray_tracing::BlasTriangleGeometry {
                                            vertex_buffer: tg.vertex_buffer.inner.as_core().id,
                                            index_buffer: tg
                                                .index_buffer
                                                .map(|buf| buf.inner.as_core().id),
                                            transform_buffer: tg
                                                .transform_buffer
                                                .map(|buf| buf.inner.as_core().id),
                                            size: tg.size,
                                            transform_buffer_offset: tg.transform_buffer_offset,
                                            first_vertex: tg.first_vertex,
                                            vertex_stride: tg.vertex_stride,
                                            first_index: tg.first_index,
                                        }
                                    });
                                wgc::ray_tracing::BlasGeometries::TriangleGeometries(
                                    Box::new(iter),
                                )
                            }
                        };
                        wgc::ray_tracing::BlasBuildEntry {
                            blas_id: e.blas.inner.as_core().id,
                            geometries,
                        }
                    });
                let tlas = tlas
                    .into_iter()
                    .map(|e| {
                        let instances = e
                            .instances
                            .iter()
                            .map(|instance: &Option<crate::TlasInstance>| {
                                instance
                                    .as_ref()
                                    .map(|instance| wgc::ray_tracing::TlasInstance {
                                        blas_id: instance.blas.as_core().id,
                                        transform: &instance.transform,
                                        custom_data: instance.custom_data,
                                        mask: instance.mask,
                                    })
                            });
                        wgc::ray_tracing::TlasPackage {
                            tlas_id: e.inner.as_core().id,
                            instances: Box::new(instances),
                            lowest_unmodified: e.lowest_unmodified,
                        }
                    });
                if let Err(cause) = self
                    .context
                    .0
                    .command_encoder_build_acceleration_structures(self.id, blas, tlas)
                {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::build_acceleration_structures_unsafe_tlas",
                        );
                }
            }
            fn transition_resources<'a>(
                &mut self,
                buffer_transitions: &mut dyn Iterator<
                    Item = wgt::BufferTransition<&'a dispatch::DispatchBuffer>,
                >,
                texture_transitions: &mut dyn Iterator<
                    Item = wgt::TextureTransition<&'a dispatch::DispatchTexture>,
                >,
            ) {
                let result = self
                    .context
                    .0
                    .command_encoder_transition_resources(
                        self.id,
                        buffer_transitions
                            .map(|t| wgt::BufferTransition {
                                buffer: t.buffer.as_core().id,
                                state: t.state,
                            }),
                        texture_transitions
                            .map(|t| wgt::TextureTransition {
                                texture: t.texture.as_core().id,
                                selector: t.selector.clone(),
                                state: t.state,
                            }),
                    );
                if let Err(cause) = result {
                    self.context
                        .handle_error_nolabel(
                            &self.error_sink,
                            cause,
                            "CommandEncoder::transition_resources",
                        );
                }
            }
        }
        impl Drop for CoreCommandEncoder {
            fn drop(&mut self) {
                self.context.0.command_encoder_drop(self.id)
            }
        }
        impl dispatch::CommandBufferInterface for CoreCommandBuffer {}
        impl Drop for CoreCommandBuffer {
            fn drop(&mut self) {
                self.context.0.command_buffer_drop(self.id)
            }
        }
        impl dispatch::ComputePassInterface for CoreComputePass {
            fn set_pipeline(&mut self, pipeline: &dispatch::DispatchComputePipeline) {
                let pipeline = pipeline.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_set_pipeline(&mut self.pass, pipeline.id)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::set_pipeline",
                        );
                }
            }
            fn set_bind_group(
                &mut self,
                index: u32,
                bind_group: Option<&dispatch::DispatchBindGroup>,
                offsets: &[crate::DynamicOffset],
            ) {
                let bg = bind_group.map(|bg| bg.as_core().id);
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_set_bind_group(&mut self.pass, index, bg, offsets)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::set_bind_group",
                        );
                }
            }
            fn set_push_constants(&mut self, offset: u32, data: &[u8]) {
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_set_push_constants(&mut self.pass, offset, data)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::set_push_constant",
                        );
                }
            }
            fn insert_debug_marker(&mut self, label: &str) {
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_insert_debug_marker(&mut self.pass, label, 0)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::insert_debug_marker",
                        );
                }
            }
            fn push_debug_group(&mut self, group_label: &str) {
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_push_debug_group(&mut self.pass, group_label, 0)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::push_debug_group",
                        );
                }
            }
            fn pop_debug_group(&mut self) {
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_pop_debug_group(&mut self.pass)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::pop_debug_group",
                        );
                }
            }
            fn write_timestamp(
                &mut self,
                query_set: &dispatch::DispatchQuerySet,
                query_index: u32,
            ) {
                let query_set = query_set.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_write_timestamp(
                        &mut self.pass,
                        query_set.id,
                        query_index,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::write_timestamp",
                        );
                }
            }
            fn begin_pipeline_statistics_query(
                &mut self,
                query_set: &dispatch::DispatchQuerySet,
                query_index: u32,
            ) {
                let query_set = query_set.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_begin_pipeline_statistics_query(
                        &mut self.pass,
                        query_set.id,
                        query_index,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::begin_pipeline_statistics_query",
                        );
                }
            }
            fn end_pipeline_statistics_query(&mut self) {
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_end_pipeline_statistics_query(&mut self.pass)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::end_pipeline_statistics_query",
                        );
                }
            }
            fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_dispatch_workgroups(&mut self.pass, x, y, z)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::dispatch_workgroups",
                        );
                }
            }
            fn dispatch_workgroups_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .compute_pass_dispatch_workgroups_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::dispatch_workgroups_indirect",
                        );
                }
            }
            fn end(&mut self) {
                if let Err(cause) = self.context.0.compute_pass_end(&mut self.pass) {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "ComputePass::end",
                        );
                }
            }
        }
        impl Drop for CoreComputePass {
            fn drop(&mut self) {
                dispatch::ComputePassInterface::end(self);
            }
        }
        impl dispatch::RenderPassInterface for CoreRenderPass {
            fn set_pipeline(&mut self, pipeline: &dispatch::DispatchRenderPipeline) {
                let pipeline = pipeline.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_pipeline(&mut self.pass, pipeline.id)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_pipeline",
                        );
                }
            }
            fn set_bind_group(
                &mut self,
                index: u32,
                bind_group: Option<&dispatch::DispatchBindGroup>,
                offsets: &[crate::DynamicOffset],
            ) {
                let bg = bind_group.map(|bg| bg.as_core().id);
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_bind_group(&mut self.pass, index, bg, offsets)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_bind_group",
                        );
                }
            }
            fn set_index_buffer(
                &mut self,
                buffer: &dispatch::DispatchBuffer,
                index_format: crate::IndexFormat,
                offset: crate::BufferAddress,
                size: Option<crate::BufferSize>,
            ) {
                let buffer = buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_index_buffer(
                        &mut self.pass,
                        buffer.id,
                        index_format,
                        offset,
                        size,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_index_buffer",
                        );
                }
            }
            fn set_vertex_buffer(
                &mut self,
                slot: u32,
                buffer: &dispatch::DispatchBuffer,
                offset: crate::BufferAddress,
                size: Option<crate::BufferSize>,
            ) {
                let buffer = buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_vertex_buffer(
                        &mut self.pass,
                        slot,
                        buffer.id,
                        offset,
                        size,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_vertex_buffer",
                        );
                }
            }
            fn set_push_constants(
                &mut self,
                stages: crate::ShaderStages,
                offset: u32,
                data: &[u8],
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_push_constants(&mut self.pass, stages, offset, data)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_push_constants",
                        );
                }
            }
            fn set_blend_constant(&mut self, color: crate::Color) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_blend_constant(&mut self.pass, color)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_blend_constant",
                        );
                }
            }
            fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_scissor_rect(&mut self.pass, x, y, width, height)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_scissor_rect",
                        );
                }
            }
            fn set_viewport(
                &mut self,
                x: f32,
                y: f32,
                width: f32,
                height: f32,
                min_depth: f32,
                max_depth: f32,
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_viewport(
                        &mut self.pass,
                        x,
                        y,
                        width,
                        height,
                        min_depth,
                        max_depth,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_viewport",
                        );
                }
            }
            fn set_stencil_reference(&mut self, reference: u32) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_set_stencil_reference(&mut self.pass, reference)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::set_stencil_reference",
                        );
                }
            }
            fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_draw(
                        &mut self.pass,
                        vertices.end - vertices.start,
                        instances.end - instances.start,
                        vertices.start,
                        instances.start,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::draw",
                        );
                }
            }
            fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_draw_indexed(
                        &mut self.pass,
                        indices.end - indices.start,
                        instances.end - instances.start,
                        indices.start,
                        base_vertex,
                        instances.start,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::draw_indexed",
                        );
                }
            }
            fn draw_mesh_tasks(
                &mut self,
                group_count_x: u32,
                group_count_y: u32,
                group_count_z: u32,
            ) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_draw_mesh_tasks(
                        &mut self.pass,
                        group_count_x,
                        group_count_y,
                        group_count_z,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::draw_mesh_tasks",
                        );
                }
            }
            fn draw_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_draw_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::draw_indirect",
                        );
                }
            }
            fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_draw_indexed_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::draw_indexed_indirect",
                        );
                }
            }
            fn draw_mesh_tasks_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_draw_mesh_tasks_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::draw_mesh_tasks_indirect",
                        );
                }
            }
            fn multi_draw_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
                count: u32,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_multi_draw_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                        count,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::multi_draw_indirect",
                        );
                }
            }
            fn multi_draw_indexed_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
                count: u32,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_multi_draw_indexed_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                        count,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::multi_draw_indexed_indirect",
                        );
                }
            }
            fn multi_draw_mesh_tasks_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
                count: u32,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_multi_draw_mesh_tasks_indirect(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                        count,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::multi_draw_mesh_tasks_indirect",
                        );
                }
            }
            fn multi_draw_indirect_count(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
                count_buffer: &dispatch::DispatchBuffer,
                count_buffer_offset: crate::BufferAddress,
                max_count: u32,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                let count_buffer = count_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_multi_draw_indirect_count(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                        count_buffer.id,
                        count_buffer_offset,
                        max_count,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::multi_draw_indirect_count",
                        );
                }
            }
            fn multi_draw_indexed_indirect_count(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
                count_buffer: &dispatch::DispatchBuffer,
                count_buffer_offset: crate::BufferAddress,
                max_count: u32,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                let count_buffer = count_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_multi_draw_indexed_indirect_count(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                        count_buffer.id,
                        count_buffer_offset,
                        max_count,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::multi_draw_indexed_indirect_count",
                        );
                }
            }
            fn multi_draw_mesh_tasks_indirect_count(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
                count_buffer: &dispatch::DispatchBuffer,
                count_buffer_offset: crate::BufferAddress,
                max_count: u32,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                let count_buffer = count_buffer.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_multi_draw_mesh_tasks_indirect_count(
                        &mut self.pass,
                        indirect_buffer.id,
                        indirect_offset,
                        count_buffer.id,
                        count_buffer_offset,
                        max_count,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::multi_draw_mesh_tasks_indirect_count",
                        );
                }
            }
            fn insert_debug_marker(&mut self, label: &str) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_insert_debug_marker(&mut self.pass, label, 0)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::insert_debug_marker",
                        );
                }
            }
            fn push_debug_group(&mut self, group_label: &str) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_push_debug_group(&mut self.pass, group_label, 0)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::push_debug_group",
                        );
                }
            }
            fn pop_debug_group(&mut self) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_pop_debug_group(&mut self.pass)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::pop_debug_group",
                        );
                }
            }
            fn write_timestamp(
                &mut self,
                query_set: &dispatch::DispatchQuerySet,
                query_index: u32,
            ) {
                let query_set = query_set.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_write_timestamp(
                        &mut self.pass,
                        query_set.id,
                        query_index,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::write_timestamp",
                        );
                }
            }
            fn begin_occlusion_query(&mut self, query_index: u32) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_begin_occlusion_query(&mut self.pass, query_index)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::begin_occlusion_query",
                        );
                }
            }
            fn end_occlusion_query(&mut self) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_end_occlusion_query(&mut self.pass)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::end_occlusion_query",
                        );
                }
            }
            fn begin_pipeline_statistics_query(
                &mut self,
                query_set: &dispatch::DispatchQuerySet,
                query_index: u32,
            ) {
                let query_set = query_set.as_core();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_begin_pipeline_statistics_query(
                        &mut self.pass,
                        query_set.id,
                        query_index,
                    )
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::begin_pipeline_statistics_query",
                        );
                }
            }
            fn end_pipeline_statistics_query(&mut self) {
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_end_pipeline_statistics_query(&mut self.pass)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::end_pipeline_statistics_query",
                        );
                }
            }
            fn execute_bundles(
                &mut self,
                render_bundles: &mut dyn Iterator<Item = &dispatch::DispatchRenderBundle>,
            ) {
                let temp_render_bundles = render_bundles
                    .map(|rb| rb.as_core().id)
                    .collect::<SmallVec<[_; 4]>>();
                if let Err(cause) = self
                    .context
                    .0
                    .render_pass_execute_bundles(&mut self.pass, &temp_render_bundles)
                {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::execute_bundles",
                        );
                }
            }
            fn end(&mut self) {
                if let Err(cause) = self.context.0.render_pass_end(&mut self.pass) {
                    self.context
                        .handle_error(
                            &self.error_sink,
                            cause,
                            self.pass.label(),
                            "RenderPass::end",
                        );
                }
            }
        }
        impl Drop for CoreRenderPass {
            fn drop(&mut self) {
                dispatch::RenderPassInterface::end(self);
            }
        }
        impl dispatch::RenderBundleEncoderInterface for CoreRenderBundleEncoder {
            fn set_pipeline(&mut self, pipeline: &dispatch::DispatchRenderPipeline) {
                let pipeline = pipeline.as_core();
                wgpu_render_bundle_set_pipeline(&mut self.encoder, pipeline.id)
            }
            fn set_bind_group(
                &mut self,
                index: u32,
                bind_group: Option<&dispatch::DispatchBindGroup>,
                offsets: &[crate::DynamicOffset],
            ) {
                let bg = bind_group.map(|bg| bg.as_core().id);
                unsafe {
                    wgpu_render_bundle_set_bind_group(
                        &mut self.encoder,
                        index,
                        bg,
                        offsets.as_ptr(),
                        offsets.len(),
                    )
                }
            }
            fn set_index_buffer(
                &mut self,
                buffer: &dispatch::DispatchBuffer,
                index_format: crate::IndexFormat,
                offset: crate::BufferAddress,
                size: Option<crate::BufferSize>,
            ) {
                let buffer = buffer.as_core();
                self.encoder.set_index_buffer(buffer.id, index_format, offset, size)
            }
            fn set_vertex_buffer(
                &mut self,
                slot: u32,
                buffer: &dispatch::DispatchBuffer,
                offset: crate::BufferAddress,
                size: Option<crate::BufferSize>,
            ) {
                let buffer = buffer.as_core();
                wgpu_render_bundle_set_vertex_buffer(
                    &mut self.encoder,
                    slot,
                    buffer.id,
                    offset,
                    size,
                )
            }
            fn set_push_constants(
                &mut self,
                stages: crate::ShaderStages,
                offset: u32,
                data: &[u8],
            ) {
                unsafe {
                    wgpu_render_bundle_set_push_constants(
                        &mut self.encoder,
                        stages,
                        offset,
                        data.len().try_into().unwrap(),
                        data.as_ptr(),
                    )
                }
            }
            fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
                wgpu_render_bundle_draw(
                    &mut self.encoder,
                    vertices.end - vertices.start,
                    instances.end - instances.start,
                    vertices.start,
                    instances.start,
                )
            }
            fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            ) {
                wgpu_render_bundle_draw_indexed(
                    &mut self.encoder,
                    indices.end - indices.start,
                    instances.end - instances.start,
                    indices.start,
                    base_vertex,
                    instances.start,
                )
            }
            fn draw_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                wgpu_render_bundle_draw_indirect(
                    &mut self.encoder,
                    indirect_buffer.id,
                    indirect_offset,
                )
            }
            fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &dispatch::DispatchBuffer,
                indirect_offset: crate::BufferAddress,
            ) {
                let indirect_buffer = indirect_buffer.as_core();
                wgpu_render_bundle_draw_indexed_indirect(
                    &mut self.encoder,
                    indirect_buffer.id,
                    indirect_offset,
                )
            }
            fn finish(
                self,
                desc: &crate::RenderBundleDescriptor<'_>,
            ) -> dispatch::DispatchRenderBundle
            where
                Self: Sized,
            {
                let (id, error) = self
                    .context
                    .0
                    .render_bundle_encoder_finish(
                        self.encoder,
                        &desc.map_label(|l| l.map(Borrowed)),
                        None,
                    );
                if let Some(err) = error {
                    self.context.handle_error_fatal(err, "RenderBundleEncoder::finish");
                }
                CoreRenderBundle { id }.into()
            }
        }
        impl dispatch::RenderBundleInterface for CoreRenderBundle {}
        impl dispatch::SurfaceInterface for CoreSurface {
            fn get_capabilities(
                &self,
                adapter: &dispatch::DispatchAdapter,
            ) -> wgt::SurfaceCapabilities {
                let adapter = adapter.as_core();
                self.context
                    .0
                    .surface_get_capabilities(self.id, adapter.id)
                    .unwrap_or_default()
            }
            fn configure(
                &self,
                device: &dispatch::DispatchDevice,
                config: &crate::SurfaceConfiguration,
            ) {
                let device = device.as_core();
                let error = self.context.0.surface_configure(self.id, device.id, config);
                if let Some(e) = error {
                    self.context
                        .handle_error_nolabel(
                            &device.error_sink,
                            e,
                            "Surface::configure",
                        );
                } else {
                    *self.configured_device.lock() = Some(device.id);
                    *self.error_sink.lock() = Some(device.error_sink.clone());
                }
            }
            fn get_current_texture(
                &self,
            ) -> (
                Option<dispatch::DispatchTexture>,
                crate::SurfaceStatus,
                dispatch::DispatchSurfaceOutputDetail,
            ) {
                let output_detail = CoreSurfaceOutputDetail {
                    context: self.context.clone(),
                    surface_id: self.id,
                }
                    .into();
                match self.context.0.surface_get_current_texture(self.id, None) {
                    Ok(wgc::present::SurfaceOutput { status, texture: texture_id }) => {
                        let data = texture_id
                            .map(|id| CoreTexture {
                                context: self.context.clone(),
                                id,
                                error_sink: Arc::new(Mutex::new(ErrorSinkRaw::new())),
                            })
                            .map(Into::into);
                        (data, status, output_detail)
                    }
                    Err(err) => {
                        let error_sink = self.error_sink.lock();
                        match error_sink.as_ref() {
                            Some(error_sink) => {
                                self.context
                                    .handle_error_nolabel(
                                        error_sink,
                                        err,
                                        "Surface::get_current_texture_view",
                                    );
                                (None, crate::SurfaceStatus::Unknown, output_detail)
                            }
                            None => {
                                self.context
                                    .handle_error_fatal(
                                        err,
                                        "Surface::get_current_texture_view",
                                    )
                            }
                        }
                    }
                }
            }
        }
        impl Drop for CoreSurface {
            fn drop(&mut self) {
                self.context.0.surface_drop(self.id)
            }
        }
        impl dispatch::SurfaceOutputDetailInterface for CoreSurfaceOutputDetail {
            fn present(&self) {
                match self.context.0.surface_present(self.surface_id) {
                    Ok(_status) => {}
                    Err(err) => self.context.handle_error_fatal(err, "Surface::present"),
                }
            }
            fn texture_discard(&self) {
                match self.context.0.surface_texture_discard(self.surface_id) {
                    Ok(_status) => {}
                    Err(err) => {
                        self.context.handle_error_fatal(err, "Surface::discard_texture")
                    }
                }
            }
        }
        impl Drop for CoreSurfaceOutputDetail {
            fn drop(&mut self) {}
        }
        impl dispatch::QueueWriteBufferInterface for CoreQueueWriteBuffer {
            fn slice(&self) -> &[u8] {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
            #[inline]
            fn slice_mut(&mut self) -> &mut [u8] {
                self.mapping.slice_mut()
            }
        }
        impl Drop for CoreQueueWriteBuffer {
            fn drop(&mut self) {}
        }
        impl dispatch::BufferMappedRangeInterface for CoreBufferMappedRange {
            #[inline]
            fn slice(&self) -> &[u8] {
                unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.size) }
            }
            #[inline]
            fn slice_mut(&mut self) -> &mut [u8] {
                unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
            }
        }
    }
    pub(crate) use wgpu_core::ContextWgpuCore;
}
mod cmp {
    //! We need to impl `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` for all handle types in wgpu.
    //!
    //! For types that have some already-unique property, we can use that property to implement these traits.
    //!
    //! For types (like WebGPU) that don't have such a property, we generate an identifier and use that.
    pub use core::sync::atomic::AtomicU64;
    use core::{num::NonZeroU64, sync::atomic::Ordering};
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    pub struct Identifier {
        inner: NonZeroU64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Identifier {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Identifier",
                "inner",
                &&self.inner,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Identifier {
        #[inline]
        fn clone(&self) -> Identifier {
            Identifier {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Identifier {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Identifier {
        #[inline]
        fn eq(&self, other: &Identifier) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Identifier {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<NonZeroU64>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Identifier {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Identifier,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Identifier {
        #[inline]
        fn cmp(&self, other: &Identifier) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.inner, &other.inner)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Identifier {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    impl Identifier {
        pub fn create() -> Self {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let inner = unsafe { NonZeroU64::new_unchecked(id) };
            Self { inner }
        }
    }
    pub(crate) use {impl_eq_ord_hash_arc_address, impl_eq_ord_hash_proxy};
}
mod dispatch {
    //! Infrastructure for dispatching calls to the appropriate "backend". The "backends" are:
    //!
    //! - `wgpu_core`: An implementation of the the wgpu api on top of various native graphics APIs.
    //! - `webgpu`: An implementation of the wgpu api which calls WebGPU directly.
    //!
    //! The interface traits are all object safe and listed in the `InterfaceTypes` trait.
    //!
    //! The method for dispatching should optimize well if only one backend is compiled in,
    //! as-if there was no dispatching at all.
    #![allow(drop_bounds)]
    #![allow(clippy::too_many_arguments)]
    #![allow(missing_docs, clippy::missing_safety_doc)]
    use crate::{Blas, Tlas, WasmNotSend, WasmNotSendSync};
    use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
    use core::{any::Any, fmt::Debug, future::Future, hash::Hash, ops::Range, pin::Pin};
    use crate::backend::wgpu_core::*;
    pub trait RequestAdapterFuture: Future<
            Output = Result<DispatchAdapter, wgt::RequestAdapterError>,
        > + WasmNotSend + 'static {}
    impl<
        T: Future<Output = Result<DispatchAdapter, wgt::RequestAdapterError>>
            + WasmNotSend + 'static,
    > RequestAdapterFuture for T {}
    pub trait RequestDeviceFuture: Future<
            Output = Result<(DispatchDevice, DispatchQueue), crate::RequestDeviceError>,
        > + WasmNotSend + 'static {}
    impl<
        T: Future<
                Output = Result<
                    (DispatchDevice, DispatchQueue),
                    crate::RequestDeviceError,
                >,
            > + WasmNotSend + 'static,
    > RequestDeviceFuture for T {}
    pub trait PopErrorScopeFuture: Future<
            Output = Option<crate::Error>,
        > + WasmNotSend + 'static {}
    impl<
        T: Future<Output = Option<crate::Error>> + WasmNotSend + 'static,
    > PopErrorScopeFuture for T {}
    pub trait ShaderCompilationInfoFuture: Future<
            Output = crate::CompilationInfo,
        > + WasmNotSend + 'static {}
    impl<
        T: Future<Output = crate::CompilationInfo> + WasmNotSend + 'static,
    > ShaderCompilationInfoFuture for T {}
    pub trait EnumerateAdapterFuture: Future<
            Output = Vec<DispatchAdapter>,
        > + WasmNotSend + 'static {}
    impl<
        T: Future<Output = Vec<DispatchAdapter>> + WasmNotSend + 'static,
    > EnumerateAdapterFuture for T {}
    pub type BoxDeviceLostCallback = Box<
        dyn FnOnce(crate::DeviceLostReason, String) + Send + 'static,
    >;
    pub type BoxSubmittedWorkDoneCallback = Box<dyn FnOnce() + Send + 'static>;
    pub type BufferMapCallback = Box<
        dyn FnOnce(Result<(), crate::BufferAsyncError>) + Send + 'static,
    >;
    pub type BlasCompactCallback = Box<
        dyn FnOnce(Result<(), crate::BlasAsyncError>) + Send + 'static,
    >;
    #[expect(dead_code)]
    pub trait AsAny {
        fn as_any(&self) -> &dyn Any;
    }
    impl<T: 'static> AsAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    pub trait CommonTraits: AsAny + Any + Debug + WasmNotSendSync {}
    impl<T: AsAny + Any + Debug + WasmNotSendSync> CommonTraits for T {}
    pub trait InstanceInterface: CommonTraits {
        fn new(desc: &crate::InstanceDescriptor) -> Self
        where
            Self: Sized;
        unsafe fn create_surface(
            &self,
            target: crate::SurfaceTargetUnsafe,
        ) -> Result<DispatchSurface, crate::CreateSurfaceError>;
        fn request_adapter(
            &self,
            options: &crate::RequestAdapterOptions<'_, '_>,
        ) -> Pin<Box<dyn RequestAdapterFuture>>;
        fn poll_all_devices(&self, force_wait: bool) -> bool;
        fn wgsl_language_features(&self) -> crate::WgslLanguageFeatures;
        fn enumerate_adapters(
            &self,
            backends: crate::Backends,
        ) -> Pin<Box<dyn EnumerateAdapterFuture>>;
    }
    pub trait AdapterInterface: CommonTraits {
        fn request_device(
            &self,
            desc: &crate::DeviceDescriptor<'_>,
        ) -> Pin<Box<dyn RequestDeviceFuture>>;
        fn is_surface_supported(&self, surface: &DispatchSurface) -> bool;
        fn features(&self) -> crate::Features;
        fn limits(&self) -> crate::Limits;
        fn downlevel_capabilities(&self) -> crate::DownlevelCapabilities;
        fn get_info(&self) -> crate::AdapterInfo;
        fn get_texture_format_features(
            &self,
            format: crate::TextureFormat,
        ) -> crate::TextureFormatFeatures;
        fn get_presentation_timestamp(&self) -> crate::PresentationTimestamp;
    }
    pub trait DeviceInterface: CommonTraits {
        fn features(&self) -> crate::Features;
        fn limits(&self) -> crate::Limits;
        fn create_shader_module(
            &self,
            desc: crate::ShaderModuleDescriptor<'_>,
            shader_bound_checks: crate::ShaderRuntimeChecks,
        ) -> DispatchShaderModule;
        unsafe fn create_shader_module_passthrough(
            &self,
            desc: &crate::ShaderModuleDescriptorPassthrough<'_>,
        ) -> DispatchShaderModule;
        fn create_bind_group_layout(
            &self,
            desc: &crate::BindGroupLayoutDescriptor<'_>,
        ) -> DispatchBindGroupLayout;
        fn create_bind_group(
            &self,
            desc: &crate::BindGroupDescriptor<'_>,
        ) -> DispatchBindGroup;
        fn create_pipeline_layout(
            &self,
            desc: &crate::PipelineLayoutDescriptor<'_>,
        ) -> DispatchPipelineLayout;
        fn create_render_pipeline(
            &self,
            desc: &crate::RenderPipelineDescriptor<'_>,
        ) -> DispatchRenderPipeline;
        fn create_mesh_pipeline(
            &self,
            desc: &crate::MeshPipelineDescriptor<'_>,
        ) -> DispatchRenderPipeline;
        fn create_compute_pipeline(
            &self,
            desc: &crate::ComputePipelineDescriptor<'_>,
        ) -> DispatchComputePipeline;
        unsafe fn create_pipeline_cache(
            &self,
            desc: &crate::PipelineCacheDescriptor<'_>,
        ) -> DispatchPipelineCache;
        fn create_buffer(&self, desc: &crate::BufferDescriptor<'_>) -> DispatchBuffer;
        fn create_texture(&self, desc: &crate::TextureDescriptor<'_>) -> DispatchTexture;
        fn create_external_texture(
            &self,
            desc: &crate::ExternalTextureDescriptor<'_>,
            planes: &[&crate::TextureView],
        ) -> DispatchExternalTexture;
        fn create_blas(
            &self,
            desc: &crate::CreateBlasDescriptor<'_>,
            sizes: crate::BlasGeometrySizeDescriptors,
        ) -> (Option<u64>, DispatchBlas);
        fn create_tlas(&self, desc: &crate::CreateTlasDescriptor<'_>) -> DispatchTlas;
        fn create_sampler(&self, desc: &crate::SamplerDescriptor<'_>) -> DispatchSampler;
        fn create_query_set(
            &self,
            desc: &crate::QuerySetDescriptor<'_>,
        ) -> DispatchQuerySet;
        fn create_command_encoder(
            &self,
            desc: &crate::CommandEncoderDescriptor<'_>,
        ) -> DispatchCommandEncoder;
        fn create_render_bundle_encoder(
            &self,
            desc: &crate::RenderBundleEncoderDescriptor<'_>,
        ) -> DispatchRenderBundleEncoder;
        fn set_device_lost_callback(&self, device_lost_callback: BoxDeviceLostCallback);
        fn on_uncaptured_error(&self, handler: Arc<dyn crate::UncapturedErrorHandler>);
        fn push_error_scope(&self, filter: crate::ErrorFilter);
        fn pop_error_scope(&self) -> Pin<Box<dyn PopErrorScopeFuture>>;
        unsafe fn start_graphics_debugger_capture(&self);
        unsafe fn stop_graphics_debugger_capture(&self);
        fn poll(
            &self,
            poll_type: wgt::PollType<u64>,
        ) -> Result<crate::PollStatus, crate::PollError>;
        fn get_internal_counters(&self) -> crate::InternalCounters;
        fn generate_allocator_report(&self) -> Option<crate::AllocatorReport>;
        fn destroy(&self);
    }
    pub trait QueueInterface: CommonTraits {
        fn write_buffer(
            &self,
            buffer: &DispatchBuffer,
            offset: crate::BufferAddress,
            data: &[u8],
        );
        fn create_staging_buffer(
            &self,
            size: crate::BufferSize,
        ) -> Option<DispatchQueueWriteBuffer>;
        fn validate_write_buffer(
            &self,
            buffer: &DispatchBuffer,
            offset: crate::BufferAddress,
            size: crate::BufferSize,
        ) -> Option<()>;
        fn write_staging_buffer(
            &self,
            buffer: &DispatchBuffer,
            offset: crate::BufferAddress,
            staging_buffer: &DispatchQueueWriteBuffer,
        );
        fn write_texture(
            &self,
            texture: crate::TexelCopyTextureInfo<'_>,
            data: &[u8],
            data_layout: crate::TexelCopyBufferLayout,
            size: crate::Extent3d,
        );
        /// Submit must always drain the iterator, even in the case of error.
        fn submit(
            &self,
            command_buffers: &mut dyn Iterator<Item = DispatchCommandBuffer>,
        ) -> u64;
        fn get_timestamp_period(&self) -> f32;
        fn on_submitted_work_done(&self, callback: BoxSubmittedWorkDoneCallback);
        fn compact_blas(&self, blas: &DispatchBlas) -> (Option<u64>, DispatchBlas);
    }
    pub trait ShaderModuleInterface: CommonTraits {
        fn get_compilation_info(&self) -> Pin<Box<dyn ShaderCompilationInfoFuture>>;
    }
    pub trait BindGroupLayoutInterface: CommonTraits {}
    pub trait BindGroupInterface: CommonTraits {}
    pub trait TextureViewInterface: CommonTraits {}
    pub trait SamplerInterface: CommonTraits {}
    pub trait BufferInterface: CommonTraits {
        fn map_async(
            &self,
            mode: crate::MapMode,
            range: Range<crate::BufferAddress>,
            callback: BufferMapCallback,
        );
        fn get_mapped_range(
            &self,
            sub_range: Range<crate::BufferAddress>,
        ) -> DispatchBufferMappedRange;
        fn unmap(&self);
        fn destroy(&self);
    }
    pub trait TextureInterface: CommonTraits {
        fn create_view(
            &self,
            desc: &crate::TextureViewDescriptor<'_>,
        ) -> DispatchTextureView;
        fn destroy(&self);
    }
    pub trait ExternalTextureInterface: CommonTraits {
        fn destroy(&self);
    }
    pub trait BlasInterface: CommonTraits {
        fn prepare_compact_async(&self, callback: BlasCompactCallback);
        fn ready_for_compaction(&self) -> bool;
    }
    pub trait TlasInterface: CommonTraits {}
    pub trait QuerySetInterface: CommonTraits {}
    pub trait PipelineLayoutInterface: CommonTraits {}
    pub trait RenderPipelineInterface: CommonTraits {
        fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout;
    }
    pub trait ComputePipelineInterface: CommonTraits {
        fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout;
    }
    pub trait PipelineCacheInterface: CommonTraits {
        fn get_data(&self) -> Option<Vec<u8>>;
    }
    pub trait CommandEncoderInterface: CommonTraits {
        fn copy_buffer_to_buffer(
            &self,
            source: &DispatchBuffer,
            source_offset: crate::BufferAddress,
            destination: &DispatchBuffer,
            destination_offset: crate::BufferAddress,
            copy_size: Option<crate::BufferAddress>,
        );
        fn copy_buffer_to_texture(
            &self,
            source: crate::TexelCopyBufferInfo<'_>,
            destination: crate::TexelCopyTextureInfo<'_>,
            copy_size: crate::Extent3d,
        );
        fn copy_texture_to_buffer(
            &self,
            source: crate::TexelCopyTextureInfo<'_>,
            destination: crate::TexelCopyBufferInfo<'_>,
            copy_size: crate::Extent3d,
        );
        fn copy_texture_to_texture(
            &self,
            source: crate::TexelCopyTextureInfo<'_>,
            destination: crate::TexelCopyTextureInfo<'_>,
            copy_size: crate::Extent3d,
        );
        fn begin_compute_pass(
            &self,
            desc: &crate::ComputePassDescriptor<'_>,
        ) -> DispatchComputePass;
        fn begin_render_pass(
            &self,
            desc: &crate::RenderPassDescriptor<'_>,
        ) -> DispatchRenderPass;
        fn finish(&mut self) -> DispatchCommandBuffer;
        fn clear_texture(
            &self,
            texture: &DispatchTexture,
            subresource_range: &crate::ImageSubresourceRange,
        );
        fn clear_buffer(
            &self,
            buffer: &DispatchBuffer,
            offset: crate::BufferAddress,
            size: Option<crate::BufferAddress>,
        );
        fn insert_debug_marker(&self, label: &str);
        fn push_debug_group(&self, label: &str);
        fn pop_debug_group(&self);
        fn write_timestamp(&self, query_set: &DispatchQuerySet, query_index: u32);
        fn resolve_query_set(
            &self,
            query_set: &DispatchQuerySet,
            first_query: u32,
            query_count: u32,
            destination: &DispatchBuffer,
            destination_offset: crate::BufferAddress,
        );
        fn mark_acceleration_structures_built<'a>(
            &self,
            blas: &mut dyn Iterator<Item = &'a Blas>,
            tlas: &mut dyn Iterator<Item = &'a Tlas>,
        );
        fn build_acceleration_structures<'a>(
            &self,
            blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
            tlas: &mut dyn Iterator<Item = &'a crate::Tlas>,
        );
        fn transition_resources<'a>(
            &mut self,
            buffer_transitions: &mut dyn Iterator<
                Item = wgt::BufferTransition<&'a DispatchBuffer>,
            >,
            texture_transitions: &mut dyn Iterator<
                Item = wgt::TextureTransition<&'a DispatchTexture>,
            >,
        );
    }
    pub trait ComputePassInterface: CommonTraits {
        fn set_pipeline(&mut self, pipeline: &DispatchComputePipeline);
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: Option<&DispatchBindGroup>,
            offsets: &[crate::DynamicOffset],
        );
        fn set_push_constants(&mut self, offset: u32, data: &[u8]);
        fn insert_debug_marker(&mut self, label: &str);
        fn push_debug_group(&mut self, group_label: &str);
        fn pop_debug_group(&mut self);
        fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32);
        fn begin_pipeline_statistics_query(
            &mut self,
            query_set: &DispatchQuerySet,
            query_index: u32,
        );
        fn end_pipeline_statistics_query(&mut self);
        fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32);
        fn dispatch_workgroups_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
        );
        fn end(&mut self);
    }
    pub trait RenderPassInterface: CommonTraits {
        fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline);
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: Option<&DispatchBindGroup>,
            offsets: &[crate::DynamicOffset],
        );
        fn set_index_buffer(
            &mut self,
            buffer: &DispatchBuffer,
            index_format: crate::IndexFormat,
            offset: crate::BufferAddress,
            size: Option<crate::BufferSize>,
        );
        fn set_vertex_buffer(
            &mut self,
            slot: u32,
            buffer: &DispatchBuffer,
            offset: crate::BufferAddress,
            size: Option<crate::BufferSize>,
        );
        fn set_push_constants(
            &mut self,
            stages: crate::ShaderStages,
            offset: u32,
            data: &[u8],
        );
        fn set_blend_constant(&mut self, color: crate::Color);
        fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32);
        fn set_viewport(
            &mut self,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            min_depth: f32,
            max_depth: f32,
        );
        fn set_stencil_reference(&mut self, reference: u32);
        fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
        fn draw_indexed(
            &mut self,
            indices: Range<u32>,
            base_vertex: i32,
            instances: Range<u32>,
        );
        fn draw_mesh_tasks(
            &mut self,
            group_count_x: u32,
            group_count_y: u32,
            group_count_z: u32,
        );
        fn draw_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
        );
        fn draw_indexed_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
        );
        fn draw_mesh_tasks_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
        );
        fn multi_draw_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
            count: u32,
        );
        fn multi_draw_indexed_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
            count: u32,
        );
        fn multi_draw_indirect_count(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
            count_buffer: &DispatchBuffer,
            count_buffer_offset: crate::BufferAddress,
            max_count: u32,
        );
        fn multi_draw_mesh_tasks_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
            count: u32,
        );
        fn multi_draw_indexed_indirect_count(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
            count_buffer: &DispatchBuffer,
            count_buffer_offset: crate::BufferAddress,
            max_count: u32,
        );
        fn multi_draw_mesh_tasks_indirect_count(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
            count_buffer: &DispatchBuffer,
            count_buffer_offset: crate::BufferAddress,
            max_count: u32,
        );
        fn insert_debug_marker(&mut self, label: &str);
        fn push_debug_group(&mut self, group_label: &str);
        fn pop_debug_group(&mut self);
        fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32);
        fn begin_occlusion_query(&mut self, query_index: u32);
        fn end_occlusion_query(&mut self);
        fn begin_pipeline_statistics_query(
            &mut self,
            query_set: &DispatchQuerySet,
            query_index: u32,
        );
        fn end_pipeline_statistics_query(&mut self);
        fn execute_bundles(
            &mut self,
            render_bundles: &mut dyn Iterator<Item = &DispatchRenderBundle>,
        );
        fn end(&mut self);
    }
    pub trait RenderBundleEncoderInterface: CommonTraits {
        fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline);
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: Option<&DispatchBindGroup>,
            offsets: &[crate::DynamicOffset],
        );
        fn set_index_buffer(
            &mut self,
            buffer: &DispatchBuffer,
            index_format: crate::IndexFormat,
            offset: crate::BufferAddress,
            size: Option<crate::BufferSize>,
        );
        fn set_vertex_buffer(
            &mut self,
            slot: u32,
            buffer: &DispatchBuffer,
            offset: crate::BufferAddress,
            size: Option<crate::BufferSize>,
        );
        fn set_push_constants(
            &mut self,
            stages: crate::ShaderStages,
            offset: u32,
            data: &[u8],
        );
        fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
        fn draw_indexed(
            &mut self,
            indices: Range<u32>,
            base_vertex: i32,
            instances: Range<u32>,
        );
        fn draw_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
        );
        fn draw_indexed_indirect(
            &mut self,
            indirect_buffer: &DispatchBuffer,
            indirect_offset: crate::BufferAddress,
        );
        fn finish(self, desc: &crate::RenderBundleDescriptor<'_>) -> DispatchRenderBundle
        where
            Self: Sized;
    }
    pub trait CommandBufferInterface: CommonTraits {}
    pub trait RenderBundleInterface: CommonTraits {}
    pub trait SurfaceInterface: CommonTraits {
        fn get_capabilities(
            &self,
            adapter: &DispatchAdapter,
        ) -> crate::SurfaceCapabilities;
        fn configure(
            &self,
            device: &DispatchDevice,
            config: &crate::SurfaceConfiguration,
        );
        fn get_current_texture(
            &self,
        ) -> (
            Option<DispatchTexture>,
            crate::SurfaceStatus,
            DispatchSurfaceOutputDetail,
        );
    }
    pub trait SurfaceOutputDetailInterface: CommonTraits {
        fn present(&self);
        fn texture_discard(&self);
    }
    pub trait QueueWriteBufferInterface: CommonTraits {
        fn slice(&self) -> &[u8];
        fn slice_mut(&mut self) -> &mut [u8];
    }
    pub trait BufferMappedRangeInterface: CommonTraits {
        fn slice(&self) -> &[u8];
        fn slice_mut(&mut self) -> &mut [u8];
    }
    pub enum DispatchInstance {
        Core(Arc<ContextWgpuCore>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchInstance {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchInstance::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchInstance {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchInstance {
        #[inline]
        fn eq(&self, other: &DispatchInstance) -> bool {
            match (self, other) {
                (DispatchInstance::Core(__self_0), DispatchInstance::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchInstance {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<ContextWgpuCore>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchInstance {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchInstance,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchInstance::Core(__self_0), DispatchInstance::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchInstance {
        #[inline]
        fn cmp(&self, other: &DispatchInstance) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchInstance::Core(__self_0), DispatchInstance::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchInstance {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchInstance::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchInstance {
        #[inline]
        fn clone(&self) -> DispatchInstance {
            match self {
                DispatchInstance::Core(__self_0) => {
                    DispatchInstance::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchInstance {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &ContextWgpuCore {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchInstance is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&ContextWgpuCore> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<ContextWgpuCore> for DispatchInstance {
        #[inline]
        fn from(value: ContextWgpuCore) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchInstance {
        type Target = dyn InstanceInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchAdapter {
        Core(Arc<CoreAdapter>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchAdapter {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchAdapter::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchAdapter {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchAdapter {
        #[inline]
        fn eq(&self, other: &DispatchAdapter) -> bool {
            match (self, other) {
                (DispatchAdapter::Core(__self_0), DispatchAdapter::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchAdapter {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreAdapter>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchAdapter {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchAdapter,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchAdapter::Core(__self_0), DispatchAdapter::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchAdapter {
        #[inline]
        fn cmp(&self, other: &DispatchAdapter) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchAdapter::Core(__self_0), DispatchAdapter::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchAdapter {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchAdapter::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchAdapter {
        #[inline]
        fn clone(&self) -> DispatchAdapter {
            match self {
                DispatchAdapter::Core(__self_0) => {
                    DispatchAdapter::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchAdapter {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreAdapter {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchAdapter is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreAdapter> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreAdapter> for DispatchAdapter {
        #[inline]
        fn from(value: CoreAdapter) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchAdapter {
        type Target = dyn AdapterInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchDevice {
        Core(Arc<CoreDevice>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchDevice {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchDevice::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchDevice {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchDevice {
        #[inline]
        fn eq(&self, other: &DispatchDevice) -> bool {
            match (self, other) {
                (DispatchDevice::Core(__self_0), DispatchDevice::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchDevice {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreDevice>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchDevice {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchDevice,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchDevice::Core(__self_0), DispatchDevice::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchDevice {
        #[inline]
        fn cmp(&self, other: &DispatchDevice) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchDevice::Core(__self_0), DispatchDevice::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchDevice {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchDevice::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchDevice {
        #[inline]
        fn clone(&self) -> DispatchDevice {
            match self {
                DispatchDevice::Core(__self_0) => {
                    DispatchDevice::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchDevice {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreDevice {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchDevice is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreDevice> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreDevice> for DispatchDevice {
        #[inline]
        fn from(value: CoreDevice) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchDevice {
        type Target = dyn DeviceInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchQueue {
        Core(Arc<CoreQueue>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchQueue {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchQueue::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchQueue {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchQueue {
        #[inline]
        fn eq(&self, other: &DispatchQueue) -> bool {
            match (self, other) {
                (DispatchQueue::Core(__self_0), DispatchQueue::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchQueue {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreQueue>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchQueue {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchQueue,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchQueue::Core(__self_0), DispatchQueue::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchQueue {
        #[inline]
        fn cmp(&self, other: &DispatchQueue) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchQueue::Core(__self_0), DispatchQueue::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchQueue {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchQueue::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchQueue {
        #[inline]
        fn clone(&self) -> DispatchQueue {
            match self {
                DispatchQueue::Core(__self_0) => {
                    DispatchQueue::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchQueue {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreQueue {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchQueue is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreQueue> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreQueue> for DispatchQueue {
        #[inline]
        fn from(value: CoreQueue) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchQueue {
        type Target = dyn QueueInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchShaderModule {
        Core(Arc<CoreShaderModule>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchShaderModule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchShaderModule::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchShaderModule {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchShaderModule {
        #[inline]
        fn eq(&self, other: &DispatchShaderModule) -> bool {
            match (self, other) {
                (
                    DispatchShaderModule::Core(__self_0),
                    DispatchShaderModule::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchShaderModule {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreShaderModule>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchShaderModule {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchShaderModule,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchShaderModule::Core(__self_0),
                    DispatchShaderModule::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchShaderModule {
        #[inline]
        fn cmp(&self, other: &DispatchShaderModule) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchShaderModule::Core(__self_0),
                    DispatchShaderModule::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchShaderModule {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchShaderModule::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchShaderModule {
        #[inline]
        fn clone(&self) -> DispatchShaderModule {
            match self {
                DispatchShaderModule::Core(__self_0) => {
                    DispatchShaderModule::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchShaderModule {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreShaderModule {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchShaderModule is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreShaderModule> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreShaderModule> for DispatchShaderModule {
        #[inline]
        fn from(value: CoreShaderModule) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchShaderModule {
        type Target = dyn ShaderModuleInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchBindGroupLayout {
        Core(Arc<CoreBindGroupLayout>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchBindGroupLayout {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchBindGroupLayout::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchBindGroupLayout {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchBindGroupLayout {
        #[inline]
        fn eq(&self, other: &DispatchBindGroupLayout) -> bool {
            match (self, other) {
                (
                    DispatchBindGroupLayout::Core(__self_0),
                    DispatchBindGroupLayout::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchBindGroupLayout {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreBindGroupLayout>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchBindGroupLayout {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchBindGroupLayout,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchBindGroupLayout::Core(__self_0),
                    DispatchBindGroupLayout::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchBindGroupLayout {
        #[inline]
        fn cmp(&self, other: &DispatchBindGroupLayout) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchBindGroupLayout::Core(__self_0),
                    DispatchBindGroupLayout::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchBindGroupLayout {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchBindGroupLayout::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchBindGroupLayout {
        #[inline]
        fn clone(&self) -> DispatchBindGroupLayout {
            match self {
                DispatchBindGroupLayout::Core(__self_0) => {
                    DispatchBindGroupLayout::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchBindGroupLayout {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreBindGroupLayout {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchBindGroupLayout is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreBindGroupLayout> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreBindGroupLayout> for DispatchBindGroupLayout {
        #[inline]
        fn from(value: CoreBindGroupLayout) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchBindGroupLayout {
        type Target = dyn BindGroupLayoutInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchBindGroup {
        Core(Arc<CoreBindGroup>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchBindGroup {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchBindGroup::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchBindGroup {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchBindGroup {
        #[inline]
        fn eq(&self, other: &DispatchBindGroup) -> bool {
            match (self, other) {
                (
                    DispatchBindGroup::Core(__self_0),
                    DispatchBindGroup::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchBindGroup {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreBindGroup>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchBindGroup {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchBindGroup,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchBindGroup::Core(__self_0),
                    DispatchBindGroup::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchBindGroup {
        #[inline]
        fn cmp(&self, other: &DispatchBindGroup) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchBindGroup::Core(__self_0),
                    DispatchBindGroup::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchBindGroup {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchBindGroup::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchBindGroup {
        #[inline]
        fn clone(&self) -> DispatchBindGroup {
            match self {
                DispatchBindGroup::Core(__self_0) => {
                    DispatchBindGroup::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchBindGroup {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreBindGroup {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchBindGroup is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreBindGroup> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreBindGroup> for DispatchBindGroup {
        #[inline]
        fn from(value: CoreBindGroup) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchBindGroup {
        type Target = dyn BindGroupInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchTextureView {
        Core(Arc<CoreTextureView>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchTextureView {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchTextureView::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchTextureView {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchTextureView {
        #[inline]
        fn eq(&self, other: &DispatchTextureView) -> bool {
            match (self, other) {
                (
                    DispatchTextureView::Core(__self_0),
                    DispatchTextureView::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchTextureView {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreTextureView>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchTextureView {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchTextureView,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchTextureView::Core(__self_0),
                    DispatchTextureView::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchTextureView {
        #[inline]
        fn cmp(&self, other: &DispatchTextureView) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchTextureView::Core(__self_0),
                    DispatchTextureView::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchTextureView {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchTextureView::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchTextureView {
        #[inline]
        fn clone(&self) -> DispatchTextureView {
            match self {
                DispatchTextureView::Core(__self_0) => {
                    DispatchTextureView::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchTextureView {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreTextureView {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchTextureView is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreTextureView> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreTextureView> for DispatchTextureView {
        #[inline]
        fn from(value: CoreTextureView) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchTextureView {
        type Target = dyn TextureViewInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchSampler {
        Core(Arc<CoreSampler>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchSampler {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchSampler::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchSampler {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchSampler {
        #[inline]
        fn eq(&self, other: &DispatchSampler) -> bool {
            match (self, other) {
                (DispatchSampler::Core(__self_0), DispatchSampler::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchSampler {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreSampler>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchSampler {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchSampler,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchSampler::Core(__self_0), DispatchSampler::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchSampler {
        #[inline]
        fn cmp(&self, other: &DispatchSampler) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchSampler::Core(__self_0), DispatchSampler::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchSampler {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchSampler::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchSampler {
        #[inline]
        fn clone(&self) -> DispatchSampler {
            match self {
                DispatchSampler::Core(__self_0) => {
                    DispatchSampler::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchSampler {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreSampler {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchSampler is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreSampler> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreSampler> for DispatchSampler {
        #[inline]
        fn from(value: CoreSampler) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchSampler {
        type Target = dyn SamplerInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchBuffer {
        Core(Arc<CoreBuffer>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchBuffer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchBuffer::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchBuffer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchBuffer {
        #[inline]
        fn eq(&self, other: &DispatchBuffer) -> bool {
            match (self, other) {
                (DispatchBuffer::Core(__self_0), DispatchBuffer::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchBuffer {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreBuffer>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchBuffer {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchBuffer,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchBuffer::Core(__self_0), DispatchBuffer::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchBuffer {
        #[inline]
        fn cmp(&self, other: &DispatchBuffer) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchBuffer::Core(__self_0), DispatchBuffer::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchBuffer {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchBuffer::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchBuffer {
        #[inline]
        fn clone(&self) -> DispatchBuffer {
            match self {
                DispatchBuffer::Core(__self_0) => {
                    DispatchBuffer::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchBuffer {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreBuffer {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchBuffer is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreBuffer> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreBuffer> for DispatchBuffer {
        #[inline]
        fn from(value: CoreBuffer) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchBuffer {
        type Target = dyn BufferInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchTexture {
        Core(Arc<CoreTexture>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchTexture {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchTexture::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchTexture {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchTexture {
        #[inline]
        fn eq(&self, other: &DispatchTexture) -> bool {
            match (self, other) {
                (DispatchTexture::Core(__self_0), DispatchTexture::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchTexture {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreTexture>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchTexture {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchTexture,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchTexture::Core(__self_0), DispatchTexture::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchTexture {
        #[inline]
        fn cmp(&self, other: &DispatchTexture) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchTexture::Core(__self_0), DispatchTexture::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchTexture {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchTexture::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchTexture {
        #[inline]
        fn clone(&self) -> DispatchTexture {
            match self {
                DispatchTexture::Core(__self_0) => {
                    DispatchTexture::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchTexture {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreTexture {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchTexture is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreTexture> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreTexture> for DispatchTexture {
        #[inline]
        fn from(value: CoreTexture) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchTexture {
        type Target = dyn TextureInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchExternalTexture {
        Core(Arc<CoreExternalTexture>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchExternalTexture {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchExternalTexture::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchExternalTexture {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchExternalTexture {
        #[inline]
        fn eq(&self, other: &DispatchExternalTexture) -> bool {
            match (self, other) {
                (
                    DispatchExternalTexture::Core(__self_0),
                    DispatchExternalTexture::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchExternalTexture {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreExternalTexture>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchExternalTexture {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchExternalTexture,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchExternalTexture::Core(__self_0),
                    DispatchExternalTexture::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchExternalTexture {
        #[inline]
        fn cmp(&self, other: &DispatchExternalTexture) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchExternalTexture::Core(__self_0),
                    DispatchExternalTexture::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchExternalTexture {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchExternalTexture::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchExternalTexture {
        #[inline]
        fn clone(&self) -> DispatchExternalTexture {
            match self {
                DispatchExternalTexture::Core(__self_0) => {
                    DispatchExternalTexture::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchExternalTexture {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreExternalTexture {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchExternalTexture is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreExternalTexture> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreExternalTexture> for DispatchExternalTexture {
        #[inline]
        fn from(value: CoreExternalTexture) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchExternalTexture {
        type Target = dyn ExternalTextureInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchBlas {
        Core(Arc<CoreBlas>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchBlas {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchBlas::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchBlas {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchBlas {
        #[inline]
        fn eq(&self, other: &DispatchBlas) -> bool {
            match (self, other) {
                (DispatchBlas::Core(__self_0), DispatchBlas::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchBlas {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreBlas>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchBlas {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchBlas,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchBlas::Core(__self_0), DispatchBlas::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchBlas {
        #[inline]
        fn cmp(&self, other: &DispatchBlas) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchBlas::Core(__self_0), DispatchBlas::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchBlas {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchBlas::Core(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchBlas {
        #[inline]
        fn clone(&self) -> DispatchBlas {
            match self {
                DispatchBlas::Core(__self_0) => {
                    DispatchBlas::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchBlas {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreBlas {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchBlas is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreBlas> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreBlas> for DispatchBlas {
        #[inline]
        fn from(value: CoreBlas) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchBlas {
        type Target = dyn BlasInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchTlas {
        Core(Arc<CoreTlas>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchTlas {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchTlas::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchTlas {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchTlas {
        #[inline]
        fn eq(&self, other: &DispatchTlas) -> bool {
            match (self, other) {
                (DispatchTlas::Core(__self_0), DispatchTlas::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchTlas {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreTlas>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchTlas {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchTlas,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchTlas::Core(__self_0), DispatchTlas::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchTlas {
        #[inline]
        fn cmp(&self, other: &DispatchTlas) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchTlas::Core(__self_0), DispatchTlas::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchTlas {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchTlas::Core(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchTlas {
        #[inline]
        fn clone(&self) -> DispatchTlas {
            match self {
                DispatchTlas::Core(__self_0) => {
                    DispatchTlas::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchTlas {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreTlas {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchTlas is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreTlas> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreTlas> for DispatchTlas {
        #[inline]
        fn from(value: CoreTlas) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchTlas {
        type Target = dyn TlasInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchQuerySet {
        Core(Arc<CoreQuerySet>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchQuerySet {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchQuerySet::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchQuerySet {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchQuerySet {
        #[inline]
        fn eq(&self, other: &DispatchQuerySet) -> bool {
            match (self, other) {
                (DispatchQuerySet::Core(__self_0), DispatchQuerySet::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchQuerySet {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreQuerySet>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchQuerySet {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchQuerySet,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchQuerySet::Core(__self_0), DispatchQuerySet::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchQuerySet {
        #[inline]
        fn cmp(&self, other: &DispatchQuerySet) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchQuerySet::Core(__self_0), DispatchQuerySet::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchQuerySet {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchQuerySet::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchQuerySet {
        #[inline]
        fn clone(&self) -> DispatchQuerySet {
            match self {
                DispatchQuerySet::Core(__self_0) => {
                    DispatchQuerySet::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchQuerySet {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreQuerySet {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchQuerySet is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreQuerySet> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreQuerySet> for DispatchQuerySet {
        #[inline]
        fn from(value: CoreQuerySet) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchQuerySet {
        type Target = dyn QuerySetInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchPipelineLayout {
        Core(Arc<CorePipelineLayout>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchPipelineLayout {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchPipelineLayout::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchPipelineLayout {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchPipelineLayout {
        #[inline]
        fn eq(&self, other: &DispatchPipelineLayout) -> bool {
            match (self, other) {
                (
                    DispatchPipelineLayout::Core(__self_0),
                    DispatchPipelineLayout::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchPipelineLayout {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CorePipelineLayout>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchPipelineLayout {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchPipelineLayout,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchPipelineLayout::Core(__self_0),
                    DispatchPipelineLayout::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchPipelineLayout {
        #[inline]
        fn cmp(&self, other: &DispatchPipelineLayout) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchPipelineLayout::Core(__self_0),
                    DispatchPipelineLayout::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchPipelineLayout {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchPipelineLayout::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchPipelineLayout {
        #[inline]
        fn clone(&self) -> DispatchPipelineLayout {
            match self {
                DispatchPipelineLayout::Core(__self_0) => {
                    DispatchPipelineLayout::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchPipelineLayout {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CorePipelineLayout {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchPipelineLayout is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CorePipelineLayout> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CorePipelineLayout> for DispatchPipelineLayout {
        #[inline]
        fn from(value: CorePipelineLayout) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchPipelineLayout {
        type Target = dyn PipelineLayoutInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchRenderPipeline {
        Core(Arc<CoreRenderPipeline>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchRenderPipeline {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchRenderPipeline::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchRenderPipeline {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchRenderPipeline {
        #[inline]
        fn eq(&self, other: &DispatchRenderPipeline) -> bool {
            match (self, other) {
                (
                    DispatchRenderPipeline::Core(__self_0),
                    DispatchRenderPipeline::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchRenderPipeline {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreRenderPipeline>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchRenderPipeline {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchRenderPipeline,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchRenderPipeline::Core(__self_0),
                    DispatchRenderPipeline::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchRenderPipeline {
        #[inline]
        fn cmp(&self, other: &DispatchRenderPipeline) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchRenderPipeline::Core(__self_0),
                    DispatchRenderPipeline::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchRenderPipeline {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchRenderPipeline::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchRenderPipeline {
        #[inline]
        fn clone(&self) -> DispatchRenderPipeline {
            match self {
                DispatchRenderPipeline::Core(__self_0) => {
                    DispatchRenderPipeline::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchRenderPipeline {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreRenderPipeline {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchRenderPipeline is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreRenderPipeline> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreRenderPipeline> for DispatchRenderPipeline {
        #[inline]
        fn from(value: CoreRenderPipeline) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchRenderPipeline {
        type Target = dyn RenderPipelineInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchComputePipeline {
        Core(Arc<CoreComputePipeline>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchComputePipeline {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchComputePipeline::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchComputePipeline {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchComputePipeline {
        #[inline]
        fn eq(&self, other: &DispatchComputePipeline) -> bool {
            match (self, other) {
                (
                    DispatchComputePipeline::Core(__self_0),
                    DispatchComputePipeline::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchComputePipeline {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreComputePipeline>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchComputePipeline {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchComputePipeline,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchComputePipeline::Core(__self_0),
                    DispatchComputePipeline::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchComputePipeline {
        #[inline]
        fn cmp(&self, other: &DispatchComputePipeline) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchComputePipeline::Core(__self_0),
                    DispatchComputePipeline::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchComputePipeline {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchComputePipeline::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchComputePipeline {
        #[inline]
        fn clone(&self) -> DispatchComputePipeline {
            match self {
                DispatchComputePipeline::Core(__self_0) => {
                    DispatchComputePipeline::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchComputePipeline {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreComputePipeline {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchComputePipeline is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreComputePipeline> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreComputePipeline> for DispatchComputePipeline {
        #[inline]
        fn from(value: CoreComputePipeline) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchComputePipeline {
        type Target = dyn ComputePipelineInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchPipelineCache {
        Core(Arc<CorePipelineCache>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchPipelineCache {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchPipelineCache::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchPipelineCache {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchPipelineCache {
        #[inline]
        fn eq(&self, other: &DispatchPipelineCache) -> bool {
            match (self, other) {
                (
                    DispatchPipelineCache::Core(__self_0),
                    DispatchPipelineCache::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchPipelineCache {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CorePipelineCache>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchPipelineCache {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchPipelineCache,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchPipelineCache::Core(__self_0),
                    DispatchPipelineCache::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchPipelineCache {
        #[inline]
        fn cmp(&self, other: &DispatchPipelineCache) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchPipelineCache::Core(__self_0),
                    DispatchPipelineCache::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchPipelineCache {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchPipelineCache::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchPipelineCache {
        #[inline]
        fn clone(&self) -> DispatchPipelineCache {
            match self {
                DispatchPipelineCache::Core(__self_0) => {
                    DispatchPipelineCache::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchPipelineCache {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CorePipelineCache {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchPipelineCache is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CorePipelineCache> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CorePipelineCache> for DispatchPipelineCache {
        #[inline]
        fn from(value: CorePipelineCache) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchPipelineCache {
        type Target = dyn PipelineCacheInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchCommandEncoder {
        Core(CoreCommandEncoder),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchCommandEncoder {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchCommandEncoder::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchCommandEncoder {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchCommandEncoder {
        #[inline]
        fn eq(&self, other: &DispatchCommandEncoder) -> bool {
            match (self, other) {
                (
                    DispatchCommandEncoder::Core(__self_0),
                    DispatchCommandEncoder::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchCommandEncoder {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreCommandEncoder>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchCommandEncoder {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchCommandEncoder,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchCommandEncoder::Core(__self_0),
                    DispatchCommandEncoder::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchCommandEncoder {
        #[inline]
        fn cmp(&self, other: &DispatchCommandEncoder) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchCommandEncoder::Core(__self_0),
                    DispatchCommandEncoder::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchCommandEncoder {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchCommandEncoder::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchCommandEncoder {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreCommandEncoder {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchCommandEncoder is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreCommandEncoder {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchCommandEncoder is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreCommandEncoder> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreCommandEncoder> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreCommandEncoder> for DispatchCommandEncoder {
        #[inline]
        fn from(value: CoreCommandEncoder) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchCommandEncoder {
        type Target = dyn CommandEncoderInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchCommandEncoder {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    pub enum DispatchComputePass {
        Core(CoreComputePass),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchComputePass {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchComputePass::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchComputePass {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchComputePass {
        #[inline]
        fn eq(&self, other: &DispatchComputePass) -> bool {
            match (self, other) {
                (
                    DispatchComputePass::Core(__self_0),
                    DispatchComputePass::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchComputePass {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreComputePass>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchComputePass {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchComputePass,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchComputePass::Core(__self_0),
                    DispatchComputePass::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchComputePass {
        #[inline]
        fn cmp(&self, other: &DispatchComputePass) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchComputePass::Core(__self_0),
                    DispatchComputePass::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchComputePass {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchComputePass::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchComputePass {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreComputePass {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchComputePass is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreComputePass {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchComputePass is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreComputePass> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreComputePass> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreComputePass> for DispatchComputePass {
        #[inline]
        fn from(value: CoreComputePass) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchComputePass {
        type Target = dyn ComputePassInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchComputePass {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    pub enum DispatchRenderPass {
        Core(CoreRenderPass),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchRenderPass {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchRenderPass::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchRenderPass {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchRenderPass {
        #[inline]
        fn eq(&self, other: &DispatchRenderPass) -> bool {
            match (self, other) {
                (
                    DispatchRenderPass::Core(__self_0),
                    DispatchRenderPass::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchRenderPass {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreRenderPass>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchRenderPass {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchRenderPass,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchRenderPass::Core(__self_0),
                    DispatchRenderPass::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchRenderPass {
        #[inline]
        fn cmp(&self, other: &DispatchRenderPass) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchRenderPass::Core(__self_0),
                    DispatchRenderPass::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchRenderPass {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchRenderPass::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchRenderPass {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreRenderPass {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchRenderPass is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreRenderPass {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchRenderPass is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreRenderPass> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreRenderPass> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreRenderPass> for DispatchRenderPass {
        #[inline]
        fn from(value: CoreRenderPass) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchRenderPass {
        type Target = dyn RenderPassInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchRenderPass {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    pub enum DispatchCommandBuffer {
        Core(CoreCommandBuffer),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchCommandBuffer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchCommandBuffer::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchCommandBuffer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchCommandBuffer {
        #[inline]
        fn eq(&self, other: &DispatchCommandBuffer) -> bool {
            match (self, other) {
                (
                    DispatchCommandBuffer::Core(__self_0),
                    DispatchCommandBuffer::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchCommandBuffer {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreCommandBuffer>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchCommandBuffer {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchCommandBuffer,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchCommandBuffer::Core(__self_0),
                    DispatchCommandBuffer::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchCommandBuffer {
        #[inline]
        fn cmp(&self, other: &DispatchCommandBuffer) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchCommandBuffer::Core(__self_0),
                    DispatchCommandBuffer::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchCommandBuffer {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchCommandBuffer::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchCommandBuffer {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreCommandBuffer {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchCommandBuffer is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreCommandBuffer {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchCommandBuffer is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreCommandBuffer> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreCommandBuffer> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreCommandBuffer> for DispatchCommandBuffer {
        #[inline]
        fn from(value: CoreCommandBuffer) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchCommandBuffer {
        type Target = dyn CommandBufferInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchCommandBuffer {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    pub enum DispatchRenderBundleEncoder {
        Core(CoreRenderBundleEncoder),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchRenderBundleEncoder {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchRenderBundleEncoder::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchRenderBundleEncoder {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchRenderBundleEncoder {
        #[inline]
        fn eq(&self, other: &DispatchRenderBundleEncoder) -> bool {
            match (self, other) {
                (
                    DispatchRenderBundleEncoder::Core(__self_0),
                    DispatchRenderBundleEncoder::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchRenderBundleEncoder {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreRenderBundleEncoder>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchRenderBundleEncoder {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchRenderBundleEncoder,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchRenderBundleEncoder::Core(__self_0),
                    DispatchRenderBundleEncoder::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchRenderBundleEncoder {
        #[inline]
        fn cmp(&self, other: &DispatchRenderBundleEncoder) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchRenderBundleEncoder::Core(__self_0),
                    DispatchRenderBundleEncoder::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchRenderBundleEncoder {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchRenderBundleEncoder::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchRenderBundleEncoder {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreRenderBundleEncoder {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchRenderBundleEncoder is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreRenderBundleEncoder {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchRenderBundleEncoder is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreRenderBundleEncoder> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreRenderBundleEncoder> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreRenderBundleEncoder> for DispatchRenderBundleEncoder {
        #[inline]
        fn from(value: CoreRenderBundleEncoder) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchRenderBundleEncoder {
        type Target = dyn RenderBundleEncoderInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchRenderBundleEncoder {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    pub enum DispatchRenderBundle {
        Core(Arc<CoreRenderBundle>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchRenderBundle {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchRenderBundle::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchRenderBundle {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchRenderBundle {
        #[inline]
        fn eq(&self, other: &DispatchRenderBundle) -> bool {
            match (self, other) {
                (
                    DispatchRenderBundle::Core(__self_0),
                    DispatchRenderBundle::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchRenderBundle {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreRenderBundle>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchRenderBundle {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchRenderBundle,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchRenderBundle::Core(__self_0),
                    DispatchRenderBundle::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchRenderBundle {
        #[inline]
        fn cmp(&self, other: &DispatchRenderBundle) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchRenderBundle::Core(__self_0),
                    DispatchRenderBundle::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchRenderBundle {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchRenderBundle::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchRenderBundle {
        #[inline]
        fn clone(&self) -> DispatchRenderBundle {
            match self {
                DispatchRenderBundle::Core(__self_0) => {
                    DispatchRenderBundle::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchRenderBundle {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreRenderBundle {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchRenderBundle is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreRenderBundle> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreRenderBundle> for DispatchRenderBundle {
        #[inline]
        fn from(value: CoreRenderBundle) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchRenderBundle {
        type Target = dyn RenderBundleInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchSurface {
        Core(Arc<CoreSurface>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchSurface {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchSurface::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchSurface {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchSurface {
        #[inline]
        fn eq(&self, other: &DispatchSurface) -> bool {
            match (self, other) {
                (DispatchSurface::Core(__self_0), DispatchSurface::Core(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchSurface {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreSurface>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchSurface {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchSurface,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (DispatchSurface::Core(__self_0), DispatchSurface::Core(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchSurface {
        #[inline]
        fn cmp(&self, other: &DispatchSurface) -> ::core::cmp::Ordering {
            match (self, other) {
                (DispatchSurface::Core(__self_0), DispatchSurface::Core(__arg1_0)) => {
                    ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchSurface {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchSurface::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchSurface {
        #[inline]
        fn clone(&self) -> DispatchSurface {
            match self {
                DispatchSurface::Core(__self_0) => {
                    DispatchSurface::Core(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl DispatchSurface {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreSurface {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchSurface is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreSurface> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreSurface> for DispatchSurface {
        #[inline]
        fn from(value: CoreSurface) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchSurface {
        type Target = dyn SurfaceInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchSurfaceOutputDetail {
        Core(Arc<CoreSurfaceOutputDetail>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchSurfaceOutputDetail {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchSurfaceOutputDetail::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchSurfaceOutputDetail {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchSurfaceOutputDetail {
        #[inline]
        fn eq(&self, other: &DispatchSurfaceOutputDetail) -> bool {
            match (self, other) {
                (
                    DispatchSurfaceOutputDetail::Core(__self_0),
                    DispatchSurfaceOutputDetail::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchSurfaceOutputDetail {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<CoreSurfaceOutputDetail>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchSurfaceOutputDetail {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchSurfaceOutputDetail,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchSurfaceOutputDetail::Core(__self_0),
                    DispatchSurfaceOutputDetail::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchSurfaceOutputDetail {
        #[inline]
        fn cmp(&self, other: &DispatchSurfaceOutputDetail) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchSurfaceOutputDetail::Core(__self_0),
                    DispatchSurfaceOutputDetail::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchSurfaceOutputDetail {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchSurfaceOutputDetail::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DispatchSurfaceOutputDetail {
        #[inline]
        fn clone(&self) -> DispatchSurfaceOutputDetail {
            match self {
                DispatchSurfaceOutputDetail::Core(__self_0) => {
                    DispatchSurfaceOutputDetail::Core(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    impl DispatchSurfaceOutputDetail {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreSurfaceOutputDetail {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchSurfaceOutputDetail is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreSurfaceOutputDetail> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreSurfaceOutputDetail> for DispatchSurfaceOutputDetail {
        #[inline]
        fn from(value: CoreSurfaceOutputDetail) -> Self {
            Self::Core(Arc::new(value))
        }
    }
    impl core::ops::Deref for DispatchSurfaceOutputDetail {
        type Target = dyn SurfaceOutputDetailInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value.as_ref(),
            }
        }
    }
    pub enum DispatchQueueWriteBuffer {
        Core(CoreQueueWriteBuffer),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchQueueWriteBuffer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchQueueWriteBuffer::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchQueueWriteBuffer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchQueueWriteBuffer {
        #[inline]
        fn eq(&self, other: &DispatchQueueWriteBuffer) -> bool {
            match (self, other) {
                (
                    DispatchQueueWriteBuffer::Core(__self_0),
                    DispatchQueueWriteBuffer::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchQueueWriteBuffer {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreQueueWriteBuffer>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchQueueWriteBuffer {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchQueueWriteBuffer,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchQueueWriteBuffer::Core(__self_0),
                    DispatchQueueWriteBuffer::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchQueueWriteBuffer {
        #[inline]
        fn cmp(&self, other: &DispatchQueueWriteBuffer) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchQueueWriteBuffer::Core(__self_0),
                    DispatchQueueWriteBuffer::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchQueueWriteBuffer {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchQueueWriteBuffer::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchQueueWriteBuffer {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreQueueWriteBuffer {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchQueueWriteBuffer is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreQueueWriteBuffer {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchQueueWriteBuffer is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreQueueWriteBuffer> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreQueueWriteBuffer> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreQueueWriteBuffer> for DispatchQueueWriteBuffer {
        #[inline]
        fn from(value: CoreQueueWriteBuffer) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchQueueWriteBuffer {
        type Target = dyn QueueWriteBufferInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchQueueWriteBuffer {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    pub enum DispatchBufferMappedRange {
        Core(CoreBufferMappedRange),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DispatchBufferMappedRange {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DispatchBufferMappedRange::Core(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Core",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DispatchBufferMappedRange {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DispatchBufferMappedRange {
        #[inline]
        fn eq(&self, other: &DispatchBufferMappedRange) -> bool {
            match (self, other) {
                (
                    DispatchBufferMappedRange::Core(__self_0),
                    DispatchBufferMappedRange::Core(__arg1_0),
                ) => __self_0 == __arg1_0,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DispatchBufferMappedRange {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CoreBufferMappedRange>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DispatchBufferMappedRange {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DispatchBufferMappedRange,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match (self, other) {
                (
                    DispatchBufferMappedRange::Core(__self_0),
                    DispatchBufferMappedRange::Core(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DispatchBufferMappedRange {
        #[inline]
        fn cmp(&self, other: &DispatchBufferMappedRange) -> ::core::cmp::Ordering {
            match (self, other) {
                (
                    DispatchBufferMappedRange::Core(__self_0),
                    DispatchBufferMappedRange::Core(__arg1_0),
                ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DispatchBufferMappedRange {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match self {
                DispatchBufferMappedRange::Core(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl DispatchBufferMappedRange {
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core(&self) -> &CoreBufferMappedRange {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchBufferMappedRange is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut(&mut self) -> &mut CoreBufferMappedRange {
            match self {
                Self::Core(value) => value,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("DispatchBufferMappedRange is not core"),
                    );
                }
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_opt(&self) -> Option<&CoreBufferMappedRange> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
        #[inline]
        #[allow(clippy::allow_attributes, unused)]
        pub fn as_core_mut_opt(&mut self) -> Option<&mut CoreBufferMappedRange> {
            match self {
                Self::Core(value) => Some(value),
                _ => None,
            }
        }
    }
    impl From<CoreBufferMappedRange> for DispatchBufferMappedRange {
        #[inline]
        fn from(value: CoreBufferMappedRange) -> Self {
            Self::Core(value)
        }
    }
    impl core::ops::Deref for DispatchBufferMappedRange {
        type Target = dyn BufferMappedRangeInterface;
        #[inline]
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
    impl core::ops::DerefMut for DispatchBufferMappedRange {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Core(value) => value,
            }
        }
    }
}
mod macros {
    //! Convenience macros
    #[doc(hidden)]
    pub mod helpers {
        pub use alloc::{borrow::Cow, string::String};
        pub use core::{include_bytes, include_str};
        pub use Some;
    }
}
pub mod util {
    //! Utility structures and functions that are built on top of the main `wgpu` API.
    //!
    //! Nothing in this module is a part of the WebGPU API specification;
    //! they are unique to the `wgpu` library.
    mod belt {
        use crate::{
            util::align_to, Buffer, BufferAddress, BufferDescriptor, BufferSize,
            BufferSlice, BufferUsages, BufferViewMut, CommandEncoder, Device, MapMode,
        };
        use alloc::vec::Vec;
        use core::fmt;
        use std::sync::mpsc;
        /// Efficiently performs many buffer writes by sharing and reusing temporary buffers.
        ///
        /// Internally it uses a ring-buffer of staging buffers that are sub-allocated.
        /// Its advantage over [`Queue::write_buffer_with()`] is that the individual allocations
        /// are cheaper; `StagingBelt` is most useful when you are writing very many small pieces
        /// of data. It can be understood as a sort of arena allocator.
        ///
        /// Using a staging belt is slightly complicated, and generally goes as follows:
        /// 1. Use [`StagingBelt::write_buffer()`] or [`StagingBelt::allocate()`] to allocate
        ///    buffer slices, then write your data to them.
        /// 2. Call [`StagingBelt::finish()`].
        /// 3. Submit all command encoders that were used in step 1.
        /// 4. Call [`StagingBelt::recall()`].
        ///
        /// [`Queue::write_buffer_with()`]: crate::Queue::write_buffer_with
        pub struct StagingBelt {
            chunk_size: BufferAddress,
            /// Chunks into which we are accumulating data to be transferred.
            active_chunks: Vec<Chunk>,
            /// Chunks that have scheduled transfers already; they are unmapped and some
            /// command encoder has one or more commands with them as source.
            closed_chunks: Vec<Chunk>,
            /// Chunks that are back from the GPU and ready to be mapped for write and put
            /// into `active_chunks`.
            free_chunks: Vec<Chunk>,
            /// When closed chunks are mapped again, the map callback sends them here.
            sender: Exclusive<mpsc::Sender<Chunk>>,
            /// Free chunks are received here to be put on `self.free_chunks`.
            receiver: Exclusive<mpsc::Receiver<Chunk>>,
        }
        impl StagingBelt {
            /// Create a new staging belt.
            ///
            /// The `chunk_size` is the unit of internal buffer allocation; writes will be
            /// sub-allocated within each chunk. Therefore, for optimal use of memory, the
            /// chunk size should be:
            ///
            /// * larger than the largest single [`StagingBelt::write_buffer()`] operation;
            /// * 1-4 times less than the total amount of data uploaded per submission
            ///   (per [`StagingBelt::finish()`]); and
            /// * bigger is better, within these bounds.
            pub fn new(chunk_size: BufferAddress) -> Self {
                let (sender, receiver) = mpsc::channel();
                StagingBelt {
                    chunk_size,
                    active_chunks: Vec::new(),
                    closed_chunks: Vec::new(),
                    free_chunks: Vec::new(),
                    sender: Exclusive::new(sender),
                    receiver: Exclusive::new(receiver),
                }
            }
            /// Allocate a staging belt slice of `size` to be copied into the `target` buffer
            /// at the specified offset.
            ///
            /// The upload will be placed into the provided command encoder. This encoder
            /// must be submitted after [`StagingBelt::finish()`] is called and before
            /// [`StagingBelt::recall()`] is called.
            ///
            /// If the `size` is greater than the size of any free internal buffer, a new buffer
            /// will be allocated for it. Therefore, the `chunk_size` passed to [`StagingBelt::new()`]
            /// should ideally be larger than every such size.
            pub fn write_buffer(
                &mut self,
                encoder: &mut CommandEncoder,
                target: &Buffer,
                offset: BufferAddress,
                size: BufferSize,
                device: &Device,
            ) -> BufferViewMut {
                let slice_of_belt = self
                    .allocate(
                        size,
                        const { BufferSize::new(crate::COPY_BUFFER_ALIGNMENT).unwrap() },
                        device,
                    );
                encoder
                    .copy_buffer_to_buffer(
                        slice_of_belt.buffer(),
                        slice_of_belt.offset(),
                        target,
                        offset,
                        size.get(),
                    );
                slice_of_belt.get_mapped_range_mut()
            }
            /// Allocate a staging belt slice with the given `size` and `alignment` and return it.
            ///
            /// To use this slice, call [`BufferSlice::get_mapped_range_mut()`] and write your data into
            /// that [`BufferViewMut`].
            /// (The view must be dropped before [`StagingBelt::finish()`] is called.)
            ///
            /// You can then record your own GPU commands to perform with the slice,
            /// such as copying it to a texture or executing a compute shader that reads it (whereas
            /// [`StagingBelt::write_buffer()`] can only write to other buffers).
            /// All commands involving this slice must be submitted after
            /// [`StagingBelt::finish()`] is called and before [`StagingBelt::recall()`] is called.
            ///
            /// If the `size` is greater than the space available in any free internal buffer, a new buffer
            /// will be allocated for it. Therefore, the `chunk_size` passed to [`StagingBelt::new()`]
            /// should ideally be larger than every such size.
            ///
            /// The chosen slice will be positioned within the buffer at a multiple of `alignment`,
            /// which may be used to meet alignment requirements for the operation you wish to perform
            /// with the slice. This does not necessarily affect the alignment of the [`BufferViewMut`].
            pub fn allocate(
                &mut self,
                size: BufferSize,
                alignment: BufferSize,
                device: &Device,
            ) -> BufferSlice<'_> {
                if !alignment.get().is_power_of_two() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "alignment must be a power of two, not {0}",
                                alignment,
                            ),
                        );
                    }
                }
                let alignment = alignment.get().max(crate::MAP_ALIGNMENT);
                let mut chunk = if let Some(index) = self
                    .active_chunks
                    .iter()
                    .position(|chunk| chunk.can_allocate(size, alignment))
                {
                    self.active_chunks.swap_remove(index)
                } else {
                    self.receive_chunks();
                    if let Some(index) = self
                        .free_chunks
                        .iter()
                        .position(|chunk| chunk.can_allocate(size, alignment))
                    {
                        self.free_chunks.swap_remove(index)
                    } else {
                        Chunk {
                            buffer: device
                                .create_buffer(
                                    &BufferDescriptor {
                                        label: Some("(wgpu internal) StagingBelt staging buffer"),
                                        size: self.chunk_size.max(size.get()),
                                        usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
                                        mapped_at_creation: true,
                                    },
                                ),
                            offset: 0,
                        }
                    }
                };
                let allocation_offset = chunk.allocate(size, alignment);
                self.active_chunks.push(chunk);
                let chunk = self.active_chunks.last().unwrap();
                chunk.buffer.slice(allocation_offset..allocation_offset + size.get())
            }
            /// Prepare currently mapped buffers for use in a submission.
            ///
            /// This must be called before the command encoder(s) provided to
            /// [`StagingBelt::write_buffer()`] are submitted.
            ///
            /// At this point, all the partially used staging buffers are closed (cannot be used for
            /// further writes) until after [`StagingBelt::recall()`] is called *and* the GPU is done
            /// copying the data from them.
            pub fn finish(&mut self) {
                for chunk in self.active_chunks.drain(..) {
                    chunk.buffer.unmap();
                    self.closed_chunks.push(chunk);
                }
            }
            /// Recall all of the closed buffers back to be reused.
            ///
            /// This must only be called after the command encoder(s) provided to
            /// [`StagingBelt::write_buffer()`] are submitted. Additional calls are harmless.
            /// Not calling this as soon as possible may result in increased buffer memory usage.
            pub fn recall(&mut self) {
                self.receive_chunks();
                for chunk in self.closed_chunks.drain(..) {
                    let sender = self.sender.get_mut().clone();
                    chunk
                        .buffer
                        .clone()
                        .slice(..)
                        .map_async(
                            MapMode::Write,
                            move |_| {
                                let _ = sender.send(chunk);
                            },
                        );
                }
            }
            /// Move all chunks that the GPU is done with (and are now mapped again)
            /// from `self.receiver` to `self.free_chunks`.
            fn receive_chunks(&mut self) {
                while let Ok(mut chunk) = self.receiver.get_mut().try_recv() {
                    chunk.offset = 0;
                    self.free_chunks.push(chunk);
                }
            }
        }
        impl fmt::Debug for StagingBelt {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("StagingBelt")
                    .field("chunk_size", &self.chunk_size)
                    .field("active_chunks", &self.active_chunks.len())
                    .field("closed_chunks", &self.closed_chunks.len())
                    .field("free_chunks", &self.free_chunks.len())
                    .finish_non_exhaustive()
            }
        }
        struct Chunk {
            buffer: Buffer,
            offset: BufferAddress,
        }
        impl Chunk {
            fn can_allocate(&self, size: BufferSize, alignment: BufferAddress) -> bool {
                let alloc_start = align_to(self.offset, alignment);
                let alloc_end = alloc_start + size.get();
                alloc_end <= self.buffer.size()
            }
            fn allocate(
                &mut self,
                size: BufferSize,
                alignment: BufferAddress,
            ) -> BufferAddress {
                let alloc_start = align_to(self.offset, alignment);
                let alloc_end = alloc_start + size.get();
                if !(alloc_end <= self.buffer.size()) {
                    ::core::panicking::panic(
                        "assertion failed: alloc_end <= self.buffer.size()",
                    )
                }
                self.offset = alloc_end;
                alloc_start
            }
        }
        use exclusive::Exclusive;
        mod exclusive {
            /// `Sync` wrapper that works by providing only exclusive access.
            ///
            /// See <https://doc.rust-lang.org/nightly/std/sync/struct.Exclusive.html>
            pub(super) struct Exclusive<T>(T);
            /// Safety: `&Exclusive` has no operations.
            unsafe impl<T> Sync for Exclusive<T> {}
            impl<T> Exclusive<T> {
                pub fn new(value: T) -> Self {
                    Self(value)
                }
                pub fn get_mut(&mut self) -> &mut T {
                    &mut self.0
                }
            }
        }
    }
    mod device {
        use alloc::borrow::ToOwned as _;
        use wgt::TextureDataOrder;
        /// Describes a [Buffer](crate::Buffer) when allocating.
        pub struct BufferInitDescriptor<'a> {
            /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
            pub label: crate::Label<'a>,
            /// Contents of a buffer on creation.
            pub contents: &'a [u8],
            /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
            /// will panic.
            pub usage: wgt::BufferUsages,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for BufferInitDescriptor<'a> {
            #[inline]
            fn clone(&self) -> BufferInitDescriptor<'a> {
                BufferInitDescriptor {
                    label: ::core::clone::Clone::clone(&self.label),
                    contents: ::core::clone::Clone::clone(&self.contents),
                    usage: ::core::clone::Clone::clone(&self.usage),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for BufferInitDescriptor<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "BufferInitDescriptor",
                    "label",
                    &self.label,
                    "contents",
                    &self.contents,
                    "usage",
                    &&self.usage,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::marker::StructuralPartialEq for BufferInitDescriptor<'a> {}
        #[automatically_derived]
        impl<'a> ::core::cmp::PartialEq for BufferInitDescriptor<'a> {
            #[inline]
            fn eq(&self, other: &BufferInitDescriptor<'a>) -> bool {
                self.label == other.label && self.contents == other.contents
                    && self.usage == other.usage
            }
        }
        #[automatically_derived]
        impl<'a> ::core::cmp::Eq for BufferInitDescriptor<'a> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<crate::Label<'a>>;
                let _: ::core::cmp::AssertParamIsEq<&'a [u8]>;
                let _: ::core::cmp::AssertParamIsEq<wgt::BufferUsages>;
            }
        }
        #[automatically_derived]
        impl<'a> ::core::hash::Hash for BufferInitDescriptor<'a> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.label, state);
                ::core::hash::Hash::hash(&self.contents, state);
                ::core::hash::Hash::hash(&self.usage, state)
            }
        }
        /// Utility methods not meant to be in the main API.
        pub trait DeviceExt {
            /// Creates a [Buffer](crate::Buffer) with data to initialize it.
            fn create_buffer_init(
                &self,
                desc: &BufferInitDescriptor<'_>,
            ) -> crate::Buffer;
            /// Upload an entire texture and its mipmaps from a source buffer.
            ///
            /// Expects all mipmaps to be tightly packed in the data buffer.
            ///
            /// See [`TextureDataOrder`] for the order in which the data is laid out in memory.
            ///
            /// Implicitly adds the `COPY_DST` usage if it is not present in the descriptor,
            /// as it is required to be able to upload the data to the gpu.
            fn create_texture_with_data(
                &self,
                queue: &crate::Queue,
                desc: &crate::TextureDescriptor<'_>,
                order: TextureDataOrder,
                data: &[u8],
            ) -> crate::Texture;
        }
        impl DeviceExt for crate::Device {
            fn create_buffer_init(
                &self,
                descriptor: &BufferInitDescriptor<'_>,
            ) -> crate::Buffer {
                if descriptor.contents.is_empty() {
                    let wgt_descriptor = crate::BufferDescriptor {
                        label: descriptor.label,
                        size: 0,
                        usage: descriptor.usage,
                        mapped_at_creation: false,
                    };
                    self.create_buffer(&wgt_descriptor)
                } else {
                    let unpadded_size = descriptor.contents.len()
                        as crate::BufferAddress;
                    let align_mask = crate::COPY_BUFFER_ALIGNMENT - 1;
                    let padded_size = ((unpadded_size + align_mask) & !align_mask)
                        .max(crate::COPY_BUFFER_ALIGNMENT);
                    let wgt_descriptor = crate::BufferDescriptor {
                        label: descriptor.label,
                        size: padded_size,
                        usage: descriptor.usage,
                        mapped_at_creation: true,
                    };
                    let buffer = self.create_buffer(&wgt_descriptor);
                    buffer
                        .slice(..)
                        .get_mapped_range_mut()[..unpadded_size as usize]
                        .copy_from_slice(descriptor.contents);
                    buffer.unmap();
                    buffer
                }
            }
            fn create_texture_with_data(
                &self,
                queue: &crate::Queue,
                desc: &crate::TextureDescriptor<'_>,
                order: TextureDataOrder,
                data: &[u8],
            ) -> crate::Texture {
                let mut desc = desc.to_owned();
                desc.usage |= crate::TextureUsages::COPY_DST;
                let texture = self.create_texture(&desc);
                let block_size = desc.format.block_copy_size(None).unwrap_or(4);
                let (block_width, block_height) = desc.format.block_dimensions();
                let layer_iterations = desc.array_layer_count();
                let outer_iteration;
                let inner_iteration;
                match order {
                    TextureDataOrder::LayerMajor => {
                        outer_iteration = layer_iterations;
                        inner_iteration = desc.mip_level_count;
                    }
                    TextureDataOrder::MipMajor => {
                        outer_iteration = desc.mip_level_count;
                        inner_iteration = layer_iterations;
                    }
                }
                let mut binary_offset = 0;
                for outer in 0..outer_iteration {
                    for inner in 0..inner_iteration {
                        let (layer, mip) = match order {
                            TextureDataOrder::LayerMajor => (outer, inner),
                            TextureDataOrder::MipMajor => (inner, outer),
                        };
                        let mut mip_size = desc.mip_level_size(mip).unwrap();
                        if desc.dimension != wgt::TextureDimension::D3 {
                            mip_size.depth_or_array_layers = 1;
                        }
                        let mip_physical = mip_size.physical_size(desc.format);
                        let width_blocks = mip_physical.width / block_width;
                        let height_blocks = mip_physical.height / block_height;
                        let bytes_per_row = width_blocks * block_size;
                        let data_size = bytes_per_row * height_blocks
                            * mip_size.depth_or_array_layers;
                        let end_offset = binary_offset + data_size as usize;
                        queue
                            .write_texture(
                                crate::TexelCopyTextureInfo {
                                    texture: &texture,
                                    mip_level: mip,
                                    origin: crate::Origin3d {
                                        x: 0,
                                        y: 0,
                                        z: layer,
                                    },
                                    aspect: wgt::TextureAspect::All,
                                },
                                &data[binary_offset..end_offset],
                                crate::TexelCopyBufferLayout {
                                    offset: 0,
                                    bytes_per_row: Some(bytes_per_row),
                                    rows_per_image: Some(height_blocks),
                                },
                                mip_physical,
                            );
                        binary_offset = end_offset;
                    }
                }
                texture
            }
        }
    }
    mod encoder {
        use core::ops::Range;
        use wgt::{BufferAddress, DynamicOffset, IndexFormat};
        use crate::{
            BindGroup, Buffer, BufferSlice, RenderBundleEncoder, RenderPass,
            RenderPipeline,
        };
        /// Methods shared by [`RenderPass`] and [`RenderBundleEncoder`].
        pub trait RenderEncoder<'a> {
            /// Sets the active bind group for a given bind group index. The bind group layout
            /// in the active pipeline when any `draw()` function is called must match the layout of this bind group.
            ///
            /// If the bind group have dynamic offsets, provide them in order of their declaration.
            fn set_bind_group(
                &mut self,
                index: u32,
                bind_group: Option<&'a BindGroup>,
                offsets: &[DynamicOffset],
            );
            /// Sets the active render pipeline.
            ///
            /// Subsequent draw calls will exhibit the behavior defined by `pipeline`.
            fn set_pipeline(&mut self, pipeline: &'a RenderPipeline);
            /// Sets the active index buffer.
            ///
            /// Subsequent calls to [`draw_indexed`](RenderEncoder::draw_indexed) on this [`RenderEncoder`] will
            /// use `buffer` as the source index buffer.
            fn set_index_buffer(
                &mut self,
                buffer_slice: BufferSlice<'a>,
                index_format: IndexFormat,
            );
            /// Assign a vertex buffer to a slot.
            ///
            /// Subsequent calls to [`draw`] and [`draw_indexed`] on this
            /// [`RenderEncoder`] will use `buffer` as one of the source vertex buffers.
            ///
            /// The `slot` refers to the index of the matching descriptor in
            /// [`VertexState::buffers`](crate::VertexState::buffers).
            ///
            /// [`draw`]: RenderEncoder::draw
            /// [`draw_indexed`]: RenderEncoder::draw_indexed
            fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>);
            /// Draws primitives from the active vertex buffer(s).
            ///
            /// The active vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
            fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
            /// Draws indexed primitives using the active index buffer and the active vertex buffers.
            ///
            /// The active index buffer can be set with [`RenderEncoder::set_index_buffer`], while the active
            /// vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
            fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            );
            /// Draws primitives from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
            ///
            /// The active vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
            fn draw_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            );
            /// Draws indexed primitives using the active index buffer and the active vertex buffers,
            /// based on the contents of the `indirect_buffer`.
            ///
            /// The active index buffer can be set with [`RenderEncoder::set_index_buffer`], while the active
            /// vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
            ///
            /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
            fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            );
            /// [`wgt::Features::PUSH_CONSTANTS`] must be enabled on the device in order to call this function.
            ///
            /// Set push constant data.
            ///
            /// Offset is measured in bytes, but must be a multiple of [`wgt::PUSH_CONSTANT_ALIGNMENT`].
            ///
            /// Data size must be a multiple of 4 and must be aligned to the 4s, so we take an array of u32.
            /// For example, with an offset of 4 and an array of `[u32; 3]`, that will write to the range
            /// of 4..16.
            ///
            /// For each byte in the range of push constant data written, the union of the stages of all push constant
            /// ranges that covers that byte must be exactly `stages`. There's no good way of explaining this simply,
            /// so here are some examples:
            ///
            /// ```text
            /// For the given ranges:
            /// - 0..4 Vertex
            /// - 4..8 Fragment
            /// ```
            ///
            /// You would need to upload this in two `set_push_constants` calls. First for the `Vertex` range, second for the `Fragment` range.
            ///
            /// ```text
            /// For the given ranges:
            /// - 0..8  Vertex
            /// - 4..12 Fragment
            /// ```
            ///
            /// You would need to upload this in three `set_push_constants` calls. First for the `Vertex` only range 0..4, second
            /// for the `Vertex | Fragment` range 4..8, third for the `Fragment` range 8..12.
            fn set_push_constants(
                &mut self,
                stages: wgt::ShaderStages,
                offset: u32,
                data: &[u8],
            );
        }
        impl<'a> RenderEncoder<'a> for RenderPass<'a> {
            #[inline(always)]
            fn set_bind_group(
                &mut self,
                index: u32,
                bind_group: Option<&'a BindGroup>,
                offsets: &[DynamicOffset],
            ) {
                Self::set_bind_group(self, index, bind_group, offsets);
            }
            #[inline(always)]
            fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
                Self::set_pipeline(self, pipeline);
            }
            #[inline(always)]
            fn set_index_buffer(
                &mut self,
                buffer_slice: BufferSlice<'a>,
                index_format: IndexFormat,
            ) {
                Self::set_index_buffer(self, buffer_slice, index_format);
            }
            #[inline(always)]
            fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
                Self::set_vertex_buffer(self, slot, buffer_slice);
            }
            #[inline(always)]
            fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
                Self::draw(self, vertices, instances);
            }
            #[inline(always)]
            fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            ) {
                Self::draw_indexed(self, indices, base_vertex, instances);
            }
            #[inline(always)]
            fn draw_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            ) {
                Self::draw_indirect(self, indirect_buffer, indirect_offset);
            }
            #[inline(always)]
            fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            ) {
                Self::draw_indexed_indirect(self, indirect_buffer, indirect_offset);
            }
            #[inline(always)]
            fn set_push_constants(
                &mut self,
                stages: wgt::ShaderStages,
                offset: u32,
                data: &[u8],
            ) {
                Self::set_push_constants(self, stages, offset, data);
            }
        }
        impl<'a> RenderEncoder<'a> for RenderBundleEncoder<'a> {
            #[inline(always)]
            fn set_bind_group(
                &mut self,
                index: u32,
                bind_group: Option<&'a BindGroup>,
                offsets: &[DynamicOffset],
            ) {
                Self::set_bind_group(self, index, bind_group, offsets);
            }
            #[inline(always)]
            fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
                Self::set_pipeline(self, pipeline);
            }
            #[inline(always)]
            fn set_index_buffer(
                &mut self,
                buffer_slice: BufferSlice<'a>,
                index_format: IndexFormat,
            ) {
                Self::set_index_buffer(self, buffer_slice, index_format);
            }
            #[inline(always)]
            fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
                Self::set_vertex_buffer(self, slot, buffer_slice);
            }
            #[inline(always)]
            fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
                Self::draw(self, vertices, instances);
            }
            #[inline(always)]
            fn draw_indexed(
                &mut self,
                indices: Range<u32>,
                base_vertex: i32,
                instances: Range<u32>,
            ) {
                Self::draw_indexed(self, indices, base_vertex, instances);
            }
            #[inline(always)]
            fn draw_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            ) {
                Self::draw_indirect(self, indirect_buffer, indirect_offset);
            }
            #[inline(always)]
            fn draw_indexed_indirect(
                &mut self,
                indirect_buffer: &'a Buffer,
                indirect_offset: BufferAddress,
            ) {
                Self::draw_indexed_indirect(self, indirect_buffer, indirect_offset);
            }
            #[inline(always)]
            fn set_push_constants(
                &mut self,
                stages: wgt::ShaderStages,
                offset: u32,
                data: &[u8],
            ) {
                Self::set_push_constants(self, stages, offset, data);
            }
        }
    }
    mod init {
        use crate::{Adapter, Instance, RequestAdapterOptions, Surface};
        /// Initialize the adapter obeying the `WGPU_ADAPTER_NAME` environment variable.
        pub async fn initialize_adapter_from_env(
            instance: &Instance,
            compatible_surface: Option<&Surface<'_>>,
        ) -> Result<Adapter, wgt::RequestAdapterError> {
            let desired_adapter_name: alloc::string::String = {
                std::env::var("WGPU_ADAPTER_NAME")
                    .as_deref()
                    .map(str::to_lowercase)
                    .map_err(|_| wgt::RequestAdapterError::EnvNotSet)?
            };
            let adapters = instance.enumerate_adapters(crate::Backends::all()).await;
            let mut chosen_adapter = None;
            for adapter in adapters {
                let info = adapter.get_info();
                if let Some(surface) = compatible_surface {
                    if !adapter.is_surface_supported(surface) {
                        continue;
                    }
                }
                if info.name.to_lowercase().contains(&desired_adapter_name) {
                    chosen_adapter = Some(adapter);
                    break;
                }
            }
            Ok(
                chosen_adapter
                    .expect("WGPU_ADAPTER_NAME set but no matching adapter found!"),
            )
        }
        /// Initialize the adapter obeying the `WGPU_ADAPTER_NAME` environment variable and if it doesn't exist fall back on a default adapter.
        pub async fn initialize_adapter_from_env_or_default(
            instance: &Instance,
            compatible_surface: Option<&Surface<'_>>,
        ) -> Result<Adapter, wgt::RequestAdapterError> {
            match initialize_adapter_from_env(instance, compatible_surface).await {
                Ok(a) => Ok(a),
                Err(_) => {
                    instance
                        .request_adapter(
                            &RequestAdapterOptions {
                                power_preference: crate::PowerPreference::from_env()
                                    .unwrap_or_default(),
                                force_fallback_adapter: false,
                                compatible_surface,
                            },
                        )
                        .await
                }
            }
        }
        /// Determines whether the [`Backends::BROWSER_WEBGPU`] backend is supported.
        ///
        /// The result can only be true if this is called from the main thread or a dedicated worker.
        /// For convenience, this is also supported on non-wasm targets, always returning false there.
        pub async fn is_browser_webgpu_supported() -> bool {
            { false }
        }
        /// Create an new instance of wgpu, but disabling [`Backends::BROWSER_WEBGPU`] if no WebGPU support was detected.
        ///
        /// If the instance descriptor enables [`Backends::BROWSER_WEBGPU`],
        /// this checks via [`is_browser_webgpu_supported`] for WebGPU support before forwarding
        /// the descriptor with or without [`Backends::BROWSER_WEBGPU`] respecitively to [`Instance::new`].
        ///
        /// You should prefer this method over [`Instance::new`] if you want to target WebGPU and automatically
        /// fall back to WebGL if WebGPU is not available.
        /// This is because WebGPU support has to be decided upon instance creation and [`Instance::new`]
        /// (being a `sync` function) can't establish WebGPU support (details see [`is_browser_webgpu_supported`]).
        ///
        /// # Panics
        ///
        /// If no backend feature for the active target platform is enabled,
        /// this method will panic, see [`Instance::enabled_backend_features()`].
        pub async fn new_instance_with_webgpu_detection(
            instance_desc: &wgt::InstanceDescriptor,
        ) -> crate::Instance {
            let mut instance_desc = instance_desc.clone();
            if instance_desc.backends.contains(wgt::Backends::BROWSER_WEBGPU)
                && !is_browser_webgpu_supported().await
            {
                instance_desc.backends.remove(wgt::Backends::BROWSER_WEBGPU);
            }
            crate::Instance::new(&instance_desc)
        }
    }
    mod mutex {
        //! Provides a [`Mutex`] for internal use based on what features are available.
        use parking_lot::Mutex as MutexInner;
        pub(crate) struct Mutex<T: ?Sized> {
            inner: MutexInner<T>,
        }
        impl<T: ?Sized> core::fmt::Debug for Mutex<T>
        where
            MutexInner<T>: core::fmt::Debug,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <MutexInner<T> as core::fmt::Debug>::fmt(&self.inner, f)
            }
        }
        impl<T: Default> Default for Mutex<T> {
            fn default() -> Self {
                Self::new(<T as Default>::default())
            }
        }
        impl<T> Mutex<T> {
            pub const fn new(value: T) -> Self {
                Self {
                    inner: MutexInner::new(value),
                }
            }
        }
        impl<T: ?Sized> Mutex<T> {
            pub fn lock(&self) -> impl core::ops::DerefMut<Target = T> + '_ {
                self.inner.lock()
            }
        }
    }
    mod texture_blitter {
        use wgt::BlendState;
        use crate::{
            include_wgsl, AddressMode, BindGroupDescriptor, BindGroupEntry,
            BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, ColorTargetState, ColorWrites, CommandEncoder, Device,
            FilterMode, FragmentState, FrontFace, LoadOp, MultisampleState,
            PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
            PrimitiveTopology, RenderPassDescriptor, RenderPipeline,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor,
            ShaderStages, StoreOp, TextureFormat, TextureSampleType, TextureView,
            TextureViewDimension, VertexState,
        };
        /// A builder for the [`TextureBlitter`] utility.
        /// If you want the default [`TextureBlitter`] use [`TextureBlitter::new`] instead.
        pub struct TextureBlitterBuilder<'a> {
            device: &'a Device,
            format: TextureFormat,
            sample_type: FilterMode,
            blend_state: Option<BlendState>,
        }
        impl<'a> TextureBlitterBuilder<'a> {
            /// Returns a new [`TextureBlitterBuilder`]
            ///
            /// # Arguments
            /// - `device` - A [`Device`]
            /// - `format` - The [`TextureFormat`] of the texture that will be copied to. This has to have the `RENDER_TARGET` usage.
            pub fn new(device: &'a Device, format: TextureFormat) -> Self {
                Self {
                    device,
                    format,
                    sample_type: FilterMode::Nearest,
                    blend_state: None,
                }
            }
            /// Sets the [`Sampler`] Filtering Mode
            pub fn sample_type(mut self, sample_type: FilterMode) -> Self {
                self.sample_type = sample_type;
                self
            }
            /// Sets the [`BlendState`] that is used.
            pub fn blend_state(mut self, blend_state: BlendState) -> Self {
                self.blend_state = Some(blend_state);
                self
            }
            /// Returns a new [`TextureBlitter`] with given settings.
            pub fn build(self) -> TextureBlitter {
                let sampler = self
                    .device
                    .create_sampler(
                        &SamplerDescriptor {
                            label: Some("wgpu::util::TextureBlitter::sampler"),
                            address_mode_u: AddressMode::ClampToEdge,
                            address_mode_v: AddressMode::ClampToEdge,
                            address_mode_w: AddressMode::ClampToEdge,
                            mag_filter: self.sample_type,
                            ..Default::default()
                        },
                    );
                let bind_group_layout = self
                    .device
                    .create_bind_group_layout(
                        &BindGroupLayoutDescriptor {
                            label: Some("wgpu::util::TextureBlitter::bind_group_layout"),
                            entries: &[
                                BindGroupLayoutEntry {
                                    binding: 0,
                                    visibility: ShaderStages::FRAGMENT,
                                    ty: BindingType::Texture {
                                        sample_type: TextureSampleType::Float {
                                            filterable: self.sample_type == FilterMode::Linear,
                                        },
                                        view_dimension: TextureViewDimension::D2,
                                        multisampled: false,
                                    },
                                    count: None,
                                },
                                BindGroupLayoutEntry {
                                    binding: 1,
                                    visibility: ShaderStages::FRAGMENT,
                                    ty: BindingType::Sampler(
                                        if self.sample_type == FilterMode::Linear {
                                            SamplerBindingType::Filtering
                                        } else {
                                            SamplerBindingType::NonFiltering
                                        },
                                    ),
                                    count: None,
                                },
                            ],
                        },
                    );
                let pipeline_layout = self
                    .device
                    .create_pipeline_layout(
                        &PipelineLayoutDescriptor {
                            label: Some("wgpu::util::TextureBlitter::pipeline_layout"),
                            bind_group_layouts: &[&bind_group_layout],
                            push_constant_ranges: &[],
                        },
                    );
                let shader = self
                    .device
                    .create_shader_module({
                        crate::ShaderModuleDescriptor {
                            label: crate::__macro_helpers::Some("blit.wgsl"),
                            source: crate::ShaderSource::Wgsl(
                                crate::__macro_helpers::Cow::Borrowed(
                                    "struct VertexOutput {\n    @builtin(position) position: vec4<f32>,\n    @location(0) tex_coords: vec2<f32>,\n}\n\n@vertex\nfn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {\n    var out: VertexOutput;\n\n    out.tex_coords = vec2<f32>(\n        f32((vi << 1u) & 2u),\n        f32(vi & 2u),\n    );\n\n    out.position = vec4<f32>(out.tex_coords * 2.0 - 1.0, 0.0, 1.0);\n\n    // Invert y so the texture is not upside down\n    out.tex_coords.y = 1.0 - out.tex_coords.y;\n    return out;\n}\n\n@group(0) @binding(0)\nvar texture: texture_2d<f32>;\n@group(0) @binding(1)\nvar texture_sampler: sampler;\n\n@fragment\nfn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {\n    return textureSample(texture, texture_sampler, vs.tex_coords);\n}",
                                ),
                            ),
                        }
                    });
                let pipeline = self
                    .device
                    .create_render_pipeline(
                        &RenderPipelineDescriptor {
                            label: Some("wgpu::uti::TextureBlitter::pipeline"),
                            layout: Some(&pipeline_layout),
                            vertex: VertexState {
                                module: &shader,
                                entry_point: Some("vs_main"),
                                compilation_options: PipelineCompilationOptions::default(),
                                buffers: &[],
                            },
                            primitive: PrimitiveState {
                                topology: PrimitiveTopology::TriangleList,
                                strip_index_format: None,
                                front_face: FrontFace::Ccw,
                                cull_mode: None,
                                unclipped_depth: false,
                                polygon_mode: wgt::PolygonMode::Fill,
                                conservative: false,
                            },
                            depth_stencil: None,
                            multisample: MultisampleState::default(),
                            fragment: Some(FragmentState {
                                module: &shader,
                                entry_point: Some("fs_main"),
                                compilation_options: PipelineCompilationOptions::default(),
                                targets: &[
                                    Some(ColorTargetState {
                                        format: self.format,
                                        blend: self.blend_state,
                                        write_mask: ColorWrites::ALL,
                                    }),
                                ],
                            }),
                            multiview: None,
                            cache: None,
                        },
                    );
                TextureBlitter {
                    pipeline,
                    bind_group_layout,
                    sampler,
                }
            }
        }
        /// Texture Blitting (Copying) Utility
        ///
        /// Use this if you want to just render/copy texture A to texture B where [`CommandEncoder::copy_texture_to_texture`] would not work because:
        /// - Textures are in incompatible formats.
        /// - Textures are of different sizes.
        /// - Your copy destination is the surface texture and does not have the `COPY_DST` usage.
        pub struct TextureBlitter {
            pipeline: RenderPipeline,
            bind_group_layout: BindGroupLayout,
            sampler: Sampler,
        }
        impl TextureBlitter {
            /// Returns a [`TextureBlitter`] with default settings.
            ///
            /// # Arguments
            /// - `device` - A [`Device`]
            /// - `format` - The [`TextureFormat`] of the texture that will be copied to. This has to have the `RENDER_TARGET` usage.
            ///
            /// Properties of the blitting (such as the [`BlendState`]) can be customised by using [`TextureBlitterBuilder`] instead.
            pub fn new(device: &Device, format: TextureFormat) -> Self {
                TextureBlitterBuilder::new(device, format).build()
            }
            /// Copies the data from the source [`TextureView`] to the target [`TextureView`]
            ///
            /// # Arguments
            /// - `device` - A [`Device`]
            /// - `encoder` - A [`CommandEncoder`]
            /// - `source` - A [`TextureView`] that gets copied. The format does not matter.
            /// - `target` - A [`TextureView`] that gets the data copied from the `source`. It has to be the same format as the format specified in [`TextureBlitter::new`]
            pub fn copy(
                &self,
                device: &Device,
                encoder: &mut CommandEncoder,
                source: &TextureView,
                target: &TextureView,
            ) {
                let bind_group = device
                    .create_bind_group(
                        &BindGroupDescriptor {
                            label: Some("wgpu::util::TextureBlitter::bind_group"),
                            layout: &self.bind_group_layout,
                            entries: &[
                                BindGroupEntry {
                                    binding: 0,
                                    resource: crate::BindingResource::TextureView(source),
                                },
                                BindGroupEntry {
                                    binding: 1,
                                    resource: crate::BindingResource::Sampler(&self.sampler),
                                },
                            ],
                        },
                    );
                let mut pass = encoder
                    .begin_render_pass(
                        &RenderPassDescriptor {
                            label: Some("wgpu::util::TextureBlitter::pass"),
                            color_attachments: &[
                                Some(crate::RenderPassColorAttachment {
                                    view: target,
                                    depth_slice: None,
                                    resolve_target: None,
                                    ops: wgt::Operations {
                                        load: LoadOp::Load,
                                        store: StoreOp::Store,
                                    },
                                }),
                            ],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        },
                    );
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                pass.draw(0..3, 0..1);
            }
        }
    }
    use alloc::{borrow::Cow, format, string::String, vec};
    use core::{mem, ptr::copy_nonoverlapping};
    pub use belt::StagingBelt;
    pub use device::{BufferInitDescriptor, DeviceExt};
    pub use encoder::RenderEncoder;
    pub use init::*;
    pub use texture_blitter::{TextureBlitter, TextureBlitterBuilder};
    pub use wgt::{
        math::*, DispatchIndirectArgs, DrawIndexedIndirectArgs, DrawIndirectArgs,
        TextureDataOrder,
    };
    pub(crate) use mutex::Mutex;
    use crate::dispatch;
    const SPIRV_MAGIC_NUMBER: u32 = 0x0723_0203;
    const fn check_spirv_len(data: &[u8]) {
        if !(data.len() % size_of::<u32>() == 0) {
            {
                ::core::panicking::panic_fmt(
                    format_args!("SPIRV data size must be a multiple of 4."),
                );
            }
        }
        if !!data.is_empty() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("SPIRV data must not be empty."),
                );
            }
        }
    }
    const fn verify_spirv_magic(words: &[u32]) {
        if !(words[0] == SPIRV_MAGIC_NUMBER) {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Wrong magic word in data. Make sure you are using a binary SPIRV file.",
                    ),
                );
            }
        }
    }
    /// Version of `make_spirv` intended for use with [`Device::create_shader_module_passthrough`].
    /// Returns a raw slice instead of [`ShaderSource`](super::ShaderSource).
    ///
    /// [`Device::create_shader_module_passthrough`]: crate::Device::create_shader_module_passthrough
    pub fn make_spirv_raw(data: &[u8]) -> Cow<'_, [u32]> {
        check_spirv_len(data);
        let mut words = if data.as_ptr().align_offset(align_of::<u32>()) == 0 {
            let (pre, words, post) = unsafe { data.align_to::<u32>() };
            if true {
                if !pre.is_empty() {
                    ::core::panicking::panic("assertion failed: pre.is_empty()")
                }
            }
            if true {
                if !post.is_empty() {
                    ::core::panicking::panic("assertion failed: post.is_empty()")
                }
            }
            Cow::from(words)
        } else {
            let mut words = ::alloc::vec::from_elem(0u32, data.len() / size_of::<u32>());
            unsafe {
                copy_nonoverlapping(
                    data.as_ptr(),
                    words.as_mut_ptr() as *mut u8,
                    data.len(),
                );
            }
            Cow::from(words)
        };
        if words[0] == SPIRV_MAGIC_NUMBER.swap_bytes() {
            for word in Cow::to_mut(&mut words) {
                *word = word.swap_bytes();
            }
        }
        verify_spirv_magic(&words);
        words
    }
    /// Version of `make_spirv_raw` used for implementing [`include_spirv!`] and [`include_spirv_raw!`] macros.
    ///
    /// Not public API. Also, don't even try calling at runtime; you'll get a stack overflow.
    ///
    /// [`include_spirv!`]: crate::include_spirv
    #[doc(hidden)]
    pub const fn make_spirv_const<const IN: usize, const OUT: usize>(
        data: [u8; IN],
    ) -> [u32; OUT] {
        #[repr(align(4))]
        struct Aligned<T: ?Sized>(T);
        check_spirv_len(&data);
        if !(IN / 4 == OUT) {
            ::core::panicking::panic("assertion failed: IN / 4 == OUT")
        }
        let aligned = Aligned(data);
        let mut words: [u32; OUT] = unsafe { mem::transmute_copy(&aligned) };
        if words[0] == SPIRV_MAGIC_NUMBER.swap_bytes() {
            let mut idx = 0;
            while idx < words.len() {
                words[idx] = words[idx].swap_bytes();
                idx += 1;
            }
        }
        verify_spirv_magic(&words);
        words
    }
    /// CPU accessible buffer used to download data back from the GPU.
    pub struct DownloadBuffer {
        _gpu_buffer: super::Buffer,
        mapped_range: dispatch::DispatchBufferMappedRange,
    }
    impl DownloadBuffer {
        /// Asynchronously read the contents of a buffer.
        pub fn read_buffer(
            device: &super::Device,
            queue: &super::Queue,
            buffer: &super::BufferSlice<'_>,
            callback: impl FnOnce(Result<Self, super::BufferAsyncError>) + Send + 'static,
        ) {
            let size = buffer.size.into();
            let download = device
                .create_buffer(
                    &super::BufferDescriptor {
                        size,
                        usage: super::BufferUsages::COPY_DST
                            | super::BufferUsages::MAP_READ,
                        mapped_at_creation: false,
                        label: None,
                    },
                );
            let mut encoder = device
                .create_command_encoder(
                    &super::CommandEncoderDescriptor {
                        label: None,
                    },
                );
            encoder
                .copy_buffer_to_buffer(buffer.buffer, buffer.offset, &download, 0, size);
            let command_buffer: super::CommandBuffer = encoder.finish();
            queue.submit(Some(command_buffer));
            download
                .clone()
                .slice(..)
                .map_async(
                    super::MapMode::Read,
                    move |result| {
                        if let Err(e) = result {
                            callback(Err(e));
                            return;
                        }
                        let mapped_range = download.inner.get_mapped_range(0..size);
                        callback(
                            Ok(Self {
                                _gpu_buffer: download,
                                mapped_range,
                            }),
                        );
                    },
                );
        }
    }
    impl core::ops::Deref for DownloadBuffer {
        type Target = [u8];
        fn deref(&self) -> &[u8] {
            self.mapped_range.slice()
        }
    }
    /// A recommended key for storing [`PipelineCache`]s for the adapter
    /// associated with the given [`AdapterInfo`](wgt::AdapterInfo)
    /// This key will define a class of adapters for which the same cache
    /// might be valid.
    ///
    /// If this returns `None`, the adapter doesn't support [`PipelineCache`].
    /// This may be because the API doesn't support application managed caches
    /// (such as browser WebGPU), or that `wgpu` hasn't implemented it for
    /// that API yet.
    ///
    /// This key could be used as a filename, as seen in the example below.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::path::PathBuf;
    /// use wgpu::PipelineCacheDescriptor;
    /// # let adapter_info = todo!();
    /// # let device: wgpu::Device = todo!();
    /// let cache_dir: PathBuf = unimplemented!("Some reasonable platform-specific cache directory for your app.");
    /// let filename = wgpu::util::pipeline_cache_key(&adapter_info);
    /// let (pipeline_cache, cache_file) = if let Some(filename) = filename {
    ///     let cache_path = cache_dir.join(&filename);
    ///     // If we failed to read the cache, for whatever reason, treat the data as lost.
    ///     // In a real app, we'd probably avoid caching entirely unless the error was "file not found".
    ///     let cache_data = std::fs::read(&cache_path).ok();
    ///     let pipeline_cache = unsafe {
    ///         device.create_pipeline_cache(&PipelineCacheDescriptor {
    ///             data: cache_data.as_deref(),
    ///             label: None,
    ///             fallback: true
    ///         })
    ///     };
    ///     (Some(pipeline_cache), Some(cache_path))
    /// } else {
    ///     (None, None)
    /// };
    ///
    /// // Run pipeline initialisation, making sure to set the `cache`
    /// // fields of your `*PipelineDescriptor` to `pipeline_cache`
    ///
    /// // And then save the resulting cache (probably off the main thread).
    /// if let (Some(pipeline_cache), Some(cache_file)) = (pipeline_cache, cache_file) {
    ///     let data = pipeline_cache.get_data();
    ///     if let Some(data) = data {
    ///         let temp_file = cache_file.with_extension("temp");
    ///         std::fs::write(&temp_file, &data)?;
    ///         std::fs::rename(&temp_file, &cache_file)?;
    ///     }
    /// }
    /// # Ok::<_, std::io::Error>(())
    /// ```
    ///
    /// [`PipelineCache`]: super::PipelineCache
    pub fn pipeline_cache_key(adapter_info: &wgt::AdapterInfo) -> Option<String> {
        match adapter_info.backend {
            wgt::Backend::Vulkan => {
                Some(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "wgpu_pipeline_cache_vulkan_{0}_{1}",
                                adapter_info.vendor,
                                adapter_info.device,
                            ),
                        )
                    }),
                )
            }
            _ => None,
        }
    }
    /// Adds extra conversion functions to `TextureFormat`.
    pub trait TextureFormatExt {
        /// Finds the [`TextureFormat`](wgt::TextureFormat) corresponding to the given
        /// [`StorageFormat`](wgc::naga::StorageFormat).
        ///
        /// # Examples
        /// ```
        /// use wgpu::util::TextureFormatExt;
        /// assert_eq!(wgpu::TextureFormat::from_storage_format(wgpu::naga::StorageFormat::Bgra8Unorm), wgpu::TextureFormat::Bgra8Unorm);
        /// ```
        fn from_storage_format(storage_format: crate::naga::StorageFormat) -> Self;
        /// Finds the [`StorageFormat`](wgc::naga::StorageFormat) corresponding to the given [`TextureFormat`](wgt::TextureFormat).
        /// Returns `None` if there is no matching storage format,
        /// which typically indicates this format is not supported
        /// for storage textures.
        ///
        /// # Examples
        /// ```
        /// use wgpu::util::TextureFormatExt;
        /// assert_eq!(wgpu::TextureFormat::Bgra8Unorm.to_storage_format(), Some(wgpu::naga::StorageFormat::Bgra8Unorm));
        /// ```
        fn to_storage_format(&self) -> Option<crate::naga::StorageFormat>;
    }
    impl TextureFormatExt for wgt::TextureFormat {
        fn from_storage_format(storage_format: crate::naga::StorageFormat) -> Self {
            wgc::map_storage_format_from_naga(storage_format)
        }
        fn to_storage_format(&self) -> Option<crate::naga::StorageFormat> {
            wgc::map_storage_format_to_naga(*self)
        }
    }
}
pub use api::*;
pub use wgt::{
    AdapterInfo, AddressMode, AllocatorReport, AstcBlock, AstcChannel, Backend,
    BackendOptions, Backends, BindGroupLayoutEntry, BindingType, BlendComponent,
    BlendFactor, BlendOperation, BlendState, BufferAddress, BufferBindingType,
    BufferSize, BufferTextureCopyInfo, BufferTransition, BufferUsages, BufferUses, Color,
    ColorTargetState, ColorWrites, CommandBufferDescriptor, CompareFunction,
    CompositeAlphaMode, CopyExternalImageDestInfo, CoreCounters, DepthBiasState,
    DepthStencilState, DeviceLostReason, DeviceType, DownlevelCapabilities,
    DownlevelFlags, DownlevelLimits, Dx12BackendOptions, Dx12Compiler, DxcShaderModel,
    DynamicOffset, ExperimentalFeatures, Extent3d, ExternalTextureFormat,
    ExternalTextureTransferFunction, Face, Features, FeaturesWGPU, FeaturesWebGPU,
    FilterMode, FrontFace, GlBackendOptions, GlFenceBehavior, Gles3MinorVersion,
    HalCounters, ImageSubresourceRange, IndexFormat, InstanceDescriptor, InstanceFlags,
    InternalCounters, Limits, MemoryBudgetThresholds, MemoryHints, MipmapFilterMode,
    MultisampleState, NoopBackendOptions, Origin2d, Origin3d, PipelineStatisticsTypes,
    PollError, PollStatus, PolygonMode, PowerPreference, PredefinedColorSpace,
    PresentMode, PresentationTimestamp, PrimitiveState, PrimitiveTopology,
    PushConstantRange, QueryType, RenderBundleDepthStencil, RequestAdapterError,
    SamplerBindingType, SamplerBorderColor, ShaderLocation, ShaderModel,
    ShaderRuntimeChecks, ShaderStages, StencilFaceState, StencilOperation, StencilState,
    StorageTextureAccess, SurfaceCapabilities, SurfaceStatus, TexelCopyBufferLayout,
    TextureAspect, TextureDimension, TextureFormat, TextureFormatFeatureFlags,
    TextureFormatFeatures, TextureSampleType, TextureTransition, TextureUsages,
    TextureUses, TextureViewDimension, Trace, VertexAttribute, VertexFormat,
    VertexStepMode, WasmNotSend, WasmNotSendSync, WasmNotSync, COPY_BUFFER_ALIGNMENT,
    COPY_BYTES_PER_ROW_ALIGNMENT, MAP_ALIGNMENT, PUSH_CONSTANT_ALIGNMENT,
    QUERY_RESOLVE_BUFFER_ALIGNMENT, QUERY_SET_MAX_QUERIES, QUERY_SIZE, VERTEX_ALIGNMENT,
};
#[expect(deprecated)]
pub use wgt::VERTEX_STRIDE_ALIGNMENT;
/// Re-export of our `naga` dependency.
///
pub use ::wgc::naga;
/// Re-export of our `raw-window-handle` dependency.
///
pub use raw_window_handle as rwh;
#[doc(hidden)]
pub use macros::helpers as __macro_helpers;
