use crate::{
    analyze::{FieldParts, StructAnalysis, StructParts, report},
    analyze_default::DefaultValue,
    data,
    utils::{ident, parse_docs, relative_path, rustfmt},
};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote as q;

pub fn generate() -> anyhow::Result<()> {
    let doc_path = relative_path("wgpu/wgpu");

    std::process::Command::new("cargo")
        .current_dir(doc_path)
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format=json")
        .arg("+nightly")
        .arg("doc")
        .output()?;

    let data = data::Data::new(
        parse_docs("wgpu/target/doc/wgpu.json")?,
        parse_docs("wgpu/target/doc/wgpu_types.json")?,
    );

    let mut wgt_structs = vec![];

    for (_, v) in data.iter_base() {
        if let rustdoc_types::ItemEnum::Use(u) = &v.inner
            && !u.is_glob
            && u.source.starts_with("wgt::")
        {
            wgt_structs.push(u.name.to_string());
        }
    }

    let mut structs = vec![];
    for (_, v) in data.iter_wgt() {
        if let Some(name) = &v.name
            && wgt_structs.contains(name)
        {
            let analysis = StructAnalysis::analyze(v, &data.wgt, &data);
            report(v, &analysis);
            structs.push(analysis);
        }
    }

    for (_, v) in data.iter_base() {
        let analysis = StructAnalysis::analyze(v, &data.base, &data);
        report(v, &analysis);
        structs.push(analysis);
    }

    let mut builders = vec![(
        "".to_string(),
        "
use wgpu::util::*;
use wgpu::*;
"
        .to_string(),
    )];

    for struct_item in structs {
        if let StructAnalysis::Parts(struct_item) = struct_item {
            builders.push(generate_struct(struct_item));
        }
    }

    let combined = builders
        .iter()
        .map(|(comment, code)| format!("{comment}\n{code}\n"))
        .collect::<Vec<String>>()
        .join("\n");

    let combined = rustfmt(combined)?;

    let output_path = relative_path("quickgpu/src/builders.rs");
    std::fs::write(output_path, combined)?;

    Ok(())
}

fn generate_struct(struct_item: StructParts) -> (String, String) {
    let comment = struct_item.default_value.get_comment();
    let name = struct_item.name;
    let struct_ident = ident(&name);
    let fn_ident = ident(&name.to_case(Case::Snake));

    let comment = format!("/*\n{comment}\n*/");

    let fn_params = struct_item.fields.iter().map(field_fn_param);
    let return_params = struct_item.fields.iter().map(field_return_param);

    let code = q!(
        #[bon::builder(state_mod(vis="pub(crate)"))]
        pub fn #fn_ident(
            #(#fn_params),*
        ) -> #struct_ident {
            #struct_ident {
                #(#return_params),*
            }
        }
    );

    let code = code.to_string();
    (comment, code)
}

pub struct GeneratedField {
    pub fn_param: TokenStream,
    pub return_param: TokenStream,
}

fn field_fn_param(field: &FieldParts) -> TokenStream {
    let field_name = ident(&field.name);

    let mut attrs = vec![];
    match &field.default_value {
        DefaultValue::None { msg: _ } => (),
        DefaultValue::Default { source: _ } => attrs.push(q!(default)),
        DefaultValue::Value { source: _, value } => attrs.push(q!(default=#value)),
    };

    let attrs = if attrs.is_empty() {
        q!()
    } else {
        q!(
            #[builder(#(#attrs)*,)]
        )
    };

    q!(
        #attrs
        #field_name: u64
    )
}

fn field_return_param(field: &FieldParts) -> TokenStream {
    let field_name = ident(&field.name);
    q!(#field_name)
}
