
<!-- intro that gets used in README and in crate docs -->

`quickgpu` wraps the `wgpu` API allowing users to write shorter, clearer code.
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

# WGPU versions

Support for each version of `wgpu` can be enabled by enabling its feature,
for example `v29`. The API will then be available in the `v29` mod of `quickgpu`.
To keep things simple, this documentation is built with just one version enabled.
To get docs for a different version, you can run `cargo doc` locally.

# Using builders

To create a builder for `wgpu::FragmentState`, you can call the
`fragment_state()` helper function, which returns a `FragmentStateBuilder`.
Alternatively, you can create a `FragmentStateBuilder` directly.

Many wgpu structs take an optional `wgpu::Label` to identify the struct.
In quickgpu, the label field will always be the only parameter to the helper
function (see the `render_pipeline_descriptor` call in the example below).

If a builder field setter accepts a single value of a type which also has a builder, you
can nest builders, and skip calling `build()` on the inner builder. In order to skip
calling `build()` on the elements of a slice, use the `arr!` macro.

# Custom bind group builder

The `create_binds` macro creates custom builders to manage your bind groups.
For example, calling `create_binds!(MyBinds, texture, sampler)` will generate
a `MyBinds` struct with:
  - a `builder` function to help you build a MyBinds instance. Set up the
`texture` and `sampler` binds, then call `build()`
  - a `layout` function which returns a BindGroupLayout
  - a `group` which returns a custom BindGroup builder. Fill in resources
    for each bind, then call `create`.

See `bunnymark/src/scene.rs` for examples of using `create_binds`.

# Example

 ```
# use wgpu_29::*;
# use quickgpu::v29::*;
# use bytemuck::{Pod, Zeroable};
#
# let (device, _queue) = wgpu_29::Device::noop(&wgpu_29::DeviceDescriptor::default());
# let shader = device.create_shader_module(include_wgsl!("../example/shaders/base.wgsl"));
# let format = TextureFormat::R8Unorm;
#
# #[derive(Pod, Zeroable, Clone, Copy)]
# #[repr(C)]
# pub struct VertexInput {
#     pub position: [f32; 4],
#     pub uv: [f32; 2],
# }
// Label "Render Pipeline" is passed to builder initializer
let render_pipeline = render_pipeline_descriptor(Some("Render Pipeline"))
    .vertex(
        vertex_state()
            .module(&shader)
            .entry_point("vs_main")
            // Use builders() to convert builders to values before passing as a slice
            .buffers(&arr![vertex_buffer_layout()
                .array_stride(size_of::<VertexInput>() as wgpu_29::BufferAddress)
                .attributes(&arr![
                    vertex_attribute()
                        .format(VertexFormat::Float32x4)
                        .offset(0u64)
                        .shader_location(0u32)
                ])]),
    )
    .fragment(
        fragment_state()
            .module(&shader)
            .entry_point("fs_main")
            .targets(&[Some(format.into())])
    )
    .build(); // Return the wgpu struct
 ```
