use crate::generate::struct_entry::{BuilderField, ident_from_path};
use quote::ToTokens;

pub fn builder_fn_docs(path: &syn::Path, fields: &[BuilderField]) -> String {
    let ident = ident_from_path(path).into_token_stream();
    let path_tokens = path.into_token_stream();
    let path_tokens = path_tokens.to_string().replace(" ", "");

    let mut top_level_doc = format!(
        "
Returns [`{ident}Builder`] for building [`{path_tokens}`]
        "
    );

    top_level_doc.push_str(&table(fields));

    top_level_doc
}

pub fn builder_docs(path: &syn::Path, fields: &[BuilderField]) -> String {
    let path_tokens = path.into_token_stream();

    let mut top_level_doc = format!(
        "
Builder for [`{path_tokens}`]
        "
    );

    top_level_doc.push_str(&table(fields));

    top_level_doc
}

pub fn table(fields: &[BuilderField]) -> String {
    let mut table_doc = format!(
        "
Set all required fields and any optionaly fields, then call `build()`.
|Builder Field|Status|
|-|-|
"
    );

    for field in fields {
        if field.field.ident.as_ref().unwrap().to_string() == "label" {
            continue;
        }

        let default_string = field
            .default_value
            .as_ref()
            .map_or("Required".to_string(), |s| {
                format!(
                    "Optional, defaults to `{}`",
                    s.into_token_stream().to_string().replace(" ", "")
                )
            });

        table_doc.push_str(&format!(
            "|{}|{}|\n",
            field.field.ident.as_ref().unwrap().into_token_stream(),
            default_string
        ));
    }

    table_doc
}
