use rustdoc_types::{Crate, Id, Item, ItemEnum};
use syn::Expr;

use crate::data::{Data, DataItem};

#[derive(Debug)]
#[allow(dead_code)]
pub enum FieldDefault {
    None { msg: String },
    Option,
    Default,
    Value { value: Expr },
    Generic,
}

impl FieldDefault {
    pub fn make_none(msg: impl Into<String>) -> Self {
        Self::None { msg: msg.into() }
    }
}

pub fn get_field_default(id: Id, krate: &Crate, data: &Data) -> FieldDefault {
    if let Some(item) = krate.index.get(&id) {
        field_default_in_this_crate(id, krate, item)
    } else {
        field_default_in_other_crate(id, krate, data)
    }
}

fn field_default_in_this_crate(id: Id, krate: &Crate, item: &Item) -> FieldDefault {
    if let Some(name) = &item.name {
        if name == "Label" {
            return FieldDefault::Default;
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
                return FieldDefault::Default;
            }
        }
    }

    FieldDefault::make_none(format!("Unhandled {:?} {:?}", item.name, id))
}

fn field_default_in_other_crate(id: Id, krate: &Crate, data: &Data) -> FieldDefault {
    #[allow(clippy::collapsible_else_if, clippy::collapsible_if)]
    if let Some(path) = krate.paths.get(&id)
        && let Some(other_krate) = path.path.first()
        && let Some(entry) = path.path.last()
    {
        if other_krate == "wgpu_types" {
            for DataItem { id, item, .. } in data.iter_wgt() {
                if item.name == Some(entry.to_string()) {
                    return get_field_default(*id, &data.wgt, data);
                }
            }

            return FieldDefault::make_none(format!("wgpu_types not found {}", entry));
        } else if !["core", "alloc"].contains(&other_krate.as_str()) {
            return FieldDefault::make_none(format!("other {}", other_krate));
        }
    }

    FieldDefault::make_none("Item not found")
}
