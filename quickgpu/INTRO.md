<!-- intro that gets used in README and in crate docs -->

 `quickgpu` wraps the [wgpu] API allowing users to write shorter, clearer code.
 It consists of builders for wgpu structs. As a wrapper library, quickgpu doesn't
 manage or own any state after a builder is done building. There's no need to convert
 all of your code to quickgpu, you can just use it where it's helpful.

 After setting fields in a builder, call `build()`, which will return the wgpu struct.
 Many wgpu structs take an optional `wgpu::Label` to identify the struct.
 In quickgpu, the label field will always be the only parameter of the builder
 initializer.

 If a builder's field accepts a single value of a type which also has a builder, you
 can nest builders, and skip calling `build()` on the inner builder. However, you
 can't skip `build()` for elements of a slice even when they are nested.


 ```
// Label "Render Pipeline" is passed to builder initializer
let render_pipeline = render_pipeline_descriptor("Render Pipeline")
    .vertex(
        vertex_state()
            .module(&shader)
            .entry_point("vs_main")
            .buffers(&[vertex_buffer_layout()
                .array_stride(size_of::<VertexInput>() as wgpu::BufferAddress)
                .attributes(&[
                    vertex_attribute()
                        .format(VertexFormat::Float32x4)
                        .offset(0u64)
                        .shader_location(0u32)
                        // Even though this is nested,
                        // build() must be called since it's inside a slice
                        .build()
                ])
                .build()]),
    )
    .fragment(
        fragment_state()
            .module(&shader)
            .entry_point("fs_main")
            .targets(&[Some(format.into())])
             // Since fragment_state is a nested builder,
             // we don't need to call build()
    )
    .build(); // Return the wgpu struct
 ```
