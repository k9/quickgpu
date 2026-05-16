use std::path::PathBuf;

use anyhow::Context;
use duct::cmd;
use syn::{Field, GenericArgument, Path, PathArguments, Type};

use crate::AResult;

pub fn libs_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("libs")
        .join(p.into())
        .to_path_buf()
}

pub fn generate_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("generate")
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
