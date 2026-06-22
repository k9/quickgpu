#[macro_export]
#[doc = r"Generate custom bind group helpers. See [top-level docs](crate#custom-bind-group-builder)."]
macro_rules! _create_binds_v27 {
    ($binds_name:ident, $($name:ident),*) => {
        pub struct $binds_name {
            $(pub $name: Binding),+
        }

        #[bon::bon]
        impl $binds_name {
            #[builder]
            pub fn new(
                $($name: impl quickgpu::v27::NestedBinding),*
            ) -> $binds_name {
                $binds_name {
                    $($name: $name.unnest()),*
                }
            }

            pub fn layout(
                &self,
                device: &quickgpu::wgpu_27::Device,
            ) -> quickgpu::wgpu_27::BindGroupLayout {
                bind_group_layout_descriptor(None)
                    .entries(&arr![
                        $(self.$name.layout_entry()),*
                    ])
                    .create_with(device)
            }

            #[builder(finish_fn = create)]
            pub fn group<'a>(
                &self,
                #[builder(finish_fn)]
                with_layout: &'a quickgpu::wgpu_27::BindGroupLayout,
                #[builder(finish_fn)]
                with_device: &quickgpu::wgpu_27::Device,
                $($name: quickgpu::wgpu_27::BindingResource<'a>),*,
            ) -> quickgpu::wgpu_27::BindGroup {
                $(let $name = self.$name.entry().resource($name));*;

                bind_group_descriptor(None)
                    .entries(&arr![
                        $($name),*
                    ])
                    .layout(with_layout)
                    .create_with(with_device)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _arr_v27 {
    () => ([]);
    ($($item:expr),+ $(,)?) => ([$($item.unnest()),+]);
}

#[macro_export]
#[doc(hidden)]
macro_rules! _arr_option_v27 {
    () => ([]);
    ($($item:expr),+ $(,)?) => ([$($item.map_or(None, |o| Some(o.unnest()))),+]);
}
