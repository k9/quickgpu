use crate::generate::Version;
use quote::quote;

pub fn intro(version: Version) -> String {
    let wgpu_source_ident = version.wgpu_source_ident();
    let wgpu_version_mod = version.wgpu_version_mod();

    format!(
        r#"
<!-- intro that gets used in README and in crate docs -->

`quickgpu` wraps the [wgpu] API allowing users to write shorter, clearer code.
It consists of builders for wgpu structs. As a wrapper library, quickgpu doesn't
manage or own any state after a builder is done building. There's no need to convert
all of your code to quickgpu, you can just use it where it's helpful.

quickgpu's goal is that for any wgpu struct SomeStruct,
as long at doesn't contain private fields,
you can call the function `quickgpu::some_struct`,
and it will return a builder. When you're done setting
fields, call `build()`, and a wgpu struct will be returned.

Even structs with zero or one fields have builders.
These are not particularly useful, but are included
so that developers don't have to memorize which structs have builders.

# WGPU Versions

There are different quickgpu crates for different wgpu major versions:

- `quickgpu` supports `wgpu` version 28
- `quickgpu27` supports `wgpu` version 27


# Using builders

To create a builder for `wgpu::FragmentState`, you can call the
`fragment_state()` helper function, which returns a `FragmentStateBuilder`.
Alternatively, you can create a `FragmentStateBuilder` directly.

Many wgpu structs take an optional `wgpu::Label` to identify the struct.
In quickgpu, the label field will always be the only parameter to the helper
function (see the `render_pipeline_descriptor` call in the example below).

If a builder field setter accepts a single value of a type which also has a builder, you
can nest builders, and skip calling `build()` on the inner builder. In order to skip
calling `build()` on the elements of a slice, use the `builders` helper function.

 ```
# use {wgpu_source_ident}::*;
# use quickgpu::{wgpu_version_mod}::*;
# use bytemuck::{{Pod, Zeroable}};
#
# let (device, _queue) = {wgpu_source_ident}::Device::noop(&{wgpu_source_ident}::DeviceDescriptor::default());
# let shader = device.create_shader_module(include_wgsl!("../example/shaders/base.wgsl"));
# let format = TextureFormat::R8Unorm;
#
# #[derive(Pod, Zeroable, Clone, Copy)]
# #[repr(C)]
# pub struct VertexInput {{
#     pub position: [f32; 4],
#     pub uv: [f32; 2],
# }}
// Label "Render Pipeline" is passed to builder initializer
let render_pipeline = render_pipeline_descriptor(Some("Render Pipeline"))
    .vertex(
        vertex_state()
            .module(&shader)
            .entry_point("vs_main")
            // Use builders() to convert builders to values before passing as a slice
            .buffers(&builders([vertex_buffer_layout()
                .array_stride(size_of::<VertexInput>() as {wgpu_source_ident}::BufferAddress)
                .attributes(&builders([
                    vertex_attribute()
                        .format(VertexFormat::Float32x4)
                        .offset(0u64)
                        .shader_location(0u32)
                ]))])),
    )
    .fragment(
        fragment_state()
            .module(&shader)
            .entry_point("fs_main")
            .targets(&[Some(format.into())])
    )
    .build(); // Return the wgpu struct
 ```
"#
    )
}

pub fn custom(version: Version) -> String {
    let wgpu_source_ident = version.wgpu_source_ident();
    quote!(
        pub mod builders;

        /// Nested is implemented on builder types to enable passing them directly
        /// into other builders without needing to call build().
        pub trait Nested<T> {
            fn unnest(self) -> T;
        }

        impl<T, N: Nested<T>> Nested<Option<T>> for Option<N> {
            fn unnest(self) -> Option<T> {
                self.map(|o| o.unnest())
            }
        }

        /// Builds an array of builders into an array of values.
        /// Useful when passing a slice of wgpu values into another builder.
        pub fn builders<T, N: Nested<T>, const COUNT: usize>(a: [N; COUNT]) -> Vec<T> {
            a.into_iter().map(|item| item.unnest()).collect()
        }

        mod render_pass_builder {
            use super::builders::render_pass_descriptor_builder::*;
            use #wgpu_source_ident::CommandEncoder;

            impl<'a, CS: Complete<'a>> RenderPassDescriptorBuilder<'a, CS> {
                pub fn begin_with(self, encoder: &'a mut CommandEncoder) -> #wgpu_source_ident::RenderPass<'a> {
                    encoder.begin_render_pass(&self.build())
                }
            }
        }
    )
    .to_string()
}
