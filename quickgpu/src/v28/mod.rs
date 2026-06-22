use std::num::NonZero;
pub mod builders;
mod create_binds;
#[doc(inline)]
pub use crate::_arr_option_v28 as arr_option;
#[doc(inline)]
pub use crate::_arr_v28 as arr;
#[doc(inline)]
pub use crate::_create_binds_v28 as create_binds;
mod render_pass_builder {
    use super::builders::render_pass_descriptor_builder::*;
    use wgpu_28::CommandEncoder;
    impl<'a, CS: state::IsComplete> RenderPassDescriptorBuilder<'a, CS> {
        pub fn begin_with(self, encoder: &'a mut CommandEncoder) -> wgpu_28::RenderPass<'a> {
            encoder.begin_render_pass(&self.build())
        }
    }
}
#[derive(bon :: Builder)]
pub struct Binding {
    pub binding: u32,
    pub visibility: wgpu_28::ShaderStages,
    pub ty: wgpu_28::BindingType,
    pub count: Option<NonZero<u32>>,
}
pub trait NestedBinding {
    fn unnest(self) -> Binding;
}
impl NestedBinding for Binding {
    fn unnest(self) -> Binding {
        self
    }
}
impl<S: binding_builder::IsComplete> NestedBinding for BindingBuilder<S> {
    fn unnest(self) -> Binding {
        self.build()
    }
}
mod layout_entry {
    use super::builders::bind_group_layout_entry_builder::*;
    pub type LayoutEntryCustom = BindGroupLayoutEntryBuilder<
        state::SetCount<state::SetTy<state::SetVisibility<state::SetBinding<state::Empty>>>>,
    >;
    impl super::Binding {
        pub fn layout_entry(&self) -> LayoutEntryCustom {
            bind_group_layout_entry()
                .binding(self.binding)
                .visibility(self.visibility)
                .ty(self.ty)
                .maybe_count(self.count)
        }
    }
}
pub use entry::EntryCustom;
mod entry {
    use super::builders::bind_group_entry_builder::*;
    pub type EntryCustom<'a> = BindGroupEntryBuilder<'a, state::SetBinding<state::Empty>>;
    impl super::Binding {
        pub fn entry<'a>(&self) -> EntryCustom<'a> {
            bind_group_entry().binding(self.binding)
        }
    }
}
pub use builders::*;

#[doc(inline)]
pub use builders::backend_options;

#[doc(inline)]
pub use builders::bind_group_descriptor;

#[doc(inline)]
pub use builders::bind_group_entry;

#[doc(inline)]
pub use builders::bind_group_layout_descriptor;

#[doc(inline)]
pub use builders::bind_group_layout_entry;

#[doc(inline)]
pub use builders::blas_build_entry;

#[doc(inline)]
pub use builders::blas_triangle_geometry;

#[doc(inline)]
pub use builders::blend_component;

#[doc(inline)]
pub use builders::blend_state;

#[doc(inline)]
pub use builders::buffer_binding;

#[doc(inline)]
pub use builders::buffer_descriptor;

#[doc(inline)]
pub use builders::buffer_init_descriptor;

#[doc(inline)]
pub use builders::buffer_transition;

#[doc(inline)]
pub use builders::color;

#[doc(inline)]
pub use builders::color_target_state;

#[doc(inline)]
pub use builders::command_buffer_descriptor;

#[doc(inline)]
pub use builders::command_encoder_descriptor;

#[doc(inline)]
pub use builders::compilation_info;

#[doc(inline)]
pub use builders::compute_pass_descriptor;

#[doc(inline)]
pub use builders::compute_pass_timestamp_writes;

#[doc(inline)]
pub use builders::compute_pipeline_descriptor;

#[doc(inline)]
pub use builders::copy_external_image_dest_info;

#[doc(inline)]
pub use builders::create_blas_descriptor;

#[doc(inline)]
pub use builders::create_tlas_descriptor;

#[doc(inline)]
pub use builders::depth_bias_state;

#[doc(inline)]
pub use builders::depth_stencil_state;

#[doc(inline)]
pub use builders::device_descriptor;

#[doc(inline)]
pub use builders::dispatch_indirect_args;

#[doc(inline)]
pub use builders::downlevel_limits;

#[doc(inline)]
pub use builders::draw_indexed_indirect_args;

#[doc(inline)]
pub use builders::draw_indirect_args;

#[doc(inline)]
pub use builders::dx12_backend_options;

#[doc(inline)]
pub use builders::extent3d;

#[doc(inline)]
pub use builders::external_texture_descriptor;

#[doc(inline)]
pub use builders::external_texture_transfer_function;

#[doc(inline)]
pub use builders::fragment_state;

#[doc(inline)]
pub use builders::gl_backend_options;

#[doc(inline)]
pub use builders::image_subresource_range;

#[doc(inline)]
pub use builders::instance_descriptor;

#[doc(inline)]
pub use builders::memory_budget_thresholds;

#[doc(inline)]
pub use builders::mesh_pipeline_descriptor;

#[doc(inline)]
pub use builders::mesh_state;

#[doc(inline)]
pub use builders::multisample_state;

#[doc(inline)]
pub use builders::noop_backend_options;

#[doc(inline)]
pub use builders::operations;

#[doc(inline)]
pub use builders::origin2d;

#[doc(inline)]
pub use builders::origin3d;

#[doc(inline)]
pub use builders::pipeline_cache_descriptor;

#[doc(inline)]
pub use builders::pipeline_compilation_options;

#[doc(inline)]
pub use builders::pipeline_layout_descriptor;

#[doc(inline)]
pub use builders::primitive_state;

#[doc(inline)]
pub use builders::query_set_descriptor;

#[doc(inline)]
pub use builders::render_bundle_depth_stencil;

#[doc(inline)]
pub use builders::render_bundle_descriptor;

#[doc(inline)]
pub use builders::render_bundle_encoder_descriptor;

#[doc(inline)]
pub use builders::render_pass_color_attachment;

#[doc(inline)]
pub use builders::render_pass_depth_stencil_attachment;

#[doc(inline)]
pub use builders::render_pass_descriptor;

#[doc(inline)]
pub use builders::render_pass_timestamp_writes;

#[doc(inline)]
pub use builders::render_pipeline_descriptor;

#[doc(inline)]
pub use builders::request_adapter_options;

#[doc(inline)]
pub use builders::request_adapter_options_base;

#[doc(inline)]
pub use builders::sampler_descriptor;

#[doc(inline)]
pub use builders::shader_module_descriptor;

#[doc(inline)]
pub use builders::shader_module_descriptor_passthrough;

#[doc(inline)]
pub use builders::shader_runtime_checks;

#[doc(inline)]
pub use builders::stencil_face_state;

#[doc(inline)]
pub use builders::stencil_state;

#[doc(inline)]
pub use builders::surface_configuration;

#[doc(inline)]
pub use builders::task_state;

#[doc(inline)]
pub use builders::texel_copy_buffer_info;

#[doc(inline)]
pub use builders::texel_copy_buffer_info_base;

#[doc(inline)]
pub use builders::texel_copy_buffer_layout;

#[doc(inline)]
pub use builders::texel_copy_texture_info;

#[doc(inline)]
pub use builders::texel_copy_texture_info_base;

#[doc(inline)]
pub use builders::texture_descriptor;

#[doc(inline)]
pub use builders::texture_transition;

#[doc(inline)]
pub use builders::texture_view_descriptor;

#[doc(inline)]
pub use builders::vertex_attribute;

#[doc(inline)]
pub use builders::vertex_buffer_layout;

#[doc(inline)]
pub use builders::vertex_state;
