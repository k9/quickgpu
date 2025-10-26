use std::path::PathBuf;

use anyhow::Context;
use duct::cmd;
use proc_macro2::Span;
use syn::Ident;

use crate::AResult;

// Relative to the workspace root
pub fn relative_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(p.into())
        .to_path_buf()
}

pub fn parse_docs(path: impl Into<PathBuf>) -> AResult<rustdoc_types::Crate> {
    let json_string = std::fs::read_to_string(path.into())?;
    Ok(serde_json::from_str::<rustdoc_types::Crate>(&json_string)?)
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
