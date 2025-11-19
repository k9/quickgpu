use std::path::PathBuf;

use anyhow::Context;
use convert_case::{Case, Casing};
use duct::cmd;
use quote::format_ident;
use syn::{Field, Ident, Path, PathArguments, Type};

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

pub fn upper_camel_ident(field: &Field) -> Ident {
    format_ident!(
        "{}",
        field
            .ident
            .as_ref()
            .unwrap()
            .to_string()
            .to_case(Case::UpperCamel)
    )
}
