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
    use crate::bind_group_code;
    use syn::parse_quote;

    #[test]
    fn simple() {
        let p = bind_group_code::bind_group_code(parse_quote! {
            pub struct MyBinds {
                pub points: BufferBind<[[f32; 4]; 6]>,
                pub size: BufferBind<u32>,
                pub pattern: TextureBind<[u8; 16]>,
                pub pattern_sampler: SamplerBind,
            }
        });

        let code = p.expect("bind_group_code should succeed for valid input");
        let rendered = code.to_string();

        assert!(rendered.contains("pub mod my_binds_mod"));
        assert!(rendered.contains("pub struct MyBinds"));
        assert!(rendered.contains("pub struct MyBindsResources"));
        assert!(rendered.contains("pub struct MyBindsEntries"));
        assert!(rendered.contains("pub struct MyBindsOffsets"));
        assert!(rendered.contains("pub struct MyBindsDeclarations"));

        // Buffer fields appear in the entries struct wrapped in Option;
        // texture/sampler do not appear there at all.
        assert!(rendered.contains("pub points : Option < for"));
        assert!(rendered.contains("pub size : Option < for"));
        assert!(!rendered.contains("pub pattern : Option"));
        assert!(!rendered.contains("pub pattern_sampler : Option"));

        // Texture/sampler resources still show up in the resources struct.
        assert!(rendered.contains("pub pattern : & 'a BoundTextureView"));
        assert!(rendered.contains("pub pattern_sampler : & 'a BoundSampler"));
    }

    #[test]
    fn rejects_non_struct() {
        let p = bind_group_code::bind_group_code(parse_quote! {
            pub enum NotAStruct { A, B }
        });
        assert!(p.is_err());
    }

    #[test]
    fn rejects_tuple_struct() {
        let p = bind_group_code::bind_group_code(parse_quote! {
            pub struct Tup(pub BufferBind<u32>);
        });
        assert!(p.is_err());
    }

    #[test]
    fn rejects_non_bind_field() {
        let p = bind_group_code::bind_group_code(parse_quote! {
            pub struct Bad {
                pub x: u32,
            }
        });
        assert!(p.is_err());
    }

    #[test]
    fn rejects_unknown_bind_kind() {
        let p = bind_group_code::bind_group_code(parse_quote! {
            pub struct Bad {
                pub x: WeirdBind<u32>,
            }
        });
        assert!(p.is_err());
    }
}
