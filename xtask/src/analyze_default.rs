use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use rustdoc_types::{Crate, Id, Item, ItemEnum, Span};

use crate::data::{Data, DataItem};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Source {
    pub source: String,
    pub filename: String,
    pub line: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum DefaultValue {
    None {
        msg: String,
    },
    Default {
        source: Option<Source>,
    },
    Value {
        source: Option<Source>,
        value: String,
    },
    Generic,
}

fn no_default(msg: String) -> DefaultValue {
    DefaultValue::None { msg }
}

impl DefaultValue {
    pub fn get_comment(&self) -> String {
        match self {
            DefaultValue::None { msg } => format!("{}\n", msg),
            DefaultValue::Default { source } | DefaultValue::Value { source, value: _ } => {
                if let Some(source) = source {
                    format!(
                        "Default from: {}:{}\n{}",
                        source.filename, source.line, source.source
                    )
                } else {
                    "".to_string()
                }
            }
            DefaultValue::Generic => "".to_string(),
        }
    }
}

pub fn get_default(id: Id, krate: &Crate, data: &Data) -> DefaultValue {
    if let Some(item) = krate.index.get(&id) {
        default_in_this_crate(id, krate, item)
    } else {
        default_in_other_crate(id, krate, data)
    }
}

fn default_in_this_crate(id: Id, krate: &Crate, item: &Item) -> DefaultValue {
    if let Some(name) = &item.name {
        if name == "LoadOp" {
            return no_default("LoadOp needs default bound".to_string());
        }

        if name == "Label" {
            return DefaultValue::Default { source: None };
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

fn default_in_other_crate(id: Id, krate: &Crate, data: &Data) -> DefaultValue {
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

    no_default("Item not found".to_string())
}

pub(crate) fn analyze_default(span: &Span) -> DefaultValue {
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

        DefaultValue::Default {
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
