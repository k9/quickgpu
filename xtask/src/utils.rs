use html5ever::tree_builder::TreeSink;
use proc_macro2::{Span, TokenStream};
use scraper::{ElementRef, Html, HtmlTreeSink};
use std::fs;
use std::path::{Path, PathBuf};
use syn::Ident;

pub(crate) fn doc_path(file: &str) -> anyhow::Result<PathBuf> {
    Ok(Path::new(env!("CARGO_WORKSPACE_DIR"))
        .join("wgpu/target/doc/wgpu/")
        .join(file)
        .canonicalize()?)
}

pub(crate) fn output(code: &[(String, TokenStream)], format: bool) {
    let code = code
        .iter()
        .map(|(idl, c)| {
            let c = if format {
                let syntax_tree = syn::parse_file(&c.to_string()).unwrap();
                prettyplease::unparse(&syntax_tree)
            } else {
                c.to_string()
            };

            format!("{}{}", idl, c)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let dest_path = Path::new(env!("CARGO_WORKSPACE_DIR")).join("quickgpu/src/builders.rs");
    fs::write(&dest_path, &code).unwrap();
}

pub(crate) fn prepare(html: Html) -> Html {
    let sink = HtmlTreeSink::new(html);
    let selector = scraper::Selector::parse(".hideme").unwrap();

    let hideme = &sink.0.borrow().select(&selector).next().map(|el| el.id());
    if let Some(hideme) = hideme {
        sink.remove_from_parent(hideme);
    };

    sink.finish()
}

pub(crate) fn text_content(struct_info: ElementRef<'_>) -> String {
    struct_info.text().collect::<Vec<&str>>().join("")
}

pub(crate) fn id(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}
