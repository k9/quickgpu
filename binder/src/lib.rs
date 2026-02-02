use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod bind_group_code;
mod field;
mod inner;
mod utils;

#[proc_macro_derive(QBind, attributes(qbind))]
pub fn derive_bind_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    inner::entry_point(input).into()
}
