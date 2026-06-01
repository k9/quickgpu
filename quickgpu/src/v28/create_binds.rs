
macro_rules! create_binds {
            // This macro takes an argument of designator `ident` and
            // creates a function named `$func_name`.
            // The `ident` designator is used for variable/function names.
            ($binds_name:ident, $($name:ident),*) => {
                pub struct $binds_name {
                    $(pub $name: Binding),+
                }

                #[bon::bon]
                impl $binds_name {
                    #[builder]
                    pub fn new(
                        $($name: impl $crate::binds::NestedBinding),*
                    ) -> $binds_name {
                        $binds_name {
                            $($name: $name.unnest()),*
                        }
                    }

                    pub fn layout(
                        &self,
                        device: &wgpu_28::Device,
                    ) -> wgpu_28::BindGroupLayout {
                        bind_group_layout_descriptor(None)
                            .entries(&builders([
                                $(self.$name.layout_entry()),*
                            ]))
                            .create_with(device)
                    }

                    #[builder(finish_fn = create)]
                    pub fn group<'a>(
                        &self,
                        #[builder(finish_fn)]
                        with_layout: &'a wgpu_28::BindGroupLayout,
                        #[builder(finish_fn)]
                        with_device: &wgpu_28::Device,
                        $($name: wgpu_28::BindingResource<'a>),*,
                    ) -> wgpu_28::BindGroup {
                        $(let $name = self.$name.entry().resource($name));*;

                        bind_group_descriptor(None)
                            .entries(&builders([
                                $($name),*
                            ]))
                            .layout(with_layout)
                            .create_with(with_device)
                    }
                }
            };
        }

pub use create_binds;
