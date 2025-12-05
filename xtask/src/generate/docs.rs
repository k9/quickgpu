use crate::generate::struct_entry::{BuilderField, ident_from_path};
use quote::ToTokens;

pub fn builder_fn_docs(path: &syn::Path, fields: &[BuilderField]) -> String {
    let ident = ident_from_path(path).into_token_stream();
    let builder_ident = format!("{ident}Builder");
    let path_tokens = path.into_token_stream();
    let path_tokens = path_tokens.to_string().replace(" ", "");

    let mut top_level_doc = format!(
        "
Returns [{builder_ident}] for building [`{path_tokens}`]
        "
    );

    top_level_doc.push_str(&table(builder_ident, fields));

    top_level_doc
}

pub fn builder_docs(path: &syn::Path, fields: &[BuilderField]) -> String {
    let ident = ident_from_path(path).into_token_stream();
    let builder_ident = format!("{ident}Builder");
    let path_tokens = path.into_token_stream();
    let path_tokens = path_tokens.to_string().replace(" ", "");

    let mut top_level_doc = format!(
        "
Builder for [`{path_tokens}`]
        "
    );

    top_level_doc.push_str(&table(builder_ident, fields));

    top_level_doc
}

pub fn table(builder_ident: String, fields: &[BuilderField]) -> String {
    let fields = fields
        .iter()
        .filter(|field| field.field.ident.as_ref().unwrap().to_string() != "label");

    let mut table_doc = if fields.clone().count() > 0 {
        format!(
            "
Set all required fields and any optional fields, then call `build()`.

Builder fields:
"
        )
    } else {
        "".to_string()
    };

    for field in fields {
        let default_string = default_string(field);
        let field_ident = field.field.ident.as_ref().unwrap().into_token_stream();

        table_doc.push_str(&format!(
            "  - [{field_ident}]({builder_ident}::{field_ident}) {default_string}\n",
        ));
    }

    table_doc
}

fn default_string(field: &BuilderField) -> String {
    field
        .default_value
        .as_ref()
        .map_or("Required".to_string(), |s| {
            let mut s = s.into_token_stream().to_string().replace(" ", "");
            if s.contains("::") {
                s = format!("[{}]", s);
            } else {
                s = format!("`{}`", s);
            }

            format!("Optional, defaults to {}", s)
        })
}

pub fn setter_docs(path: &syn::Path, field: &BuilderField) -> String {
    let default_string = default_string(&field);
    let field_ident = field.field.ident.as_ref().unwrap().into_token_stream();
    let path = path.into_token_stream().to_string().replace(" ", "");

    format!("Setter for [{path}::{field_ident}]. {default_string}\n",)
}
