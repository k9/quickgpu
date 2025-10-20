use std::cmp::Ordering;

use crate::{
    analyze::core::StructParts,
    output::{
        field::{field_details, field_fn_param, field_return_param},
        types::generic_params,
    },
    utils::ident,
};
use convert_case::{Case, Casing};
use quote::quote as q;

#[derive(Debug)]
pub struct UseInstance {
    pub source: String,
    pub name: String,
}

pub fn output_struct(struct_item: StructParts) -> anyhow::Result<(String, String)> {
    let comment = struct_item.default_value.get_comment();
    let name = struct_item.type_alias_map.map_name(&struct_item.name);
    let struct_ident = ident(&name);
    let struct_generics = generic_params(&struct_item)?;
    let fn_ident = ident(
        &name
            .to_case(Case::Snake)
            .replace("_2_d", "2d")
            .replace("_3_d", "3d"),
    );

    let comment = format!("/*\n{comment}\n*/");

    let mut details = vec![];
    for f in &struct_item.fields {
        details.push(field_details(
            f,
            &struct_item.path,
            &struct_item.type_alias_map,
        )?)
    }

    details.sort_by(|a, _b| {
        if a.start_fn.is_some() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let mut fn_params = vec![];
    for f in details {
        fn_params.push(field_fn_param(f)?)
    }

    let mut return_params = vec![];
    for f in &struct_item.fields {
        return_params.push(field_return_param(f)?)
    }

    let path = struct_item.path;
    let top_level_doc = format!(
        "
        Returns [`{struct_ident}Builder`] for building [`{path}`]

        |Setter|Status|
        |-|-|
        |width|Required|
        |height|Required|
        |fill|Optional - default 0u32|
    "
    );
    let code = q!(
        #[doc = #top_level_doc]
        #[bon::builder(state_mod(vis="pub(crate)"), finish_fn=build, derive(Into))]
        pub fn #fn_ident #struct_generics(
            #(#fn_params),*
        ) -> #struct_ident #struct_generics {
            #struct_ident {
                #(#return_params),*
            }
        }
    );

    let code = code.to_string();
    Ok((comment, code))
}
