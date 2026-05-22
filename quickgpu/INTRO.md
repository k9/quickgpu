
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
# use wgpu_28::*;
# use quickgpu::*;
# use bytemuck::{Pod, Zeroable};
#
# let (device, _queue) = wgpu_28::Device::noop(&wgpu_28::DeviceDescriptor::default());
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
            .buffers(&builders([vertex_buffer_layout()
                .array_stride(size_of::<VertexInput>() as wgpu_28::BufferAddress)
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
