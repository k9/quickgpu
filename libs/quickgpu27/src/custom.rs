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
    use super::super::builders::render_pass_descriptor_builder::*;
    use wgpu::CommandEncoder;

    impl<'a, CS: Complete<'a>> RenderPassDescriptorBuilder<'a, CS> {
        pub fn begin_with(self, encoder: &'a mut CommandEncoder) -> wgpu::RenderPass<'a> {
            encoder.begin_render_pass(&self.build())
        }
    }
}

mod buffer_binding_builder {
    use crate::builders::bind_group_entry_builder::{self as bge_builder};
    use crate::builders::buffer_binding_builder::*;

    impl<'a, CS: Complete<'a>> BufferBindingBuilder<'a, CS> {
        pub fn as_entry(
            self,
            binding: u32,
        ) -> bge_builder::BindGroupEntryBuilder<
            'a,
            bge_builder::SetBinding<bge_builder::SetResource<bge_builder::Empty>>,
        > {
            bge_builder::bind_group_entry()
                .resource(wgpu::BindingResource::Buffer(self.build()))
                .binding(binding)
        }
    }
}
