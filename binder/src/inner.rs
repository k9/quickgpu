use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::bind_group_code;

pub const PUBLIC_NAMED: &str =
    "BindGroup derive input must be struct with all public named fields.";

pub(crate) fn entry_point(input: DeriveInput) -> TokenStream {
    #[cfg(test)]
    env_logger::init();

    let code = match bind_group_code::bind_group_code(input) {
        Ok(value) => value,
        Err(value) => return value,
    };

    log::debug!("code {}", code);

    code
}

#[cfg(test)]
mod tests {
    use crate::inner::entry_point;
    use crate::utils::prettyprint;
    use syn::parse_quote;

    #[test]
    fn simple() {
        println!(
            "\n{}\n",
            prettyprint(entry_point(parse_quote! {
                #[derive(QBind)]
                struct A {
                    #[qbind(
                        ty(vec4<f32>),
                        stages(ShaderStages::VERTEX),
                        usage(a),
                        binding_type(4)
                    )]
                    pub x: Vector4<f32>,
                    #[qbind(
                        ty(u32),
                        stages(ShaderStages::VERTEX),
                        usage(b),
                        binding_type(45)
                    )]
                    pub yyy: u32
                }
            }))
            .replace(
                "\\n", "
"
            )
        );
    }
}
