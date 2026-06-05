use std::path::PathBuf;

use anyhow::Context;
use convert_case::{Boundary, Case, Casing};
use duct::cmd;
use syn::{Field, GenericArgument, Path, PathArguments, Type};

use crate::AResult;

// Relative to the workspace root
pub(crate) fn relative_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(p.into())
        .to_path_buf()
}

pub(crate) fn rustfmt(path: PathBuf) -> AResult<()> {
    cmd!("rustfmt", path.into_os_string()).run()?;

    Ok(())
}

pub(crate) fn final_path(path: &str) -> AResult<String> {
    Ok(path
        .split("::")
        .last()
        .context("Problem parsing path")?
        .trim()
        .to_string())
}

#[derive(PartialEq)]
pub enum OptionType {
    None,
    Option,
    Label,
}

pub(crate) fn option_type(field: &Field) -> OptionType {
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

pub(crate) fn without_args(path: &Path) -> Path {
    let mut path = path.clone();
    for segment in path.segments.iter_mut() {
        segment.arguments = PathArguments::None;
    }

    path
}

pub(crate) fn option_argument(ty: &mut Type) -> Option<&mut GenericArgument> {
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

pub(crate) fn snake(s: impl ToString) -> String {
    #[allow(clippy::disallowed_methods)]
    s.to_string()
        .set_boundaries(&[Boundary::LowerUpper, Boundary::DigitUpper])
        .to_case(Case::Snake)
}

pub(crate) fn upper_camel(s: impl ToString) -> String {
    #[allow(clippy::disallowed_methods)]
    s.to_string().to_case(Case::UpperCamel)
}
