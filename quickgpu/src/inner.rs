

use std::ops::Range;
use std::num::NonZeroU32;



# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn request_adapter_options_base < S > (power_preference : instance :: PowerPreference , force_fallback_adapter : bool , compatible_surface : Option < S >) -> wgpu :: RequestAdapterOptionsBase < S > { wgpu :: RequestAdapterOptionsBase { power_preference , force_fallback_adapter , compatible_surface } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texel_copy_texture_info_base < T > (texture : T , mip_level : u32 , origin : Origin3d , aspect : TextureAspect) -> wgpu :: TexelCopyTextureInfoBase < T > { wgpu :: TexelCopyTextureInfoBase { texture , mip_level , origin , aspect } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texel_copy_buffer_info_base < B > (buffer : B , layout : TexelCopyBufferLayout) -> wgpu :: TexelCopyBufferInfoBase < B > { wgpu :: TexelCopyBufferInfoBase { buffer , layout } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn operations < V > (load : command :: LoadOp < V > , store : command :: StoreOp) -> wgpu :: Operations < V > { wgpu :: Operations { load , store } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_descriptor < 'a > (label : Label < 'a > , layout : & 'a BindGroupLayout , entries : & 'a [BindGroupEntry < 'a >]) -> wgpu :: BindGroupDescriptor < 'a > { wgpu :: BindGroupDescriptor { label , layout , entries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_entry < 'a > (binding : u32 , resource : BindingResource < 'a >) -> wgpu :: BindGroupEntry < 'a > { wgpu :: BindGroupEntry { binding , resource } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_binding < 'a > (buffer : & 'a Buffer , offset : BufferAddress , size : Option < BufferSize >) -> wgpu :: BufferBinding < 'a > { wgpu :: BufferBinding { buffer , offset , size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_layout_descriptor < 'a > (label : Label < 'a > , entries : & 'a [BindGroupLayoutEntry]) -> wgpu :: BindGroupLayoutDescriptor < 'a > { wgpu :: BindGroupLayoutDescriptor { label , entries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blas_build_entry < 'a > (blas : & 'a Blas , geometry : BlasGeometries < 'a >) -> wgpu :: BlasBuildEntry < 'a > { wgpu :: BlasBuildEntry { blas , geometry } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blas_triangle_geometry < 'a > (size : & 'a BlasTriangleGeometrySizeDescriptor , vertex_buffer : & 'a Buffer , first_vertex : u32 , vertex_stride : BufferAddress , index_buffer : Option < & 'a Buffer > , first_index : Option < u32 > , transform_buffer : Option < & 'a Buffer > , transform_buffer_offset : Option < BufferAddress >) -> wgpu :: BlasTriangleGeometry < 'a > { wgpu :: BlasTriangleGeometry { size , vertex_buffer , first_vertex , vertex_stride , index_buffer , first_index , transform_buffer , transform_buffer_offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_cache_descriptor < 'a > (label : Label < 'a > , data : Option < & 'a [u8] > , fallback : bool) -> wgpu :: PipelineCacheDescriptor < 'a > { wgpu :: PipelineCacheDescriptor { label , data , fallback } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_compilation_options < 'a > (constants : & 'a [(& 'a str , f64)] , zero_initialize_workgroup_memory : bool) -> wgpu :: PipelineCompilationOptions < 'a > { wgpu :: PipelineCompilationOptions { constants , zero_initialize_workgroup_memory } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pass_descriptor < 'a > (# [builder (default)] label : Label < 'a > , timestamp_writes : Option < ComputePassTimestampWrites < 'a > >) -> wgpu :: ComputePassDescriptor < 'a > { wgpu :: ComputePassDescriptor { label , timestamp_writes } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pass_timestamp_writes < 'a > (query_set : & 'a QuerySet , beginning_of_pass_write_index : Option < u32 > , end_of_pass_write_index : Option < u32 >) -> wgpu :: ComputePassTimestampWrites < 'a > { wgpu :: ComputePassTimestampWrites { query_set , beginning_of_pass_write_index , end_of_pass_write_index } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pipeline_descriptor < 'a > (label : Label < 'a > , layout : Option < & 'a PipelineLayout > , module : & 'a ShaderModule , entry_point : Option < & 'a str > , compilation_options : PipelineCompilationOptions < 'a > , cache : Option < & 'a PipelineCache >) -> wgpu :: ComputePipelineDescriptor < 'a > { wgpu :: ComputePipelineDescriptor { label , layout , module , entry_point , compilation_options , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_layout_descriptor < 'a > (# [builder (default)] label : Label < 'a > , # [builder (default)] bind_group_layouts : & 'a [& 'a BindGroupLayout] , # [builder (default)] push_constant_ranges : & 'a [PushConstantRange]) -> wgpu :: PipelineLayoutDescriptor < 'a > { wgpu :: PipelineLayoutDescriptor { label , bind_group_layouts , push_constant_ranges } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_bundle_encoder_descriptor < 'a > (# [builder (default)] label : Label < 'a > , # [builder (default)] color_formats : & 'a [Option < TextureFormat >] , depth_stencil : Option < RenderBundleDepthStencil > , # [builder (default)] sample_count : u32 , multiview : Option < NonZeroU32 >) -> wgpu :: RenderBundleEncoderDescriptor < 'a > { wgpu :: RenderBundleEncoderDescriptor { label , color_formats , depth_stencil , sample_count , multiview } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_descriptor < 'a > (# [builder (default)] label : Label < 'a > , # [builder (default)] color_attachments : & 'a [Option < RenderPassColorAttachment < 'a > >] , depth_stencil_attachment : Option < RenderPassDepthStencilAttachment < 'a > > , timestamp_writes : Option < RenderPassTimestampWrites < 'a > > , occlusion_query_set : Option < & 'a QuerySet >) -> wgpu :: RenderPassDescriptor < 'a > { wgpu :: RenderPassDescriptor { label , color_attachments , depth_stencil_attachment , timestamp_writes , occlusion_query_set } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_depth_stencil_attachment < 'tex > (view : & 'tex TextureView , depth_ops : Option < Operations < f32 > > , stencil_ops : Option < Operations < u32 > >) -> wgpu :: RenderPassDepthStencilAttachment < 'tex > { wgpu :: RenderPassDepthStencilAttachment { view , depth_ops , stencil_ops } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_color_attachment < 'tex > (view : & 'tex TextureView , depth_slice : Option < u32 > , resolve_target : Option < & 'tex TextureView > , ops : Operations < Color >) -> wgpu :: RenderPassColorAttachment < 'tex > { wgpu :: RenderPassColorAttachment { view , depth_slice , resolve_target , ops } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_timestamp_writes < 'a > (query_set : & 'a QuerySet , beginning_of_pass_write_index : Option < u32 > , end_of_pass_write_index : Option < u32 >) -> wgpu :: RenderPassTimestampWrites < 'a > { wgpu :: RenderPassTimestampWrites { query_set , beginning_of_pass_write_index , end_of_pass_write_index } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn mesh_pipeline_descriptor < 'a > (label : Label < 'a > , layout : Option < & 'a PipelineLayout > , task : Option < TaskState < 'a > > , mesh : MeshState < 'a > , primitive : PrimitiveState , depth_stencil : Option < DepthStencilState > , multisample : MultisampleState , fragment : Option < FragmentState < 'a > > , multiview : Option < NonZeroU32 > , cache : Option < & 'a PipelineCache >) -> wgpu :: MeshPipelineDescriptor < 'a > { wgpu :: MeshPipelineDescriptor { label , layout , task , mesh , primitive , depth_stencil , multisample , fragment , multiview , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pipeline_descriptor < 'a > (label : Label < 'a > , layout : Option < & 'a PipelineLayout > , vertex : VertexState < 'a > , primitive : PrimitiveState , depth_stencil : Option < DepthStencilState > , multisample : MultisampleState , fragment : Option < FragmentState < 'a > > , multiview : Option < NonZeroU32 > , cache : Option < & 'a PipelineCache >) -> wgpu :: RenderPipelineDescriptor < 'a > { wgpu :: RenderPipelineDescriptor { label , layout , vertex , primitive , depth_stencil , multisample , fragment , multiview , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn mesh_state < 'a > (module : & 'a ShaderModule , entry_point : Option < & 'a str > , compilation_options : PipelineCompilationOptions < 'a >) -> wgpu :: MeshState < 'a > { wgpu :: MeshState { module , entry_point , compilation_options } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn task_state < 'a > (module : & 'a ShaderModule , entry_point : Option < & 'a str > , compilation_options : PipelineCompilationOptions < 'a >) -> wgpu :: TaskState < 'a > { wgpu :: TaskState { module , entry_point , compilation_options } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn fragment_state < 'a > (module : & 'a ShaderModule , entry_point : Option < & 'a str > , compilation_options : PipelineCompilationOptions < 'a > , targets : & 'a [Option < ColorTargetState >]) -> wgpu :: FragmentState < 'a > { wgpu :: FragmentState { module , entry_point , compilation_options , targets } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_state < 'a > (module : & 'a ShaderModule , entry_point : Option < & 'a str > , compilation_options : PipelineCompilationOptions < 'a > , buffers : & 'a [VertexBufferLayout < 'a >]) -> wgpu :: VertexState < 'a > { wgpu :: VertexState { module , entry_point , compilation_options , buffers } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_buffer_layout < 'a > (array_stride : BufferAddress , step_mode : VertexStepMode , attributes : & 'a [VertexAttribute]) -> wgpu :: VertexBufferLayout < 'a > { wgpu :: VertexBufferLayout { array_stride , step_mode , attributes } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn shader_module_descriptor < 'a > (label : Label < 'a > , source : ShaderSource < 'a >) -> wgpu :: ShaderModuleDescriptor < 'a > { wgpu :: ShaderModuleDescriptor { label , source } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compilation_info (messages : Vec < CompilationMessage >) -> wgpu :: CompilationInfo { wgpu :: CompilationInfo { messages } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_attribute (format : ray_tracing :: VertexFormat , offset : command :: BufferAddress , shader_location : ShaderLocation) -> wgpu :: VertexAttribute { wgpu :: VertexAttribute { format , offset , shader_location } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_transition < T > (texture : T , selector : Option < command :: TextureSelector > , state : TextureUses) -> wgpu :: TextureTransition < T > { wgpu :: TextureTransition { texture , selector , state } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texel_copy_buffer_layout (# [builder (default)] offset : command :: BufferAddress , bytes_per_row : Option < u32 > , rows_per_image : Option < u32 >) -> wgpu :: TexelCopyBufferLayout { wgpu :: TexelCopyBufferLayout { offset , bytes_per_row , rows_per_image } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn stencil_state (# [builder (default)] front : StencilFaceState , # [builder (default)] back : StencilFaceState , # [builder (default)] read_mask : u32 , # [builder (default)] write_mask : u32) -> wgpu :: StencilState { wgpu :: StencilState { front , back , read_mask , write_mask } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn stencil_face_state (# [builder (default = CompareFunction :: Always)] compare : CompareFunction , # [builder (default = StencilOperation :: Keep)] fail_op : StencilOperation , # [builder (default = StencilOperation :: Keep)] depth_fail_op : StencilOperation , # [builder (default = StencilOperation :: Keep)] pass_op : StencilOperation) -> wgpu :: StencilFaceState { wgpu :: StencilFaceState { compare , fail_op , depth_fail_op , pass_op } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn shader_runtime_checks (bounds_checks : bool , force_loop_bounding : bool) -> wgpu :: ShaderRuntimeChecks { wgpu :: ShaderRuntimeChecks { bounds_checks , force_loop_bounding } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_bundle_depth_stencil (format : TextureFormat , depth_read_only : bool , stencil_read_only : bool) -> wgpu :: RenderBundleDepthStencil { wgpu :: RenderBundleDepthStencil { format , depth_read_only , stencil_read_only } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn push_constant_range (stages : command :: ShaderStages , range : Range < u32 >) -> wgpu :: PushConstantRange { wgpu :: PushConstantRange { stages , range } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn primitive_state (# [builder (default)] topology : PrimitiveTopology , strip_index_format : Option < command :: IndexFormat > , # [builder (default)] front_face : FrontFace , cull_mode : Option < Face > , # [builder (default)] unclipped_depth : bool , # [builder (default)] polygon_mode : PolygonMode , # [builder (default)] conservative : bool) -> wgpu :: PrimitiveState { wgpu :: PrimitiveState { topology , strip_index_format , front_face , cull_mode , unclipped_depth , polygon_mode , conservative } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn origin_3_d (# [builder (default = 0)] x : u32 , # [builder (default = 0)] y : u32 , # [builder (default = 0)] z : u32) -> wgpu :: Origin3d { wgpu :: Origin3d { x , y , z } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn origin_2_d (x : u32 , y : u32) -> wgpu :: Origin2d { wgpu :: Origin2d { x , y } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn noop_backend_options (# [builder (default)] enable : bool) -> wgpu :: NoopBackendOptions { wgpu :: NoopBackendOptions { enable } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn multisample_state (count : u32 , mask : u64 , alpha_to_coverage_enabled : bool) -> wgpu :: MultisampleState { wgpu :: MultisampleState { count , mask , alpha_to_coverage_enabled } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn memory_budget_thresholds (for_resource_creation : Option < u8 > , for_device_loss : Option < u8 >) -> wgpu :: MemoryBudgetThresholds { wgpu :: MemoryBudgetThresholds { for_resource_creation , for_device_loss } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn instance_descriptor (# [builder (default)] backends : instance :: Backends , # [builder (default)] flags : InstanceFlags , # [builder (default)] memory_budget_thresholds : MemoryBudgetThresholds , # [builder (default)] backend_options : BackendOptions) -> wgpu :: InstanceDescriptor { wgpu :: InstanceDescriptor { backends , flags , memory_budget_thresholds , backend_options } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn image_subresource_range (# [builder (default)] aspect : TextureAspect , # [builder (default)] base_mip_level : u32 , mip_level_count : Option < u32 > , # [builder (default)] base_array_layer : u32 , array_layer_count : Option < u32 >) -> wgpu :: ImageSubresourceRange { wgpu :: ImageSubresourceRange { aspect , base_mip_level , mip_level_count , base_array_layer , array_layer_count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn gl_backend_options (# [builder (default)] gles_minor_version : Gles3MinorVersion , # [builder (default)] fence_behavior : GlFenceBehavior) -> wgpu :: GlBackendOptions { wgpu :: GlBackendOptions { gles_minor_version , fence_behavior } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn external_texture_transfer_function (a : f32 , b : f32 , g : f32 , k : f32) -> wgpu :: ExternalTextureTransferFunction { wgpu :: ExternalTextureTransferFunction { a , b , g , k } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn extent_3_d (width : u32 , height : u32 , depth_or_array_layers : u32) -> wgpu :: Extent3d { wgpu :: Extent3d { width , height , depth_or_array_layers } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn dx_12_backend_options (# [builder (default)] shader_compiler : Dx12Compiler , # [builder (default)] presentation_system : wgt :: Dx12SwapchainKind , # [builder (default)] latency_waitable_object : wgt :: Dx12UseFrameLatencyWaitableObject) -> wgpu :: Dx12BackendOptions { wgpu :: Dx12BackendOptions { shader_compiler , presentation_system , latency_waitable_object } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn downlevel_limits () -> wgpu :: DownlevelLimits { wgpu :: DownlevelLimits { } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn depth_stencil_state (format : TextureFormat , depth_write_enabled : bool , depth_compare : CompareFunction , stencil : StencilState , bias : DepthBiasState) -> wgpu :: DepthStencilState { wgpu :: DepthStencilState { format , depth_write_enabled , depth_compare , stencil , bias } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn depth_bias_state (# [builder (default)] constant : i32 , # [builder (default)] slope_scale : f32 , # [builder (default)] clamp : f32) -> wgpu :: DepthBiasState { wgpu :: DepthBiasState { constant , slope_scale , clamp } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn core_counters () -> wgpu :: CoreCounters { wgpu :: CoreCounters { } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn copy_external_image_dest_info < T > (texture : T , mip_level : u32 , origin : Origin3d , aspect : TextureAspect , color_space : PredefinedColorSpace , premultiplied_alpha : bool) -> wgpu :: CopyExternalImageDestInfo < T > { wgpu :: CopyExternalImageDestInfo { texture , mip_level , origin , aspect , color_space , premultiplied_alpha } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn command_buffer_descriptor < L > (# [builder (default)] label : L) -> wgpu :: CommandBufferDescriptor < L > { wgpu :: CommandBufferDescriptor { label } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn color_target_state (format : TextureFormat , blend : Option < BlendState > , write_mask : ColorWrites) -> wgpu :: ColorTargetState { wgpu :: ColorTargetState { format , blend , write_mask } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn color (# [builder (default)] r : f64 , # [builder (default)] g : f64 , # [builder (default)] b : f64 , # [builder (default)] a : f64) -> wgpu :: Color { wgpu :: Color { r , g , b , a } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_transition < T > (buffer : T , state : scratch :: BufferUses) -> wgpu :: BufferTransition < T > { wgpu :: BufferTransition { buffer , state } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blend_state (color : BlendComponent , alpha : BlendComponent) -> wgpu :: BlendState { wgpu :: BlendState { color , alpha } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blend_component (# [builder (default = BlendFactor :: One)] src_factor : BlendFactor , # [builder (default = BlendFactor :: Zero)] dst_factor : BlendFactor , # [builder (default = BlendOperation :: Add)] operation : BlendOperation) -> wgpu :: BlendComponent { wgpu :: BlendComponent { src_factor , dst_factor , operation } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_layout_entry (binding : u32 , visibility : command :: ShaderStages , ty : validation :: BindingType , count : Option < NonZeroU32 >) -> wgpu :: BindGroupLayoutEntry { wgpu :: BindGroupLayoutEntry { binding , visibility , ty , count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn backend_options (# [builder (default)] gl : GlBackendOptions , # [builder (default)] dx12 : Dx12BackendOptions , # [builder (default)] noop : NoopBackendOptions) -> wgpu :: BackendOptions { wgpu :: BackendOptions { gl , dx12 , noop } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn draw_indirect_args (# [builder (default)] vertex_count : u32 , # [builder (default)] instance_count : u32 , # [builder (default)] first_vertex : u32 , # [builder (default)] first_instance : u32) -> wgpu :: wgt :: DrawIndirectArgs { wgpu :: wgt :: DrawIndirectArgs { vertex_count , instance_count , first_vertex , first_instance } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn draw_indexed_indirect_args (# [builder (default)] index_count : u32 , # [builder (default)] instance_count : u32 , # [builder (default)] first_index : u32 , # [builder (default)] base_vertex : i32 , # [builder (default)] first_instance : u32) -> wgpu :: wgt :: DrawIndexedIndirectArgs { wgpu :: wgt :: DrawIndexedIndirectArgs { index_count , instance_count , first_index , base_vertex , first_instance } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn dispatch_indirect_args (# [builder (default)] x : u32 , # [builder (default)] y : u32 , # [builder (default)] z : u32) -> wgpu :: wgt :: DispatchIndirectArgs { wgpu :: wgt :: DispatchIndirectArgs { x , y , z } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_init_descriptor < 'a > (label : Label < 'a > , contents : & 'a [u8] , usage : BufferUsages) -> wgpu :: util :: BufferInitDescriptor < 'a > { wgpu :: util :: BufferInitDescriptor { label , contents , usage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn internal_counter () -> wgpu :: wgt :: InternalCounter { wgpu :: wgt :: InternalCounter { } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn allocation_report (name : String , offset : u64 , size : u64) -> wgpu :: wgt :: AllocationReport { wgpu :: wgt :: AllocationReport { name , offset , size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn memory_block_report (size : u64 , allocations : Range < usize >) -> wgpu :: wgt :: MemoryBlockReport { wgpu :: wgt :: MemoryBlockReport { size , allocations } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn create_shader_module_descriptor_passthrough < 'a , L > (entry_point : String , label : L , num_workgroups : (u32 , u32 , u32) , runtime_checks : ShaderRuntimeChecks , spirv : Option < Cow < 'a , [u32] > > , dxil : Option < Cow < 'a , [u8] > > , msl : Option < Cow < 'a , str > > , hlsl : Option < Cow < 'a , str > > , glsl : Option < Cow < 'a , str > > , wgsl : Option < Cow < 'a , str > >) -> wgpu :: wgt :: CreateShaderModuleDescriptorPassthrough < 'a , L > { wgpu :: wgt :: CreateShaderModuleDescriptorPassthrough { entry_point , label , num_workgroups , runtime_checks , spirv , dxil , msl , hlsl , glsl , wgsl } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn create_tlas_descriptor < L > (label : L , max_instances : u32 , flags : AccelerationStructureBuildFlags , update_mode : wgt :: AccelerationStructureUpdateMode) -> wgpu :: wgt :: CreateTlasDescriptor < L > { wgpu :: wgt :: CreateTlasDescriptor { label , max_instances , flags , update_mode } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn create_blas_descriptor < L > (label : L , flags : AccelerationStructureBuildFlags , update_mode : wgt :: AccelerationStructureUpdateMode) -> wgpu :: wgt :: CreateBlasDescriptor < L > { wgpu :: wgt :: CreateBlasDescriptor { label , flags , update_mode } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blas_triangle_geometry_size_descriptor (vertex_format : ray_tracing :: VertexFormat , vertex_count : u32 , index_format : Option < command :: IndexFormat > , index_count : Option < u32 > , flags : ray_tracing :: AccelerationStructureGeometryFlags) -> wgpu :: wgt :: BlasTriangleGeometrySizeDescriptor { wgpu :: wgt :: BlasTriangleGeometrySizeDescriptor { vertex_format , vertex_count , index_format , index_count , flags } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn query_set_descriptor < L > (label : L , ty : QueryType , count : u32) -> wgpu :: wgt :: QuerySetDescriptor < L > { wgpu :: wgt :: QuerySetDescriptor { label , ty , count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_bundle_descriptor < L > (label : L) -> wgpu :: wgt :: RenderBundleDescriptor < L > { wgpu :: wgt :: RenderBundleDescriptor { label } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn sampler_descriptor < L > (label : L , address_mode_u : AddressMode , address_mode_v : AddressMode , address_mode_w : AddressMode , mag_filter : FilterMode , min_filter : FilterMode , mipmap_filter : FilterMode , lod_min_clamp : f32 , lod_max_clamp : f32 , compare : Option < CompareFunction > , anisotropy_clamp : u16 , border_color : Option < SamplerBorderColor >) -> wgpu :: wgt :: SamplerDescriptor < L > { wgpu :: wgt :: SamplerDescriptor { label , address_mode_u , address_mode_v , address_mode_w , mag_filter , min_filter , mipmap_filter , lod_min_clamp , lod_max_clamp , compare , anisotropy_clamp , border_color } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn external_texture_descriptor < L > (label : L , width : u32 , height : u32 , format : ExternalTextureFormat , yuv_conversion_matrix : [f32 ; 16] , gamut_conversion_matrix : [f32 ; 9] , src_transfer_function : ExternalTextureTransferFunction , dst_transfer_function : ExternalTextureTransferFunction , sample_transform : [f32 ; 6] , load_transform : [f32 ; 6]) -> wgpu :: wgt :: ExternalTextureDescriptor < L > { wgpu :: wgt :: ExternalTextureDescriptor { label , width , height , format , yuv_conversion_matrix , gamut_conversion_matrix , src_transfer_function , dst_transfer_function , sample_transform , load_transform } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_descriptor < L , V > (label : L , size : command :: Extent3d , mip_level_count : u32 , sample_count : u32 , dimension : TextureDimension , format : TextureFormat , usage : command :: TextureUsages , view_formats : V) -> wgpu :: wgt :: TextureDescriptor < L , V > { wgpu :: wgt :: TextureDescriptor { label , size , mip_level_count , sample_count , dimension , format , usage , view_formats } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_view_descriptor < L > (# [builder (default)] label : L , format : Option < TextureFormat > , dimension : Option < command :: TextureViewDimension > , usage : Option < command :: TextureUsages > , # [builder (default)] aspect : TextureAspect , # [builder (default)] base_mip_level : u32 , mip_level_count : Option < u32 > , # [builder (default)] base_array_layer : u32 , array_layer_count : Option < u32 >) -> wgpu :: wgt :: TextureViewDescriptor < L > { wgpu :: wgt :: TextureViewDescriptor { label , format , dimension , usage , aspect , base_mip_level , mip_level_count , base_array_layer , array_layer_count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn surface_configuration < V > (usage : command :: TextureUsages , format : TextureFormat , width : u32 , height : u32 , present_mode : PresentMode , desired_maximum_frame_latency : u32 , alpha_mode : CompositeAlphaMode , view_formats : V) -> wgpu :: wgt :: SurfaceConfiguration < V > { wgpu :: wgt :: SurfaceConfiguration { usage , format , width , height , present_mode , desired_maximum_frame_latency , alpha_mode , view_formats } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_selector (mips : Range < u32 > , layers : Range < u32 >) -> wgpu :: wgt :: TextureSelector { wgpu :: wgt :: TextureSelector { mips , layers } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn command_encoder_descriptor < L > (label : L) -> wgpu :: wgt :: CommandEncoderDescriptor < L > { wgpu :: wgt :: CommandEncoderDescriptor { label } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_descriptor < L > (label : L , size : command :: BufferAddress , usage : command :: BufferUsages , mapped_at_creation : bool) -> wgpu :: wgt :: BufferDescriptor < L > { wgpu :: wgt :: BufferDescriptor { label , size , usage , mapped_at_creation } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn device_descriptor < L > (# [builder (default)] label : L , # [builder (default)] required_features : Features , # [builder (default)] required_limits : Limits , # [builder (default)] experimental_features : ExperimentalFeatures , # [builder (default)] memory_hints : MemoryHints , # [builder (default)] trace : Trace) -> wgpu :: wgt :: DeviceDescriptor < L > { wgpu :: wgt :: DeviceDescriptor { label , required_features , required_limits , experimental_features , memory_hints , trace } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn diagnostic_filter_node (inner : diagnostic_filter :: DiagnosticFilter , parent : Option < hal :: Handle < hal :: DiagnosticFilterNode > >) -> wgpu :: hal :: DiagnosticFilterNode { wgpu :: hal :: DiagnosticFilterNode { inner , parent } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn scalar (kind : hal :: ScalarKind , width : hal :: Bytes) -> wgpu :: hal :: Scalar { wgpu :: hal :: Scalar { kind , width } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn struct_member (name : Option < String > , ty : hal :: Handle < hal :: Type > , binding : Option < hal :: Binding > , offset : u32) -> wgpu :: hal :: StructMember { wgpu :: hal :: StructMember { name , ty , binding , offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn type (name : Option < String > , inner : hal :: TypeInner) -> wgpu :: hal :: Type { wgpu :: hal :: Type { name , inner } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn override (name : Option < String > , id : Option < u16 > , ty : hal :: Handle < hal :: Type > , init : Option < hal :: Handle < hal :: Expression > >) -> wgpu :: hal :: Override { wgpu :: hal :: Override { name , id , ty , init } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn constant (name : Option < String > , ty : hal :: Handle < hal :: Type > , init : hal :: Handle < hal :: Expression >) -> wgpu :: hal :: Constant { wgpu :: hal :: Constant { name , ty , init } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn resource_binding (group : u32 , binding : u32) -> wgpu :: hal :: ResourceBinding { wgpu :: hal :: ResourceBinding { group , binding } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn global_variable (name : Option < String > , space : hal :: AddressSpace , binding : Option < hal :: ResourceBinding > , ty : hal :: Handle < hal :: Type > , init : Option < hal :: Handle < hal :: Expression > >) -> wgpu :: hal :: GlobalVariable { wgpu :: hal :: GlobalVariable { name , space , binding , ty , init } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn local_variable (name : Option < String > , ty : hal :: Handle < hal :: Type > , init : Option < hal :: Handle < hal :: Expression > >) -> wgpu :: hal :: LocalVariable { wgpu :: hal :: LocalVariable { name , ty , init } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn switch_case (value : hal :: SwitchValue , body : hal :: Block , fall_through : bool) -> wgpu :: hal :: SwitchCase { wgpu :: hal :: SwitchCase { value , body , fall_through } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn function_argument (name : Option < String > , ty : hal :: Handle < hal :: Type > , binding : Option < hal :: Binding >) -> wgpu :: hal :: FunctionArgument { wgpu :: hal :: FunctionArgument { name , ty , binding } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn function_result (ty : hal :: Handle < hal :: Type > , binding : Option < hal :: Binding >) -> wgpu :: hal :: FunctionResult { wgpu :: hal :: FunctionResult { ty , binding } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn function (name : Option < String > , # [builder (default)] arguments : Vec < hal :: FunctionArgument > , result : Option < hal :: FunctionResult > , # [builder (default)] local_variables : hal :: Arena < hal :: LocalVariable > , # [builder (default)] expressions : hal :: Arena < hal :: Expression > , # [builder (default)] named_expressions : NamedExpressions , # [builder (default)] body : hal :: Block , diagnostic_filter_leaf : Option < hal :: Handle < hal :: DiagnosticFilterNode > >) -> wgpu :: hal :: Function { wgpu :: hal :: Function { name , arguments , result , local_variables , expressions , named_expressions , body , diagnostic_filter_leaf } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn entry_point (name : String , stage : hal :: ShaderStage , early_depth_test : Option < hal :: EarlyDepthTest > , workgroup_size : [u32 ; 3] , workgroup_size_overrides : Option < [Option < hal :: Handle < hal :: Expression > > ; 3] > , function : hal :: Function) -> wgpu :: hal :: EntryPoint { wgpu :: hal :: EntryPoint { name , stage , early_depth_test , workgroup_size , workgroup_size_overrides , function } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn special_types (ray_desc : Option < hal :: Handle < hal :: Type > > , ray_intersection : Option < hal :: Handle < hal :: Type > > , ray_vertex_return : Option < hal :: Handle < hal :: Type > > , external_texture_params : Option < hal :: Handle < hal :: Type > > , external_texture_transfer_function : Option < hal :: Handle < hal :: Type > > , # [builder (default)] predeclared_types : hal :: FastIndexMap < hal :: PredeclaredType , hal :: Handle < hal :: Type > >) -> wgpu :: hal :: SpecialTypes { wgpu :: hal :: SpecialTypes { ray_desc , ray_intersection , ray_vertex_return , external_texture_params , external_texture_transfer_function , predeclared_types } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn doc_comments (# [builder (default)] types : hal :: FastIndexMap < hal :: Handle < hal :: Type > , Vec < String > > , # [builder (default)] struct_members : hal :: FastIndexMap < (hal :: Handle < hal :: Type > , usize) , Vec < String > > , # [builder (default)] entry_points : hal :: FastIndexMap < usize , Vec < String > > , # [builder (default)] functions : hal :: FastIndexMap < hal :: Handle < hal :: Function > , Vec < String > > , # [builder (default)] constants : hal :: FastIndexMap < hal :: Handle < hal :: Constant > , Vec < String > > , # [builder (default)] global_variables : hal :: FastIndexMap < hal :: Handle < hal :: GlobalVariable > , Vec < String > > , # [builder (default)] module : Vec < String >) -> wgpu :: hal :: DocComments { wgpu :: hal :: DocComments { types , struct_members , entry_points , functions , constants , global_variables , module } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn module (# [builder (default)] types : hal :: UniqueArena < hal :: Type > , # [builder (default)] special_types : hal :: SpecialTypes , # [builder (default)] constants : hal :: Arena < hal :: Constant > , # [builder (default)] overrides : hal :: Arena < hal :: Override > , # [builder (default)] global_variables : hal :: Arena < hal :: GlobalVariable > , # [builder (default)] global_expressions : hal :: Arena < hal :: Expression > , # [builder (default)] functions : hal :: Arena < hal :: Function > , # [builder (default)] entry_points : Vec < hal :: EntryPoint > , # [builder (default)] diagnostic_filters : hal :: Arena < hal :: DiagnosticFilterNode > , diagnostic_filter_leaf : Option < hal :: Handle < hal :: DiagnosticFilterNode > > , doc_comments : Option < Box < hal :: DocComments > >) -> wgpu :: hal :: Module { wgpu :: hal :: Module { types , special_types , constants , overrides , global_variables , global_expressions , functions , entry_points , diagnostic_filters , diagnostic_filter_leaf , doc_comments } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn dyn_open_device (device : Box < dyn hal :: DynDevice > , queue : Box < dyn hal :: DynQueue >) -> wgpu :: hal :: DynOpenDevice { wgpu :: hal :: DynOpenDevice { device , queue } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn dyn_exposed_adapter (adapter : Box < dyn hal :: DynAdapter > , info : AdapterInfo , features : Features , capabilities : hal :: Capabilities) -> wgpu :: hal :: DynExposedAdapter { wgpu :: hal :: DynExposedAdapter { adapter , info , features , capabilities } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn dyn_acquired_surface_texture (texture : Box < dyn hal :: DynSurfaceTexture > , suboptimal : bool) -> wgpu :: hal :: DynAcquiredSurfaceTexture { wgpu :: hal :: DynAcquiredSurfaceTexture { texture , suboptimal } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn tlas_instance (transform : [f32 ; 12] , custom_data : u32 , mask : u8 , blas_address : u64) -> wgpu :: hal :: TlasInstance { wgpu :: hal :: TlasInstance { transform , custom_data , mask , blas_address } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_barrier (usage : hal :: StateTransition < hal :: AccelerationStructureUses >) -> wgpu :: hal :: AccelerationStructureBarrier { wgpu :: hal :: AccelerationStructureBarrier { usage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_triangle_transform < 'a , B : DynBuffer + ? Sized > (buffer : & 'a B , offset : u32) -> wgpu :: hal :: AccelerationStructureTriangleTransform < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: AccelerationStructureTriangleTransform { buffer , offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_triangle_indices < 'a , B : DynBuffer + ? Sized > (format : IndexFormat , buffer : Option < & 'a B > , offset : u32 , count : u32) -> wgpu :: hal :: AccelerationStructureTriangleIndices < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: AccelerationStructureTriangleIndices { format , buffer , offset , count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_instances < 'a , B : DynBuffer + ? Sized > (buffer : Option < & 'a B > , offset : u32 , count : u32) -> wgpu :: hal :: AccelerationStructureInstances < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: AccelerationStructureInstances { buffer , offset , count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_copy (copy_flags : wgt :: AccelerationStructureCopy , type_flags : wgt :: AccelerationStructureType) -> wgpu :: hal :: AccelerationStructureCopy { wgpu :: hal :: AccelerationStructureCopy { copy_flags , type_flags } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_aab_bs < 'a , B : DynBuffer + ? Sized > (buffer : Option < & 'a B > , offset : u32 , count : u32 , stride : BufferAddress , flags : hal :: AccelerationStructureGeometryFlags) -> wgpu :: hal :: AccelerationStructureAABBs < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: AccelerationStructureAABBs { buffer , offset , count , stride , flags } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_triangles < 'a , B : DynBuffer + ? Sized > (vertex_buffer : Option < & 'a B > , vertex_format : VertexFormat , first_vertex : u32 , vertex_count : u32 , vertex_stride : BufferAddress , indices : Option < hal :: AccelerationStructureTriangleIndices < 'a , B > > , transform : Option < hal :: AccelerationStructureTriangleTransform < 'a , B > > , flags : hal :: AccelerationStructureGeometryFlags) -> wgpu :: hal :: AccelerationStructureTriangles < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: AccelerationStructureTriangles { vertex_buffer , vertex_format , first_vertex , vertex_count , vertex_stride , indices , transform , flags } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn get_acceleration_structure_build_sizes_descriptor < 'a , B : DynBuffer + ? Sized > (entries : & 'a hal :: AccelerationStructureEntries < 'a , B > , flags : hal :: AccelerationStructureBuildFlags) -> wgpu :: hal :: GetAccelerationStructureBuildSizesDescriptor < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: GetAccelerationStructureBuildSizesDescriptor { entries , flags } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn build_acceleration_structure_descriptor < 'a , B : DynBuffer + ? Sized , A : DynAccelerationStructure + ? Sized , > (entries : & 'a hal :: AccelerationStructureEntries < 'a , B > , mode : hal :: AccelerationStructureBuildMode , flags : hal :: AccelerationStructureBuildFlags , source_acceleration_structure : Option < & 'a A > , destination_acceleration_structure : & 'a A , scratch_buffer : & 'a B , scratch_buffer_offset : BufferAddress) -> wgpu :: hal :: BuildAccelerationStructureDescriptor < 'a , B : DynBuffer + ? Sized , A : DynAccelerationStructure + ? Sized , > { wgpu :: hal :: BuildAccelerationStructureDescriptor { entries , mode , flags , source_acceleration_structure , destination_acceleration_structure , scratch_buffer , scratch_buffer_offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_build_sizes (# [builder (default)] acceleration_structure_size : BufferAddress , # [builder (default)] update_scratch_size : BufferAddress , # [builder (default)] build_scratch_size : BufferAddress) -> wgpu :: hal :: AccelerationStructureBuildSizes { wgpu :: hal :: AccelerationStructureBuildSizes { acceleration_structure_size , update_scratch_size , build_scratch_size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acceleration_structure_descriptor < 'a > (label : hal :: Label < 'a > , size : BufferAddress , format : hal :: AccelerationStructureFormat , allow_compaction : bool) -> wgpu :: hal :: AccelerationStructureDescriptor < 'a > { wgpu :: hal :: AccelerationStructureDescriptor { label , size , format , allow_compaction } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pass_descriptor < 'a , Q : DynQuerySet + ? Sized > (label : hal :: Label < 'a > , timestamp_writes : Option < hal :: PassTimestampWrites < 'a , Q > >) -> wgpu :: hal :: ComputePassDescriptor < 'a , Q : DynQuerySet + ? Sized > { wgpu :: hal :: ComputePassDescriptor { label , timestamp_writes } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_descriptor < 'a , Q : DynQuerySet + ? Sized , T : DynTextureView + ? Sized , > (label : hal :: Label < 'a > , extent : Extent3d , sample_count : u32 , color_attachments : & 'a [Option < hal :: ColorAttachment < 'a , T > >] , depth_stencil_attachment : Option < hal :: DepthStencilAttachment < 'a , T > > , multiview : Option < NonZeroU32 > , timestamp_writes : Option < hal :: PassTimestampWrites < 'a , Q > > , occlusion_query_set : Option < & 'a Q >) -> wgpu :: hal :: RenderPassDescriptor < 'a , Q : DynQuerySet + ? Sized , T : DynTextureView + ? Sized , > { wgpu :: hal :: RenderPassDescriptor { label , extent , sample_count , color_attachments , depth_stencil_attachment , multiview , timestamp_writes , occlusion_query_set } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pass_timestamp_writes < 'a , Q : DynQuerySet + ? Sized > (query_set : & 'a Q , beginning_of_pass_write_index : Option < u32 > , end_of_pass_write_index : Option < u32 >) -> wgpu :: hal :: PassTimestampWrites < 'a , Q : DynQuerySet + ? Sized > { wgpu :: hal :: PassTimestampWrites { query_set , beginning_of_pass_write_index , end_of_pass_write_index } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn depth_stencil_attachment < 'a , T : DynTextureView + ? Sized > (target : hal :: Attachment < 'a , T > , depth_ops : hal :: AttachmentOps , stencil_ops : hal :: AttachmentOps , clear_value : (f32 , u32)) -> wgpu :: hal :: DepthStencilAttachment < 'a , T : DynTextureView + ? Sized > { wgpu :: hal :: DepthStencilAttachment { target , depth_ops , stencil_ops , clear_value } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn color_attachment < 'a , T : DynTextureView + ? Sized > (target : hal :: Attachment < 'a , T > , depth_slice : Option < u32 > , resolve_target : Option < hal :: Attachment < 'a , T > > , ops : hal :: AttachmentOps , clear_value : Color) -> wgpu :: hal :: ColorAttachment < 'a , T : DynTextureView + ? Sized > { wgpu :: hal :: ColorAttachment { target , depth_slice , resolve_target , ops , clear_value } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn attachment < 'a , T : DynTextureView + ? Sized > (view : & 'a T , usage : TextureUses) -> wgpu :: hal :: Attachment < 'a , T : DynTextureView + ? Sized > { wgpu :: hal :: Attachment { view , usage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_texture_copy (buffer_layout : TexelCopyBufferLayout , texture_base : hal :: TextureCopyBase , size : hal :: CopyExtent) -> wgpu :: hal :: BufferTextureCopy { wgpu :: hal :: BufferTextureCopy { buffer_layout , texture_base , size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_copy (src_base : hal :: TextureCopyBase , dst_base : hal :: TextureCopyBase , size : hal :: CopyExtent) -> wgpu :: hal :: TextureCopy { wgpu :: hal :: TextureCopy { src_base , dst_base , size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn copy_extent (width : u32 , height : u32 , depth : u32) -> wgpu :: hal :: CopyExtent { wgpu :: hal :: CopyExtent { width , height , depth } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_copy_base (mip_level : u32 , array_layer : u32 , origin : Origin3d , aspect : hal :: FormatAspects) -> wgpu :: hal :: TextureCopyBase { wgpu :: hal :: TextureCopyBase { mip_level , array_layer , origin , aspect } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_copy (src_offset : BufferAddress , dst_offset : BufferAddress , size : BufferSize) -> wgpu :: hal :: BufferCopy { wgpu :: hal :: BufferCopy { src_offset , dst_offset , size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_barrier < 'a , T : DynTexture + ? Sized > (texture : & 'a T , range : ImageSubresourceRange , usage : hal :: StateTransition < TextureUses >) -> wgpu :: hal :: TextureBarrier < 'a , T : DynTexture + ? Sized > { wgpu :: hal :: TextureBarrier { texture , range , usage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_barrier < 'a , B : DynBuffer + ? Sized > (buffer : & 'a B , usage : hal :: StateTransition < BufferUses >) -> wgpu :: hal :: BufferBarrier < 'a , B : DynBuffer + ? Sized > { wgpu :: hal :: BufferBarrier { buffer , usage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn state_transition < T > (from : T , to : T) -> wgpu :: hal :: StateTransition < T > { wgpu :: hal :: StateTransition { from , to } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn rect < T > (x : T , y : T , w : T , h : T) -> wgpu :: hal :: Rect < T > { wgpu :: hal :: Rect { x , y , w , h } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn surface_configuration (maximum_frame_latency : u32 , present_mode : PresentMode , composite_alpha_mode : CompositeAlphaMode , format : TextureFormat , extent : Extent3d , usage : TextureUses , view_formats : Vec < TextureFormat >) -> wgpu :: hal :: SurfaceConfiguration { wgpu :: hal :: SurfaceConfiguration { maximum_frame_latency , present_mode , composite_alpha_mode , format , extent , usage , view_formats } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pipeline_descriptor < 'a , Pl : DynPipelineLayout + ? Sized , M : DynShaderModule + ? Sized , Pc : DynPipelineCache + ? Sized , > (label : hal :: Label < 'a > , layout : & 'a Pl , vertex_processor : hal :: VertexProcessor < 'a , M > , primitive : PrimitiveState , depth_stencil : Option < DepthStencilState > , multisample : MultisampleState , fragment_stage : Option < hal :: ProgrammableStage < 'a , M > > , color_targets : & 'a [Option < ColorTargetState >] , multiview : Option < NonZeroU32 > , cache : Option < & 'a Pc >) -> wgpu :: hal :: RenderPipelineDescriptor < 'a , Pl : DynPipelineLayout + ? Sized , M : DynShaderModule + ? Sized , Pc : DynPipelineCache + ? Sized , > { wgpu :: hal :: RenderPipelineDescriptor { label , layout , vertex_processor , primitive , depth_stencil , multisample , fragment_stage , color_targets , multiview , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_buffer_layout < 'a > (array_stride : BufferAddress , step_mode : VertexStepMode , attributes : & 'a [VertexAttribute]) -> wgpu :: hal :: VertexBufferLayout < 'a > { wgpu :: hal :: VertexBufferLayout { array_stride , step_mode , attributes } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_cache_descriptor < 'a > (label : hal :: Label < 'a > , data : Option < & 'a [u8] >) -> wgpu :: hal :: PipelineCacheDescriptor < 'a > { wgpu :: hal :: PipelineCacheDescriptor { label , data } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pipeline_descriptor < 'a , Pl : DynPipelineLayout + ? Sized , M : DynShaderModule + ? Sized , Pc : DynPipelineCache + ? Sized , > (label : hal :: Label < 'a > , layout : & 'a Pl , stage : hal :: ProgrammableStage < 'a , M > , cache : Option < & 'a Pc >) -> wgpu :: hal :: ComputePipelineDescriptor < 'a , Pl : DynPipelineLayout + ? Sized , M : DynShaderModule + ? Sized , Pc : DynPipelineCache + ? Sized , > { wgpu :: hal :: ComputePipelineDescriptor { label , layout , stage , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn programmable_stage < 'a , M : DynShaderModule + ? Sized > (module : & 'a M , entry_point : & 'a str , constants : & 'a naga :: back :: PipelineConstants , zero_initialize_workgroup_memory : bool) -> wgpu :: hal :: ProgrammableStage < 'a , M : DynShaderModule + ? Sized > { wgpu :: hal :: ProgrammableStage { module , entry_point , constants , zero_initialize_workgroup_memory } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn debug_source (file_name : Cow < 'static , str > , source_code : Cow < 'static , str >) -> wgpu :: hal :: DebugSource { wgpu :: hal :: DebugSource { file_name , source_code } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn shader_module_descriptor < 'a > (label : hal :: Label < 'a > , runtime_checks : ShaderRuntimeChecks) -> wgpu :: hal :: ShaderModuleDescriptor < 'a > { wgpu :: hal :: ShaderModuleDescriptor { label , runtime_checks } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn naga_shader (# [builder (default)] module : Cow < 'static , naga :: Module > , # [builder (default)] info : naga :: valid :: ModuleInfo , debug_source : Option < hal :: DebugSource >) -> wgpu :: hal :: NagaShader { wgpu :: hal :: NagaShader { module , info , debug_source } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn command_encoder_descriptor < 'a , Q : DynQueue + ? Sized > (label : hal :: Label < 'a > , queue : & 'a Q) -> wgpu :: hal :: CommandEncoderDescriptor < 'a , Q : DynQueue + ? Sized > { wgpu :: hal :: CommandEncoderDescriptor { label , queue } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_descriptor < 'a , Bgl : DynBindGroupLayout + ? Sized , B : DynBuffer + ? Sized , S : DynSampler + ? Sized , T : DynTextureView + ? Sized , A : DynAccelerationStructure + ? Sized , > (label : hal :: Label < 'a > , layout : & 'a Bgl , buffers : & 'a [hal :: BufferBinding < 'a , B >] , samplers : & 'a [& 'a S] , textures : & 'a [hal :: TextureBinding < 'a , T >] , entries : & 'a [hal :: BindGroupEntry] , acceleration_structures : & 'a [& 'a A] , external_textures : & 'a [hal :: ExternalTextureBinding < 'a , B , T >]) -> wgpu :: hal :: BindGroupDescriptor < 'a , Bgl : DynBindGroupLayout + ? Sized , B : DynBuffer + ? Sized , S : DynSampler + ? Sized , T : DynTextureView + ? Sized , A : DynAccelerationStructure + ? Sized , > { wgpu :: hal :: BindGroupDescriptor { label , layout , buffers , samplers , textures , entries , acceleration_structures , external_textures } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_entry (binding : u32 , resource_index : u32 , count : u32) -> wgpu :: hal :: BindGroupEntry { wgpu :: hal :: BindGroupEntry { binding , resource_index , count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn external_texture_binding < 'a , B : DynBuffer + ? Sized , T : DynTextureView + ? Sized , > (planes : [hal :: TextureBinding < 'a , T > ; 3] , params : hal :: BufferBinding < 'a , B >) -> wgpu :: hal :: ExternalTextureBinding < 'a , B : DynBuffer + ? Sized , T : DynTextureView + ? Sized , > { wgpu :: hal :: ExternalTextureBinding { planes , params } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_binding < 'a , T : DynTextureView + ? Sized > (view : & 'a T , usage : TextureUses) -> wgpu :: hal :: TextureBinding < 'a , T : DynTextureView + ? Sized > { wgpu :: hal :: TextureBinding { view , usage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_layout_descriptor < 'a , B : DynBindGroupLayout + ? Sized > (label : hal :: Label < 'a > , flags : hal :: PipelineLayoutFlags , bind_group_layouts : & 'a [& 'a B] , push_constant_ranges : & 'a [PushConstantRange]) -> wgpu :: hal :: PipelineLayoutDescriptor < 'a , B : DynBindGroupLayout + ? Sized > { wgpu :: hal :: PipelineLayoutDescriptor { label , flags , bind_group_layouts , push_constant_ranges } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_layout_descriptor < 'a > (label : hal :: Label < 'a > , flags : hal :: BindGroupLayoutFlags , entries : & 'a [BindGroupLayoutEntry]) -> wgpu :: hal :: BindGroupLayoutDescriptor < 'a > { wgpu :: hal :: BindGroupLayoutDescriptor { label , flags , entries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn sampler_descriptor < 'a > (label : hal :: Label < 'a > , address_modes : [AddressMode ; 3] , mag_filter : FilterMode , min_filter : FilterMode , mipmap_filter : FilterMode , lod_clamp : Range < f32 > , compare : Option < CompareFunction > , anisotropy_clamp : u16 , border_color : Option < SamplerBorderColor >) -> wgpu :: hal :: SamplerDescriptor < 'a > { wgpu :: hal :: SamplerDescriptor { label , address_modes , mag_filter , min_filter , mipmap_filter , lod_clamp , compare , anisotropy_clamp , border_color } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_view_descriptor < 'a > (label : hal :: Label < 'a > , format : TextureFormat , dimension : TextureViewDimension , usage : TextureUses , range : ImageSubresourceRange) -> wgpu :: hal :: TextureViewDescriptor < 'a > { wgpu :: hal :: TextureViewDescriptor { label , format , dimension , usage , range } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_descriptor < 'a > (label : hal :: Label < 'a > , size : Extent3d , mip_level_count : u32 , sample_count : u32 , dimension : TextureDimension , format : TextureFormat , usage : TextureUses , memory_flags : hal :: MemoryFlags , view_formats : Vec < TextureFormat >) -> wgpu :: hal :: TextureDescriptor < 'a > { wgpu :: hal :: TextureDescriptor { label , size , mip_level_count , sample_count , dimension , format , usage , memory_flags , view_formats } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_descriptor < 'a > (label : hal :: Label < 'a > , size : BufferAddress , usage : BufferUses , memory_flags : hal :: MemoryFlags) -> wgpu :: hal :: BufferDescriptor < 'a > { wgpu :: hal :: BufferDescriptor { label , size , usage , memory_flags } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_mapping (ptr : NonNull < u8 > , is_coherent : bool) -> wgpu :: hal :: BufferMapping { wgpu :: hal :: BufferMapping { ptr , is_coherent } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn open_device < A : Api > (device : A :: Device , queue : A :: Queue) -> wgpu :: hal :: OpenDevice < A : Api > { wgpu :: hal :: OpenDevice { device , queue } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn acquired_surface_texture < A : Api > (texture : A :: SurfaceTexture , suboptimal : bool) -> wgpu :: hal :: AcquiredSurfaceTexture < A : Api > { wgpu :: hal :: AcquiredSurfaceTexture { texture , suboptimal } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn exposed_adapter < A : Api > (adapter : A :: Adapter , info : AdapterInfo , features : Features , capabilities : hal :: Capabilities) -> wgpu :: hal :: ExposedAdapter < A : Api > { wgpu :: hal :: ExposedAdapter { adapter , info , features , capabilities } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn capabilities (limits : Limits , alignments : hal :: Alignments , downlevel : DownlevelCapabilities) -> wgpu :: hal :: Capabilities { wgpu :: hal :: Capabilities { limits , alignments , downlevel } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn alignments (buffer_copy_offset : BufferSize , buffer_copy_pitch : BufferSize , uniform_bounds_check_alignment : BufferSize , raw_tlas_instance_size : usize , ray_tracing_scratch_buffer_alignment : u32) -> wgpu :: hal :: Alignments { wgpu :: hal :: Alignments { buffer_copy_offset , buffer_copy_pitch , uniform_bounds_check_alignment , raw_tlas_instance_size , ray_tracing_scratch_buffer_alignment } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn instance_descriptor < 'a > (name : & 'a str , flags : InstanceFlags , memory_budget_thresholds : MemoryBudgetThresholds , backend_options : BackendOptions) -> wgpu :: hal :: InstanceDescriptor < 'a > { wgpu :: hal :: InstanceDescriptor { name , flags , memory_budget_thresholds , backend_options } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn layout_error (ty : Handle < Type > , inner : proc :: LayoutErrorInner) -> wgpu :: hal :: proc :: LayoutError { wgpu :: hal :: proc :: LayoutError { ty , inner } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn uniformity (non_uniform_result : valid :: analyzer :: NonUniformResult , requirements : valid :: UniformityRequirements) -> wgpu :: hal :: valid :: Uniformity { wgpu :: hal :: valid :: Uniformity { non_uniform_result , requirements } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn rule (arguments : Vec < front :: TypeResolution > , conclusion : proc :: Conclusion) -> wgpu :: hal :: proc :: Rule { wgpu :: hal :: proc :: Rule { arguments , conclusion } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn resolve_context < 'a > (constants : & 'a Arena < Constant > , overrides : & 'a Arena < Override > , types : & 'a UniqueArena < Type > , special_types : & 'a SpecialTypes , global_vars : & 'a Arena < GlobalVariable > , local_vars : & 'a Arena < LocalVariable > , functions : & 'a Arena < Function > , arguments : & 'a [FunctionArgument]) -> wgpu :: hal :: front :: ResolveContext < 'a > { wgpu :: hal :: front :: ResolveContext { constants , overrides , types , special_types , global_vars , local_vars , functions , arguments } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn type_layout (size : u32 , alignment : proc :: Alignment) -> wgpu :: hal :: proc :: TypeLayout { wgpu :: hal :: proc :: TypeLayout { size , alignment } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bounds_check_policies (# [builder (default)] index : proc :: BoundsCheckPolicy , # [builder (default)] buffer : proc :: BoundsCheckPolicy , # [builder (default)] image_load : proc :: BoundsCheckPolicy , # [builder (default)] binding_array : proc :: BoundsCheckPolicy) -> wgpu :: hal :: proc :: BoundsCheckPolicies { wgpu :: hal :: proc :: BoundsCheckPolicies { index , buffer , image_load , binding_array } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn global_ctx < 'a > (types : & 'a UniqueArena < Type > , constants : & 'a Arena < Constant > , overrides : & 'a Arena < Override > , global_expressions : & 'a Arena < Expression >) -> wgpu :: hal :: proc :: GlobalCtx < 'a > { wgpu :: hal :: proc :: GlobalCtx { types , constants , overrides , global_expressions } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn shader_error < E > (source : String , label : Option < String > , inner : Box < E >) -> wgpu :: hal :: error :: ShaderError < E > { wgpu :: hal :: error :: ShaderError { source , label , inner } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn diagnostic_filter (new_severity : diagnostic_filter :: Severity , triggering_rule : diagnostic_filter :: FilterableTriggeringRule) -> wgpu :: hal :: diagnostic_filter :: DiagnosticFilter { wgpu :: hal :: diagnostic_filter :: DiagnosticFilter { new_severity , triggering_rule } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn function_tracer < 'a > (function : & 'a Function , constants : & 'a Arena < Constant > , overrides : & 'a Arena < Override > , functions_pending : & 'a mut HandleSet < Function > , functions_used : & 'a mut HandleSet < Function > , types_used : & 'a mut HandleSet < Type > , global_variables_used : & 'a mut HandleSet < GlobalVariable > , constants_used : & 'a mut HandleSet < Constant > , overrides_used : & 'a mut HandleSet < Override > , global_expressions_used : & 'a mut HandleSet < Expression > , expressions_used : HandleSet < Expression >) -> wgpu :: hal :: compact :: FunctionTracer < 'a > { wgpu :: hal :: compact :: FunctionTracer { function , constants , overrides , functions_pending , functions_used , types_used , global_variables_used , constants_used , overrides_used , global_expressions_used , expressions_used } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn function_ctx < 'a > (ty : back :: FunctionType , info : & 'a valid :: FunctionInfo , expressions : & 'a Arena < Expression > , named_expressions : & 'a NamedExpressions) -> wgpu :: hal :: back :: FunctionCtx < 'a > { wgpu :: hal :: back :: FunctionCtx { ty , info , expressions , named_expressions } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn sampler_descriptor < 'a > (label : Label < 'a > , address_modes : [AddressMode ; 3] , mag_filter : FilterMode , min_filter : FilterMode , mipmap_filter : FilterMode , lod_min_clamp : f32 , lod_max_clamp : f32 , compare : Option < CompareFunction > , anisotropy_clamp : u16 , border_color : Option < SamplerBorderColor >) -> wgpu :: wgc :: resource :: SamplerDescriptor < 'a > { wgpu :: wgc :: resource :: SamplerDescriptor { label , address_modes , mag_filter , min_filter , mipmap_filter , lod_min_clamp , lod_max_clamp , compare , anisotropy_clamp , border_color } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn texture_view_descriptor < 'a > (# [builder (default)] label : Label < 'a > , format : Option < TextureFormat > , dimension : Option < command :: TextureViewDimension > , usage : Option < command :: TextureUsages > , # [builder (default)] range : ImageSubresourceRange) -> wgpu :: wgc :: resource :: TextureViewDescriptor < 'a > { wgpu :: wgc :: resource :: TextureViewDescriptor { label , format , dimension , usage , range } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_map_operation (host : resource :: HostMap , callback : Option < resource :: BufferMapCallback >) -> wgpu :: wgc :: resource :: BufferMapOperation { wgpu :: wgc :: resource :: BufferMapOperation { host , callback } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn registry_report (# [builder (default)] num_allocated : usize , # [builder (default)] num_kept_from_user : usize , # [builder (default)] num_released_from_user : usize , # [builder (default)] element_size : usize) -> wgpu :: wgc :: global :: RegistryReport { wgpu :: wgc :: global :: RegistryReport { num_allocated , num_kept_from_user , num_released_from_user , element_size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn arc_tlas_package (tlas : Arc < hub :: Tlas > , instances : Vec < Option < ray_tracing :: ArcTlasInstance > > , lowest_unmodified : u32) -> wgpu :: wgc :: ray_tracing :: ArcTlasPackage { wgpu :: wgc :: ray_tracing :: ArcTlasPackage { tlas , instances , lowest_unmodified } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn arc_tlas_instance (blas : Arc < hub :: Blas > , transform : [f32 ; 12] , custom_data : u32 , mask : u8) -> wgpu :: wgc :: ray_tracing :: ArcTlasInstance { wgpu :: wgc :: ray_tracing :: ArcTlasInstance { blas , transform , custom_data , mask } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn arc_blas_build_entry (blas : Arc < hub :: Blas > , geometries : ray_tracing :: ArcBlasGeometries) -> wgpu :: wgc :: ray_tracing :: ArcBlasBuildEntry { wgpu :: wgc :: ray_tracing :: ArcBlasBuildEntry { blas , geometries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blas_triangle_geometry_info (size : wgt :: BlasTriangleGeometrySizeDescriptor , first_vertex : u32 , vertex_stride : command :: BufferAddress , first_index : Option < u32 > , transform_buffer_offset : Option < command :: BufferAddress >) -> wgpu :: wgc :: ray_tracing :: BlasTriangleGeometryInfo { wgpu :: wgc :: ray_tracing :: BlasTriangleGeometryInfo { size , first_vertex , vertex_stride , first_index , transform_buffer_offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn arc_blas_triangle_geometry (size : wgt :: BlasTriangleGeometrySizeDescriptor , vertex_buffer : Arc < timestamp_normalization :: Buffer > , index_buffer : Option < Arc < timestamp_normalization :: Buffer > > , transform_buffer : Option < Arc < timestamp_normalization :: Buffer > > , first_vertex : u32 , vertex_stride : command :: BufferAddress , first_index : Option < u32 > , transform_buffer_offset : Option < command :: BufferAddress >) -> wgpu :: wgc :: ray_tracing :: ArcBlasTriangleGeometry { wgpu :: wgc :: ray_tracing :: ArcBlasTriangleGeometry { size , vertex_buffer , index_buffer , transform_buffer , first_vertex , vertex_stride , first_index , transform_buffer_offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn trace_tlas_package (tlas_id : as_hal :: TlasId , instances : Vec < Option < ray_tracing :: TraceTlasInstance > > , lowest_unmodified : u32) -> wgpu :: wgc :: ray_tracing :: TraceTlasPackage { wgpu :: wgc :: ray_tracing :: TraceTlasPackage { tlas_id , instances , lowest_unmodified } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn trace_tlas_instance (blas_id : as_hal :: BlasId , transform : [f32 ; 12] , custom_data : u32 , mask : u8) -> wgpu :: wgc :: ray_tracing :: TraceTlasInstance { wgpu :: wgc :: ray_tracing :: TraceTlasInstance { blas_id , transform , custom_data , mask } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn trace_blas_build_entry (blas_id : as_hal :: BlasId , geometries : ray_tracing :: TraceBlasGeometries) -> wgpu :: wgc :: ray_tracing :: TraceBlasBuildEntry { wgpu :: wgc :: ray_tracing :: TraceBlasBuildEntry { blas_id , geometries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn trace_blas_triangle_geometry (size : wgt :: BlasTriangleGeometrySizeDescriptor , vertex_buffer : as_hal :: BufferId , index_buffer : Option < as_hal :: BufferId > , transform_buffer : Option < as_hal :: BufferId > , first_vertex : u32 , vertex_stride : command :: BufferAddress , first_index : Option < u32 > , transform_buffer_offset : Option < command :: BufferAddress >) -> wgpu :: wgc :: ray_tracing :: TraceBlasTriangleGeometry { wgpu :: wgc :: ray_tracing :: TraceBlasTriangleGeometry { size , vertex_buffer , index_buffer , transform_buffer , first_vertex , vertex_stride , first_index , transform_buffer_offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn tlas_package < 'a > (tlas_id : as_hal :: TlasId , instances : Box < dyn Iterator < Item = Option < TlasInstance < 'a > > > + 'a > , lowest_unmodified : u32) -> wgpu :: wgc :: ray_tracing :: TlasPackage < 'a > { wgpu :: wgc :: ray_tracing :: TlasPackage { tlas_id , instances , lowest_unmodified } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn tlas_instance < 'a > (blas_id : as_hal :: BlasId , transform : & 'a [f32 ; 12] , custom_data : u32 , mask : u8) -> wgpu :: wgc :: ray_tracing :: TlasInstance < 'a > { wgpu :: wgc :: ray_tracing :: TlasInstance { blas_id , transform , custom_data , mask } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn tlas_build_entry (tlas_id : as_hal :: TlasId , instance_buffer_id : as_hal :: BufferId , instance_count : u32) -> wgpu :: wgc :: ray_tracing :: TlasBuildEntry { wgpu :: wgc :: ray_tracing :: TlasBuildEntry { tlas_id , instance_buffer_id , instance_count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blas_build_entry < 'a > (blas_id : as_hal :: BlasId , geometries : ray_tracing :: BlasGeometries < 'a >) -> wgpu :: wgc :: ray_tracing :: BlasBuildEntry < 'a > { wgpu :: wgc :: ray_tracing :: BlasBuildEntry { blas_id , geometries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn blas_triangle_geometry < 'a > (size : & 'a wgt :: BlasTriangleGeometrySizeDescriptor , vertex_buffer : as_hal :: BufferId , index_buffer : Option < as_hal :: BufferId > , transform_buffer : Option < as_hal :: BufferId > , first_vertex : u32 , vertex_stride : command :: BufferAddress , first_index : Option < u32 > , transform_buffer_offset : Option < command :: BufferAddress >) -> wgpu :: wgc :: ray_tracing :: BlasTriangleGeometry < 'a > { wgpu :: wgc :: ray_tracing :: BlasTriangleGeometry { size , vertex_buffer , index_buffer , transform_buffer , first_vertex , vertex_stride , first_index , transform_buffer_offset } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn surface_output < T = id :: TextureId > (status : present :: Status , texture : Option < T >) -> wgpu :: wgc :: present :: SurfaceOutput < T = id :: TextureId > { wgpu :: wgc :: present :: SurfaceOutput { status , texture } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_step (stride : command :: BufferAddress , last_stride : command :: BufferAddress , mode : command :: VertexStepMode) -> wgpu :: wgc :: pipeline :: VertexStep { wgpu :: wgc :: pipeline :: VertexStep { stride , last_stride , mode } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_cache_descriptor < 'a > (label : Label < 'a > , data : Option < Cow < 'a , [u8] > > , fallback : bool) -> wgpu :: wgc :: pipeline :: PipelineCacheDescriptor < 'a > { wgpu :: wgc :: pipeline :: PipelineCacheDescriptor { label , data , fallback } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn mesh_pipeline_descriptor < 'a , PLL = PipelineLayoutId , SM = ShaderModuleId , PLC = PipelineCacheId , > (label : Label < 'a > , layout : Option < PLL > , task : Option < pipeline :: TaskState < 'a , SM > > , mesh : pipeline :: MeshState < 'a , SM > , primitive : PrimitiveState , depth_stencil : Option < DepthStencilState > , multisample : MultisampleState , fragment : Option < pipeline :: FragmentState < 'a , SM > > , multiview : Option < NonZeroU32 > , cache : Option < PLC >) -> wgpu :: wgc :: pipeline :: MeshPipelineDescriptor < 'a , PLL = PipelineLayoutId , SM = ShaderModuleId , PLC = PipelineCacheId , > { wgpu :: wgc :: pipeline :: MeshPipelineDescriptor { label , layout , task , mesh , primitive , depth_stencil , multisample , fragment , multiview , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pipeline_descriptor < 'a , PLL = PipelineLayoutId , SM = ShaderModuleId , PLC = PipelineCacheId , > (label : Label < 'a > , layout : Option < PLL > , vertex : pipeline :: VertexState < 'a , SM > , primitive : PrimitiveState , depth_stencil : Option < DepthStencilState > , multisample : MultisampleState , fragment : Option < pipeline :: FragmentState < 'a , SM > > , multiview : Option < NonZeroU32 > , cache : Option < PLC >) -> wgpu :: wgc :: pipeline :: RenderPipelineDescriptor < 'a , PLL = PipelineLayoutId , SM = ShaderModuleId , PLC = PipelineCacheId , > { wgpu :: wgc :: pipeline :: RenderPipelineDescriptor { label , layout , vertex , primitive , depth_stencil , multisample , fragment , multiview , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn mesh_state < 'a , SM = ShaderModuleId > (stage : pipeline :: ProgrammableStageDescriptor < 'a , SM >) -> wgpu :: wgc :: pipeline :: MeshState < 'a , SM = ShaderModuleId > { wgpu :: wgc :: pipeline :: MeshState { stage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn task_state < 'a , SM = ShaderModuleId > (stage : pipeline :: ProgrammableStageDescriptor < 'a , SM >) -> wgpu :: wgc :: pipeline :: TaskState < 'a , SM = ShaderModuleId > { wgpu :: wgc :: pipeline :: TaskState { stage } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn fragment_state < 'a , SM = ShaderModuleId > (stage : pipeline :: ProgrammableStageDescriptor < 'a , SM > , targets : Cow < 'a , [Option < ColorTargetState >] >) -> wgpu :: wgc :: pipeline :: FragmentState < 'a , SM = ShaderModuleId > { wgpu :: wgc :: pipeline :: FragmentState { stage , targets } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_state < 'a , SM = ShaderModuleId > (stage : pipeline :: ProgrammableStageDescriptor < 'a , SM > , buffers : Cow < 'a , [pipeline :: VertexBufferLayout < 'a >] >) -> wgpu :: wgc :: pipeline :: VertexState < 'a , SM = ShaderModuleId > { wgpu :: wgc :: pipeline :: VertexState { stage , buffers } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn vertex_buffer_layout < 'a > (array_stride : command :: BufferAddress , step_mode : command :: VertexStepMode , attributes : Cow < 'a , [VertexAttribute] >) -> wgpu :: wgc :: pipeline :: VertexBufferLayout < 'a > { wgpu :: wgc :: pipeline :: VertexBufferLayout { array_stride , step_mode , attributes } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pipeline_descriptor < 'a , PLL = PipelineLayoutId , SM = ShaderModuleId , PLC = PipelineCacheId , > (label : Label < 'a > , layout : Option < PLL > , stage : pipeline :: ProgrammableStageDescriptor < 'a , SM > , cache : Option < PLC >) -> wgpu :: wgc :: pipeline :: ComputePipelineDescriptor < 'a , PLL = PipelineLayoutId , SM = ShaderModuleId , PLC = PipelineCacheId , > { wgpu :: wgc :: pipeline :: ComputePipelineDescriptor { label , layout , stage , cache } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn programmable_stage_descriptor < 'a , SM = ShaderModuleId > (module : SM , entry_point : Option < Cow < 'a , str > > , constants : naga :: back :: PipelineConstants , zero_initialize_workgroup_memory : bool) -> wgpu :: wgc :: pipeline :: ProgrammableStageDescriptor < 'a , SM = ShaderModuleId > { wgpu :: wgc :: pipeline :: ProgrammableStageDescriptor { module , entry_point , constants , zero_initialize_workgroup_memory } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn shader_module_descriptor < 'a > (label : Label < 'a > , runtime_checks : ShaderRuntimeChecks) -> wgpu :: wgc :: pipeline :: ShaderModuleDescriptor < 'a > { wgpu :: wgc :: pipeline :: ShaderModuleDescriptor { label , runtime_checks } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn hub_report (adapters : hub :: RegistryReport , devices : hub :: RegistryReport , queues : hub :: RegistryReport , pipeline_layouts : hub :: RegistryReport , shader_modules : hub :: RegistryReport , bind_group_layouts : hub :: RegistryReport , bind_groups : hub :: RegistryReport , command_encoders : hub :: RegistryReport , command_buffers : hub :: RegistryReport , render_bundles : hub :: RegistryReport , render_pipelines : hub :: RegistryReport , compute_pipelines : hub :: RegistryReport , pipeline_caches : hub :: RegistryReport , query_sets : hub :: RegistryReport , buffers : hub :: RegistryReport , textures : hub :: RegistryReport , texture_views : hub :: RegistryReport , external_textures : hub :: RegistryReport , samplers : hub :: RegistryReport) -> wgpu :: wgc :: global :: HubReport { wgpu :: wgc :: global :: HubReport { adapters , devices , queues , pipeline_layouts , shader_modules , bind_group_layouts , bind_groups , command_encoders , command_buffers , render_bundles , render_pipelines , compute_pipelines , pipeline_caches , query_sets , buffers , textures , texture_views , external_textures , samplers } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn global_report (surfaces : hub :: RegistryReport , hub : hub :: HubReport) -> wgpu :: wgc :: global :: GlobalReport { wgpu :: wgc :: global :: GlobalReport { surfaces , hub } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn context_error (fn_ident : & 'static str , source : error :: ContextErrorSource , label : String) -> wgpu :: wgc :: error :: ContextError { wgpu :: wgc :: error :: ContextError { fn_ident , source , label } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn user_closures (# [builder (default)] mappings : Vec < resource :: BufferMapPendingClosure > , # [builder (default)] blas_compact_ready : Vec < ray_tracing :: BlasCompactReadyPendingClosure > , # [builder (default)] submissions : SmallVec < [device :: life :: SubmittedWorkDoneClosure ; 1] > , # [builder (default)] device_lost_invocations : SmallVec < [device :: DeviceLostInvocation ; 1] >) -> wgpu :: wgc :: device :: UserClosures { wgpu :: wgc :: device :: UserClosures { mappings , blas_compact_ready , submissions , device_lost_invocations } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn late_min_buffer_binding_size_mismatch (group_index : u32 , compact_index : usize , shader_size : command :: BufferAddress , bound_size : command :: BufferAddress) -> wgpu :: wgc :: binding_model :: LateMinBufferBindingSizeMismatch { wgpu :: wgc :: binding_model :: LateMinBufferBindingSizeMismatch { group_index , compact_index , shader_size , bound_size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn base_pass < C , E > (label : Option < String > , error : Option < E > , commands : Vec < C > , dynamic_offsets : Vec < command :: DynamicOffset > , string_data : Vec < u8 > , push_constant_data : Vec < u32 >) -> wgpu :: wgc :: command :: BasePass < C , E > { wgpu :: wgc :: command :: BasePass { label , error , commands , dynamic_offsets , string_data , push_constant_data } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pass_timestamp_writes < QS = id :: QuerySetId > (query_set : QS , beginning_of_pass_write_index : Option < u32 > , end_of_pass_write_index : Option < u32 >) -> wgpu :: wgc :: command :: PassTimestampWrites < QS = id :: QuerySetId > { wgpu :: wgc :: command :: PassTimestampWrites { query_set , beginning_of_pass_write_index , end_of_pass_write_index } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pass_channel < V > (load_op : Option < command :: LoadOp < V > > , store_op : Option < command :: StoreOp > , read_only : bool) -> wgpu :: wgc :: command :: PassChannel < V > { wgpu :: wgc :: command :: PassChannel { load_op , store_op , read_only } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_color_attachment < TV = id :: TextureViewId > (view : TV , depth_slice : Option < u32 > , resolve_target : Option < TV > , load_op : command :: LoadOp < command :: Color > , store_op : command :: StoreOp) -> wgpu :: wgc :: command :: RenderPassColorAttachment < TV = id :: TextureViewId > { wgpu :: wgc :: command :: RenderPassColorAttachment { view , depth_slice , resolve_target , load_op , store_op } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_depth_stencil_attachment (view : as_hal :: TextureViewId , depth : command :: PassChannel < Option < f32 > > , stencil : command :: PassChannel < Option < u32 > >) -> wgpu :: wgc :: command :: RenderPassDepthStencilAttachment { wgpu :: wgc :: command :: RenderPassDepthStencilAttachment { view , depth , stencil } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn arc_render_pass_depth_stencil_attachment (view : Arc < hub :: TextureView > , depth : command :: ResolvedPassChannel < f32 > , stencil : command :: ResolvedPassChannel < u32 >) -> wgpu :: wgc :: command :: ArcRenderPassDepthStencilAttachment { wgpu :: wgc :: command :: ArcRenderPassDepthStencilAttachment { view , depth , stencil } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_pass_descriptor < 'a > (# [builder (default)] label : Label < 'a > , # [builder (default)] color_attachments : Cow < 'a , [Option < command :: RenderPassColorAttachment >] > , depth_stencil_attachment : Option < & 'a command :: RenderPassDepthStencilAttachment > , timestamp_writes : Option < & 'a command :: PassTimestampWrites > , occlusion_query_set : Option < id :: QuerySetId >) -> wgpu :: wgc :: command :: RenderPassDescriptor < 'a > { wgpu :: wgc :: command :: RenderPassDescriptor { label , color_attachments , depth_stencil_attachment , timestamp_writes , occlusion_query_set } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn rect < T > (# [builder (default)] x : T , # [builder (default)] y : T , # [builder (default)] w : T , # [builder (default)] h : T) -> wgpu :: wgc :: command :: Rect < T > { wgpu :: wgc :: command :: Rect { x , y , w , h } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn compute_pass_descriptor < 'a , PTW = PassTimestampWrites > (# [builder (default)] label : Label < 'a > , timestamp_writes : Option < PTW >) -> wgpu :: wgc :: command :: ComputePassDescriptor < 'a , PTW = PassTimestampWrites > { wgpu :: wgc :: command :: ComputePassDescriptor { label , timestamp_writes } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn render_bundle_encoder_descriptor < 'a > (# [builder (default)] label : Label < 'a > , # [builder (default)] color_formats : Cow < 'a , [Option < wgt :: TextureFormat >] > , depth_stencil : Option < wgt :: RenderBundleDepthStencil > , # [builder (default)] sample_count : u32 , multiview : Option < NonZeroU32 >) -> wgpu :: wgc :: command :: RenderBundleEncoderDescriptor < 'a > { wgpu :: wgc :: command :: RenderBundleEncoderDescriptor { label , color_formats , depth_stencil , sample_count , multiview } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn buffer_binding < B = BufferId > (buffer : B , offset : command :: BufferAddress , size : Option < command :: BufferSize >) -> wgpu :: wgc :: binding_model :: BufferBinding < B = BufferId > { wgpu :: wgc :: binding_model :: BufferBinding { buffer , offset , size } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn pipeline_layout_descriptor < 'a , BGL = BindGroupLayoutId > (label : Label < 'a > , bind_group_layouts : Cow < 'a , [BGL] > , push_constant_ranges : Cow < 'a , [timestamp_normalization :: PushConstantRange] >) -> wgpu :: wgc :: binding_model :: PipelineLayoutDescriptor < 'a , BGL = BindGroupLayoutId > { wgpu :: wgc :: binding_model :: PipelineLayoutDescriptor { label , bind_group_layouts , push_constant_ranges } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_layout_descriptor < 'a > (label : Label < 'a > , entries : Cow < 'a , [validation :: BindGroupLayoutEntry] >) -> wgpu :: wgc :: binding_model :: BindGroupLayoutDescriptor < 'a > { wgpu :: wgc :: binding_model :: BindGroupLayoutDescriptor { label , entries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_descriptor < 'a , BGL = BindGroupLayoutId , B = BufferId , S = SamplerId , TV = TextureViewId , TLAS = TlasId , ET = ExternalTextureId , > (label : Label < 'a > , layout : BGL , entries : Cow < 'a , [binding_model :: BindGroupEntry < 'a , B , S , TV , TLAS , ET >] >) -> wgpu :: wgc :: binding_model :: BindGroupDescriptor < 'a , BGL = BindGroupLayoutId , B = BufferId , S = SamplerId , TV = TextureViewId , TLAS = TlasId , ET = ExternalTextureId , > { wgpu :: wgc :: binding_model :: BindGroupDescriptor { label , layout , entries } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn bind_group_entry < 'a , B = BufferId , S = SamplerId , TV = TextureViewId , TLAS = TlasId , ET = ExternalTextureId , > (binding : u32 , resource : binding_model :: BindingResource < 'a , B , S , TV , TLAS , ET >) -> wgpu :: wgc :: binding_model :: BindGroupEntry < 'a , B = BufferId , S = SamplerId , TV = TextureViewId , TLAS = TlasId , ET = ExternalTextureId , > { wgpu :: wgc :: binding_model :: BindGroupEntry { binding , resource } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn binding_type_max_count_error (kind : binding_model :: BindingTypeMaxCountErrorKind , zone : binding_model :: BindingZone , limit : u32 , count : u32) -> wgpu :: wgc :: binding_model :: BindingTypeMaxCountError { wgpu :: wgc :: binding_model :: BindingTypeMaxCountError { kind , zone , limit , count } }


# [bon :: builder (state_mod (vis = "pub(crate)") , finish_fn = build ,)] pub fn external_texture_params (yuv_conversion_matrix : [f32 ; 16] , gamut_conversion_matrix : [f32 ; 12] , src_transfer_function : ExternalTextureTransferFunction , dst_transfer_function : ExternalTextureTransferFunction , sample_transform : [f32 ; 6] , load_transform : [f32 ; 6] , size : [u32 ; 2] , num_planes : u32 , _padding : [u8 ; 4]) -> wgpu :: wgc :: device :: resource :: ExternalTextureParams { wgpu :: wgc :: device :: resource :: ExternalTextureParams { yuv_conversion_matrix , gamut_conversion_matrix , src_transfer_function , dst_transfer_function , sample_transform , load_transform , size , num_planes , _padding } }
