use anyhow::bail;
use convert_case::{Case, Casing};
use quote::quote as q;
use syn::{Fields, FieldsNamed, ImplItem, Stmt, Visibility, spanned::Spanned};

use discover_exports::{Analysis, AnalysisStruct, crate_graph::node_ident, utils::id};

use super::{AResult, SKIP};

pub(crate) fn output_struct(entry: AnalysisStruct) -> AResult<(String, String)> {
    let comment = "".to_string();
    let item = &entry.item;
    let ident = &item.ident;
    let fn_ident = id(ident.to_string().to_case(Case::Snake).as_str());
    let path = &entry.path;
    let generics = &item.generics;

    let fn_params = struct_fields(&entry)?.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        q!(#ident: #ty)
    });

    let struct_values = struct_fields(&entry)?.named.iter().map(|f| {
        let ident = &f.ident;
        q!(#ident)
    });

    let code = q! {
        #[bon::builder(
            //builder_type(doc __builder_type_docs__),
            state_mod(vis="pub(crate)"),
            finish_fn=build,
        )]
        pub fn #fn_ident #generics(
            #(#fn_params),*
        ) -> #(#path)::* #generics {
            #(#path)::* {
                #(#struct_values),*
            }
        }
    };

    Ok((comment, code.to_string()))
}

pub fn filter_struct(
    analysis: &Analysis,
    root_index: NodeIndex,
    exported: &AnalysisStruct,
) -> AResult<Option<&AnalysisStruct>> {
    let ident = node_ident(analysis, root_index, index);
    if SKIP.contains(&exported.item.ident.to_string().as_str()) {
        log::debug!("Skipping {} since it's in skip list", exported.item.ident);

        return Ok(None);
    }

    let Ok(fields) = &struct_fields(&exported) else {
        log::debug!(
            "Skipping {} since it doesn't have named fields",
            exported.item.ident
        );

        return Ok(None);
    };

    if fields
        .named
        .iter()
        .any(|f| !matches!(f.vis, Visibility::Public(_)))
    {
        log::debug!(
            "Skipping {} since it has non-public fields",
            exported.item.ident
        );

        return Ok(None);
    };

    for impl_item in &exported.impls {
        if let Some((_, trait_item, _)) = &impl_item.trait_
            && trait_item
                .segments
                .last()
                .is_some_and(|segment| segment.ident.to_string() == "Default")
        {
            let ty = &impl_item.self_ty;
            if impl_item
                .attrs
                .iter()
                .any(|attr| q!(# [automatically_derived]).to_string() == q!(#attr).to_string())
            {
                println!("{} derived", q!(#ty));
            } else if let ImplItem::Fn(func) = &impl_item.items[0]
                && let Some(Stmt::Expr(expr, _)) = func.block.stmts.last()
            {
                if let syn::Expr::Path(expr_path) = expr {
                    println!("{} _PATH_", expr_path.span().source_text().unwrap());
                } else {
                    println!("{} _CUSTOM_", expr.span().source_text().unwrap());
                };
            }
        }
    }

    for f in &fields.named {
        let ident = &f.ident;
        let ty = &f.ty;
        log::debug!("    {}", q!(#ident: #ty));
    }

    Ok(Some(exported))
}

pub fn struct_fields(entry: &AnalysisStruct) -> AResult<&FieldsNamed> {
    let Fields::Named(named) = &entry.item.fields else {
        bail!("Struct doesn't have named fields {}", entry.item.ident);
    };

    Ok(named)
}
