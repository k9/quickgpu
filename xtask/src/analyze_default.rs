use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use quote::quote as q;
use rustdoc_types::{Crate, Id, Item, ItemEnum, Span};
use syn::{Expr, ImplItem, ItemImpl};

use crate::data::{Data, DataItem};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Source {
    pub source: String,
    pub filename: String,
    pub line: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum StructDefault {
    None {
        msg: String,
    },
    Fields {
        source: Option<Source>,
        fields: HashMap<String, Expr>,
    },
    NoFields {
        source: Option<Source>,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FieldDefault {
    None { msg: String },
    Default { source: Option<Source> },
    Value { source: Option<Source>, value: Expr },
    Generic,
}

pub fn no_default(msg: impl Into<String>) -> StructDefault {
    StructDefault::None { msg: msg.into() }
}

impl StructDefault {
    pub fn get_comment(&self) -> String {
        match self {
            StructDefault::None { msg } => format!("{}\n", msg),
            StructDefault::NoFields { source, .. } | StructDefault::Fields { source, .. } => {
                if let Some(source) = source {
                    format!(
                        "Default from: {}:{}\n{}",
                        source.filename, source.line, source.source
                    )
                } else {
                    "".to_string()
                }
            }
        }
    }
}

pub fn get_default(id: Id, krate: &Crate, data: &Data) -> StructDefault {
    if let Some(item) = krate.index.get(&id) {
        default_in_this_crate(id, krate, item)
    } else {
        default_in_other_crate(id, krate, data)
    }
}

fn default_in_this_crate(id: Id, krate: &Crate, item: &Item) -> StructDefault {
    if let Some(name) = &item.name {
        if name == "Label" {
            return StructDefault::Default { source: None };
        }
    };

    if let Some(impls) = match &item.inner {
        ItemEnum::Struct(s) => Some(s.impls.clone()),
        ItemEnum::Enum(e) => Some(e.impls.clone()),
        _ => None,
    } {
        for impl_ in impls {
            if let Some(item) = krate.index.get(&impl_)
                && let ItemEnum::Impl(impl_) = &item.inner
                && impl_.trait_.as_ref().map(|trait_| &trait_.path) == Some(&"Default".to_string())
            {
                return analyze_default(item.span.as_ref().unwrap());
            }
        }
    }

    no_default(format!("Unhandled {:?} {:?}", item.name, id))
}

fn default_in_other_crate(id: Id, krate: &Crate, data: &Data) -> StructDefault {
    #[allow(clippy::collapsible_else_if, clippy::collapsible_if)]
    if let Some(path) = krate.paths.get(&id)
        && let Some(other_krate) = path.path.first()
        && let Some(entry) = path.path.last()
    {
        if other_krate == "wgpu_types" {
            for DataItem { id, item, .. } in data.iter_wgt() {
                if item.name == Some(entry.to_string()) {
                    return get_default(*id, &data.wgt, data);
                }
            }

            return no_default(format!("wgpu_types not found {}", entry));
        } else if !["core", "alloc"].contains(&other_krate.as_str()) {
            return no_default(format!("other {}", other_krate));
        }
    }

    no_default("Item not found")
}

pub(crate) fn analyze_default(span: &Span) -> StructDefault {
    let path = Path::new("/Users/work/src/wgpu/").join(&span.filename);
    let filename = span.filename.to_str().unwrap().to_string();

    if let Ok(input) = File::open(&path) {
        let buffered = BufReader::new(input);
        let mut lines = buffered.lines().enumerate();

        let mut source = "".to_string();
        while let Some((num, Ok(contents))) = lines.next() {
            if num >= span.begin.0 - 1 && num < span.end.0 {
                source.push_str(&format!("{contents}\n"));
            }
        }

        if let Ok(item_impl) = syn::parse_str::<ItemImpl>(source.as_str()) {
            if let Some(item) = item_impl.items.first() {
                if let ImplItem::Fn(func) = item {
                    if let Some(stmt) = func.block.stmts.last() {
                        if let syn::Stmt::Expr(expr, _) = stmt {
                            if let syn::Expr::Struct(struct_literal) = expr {
                                let mut fields = HashMap::new();

                                for f in &struct_literal.fields {
                                    let member = &f.member;
                                    let expr = &f.expr;

                                    fields.insert(q!(#member).to_string(), expr.clone());
                                }

                                return StructDefault::Fields {
                                    source: Some(Source {
                                        source: source.clone(),
                                        filename: filename.clone(),
                                        line: span.begin.0 as u64,
                                    }),
                                    fields,
                                };
                            }
                        }
                    }
                }
            }
        }

        StructDefault::Default {
            source: Some(Source {
                source,
                filename,
                line: span.begin.0 as u64,
            }),
        }
    } else {
        no_default(format!("Can't find file {filename}"))
    }
}
