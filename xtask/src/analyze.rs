use rustdoc_types::{Crate, Generics, Id, Item, ItemEnum, Span, StructKind, Type, Visibility};

use crate::{
    analyze_default::{DefaultValue, get_default},
    data::Data,
    type_alias_helpers::TypeAliasMap,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct FieldParts {
    pub name: String,
    pub ty: Type,
    pub default_value: DefaultValue,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct StructParts {
    pub name: String,
    pub generics: Generics,
    pub default_value: DefaultValue,
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

        if !item.span.as_ref().is_some_and(|span| {
            span.filename.starts_with("wgpu/src/api/")
                || span.filename.starts_with("wgpu/src/util/")
                || span.filename.starts_with("wgpu-types/src/")
        }) {
            return StructAnalysis::WrongPath(item.span.clone());
        }

        let mut fields = vec![];

        for field in source_fields {
            match analyze_field(field, krate, data) {
                FieldAnalysis::Parts(parts) => fields.push(parts),
                x => return StructAnalysis::FieldIssue(x),
            };
        }

        let default_value = get_default(item.id, krate, data);

        Self::Parts(StructParts {
            name,
            generics: s.generics.clone(),
            fields,
            default_value,
            type_alias_map,
        })
    }
}

fn analyze_field(field: Id, krate: &Crate, data: &Data) -> FieldAnalysis {
    let Some(field) = krate.index.get(&field) else {
        return FieldAnalysis::NotFound(field);
    };

    let Some(name) = field.name.as_ref().map(|x| x.to_string()) else {
        return FieldAnalysis::NoName(field.id);
    };

    let ItemEnum::StructField(struct_field) = &field.inner else {
        return FieldAnalysis::NotPath(format!("{:?}", field.inner));
    };

    match struct_field {
        Type::ResolvedPath(path) => FieldAnalysis::Parts(FieldParts {
            name,
            default_value: get_default(path.id, krate, data),
            ty: struct_field.clone(),
        }),
        Type::Generic(_) => FieldAnalysis::Parts(FieldParts {
            name,
            default_value: DefaultValue::Generic,
            ty: struct_field.clone(),
        }),
        _ => FieldAnalysis::Parts(FieldParts {
            name,
            default_value: DefaultValue::None {
                msg: "Field type not ResolvedPath".to_string(),
            },
            ty: struct_field.clone(),
        }),
    }
}

pub fn report(_v: &Item, analysis: &StructAnalysis) {
    match analysis {
        StructAnalysis::Parts(p) => {
            //println!("{}", p.name);
            //println!("    default: {:?}", p.default_value);
            //println!("    fields:");
            for _f in &p.fields {
                //    println!("        {:?} {:?}", f.name, f.default_value);
            }
        }
        StructAnalysis::NotStruct => {}
        StructAnalysis::HasNotVisibleFields => {}
        //StructAnalysis::FieldNotPath(_) => {}
        StructAnalysis::NotPlain => {}
        StructAnalysis::WrongPath(_) => {}
        _ => (),
    }
}
