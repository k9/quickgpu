use syn::Error;

use proc_macro2::TokenStream;

use proc_macro2::Span;

pub fn err(span: Span, message: &str) -> TokenStream {
    Error::new(span, message).into_compile_error()
}

#[allow(dead_code)]
pub fn prettyprint(t: TokenStream) -> String {
    prettyplease::unparse(&syn::parse2(t).unwrap())
}
