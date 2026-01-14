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

mod render_builder {
    use super::super::builders::render_pipeline_descriptor_builder::*;

    impl<'a, CS: Complete<'a>> RenderPipelineDescriptorBuilder<'a, CS> {
        pub fn create_with(self, device: &wgpu::Device) -> wgpu::RenderPipeline {
            device.create_render_pipeline(&self.build())
        }
    }
}

mod buffer_init_builder {
    use super::super::builders::buffer_init_descriptor_builder::*;
    use wgpu::{util::DeviceExt, Device};

    impl<'a, CS: Complete<'a>> BufferInitDescriptorBuilder<'a, CS> {
        pub fn create_with(self, device: &Device) -> wgpu::Buffer {
            device.create_buffer_init(&self.build())
        }
    }
}

mod command_encoder_builder {
    use super::super::builders::command_encoder_descriptor_builder::*;
    use wgpu::Device;

    impl<'a, CS: Complete<'a>> CommandEncoderDescriptorBuilder<'a, CS> {
        pub fn create_with(self, device: &Device) -> wgpu::CommandEncoder {
            device.create_command_encoder(&self.build())
        }
    }
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
