use std::cmp::Ordering;

use crate::{
    AResult,
    analyze::core::StructParts,
    output::{
        field::{field_details, field_fn_param, field_return_param},
        types::generic_params,
    },
    utils::ident,
};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote as q;

#[derive(Debug)]
pub struct UseInstance {
    pub source: String,
    pub name: String,
}

pub struct StructOutput {
    pub comment: String,
    pub code: String,
    pub initializer: TokenStream,
    pub builder_struct: TokenStream,
}

pub fn output_struct(struct_item: StructParts, builder_types: &[String]) -> AResult<StructOutput> {
    let comment = struct_item.default_value.get_comment();
    let name = struct_item.type_alias_map.map_name(&struct_item.name);
    let struct_ident = ident(&name);
    let struct_generics = generic_params(&struct_item, &[])?;
    let build_generics = generic_params(&struct_item, &[ident("BuilderTypeState")])?;
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

    let path = struct_item.path;
    let mut top_level_doc = format!(
        "
Returns [`{struct_ident}Builder`] for building [`{path}`]
"
    );

    let mut table_doc = format!(
        "
|Builder Field|Status|
|-|-|
"
    );

    for field in &details {
        let default_string = field
            .default_string
            .clone()
            .map_or("Required".to_string(), |s| {
                format!("Defaults to `{}`", s.replace(" ", ""))
            });

        table_doc.push_str(&format!("|{}|{}|\n", field.name, default_string));
    }

    top_level_doc.push_str(&table_doc);

    let mut fn_params = vec![];
    for f in &details {
        fn_params.push(field_fn_param(f)?)
    }

    let mut return_params = vec![];
    for f in &struct_item.fields {
        return_params.push(field_return_param(f)?)
    }

    let initializer = q!(#fn_ident);
    let builder_struct_ident = ident(&format!("{}Builder", struct_ident));
    let builder_mod_ident = ident(&format!("{}_builder", fn_ident).replace("dx_12", "dx12"));
    let builder_struct = q!(#builder_struct_ident);

    let mut code = q!(
        #[doc = #top_level_doc]
        #[bon::builder(
            builder_type(doc __builder_type_docs__),
            state_mod(vis="pub(crate)"),
            finish_fn=build,
        )]
        #[builder(derive(Into))]
        pub fn #fn_ident #struct_generics(
            #(#fn_params),*
        ) -> #struct_ident #struct_generics {
            #struct_ident {
                #(#return_params),*
            }
        }
    );

    for d in &details {
        if builder_types.contains(&d.ty.to_string()) {
            let field_fn = &d.name;
            let field_builder_fn = ident(&format!("{}_builder", d.name));
            let field_state_ident = ident(&d.name.to_string().to_case(Case::Pascal));
            let field_set_ident =
                ident(&format!("Set{}", d.name.to_string().to_case(Case::Pascal)));

            panic!("{:?} {:?}", builder_struct_ident, d.path);
            let field_builder_path = d.path.split("::").last().unwrap();
            let field_builder_struct = ident(&format!("{}Builder", field_builder_path));

            let field_builder_mod_ident = ident(&format!(
                "{}_builder",
                field_builder_path.to_case(Case::Snake)
            ));

            code = q!(
                #code

                impl #build_generics #builder_struct_ident #build_generics
                where
                    BuilderTypeState: #builder_mod_ident::State,
                    BuilderTypeState::#field_state_ident: #builder_mod_ident::IsUnset,
                {
                    pub fn #field_builder_fn<FieldBuilderTypeState: #field_builder_mod_ident::IsComplete>(
                        self,
                        v: #field_builder_struct,
                    ) -> #field_builder_struct<
                        'a,
                        #field_builder_mod_ident::#field_set_ident<FieldBuilderTypeState>,
                    > {
                        self.#field_fn(v.build())
                    }
                }
            );
        }
    }

    let table_doc = format!(
        "
Builder for [`{path}`]. Create with [`{fn_ident}`]
{table_doc}
"
    );

    let table_doc = table_doc.replace("\n", "\n///");
    let table_doc = format!(
        "{{
{table_doc}
}}"
    );

    let code = code
        .to_string()
        .replace("__builder_type_docs__", &table_doc);

    Ok(StructOutput {
        comment,
        code,
        initializer,
        builder_struct,
    })
}
