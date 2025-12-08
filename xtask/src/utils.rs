use std::path::PathBuf;

use anyhow::Context;
use convert_case::{Case, Casing};
use duct::cmd;
use quote::format_ident;
use syn::{Field, GenericArgument, Ident, Path, PathArguments, Type};

use crate::AResult;

// Relative to the workspace root
pub fn relative_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(p.into())
        .to_path_buf()
}

pub fn rustfmt(path: PathBuf) -> AResult<()> {
    cmd!("rustfmt", path.into_os_string()).run()?;

    Ok(())
}

pub fn final_path(path: &str) -> AResult<String> {
    Ok(path
        .split("::")
        .last()
        .context("Problem parsing path")?
        .to_string())
}

#[derive(PartialEq)]
pub enum OptionType {
    None,
    Option,
    Label,
}

pub fn option_type(field: &Field) -> OptionType {
    if let Type::Path(path) = &field.ty {
        let ident = path.path.segments.last().map(|s| s.ident.to_string());

        if ident == Some("Option".to_string()) {
            return OptionType::Option;
        } else if ident == Some("Label".to_string()) {
            return OptionType::Label;
        }
    }

    OptionType::None
}

pub fn without_args(path: &Path) -> Path {
    let mut path = path.clone();
    for segment in path.segments.iter_mut() {
        segment.arguments = PathArguments::None;
    }

    path
}

pub enum FieldIdent {
    Original,
    UpperCamel,
    Value,
    Empty,
    Set,
    Optional,
    SetterFn,
    SetterMaybeFn,
}

pub fn field_ident(field: &Field, field_ident: FieldIdent) -> Ident {
    let ident = field.ident.as_ref().unwrap();

    let upper = format_ident!("{}", ident.to_string().to_case(Case::UpperCamel));

    match field_ident {
        FieldIdent::Original => ident.clone(),
        FieldIdent::UpperCamel => upper,
        FieldIdent::Value => format_ident!("{}Value", upper),
        FieldIdent::Empty => format_ident!("{}Empty", upper),
        FieldIdent::Optional => format_ident!("{}Optional", upper),
        FieldIdent::Set => format_ident!("Set{}", upper),
        FieldIdent::SetterFn => format_ident!("{}", ident),
        FieldIdent::SetterMaybeFn => format_ident!("maybe_{}", ident),
    }
}

pub enum StructIdent {
    Builder,
    BuilderMod,
    Fn,
}

pub fn struct_ident(ident: &Ident, struct_ident: StructIdent) -> Ident {
    let snake = format_ident!("{}", ident.to_string().to_case(Case::Snake));

    match struct_ident {
        StructIdent::Builder => format_ident!("{}Builder", ident),
        StructIdent::BuilderMod => format_ident!("{}_builder", snake),
        StructIdent::Fn => format_ident!("{}", snake),
    }
}

pub fn option_argument(ty: &mut Type) -> Option<&mut GenericArgument> {
    if let Type::Path(path) = ty
        && let Some(last) = path.path.segments.last_mut()
        && let PathArguments::AngleBracketed(arguments) = &mut last.arguments
        && let Some(arg) = arguments.args.first_mut()
    {
        Some(arg)
    } else {
        None
    }
}
