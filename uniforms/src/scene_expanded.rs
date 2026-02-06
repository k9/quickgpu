Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "ty", span: #0 bytes(11023..11025) }, arguments: PathArguments::None }] }
Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "stages", span: #0 bytes(11040..11046) }, arguments: PathArguments::None }] }
Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "usage", span: #0 bytes(11103..11108) }, arguments: PathArguments::None }] }
pub mod scene {
    use std::{borrow::Cow, time::Instant};
    use binder::QBind;
    use bytemuck::NoUninit;
    use quickgpu::*;
    use wgpu::{
        BindGroup, Color, CommandBuffer, Device, LoadOp, Queue, RenderPipeline,
        ShaderSource, TextureFormat,
    };
    use crate::app::RenderTextures;
    pub struct GPUState<'a> {
        pub render_textures: RenderTextures<'a>,
        pub device: &'a Device,
        pub queue: &'a Queue,
    }
    pub struct Scene {
        render_pipeline: RenderPipeline,
        group: [BindGroup; 2],
        resources: [TriangleResources; 2],
        start: Instant,
    }
    #[repr(C)]
    pub struct Triangle {
        #[qbind(
            ty(f32),
            stages(ShaderStages::VERTEX|ShaderStages::FRAGMENT),
            usage(BufferUsages::UNIFORM|BufferUsages::COPY_DST),
            binding_type(5)
        )]
        pub redness: f32,
        #[qbind(
            ty(vec2<f32>),
            stages(ShaderStages::VERTEX),
            usage(BufferUsages::UNIFORM|BufferUsages::COPY_DST),
            binding_type(
                BindingType::Buffer{has_dynamic_offset:false,
                min_binding_size:None,
                ty:BufferBindingType::Storage{read_only:true}}
            )
        )]
        pub offset: [f32; 2],
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Triangle {}
    #[automatically_derived]
    impl ::core::clone::Clone for Triangle {
        #[inline]
        fn clone(&self) -> Triangle {
            let _: ::core::clone::AssertParamIsClone<f32>;
            let _: ::core::clone::AssertParamIsClone<[f32; 2]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Triangle {}
    const _: () = {
        if !(::core::mem::size_of::<Triangle>()
            == (::core::mem::size_of::<f32>() + ::core::mem::size_of::<[f32; 2]>()))
        {
            {
                ::std::rt::begin_panic(
                    "derive(NoUninit) was applied to a type with padding",
                );
            }
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::NoUninit>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::NoUninit>() {}
            assert_impl::<[f32; 2]>();
        }
    };
    unsafe impl ::bytemuck::NoUninit for Triangle {}
    pub fn shader_source() -> String {
        let green_wgsl = Triangle::WGSL;
        ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "\n{0}\n\n@vertex\nfn vs_main(\n    @builtin(vertex_index) vertex_index: u32,\n) -> @builtin(position) vec4f {{\n    let pos = array(\n          vec2f(0.0,  0.25),  // top center\n          vec2f(-0.25, -0.25),  // bottom left\n          vec2f(0.25, -0.25)   // bottom right\n    );\n    \n    return vec4f(pos[vertex_index] + offset, 0.0, 1.0);\n}}\n\n@fragment\nfn fs_main() -> @location(0) vec4<f32> {{\n    return vec4<f32>(\n        redness,\n        0.3,\n        0.3,\n        1.0\n    );\n}}\n",
                    green_wgsl,
                ),
            )
        })
    }
    impl Scene {
        pub fn new(device: &Device, format: TextureFormat, sample_count: u32) -> Self {
            let shader = device
                .create_shader_module(
                    shader_module_descriptor(None)
                        .source(ShaderSource::Wgsl(Cow::Owned(shader_source())))
                        .build(),
                );
            let bgl = device
                .create_bind_group_layout(
                    &bind_group_layout_descriptor(Some("bgl"))
                        .entries(&Triangle::layout_entries())
                        .build(),
                );
            let data = [
                Triangle {
                    redness: 0.0,
                    offset: [0.0, 0.33],
                },
                Triangle {
                    redness: 0.0,
                    offset: [0.0, -0.33],
                },
            ];
            let resources = [
                Triangle::resources(device, &data[0]),
                Triangle::resources(device, &data[1]),
            ];
            let groups = [
                Triangle::bind_group(device, None, &bgl, &resources[0]),
                Triangle::bind_group(device, None, &bgl, &resources[1]),
            ];
            let layout = device
                .create_pipeline_layout(
                    &pipeline_layout_descriptor(Some("Layout"))
                        .bind_group_layouts(&[&bgl])
                        .build(),
                );
            let render_pipeline = render_pipeline_descriptor(Some("Render Pipeline"))
                .layout(&layout)
                .vertex(
                    vertex_state().module(&shader).entry_point("vs_main").buffers(&[]),
                )
                .fragment(
                    fragment_state()
                        .module(&shader)
                        .entry_point("fs_main")
                        .targets(&[Some(format.into())]),
                )
                .primitive(primitive_state().cull_mode(wgpu::Face::Back))
                .multisample(multisample_state().count(sample_count))
                .create_with(device);
            Scene {
                render_pipeline,
                group: groups,
                resources,
                start: Instant::now(),
            }
        }
        #[must_use]
        pub fn render(
            &mut self,
            GPUState { render_textures, device, queue }: GPUState,
        ) -> CommandBuffer {
            let elapsed = self.start.elapsed().as_secs_f32();
            let t = (elapsed * 0.5).fract();
            self.resources[0].write_redness(queue, &linear(t));
            self.resources[0].write_offset(queue, &[-0.33, linear(t) * 0.66 - 0.33]);
            self.resources[1]
                .write(
                    queue,
                    Triangle {
                        redness: cubic(t),
                        offset: [0.33, cubic(t) * 0.66 - 0.33],
                    },
                );
            let mut encoder = command_encoder_descriptor(None).create_with(device);
            {
                let color_attachments = builders([
                    Some(
                        render_pass_color_attachment()
                            .view(render_textures.view)
                            .maybe_resolve_target(render_textures.resolve_target)
                            .ops(operations().load(LoadOp::Clear(Color::WHITE))),
                    ),
                ]);
                let mut render_pass = render_pass_descriptor(Some("Render Pass"))
                    .color_attachments(&color_attachments)
                    .begin_with(&mut encoder);
                render_pass.set_bind_group(0, Some(&self.group[0]), &[]);
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw(0..3, 0..1);
                render_pass.set_bind_group(0, Some(&self.group[1]), &[]);
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw(0..3, 0..1);
            }
            encoder.finish()
        }
    }
    pub fn linear(t: f32) -> f32 {
        let value = t * 2.0;
        if value < 1.0 { value } else { 2.0 - value }
    }
    pub fn cubic(t: f32) -> f32 {
        let t = linear(t);
        if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
    }
}
