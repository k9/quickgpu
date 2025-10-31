use std::borrow::Borrow;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use anyhow::bail;
use proc_macro2::Span;

use syn::Ident;
use syn::Item;
use syn::ItemMod;
use syn::Path;
use syn::Token;
use syn::Visibility;
use syn::token::Brace;

use crate::analysis::AnalysisMod;
use crate::analysis::CrateRoot;

pub fn id<'a>(s: impl Into<&'a str>) -> Ident {
    Ident::new(s.into(), Span::call_site())
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

pub fn krate(name: &str, crate_root: bool, items: Vec<Item>) -> AnalysisMod {
    AnalysisMod {
        item: ItemMod {
            attrs: vec![],
            vis: syn::Visibility::Public(Token![pub](Span::call_site())),
            unsafety: None,
            mod_token: Token![mod](Span::call_site()),
            ident: id(name),
            content: Some((Brace(Span::call_site()), items)),
            semi: None,
        },
        crate_root: if crate_root {
            Some(CrateRoot::default())
        } else {
            None
        },
    }
}

pub fn path_segments(path: &Path) -> Vec<&Ident> {
    path.borrow()
        .segments
        .iter()
        .map(|seg| &seg.ident)
        .collect::<Vec<_>>()
}

pub fn path_string(path: &[Ident]) -> String {
    path.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join("::")
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
