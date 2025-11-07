use std::{
    borrow::Borrow,
    fs::File,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::bail;
use proc_macro2::Span;

use syn::{Ident, Path, Visibility};

pub fn id(s: impl Into<String>) -> Ident {
    Ident::new(&s.into(), Span::call_site())
}

pub fn write_expanded(output_path: &PathBuf, crate_path: &PathBuf) -> Result<(), anyhow::Error> {
    let Ok(output) = File::create(output_path) else {
        bail!("Failed to create {:?}", &output_path);
    };

    Command::new("cargo")
        .arg("expand")
        .current_dir(crate_path)
        .stdout(Stdio::from(output))
        .status()?;

    Ok(())
}

// Relative to the workspace root
#[allow(dead_code)]
pub fn relative_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(p.into())
        .to_path_buf()
}

pub fn path_segments(path: &Path) -> Vec<Ident> {
    path.borrow()
        .segments
        .iter()
        .map(|seg| seg.ident.clone())
        .collect::<Vec<_>>()
}

pub fn path_string(path: &[Ident]) -> String {
    path.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

pub fn path_from_string(path: &str) -> Vec<Ident> {
    path.split("::").into_iter().map(|s| id(s)).collect()
}

pub fn path_refs_string(path: &Path) -> String {
    let path = path_segments(path);
    path.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

pub trait IsPublic {
    fn is_public(&self) -> bool;
}

impl IsPublic for Visibility {
    fn is_public(&self) -> bool {
        matches!(self, Visibility::Public(_))
    }
}
