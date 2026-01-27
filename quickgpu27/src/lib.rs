#![doc=include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INTRO.md"))]

pub mod custom;
pub use custom::*;
pub mod builders;

#[doc(inline)]
pub use builders::backend_options_builder::backend_options;

#[doc(inline)]
pub use builders::bind_group_descriptor_builder::bind_group_descriptor;

#[doc(inline)]
pub use builders::bind_group_entry_builder::bind_group_entry;

#[doc(inline)]
pub use builders::bind_group_layout_descriptor_builder::bind_group_layout_descriptor;

#[doc(inline)]
pub use builders::bind_group_layout_entry_builder::bind_group_layout_entry;

#[doc(inline)]
pub use builders::blas_build_entry_builder::blas_build_entry;

#[doc(inline)]
pub use builders::blas_triangle_geometry_builder::blas_triangle_geometry;

#[doc(inline)]
pub use builders::blend_component_builder::blend_component;

#[doc(inline)]
pub use builders::blend_state_builder::blend_state;

#[doc(inline)]
pub use builders::buffer_binding_builder::buffer_binding;

#[doc(inline)]
pub use builders::buffer_descriptor_builder::buffer_descriptor;

#[doc(inline)]
pub use builders::buffer_init_descriptor_builder::buffer_init_descriptor;

#[doc(inline)]
pub use builders::buffer_transition_builder::buffer_transition;

#[doc(inline)]
pub use builders::color_builder::color;

#[doc(inline)]
pub use builders::color_target_state_builder::color_target_state;

#[doc(inline)]
pub use builders::command_buffer_descriptor_builder::command_buffer_descriptor;

#[doc(inline)]
pub use builders::command_encoder_descriptor_builder::command_encoder_descriptor;

#[doc(inline)]
pub use builders::compilation_info_builder::compilation_info;

#[doc(inline)]
pub use builders::compute_pass_descriptor_builder::compute_pass_descriptor;

#[doc(inline)]
pub use builders::compute_pass_timestamp_writes_builder::compute_pass_timestamp_writes;

#[doc(inline)]
pub use builders::compute_pipeline_descriptor_builder::compute_pipeline_descriptor;

#[doc(inline)]
pub use builders::copy_external_image_dest_info_builder::copy_external_image_dest_info;

#[doc(inline)]
pub use builders::create_blas_descriptor_builder::create_blas_descriptor;

#[doc(inline)]
pub use builders::create_tlas_descriptor_builder::create_tlas_descriptor;

#[doc(inline)]
pub use builders::depth_bias_state_builder::depth_bias_state;

#[doc(inline)]
pub use builders::depth_stencil_state_builder::depth_stencil_state;

#[doc(inline)]
pub use builders::device_descriptor_builder::device_descriptor;

#[doc(inline)]
pub use builders::dispatch_indirect_args_builder::dispatch_indirect_args;

#[doc(inline)]
pub use builders::downlevel_limits_builder::downlevel_limits;

#[doc(inline)]
pub use builders::draw_indexed_indirect_args_builder::draw_indexed_indirect_args;

#[doc(inline)]
pub use builders::draw_indirect_args_builder::draw_indirect_args;

#[doc(inline)]
pub use builders::dx_12_backend_options_builder::dx_12_backend_options;

#[doc(inline)]
pub use builders::extent_3_d_builder::extent_3_d;

#[doc(inline)]
pub use builders::external_texture_descriptor_builder::external_texture_descriptor;

#[doc(inline)]
pub use builders::external_texture_transfer_function_builder::external_texture_transfer_function;

#[doc(inline)]
pub use builders::fragment_state_builder::fragment_state;

#[doc(inline)]
pub use builders::gl_backend_options_builder::gl_backend_options;

#[doc(inline)]
pub use builders::image_subresource_range_builder::image_subresource_range;

#[doc(inline)]
pub use builders::instance_descriptor_builder::instance_descriptor;

#[doc(inline)]
pub use builders::memory_budget_thresholds_builder::memory_budget_thresholds;

#[doc(inline)]
pub use builders::mesh_pipeline_descriptor_builder::mesh_pipeline_descriptor;

#[doc(inline)]
pub use builders::mesh_state_builder::mesh_state;

#[doc(inline)]
pub use builders::multisample_state_builder::multisample_state;

#[doc(inline)]
pub use builders::noop_backend_options_builder::noop_backend_options;

#[doc(inline)]
pub use builders::operations_builder::operations;

#[doc(inline)]
pub use builders::origin_2_d_builder::origin_2_d;

#[doc(inline)]
pub use builders::origin_3_d_builder::origin_3_d;

#[doc(inline)]
pub use builders::pipeline_cache_descriptor_builder::pipeline_cache_descriptor;

#[doc(inline)]
pub use builders::pipeline_compilation_options_builder::pipeline_compilation_options;

#[doc(inline)]
pub use builders::pipeline_layout_descriptor_builder::pipeline_layout_descriptor;

#[doc(inline)]
pub use builders::primitive_state_builder::primitive_state;

#[doc(inline)]
pub use builders::push_constant_range_builder::push_constant_range;

#[doc(inline)]
pub use builders::query_set_descriptor_builder::query_set_descriptor;

#[doc(inline)]
pub use builders::render_bundle_depth_stencil_builder::render_bundle_depth_stencil;

#[doc(inline)]
pub use builders::render_bundle_descriptor_builder::render_bundle_descriptor;

#[doc(inline)]
pub use builders::render_bundle_encoder_descriptor_builder::render_bundle_encoder_descriptor;

#[doc(inline)]
pub use builders::render_pass_color_attachment_builder::render_pass_color_attachment;

#[doc(inline)]
pub use builders::render_pass_depth_stencil_attachment_builder::render_pass_depth_stencil_attachment;

#[doc(inline)]
pub use builders::render_pass_descriptor_builder::render_pass_descriptor;

#[doc(inline)]
pub use builders::render_pass_timestamp_writes_builder::render_pass_timestamp_writes;

#[doc(inline)]
pub use builders::render_pipeline_descriptor_builder::render_pipeline_descriptor;

#[doc(inline)]
pub use builders::request_adapter_options_builder::request_adapter_options;

#[doc(inline)]
pub use builders::request_adapter_options_base_builder::request_adapter_options_base;

#[doc(inline)]
pub use builders::sampler_descriptor_builder::sampler_descriptor;

#[doc(inline)]
pub use builders::shader_module_descriptor_builder::shader_module_descriptor;

#[doc(inline)]
pub use builders::shader_module_descriptor_passthrough_builder::shader_module_descriptor_passthrough;

#[doc(inline)]
pub use builders::shader_runtime_checks_builder::shader_runtime_checks;

#[doc(inline)]
pub use builders::stencil_face_state_builder::stencil_face_state;

#[doc(inline)]
pub use builders::stencil_state_builder::stencil_state;

#[doc(inline)]
pub use builders::surface_configuration_builder::surface_configuration;

#[doc(inline)]
pub use builders::task_state_builder::task_state;

#[doc(inline)]
pub use builders::texel_copy_buffer_info_builder::texel_copy_buffer_info;

#[doc(inline)]
pub use builders::texel_copy_buffer_info_base_builder::texel_copy_buffer_info_base;

#[doc(inline)]
pub use builders::texel_copy_buffer_layout_builder::texel_copy_buffer_layout;

#[doc(inline)]
pub use builders::texel_copy_texture_info_builder::texel_copy_texture_info;

#[doc(inline)]
pub use builders::texel_copy_texture_info_base_builder::texel_copy_texture_info_base;

#[doc(inline)]
pub use builders::texture_descriptor_builder::texture_descriptor;

#[doc(inline)]
pub use builders::texture_transition_builder::texture_transition;

#[doc(inline)]
pub use builders::texture_view_descriptor_builder::texture_view_descriptor;

#[doc(inline)]
pub use builders::vertex_attribute_builder::vertex_attribute;

#[doc(inline)]
pub use builders::vertex_buffer_layout_builder::vertex_buffer_layout;

#[doc(inline)]
pub use builders::vertex_state_builder::vertex_state;
