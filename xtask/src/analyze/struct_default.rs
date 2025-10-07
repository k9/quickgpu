use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use quote::{ToTokens, quote as q};
use rustdoc_types::{Attribute, Crate, Id, Item, ItemEnum, Span};
use syn::{Expr, ImplItem, ItemImpl, ItemStruct};

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
    Derived,
}

impl StructDefault {
    pub fn make_none(msg: impl Into<String>) -> StructDefault {
        StructDefault::None { msg: msg.into() }
    }
}

impl StructDefault {
    pub fn get_comment(&self) -> String {
        match self {
            StructDefault::None { msg } => format!("{}\n", msg),
            StructDefault::Derived => "Derived default".to_string(),
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

pub fn get_struct_default(id: Id, krate: &Crate, data: &Data) -> StructDefault {
    if let Some(item) = krate.index.get(&id) {
        struct_default_in_this_crate(id, krate, item)
    } else {
        struct_default_in_other_crate(id, krate, data)
    }
}

fn struct_default_in_this_crate(id: Id, krate: &Crate, item: &Item) -> StructDefault {
    if let Some(impls) = match &item.inner {
        ItemEnum::Struct(s) => Some(s.impls.clone()),
        ItemEnum::Enum(e) => Some(e.impls.clone()),
        _ => None,
    } {
        let impls: Vec<&Item> = impls.iter().filter_map(|i| krate.index.get(i)).collect();

        for impl_ in impls.iter() {
            if let ItemEnum::Impl(default) = &impl_.inner
                && default.trait_.as_ref().map(|trait_| &trait_.path)
                    == Some(&"Default".to_string())
            {
                if impl_.attrs.contains(&Attribute::AutomaticallyDerived) {
                    return StructDefault::Derived;
                } else {
                    return analyze_default(impl_.span.as_ref().unwrap(), &impls, krate);
                }
            }
        }
    }

    StructDefault::make_none(format!("Unhandled {:?} {:?}", item.name, id))
}

fn struct_default_in_other_crate(id: Id, krate: &Crate, data: &Data) -> StructDefault {
    #[allow(clippy::collapsible_else_if, clippy::collapsible_if)]
    if let Some(path) = krate.paths.get(&id)
        && let Some(other_krate) = path.path.first()
        && let Some(entry) = path.path.last()
    {
        if other_krate == "wgpu_types" {
            for DataItem { id, item, .. } in data.iter_wgt() {
                if item.name == Some(entry.to_string()) {
                    return get_struct_default(*id, &data.wgt, data);
                }
            }

            return StructDefault::make_none(format!("wgpu_types not found {}", entry));
        } else if !["core", "alloc"].contains(&other_krate.as_str()) {
            return StructDefault::make_none(format!("other {}", other_krate));
        }
    }

    StructDefault::make_none("Item not found")
}

pub(crate) fn analyze_default(span: &Span, impls: &[&Item], krate: &Crate) -> StructDefault {
    let Some((filename, source)) = source_from_span(span) else {
        return StructDefault::make_none(format!("Can't get source from {:?}", span.filename));
    };

    if let Ok(item_impl) = syn::parse_str::<ItemImpl>(source.as_str())
        && let Some(item) = item_impl.items.first()
        && let ImplItem::Fn(func) = item
        && let Some(stmt) = func.block.stmts.last()
        && let syn::Stmt::Expr(expr, _) = stmt
    {
        if let syn::Expr::Struct(struct_literal) = expr {
            let fields = struct_fields(struct_literal);

            return StructDefault::Fields {
                source: Some(Source {
                    source: source.clone(),
                    filename: filename.clone(),
                    line: span.begin.0 as u64,
                }),
                fields,
            };
        } else if let syn::Expr::Path(path) = expr
            && let Some(name) = path.path.segments.last()
        {
            for impl_ in impls {
                if let ItemEnum::Impl(impl_) = &impl_.inner {
                    for id in &impl_.items {
                        if let Some(inner) = krate.index.get(&id)
                            && inner.name == Some(name.to_token_stream().to_string())
                            && let ItemEnum::AssocConst { .. } = &inner.inner
                            && let Some(span) = &inner.span
                            && let Some((filename, source)) = source_from_span(span)
                            && let Ok(item) = syn::parse_str::<ImplItem>(source.as_str())
                            && let ImplItem::Const(item) = item
                            && let syn::Expr::Struct(struct_literal) = item.expr
                        {
                            let fields = struct_fields(&struct_literal);

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
    } else if let Ok(item) =
        syn::parse_str::<ItemStruct>(format!("{}\npub struct S {{}}", source).as_str())
    {
        println!("attr {:?}", item.attrs);
    }

    StructDefault::NoFields {
        source: Some(Source {
            source,
            filename,
            line: span.begin.0 as u64,
        }),
    }
}

fn struct_fields(struct_literal: &syn::ExprStruct) -> HashMap<String, Expr> {
    let mut fields = HashMap::new();

    for f in &struct_literal.fields {
        let member = &f.member;
        let expr = &f.expr;

        fields.insert(q!(#member).to_string(), expr.clone());
    }

    fields
}

fn source_from_span(span: &Span) -> Option<(String, String)> {
    let path = Path::new("/Users/work/src/wgpu/").join(&span.filename);

    let Ok(filename) = span.filename.clone().into_os_string().into_string() else {
        return None;
    };

    let Ok(input) = File::open(&path) else {
        return None;
    };

    let buffered = BufReader::new(input);
    let mut lines = buffered.lines().enumerate();

    let mut source = "".to_string();
    while let Some((num, Ok(contents))) = lines.next() {
        if num >= span.begin.0 - 1 && num < span.end.0 {
            source.push_str(&format!("{contents}\n"));
        }
    }

    Some((filename, source))
}
