use quote::quote as q;
use regex::Regex;
use rustdoc_types::{Crate, Generics, Id, Item, ItemEnum, Span, StructKind, Type, Visibility};

use crate::{
    analyze::{
        field_default::{FieldDefault, get_field_default},
        struct_default::{StructDefault, get_struct_default},
    },
    data::Data,
    type_alias_helpers::TypeAliasMap,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct FieldParts {
    pub name: String,
    pub ty: Type,
    pub default_value: FieldDefault,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StructParts {
    pub name: String,
    pub generics: Generics,
    pub default_value: StructDefault,
    pub fields: Vec<FieldParts>,
    pub type_alias_map: TypeAliasMap,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum StructAnalysis {
    Parts(StructParts),
    NotStruct,
    NotVisible,
    NoName,
    NotPlain,
    HasNotVisibleFields,
    WrongPath(Option<Span>),
    FieldIssue(FieldAnalysis),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FieldAnalysis {
    Parts(FieldParts),
    NotFound(Id),
    NoName(Id),
    NotPath(String),
}

impl StructAnalysis {
    pub fn analyze(
        item: &Item,
        krate: &Crate,
        data: &Data,
        type_alias_map: TypeAliasMap,
    ) -> StructAnalysis {
        let ItemEnum::Struct(s) = &item.inner else {
            return StructAnalysis::NotStruct;
        };

        if item.visibility != Visibility::Public {
            return StructAnalysis::NotVisible;
        };

        let Some(name) = item.name.as_ref().map(|x| x.to_string()) else {
            return StructAnalysis::NoName;
        };

        let StructKind::Plain {
            fields: source_fields,
            has_stripped_fields,
        } = s.kind.clone()
        else {
            return StructAnalysis::NotPlain;
        };

        if has_stripped_fields {
            return StructAnalysis::HasNotVisibleFields;
        };

        let api = Regex::new(r"wgpu-[0-9\.]*/src/api/").unwrap();
        let util = Regex::new(r"wgpu-[0-9\.]*/src/util/").unwrap();
        let types = Regex::new(r"wgpu-types-[0-9\.]*/src/").unwrap();
        if !item.span.as_ref().is_some_and(|span| {
            let filename = span.filename.clone().into_os_string();
            let Some(filename) = filename.to_str() else {
                return false;
            };

            api.is_match(filename) || util.is_match(filename) || types.is_match(filename)
        }) {
            return StructAnalysis::WrongPath(item.span.clone());
        }

        let mut fields = vec![];

        let default_value = get_struct_default(item.id, krate, data);

        for field in source_fields {
            match analyze_field(field, krate, data, &default_value) {
                FieldAnalysis::Parts(parts) => fields.push(parts),
                x => return StructAnalysis::FieldIssue(x),
            };
        }

        Self::Parts(StructParts {
            name,
            generics: s.generics.clone(),
            fields,
            default_value,
            type_alias_map,
        })
    }
}

fn analyze_field(
    field: Id,
    krate: &Crate,
    data: &Data,
    struct_default: &StructDefault,
) -> FieldAnalysis {
    let Some(field) = krate.index.get(&field) else {
        return FieldAnalysis::NotFound(field);
    };

    let Some(name) = field.name.as_ref().map(|x| x.to_string()) else {
        return FieldAnalysis::NoName(field.id);
    };

    let ItemEnum::StructField(struct_field) = &field.inner else {
        return FieldAnalysis::NotPath(format!("{:?}", field.inner));
    };

    if let StructDefault::Fields { fields, .. } = struct_default {
        if let Some(value) = fields.get(&name) {
            let default_value = if let Type::ResolvedPath(path) = struct_field
                && path.path == "Option"
            {
                FieldDefault::make_none("Option doesn't need default")
            } else if q!(#value).to_string() == q!(Default::default()).to_string() {
                FieldDefault::Default
            } else {
                FieldDefault::Value {
                    value: value.clone(),
                }
            };

            return FieldAnalysis::Parts(FieldParts {
                name,
                ty: struct_field.clone(),
                default_value,
            });
        }
    };

    if let StructDefault::Derived = struct_default {
        let default_value = if let Type::ResolvedPath(path) = struct_field
            && path.path == "Option"
        {
            FieldDefault::make_none("Option doesn't need default")
        } else {
            FieldDefault::Default
        };

        return FieldAnalysis::Parts(FieldParts {
            name,
            ty: struct_field.clone(),
            default_value,
        });
    };

    match struct_field {
        Type::ResolvedPath(path) => FieldAnalysis::Parts(FieldParts {
            name,
            default_value: get_field_default(path.id, krate, data),
            ty: struct_field.clone(),
        }),
        Type::Generic(_) => FieldAnalysis::Parts(FieldParts {
            name,
            default_value: FieldDefault::Generic,
            ty: struct_field.clone(),
        }),
        _ => FieldAnalysis::Parts(FieldParts {
            name,
            default_value: FieldDefault::make_none("Field type not ResolvedPath"),
            ty: struct_field.clone(),
        }),
    }
}

pub fn report(_v: &Item, analysis: &StructAnalysis) {
    // For debugging, can match some variants here and print out analysis
    match analysis {
        _ => (),
    }
}
