use std::path::PathBuf;

use anyhow::Context;
use duct::cmd;
use proc_macro2::Span;
use syn::{Field, Ident, Path, PathArguments, Type};

use crate::AResult;

// Relative to the workspace root
pub fn relative_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(p.into())
        .to_path_buf()
}

pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
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

pub fn is_option(field: &Field) -> bool {
    if let Type::Path(path) = &field.ty
        && path.path.segments.last().map(|s| s.ident.to_string()) == Some("Option".to_string())
    {
        true
    } else {
        false
    }
}

pub fn without_args(path: &Path) -> Path {
    let mut path = path.clone();
    for segment in path.segments.iter_mut() {
        segment.arguments = PathArguments::None;
    }

    path
}
