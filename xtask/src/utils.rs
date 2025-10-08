use std::{
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
};

use anyhow::Context;
use proc_macro2::Span;
use syn::Ident;

// Relative to the workspace root
pub fn relative_path(p: impl Into<PathBuf>) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(p.into())
        .to_path_buf()
}

pub fn parse_docs(path: impl Into<PathBuf>) -> Result<rustdoc_types::Crate, anyhow::Error> {
    let json_string = std::fs::read_to_string(path.into())?;
    Ok(serde_json::from_str::<rustdoc_types::Crate>(&json_string)?)
}

pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

pub fn rustfmt(code: String) -> anyhow::Result<String> {
    let mut cmd = std::process::Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = cmd
        .stdin
        .take()
        .context("Can't access stdin during rustfmt")?;

    let mut stdout = cmd
        .stdout
        .take()
        .context("Can't access stdout during rustfmt")?;

    std::thread::spawn(move || {
        stdin.write_all(&code.into_bytes()).unwrap();
    });

    cmd.wait()?;

    let mut output_string = "".to_string();
    stdout.read_to_string(&mut output_string)?;

    Ok(output_string)
}

pub fn final_path(path: &str) -> anyhow::Result<String> {
    Ok(path
        .split("::")
        .last()
        .context("Problem parsing path")?
        .to_string())
}
