use std::process::Stdio;

use std::process::Command;

use std::fs::File;

use std::path::PathBuf;

use anyhow::bail;
use proc_macro2::Span;

use quote::ToTokens;
use syn::Ident;
use syn::Item;
use syn::ItemMod;
use syn::Token;
use syn::token::Brace;

pub fn id<'a>(s: impl Into<&'a str>) -> Ident {
    Ident::new(s.into(), Span::call_site())
}

pub fn token_string<'a>(s: &impl ToTokens) -> String {
    s.to_token_stream().to_string()
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

pub fn dummy_module(name: &str, items: Vec<Item>) -> ItemMod {
    ItemMod {
        attrs: vec![],
        vis: syn::Visibility::Public(Token![pub](Span::call_site())),
        unsafety: None,
        mod_token: Token![mod](Span::call_site()),
        ident: id(name),
        content: Some((Brace(Span::call_site()), items)),
        semi: None,
    }
}
