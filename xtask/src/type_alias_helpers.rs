use rustdoc_types::{
    GenericArg, GenericArgs, GenericParamDefKind, Generics, Item, ItemEnum, Path, Type, TypeAlias,
};

#[derive(Debug)]
pub struct GenericParamsList {
    pub lifetimes: Vec<String>,
    pub types: Vec<String>,
}

#[derive(Debug)]
pub struct GenericArgsList {
    pub lifetimes: Vec<String>,
    pub types: Vec<Type>,
}

#[derive(Debug)]
pub enum TypeAliasMap {
    Map {
        name: String,
        filename: String,
        alias_generics: Generics,
        alias_args: GenericArgsList,
        target_params: GenericParamsList,
    },
    None,
}

impl TypeAliasMap {
    pub fn map_generics(&self, base: &Generics) -> Generics {
        match self {
            TypeAliasMap::Map { alias_generics, .. } => alias_generics,
            TypeAliasMap::None => base,
        }
        .clone()
    }

    pub fn map_lifetime(&self, lifetime: &str) -> String {
        if let TypeAliasMap::Map {
            target_params,
            alias_args,
            ..
        } = self
            && let Some(index) = target_params.lifetimes.iter().position(|l| l == lifetime)
        {
            return alias_args.lifetimes[index].to_string();
        };

        lifetime.to_string()
    }

    pub fn map_generic(&self, generic: &str) -> Type {
        if let TypeAliasMap::Map {
            target_params,
            alias_args,
            ..
        } = self
        {
            if let Some(index) = target_params.types.iter().position(|l| l == generic) {
                return alias_args.types[index].clone();
            } else {
                println!("Couldn't find generic {:?}", generic);
            }
        };

        Type::Generic(generic.to_string())
    }

    pub(crate) fn map_name(&self, name: &str) -> String {
        if let TypeAliasMap::Map { name, .. } = self {
            return name.clone();
        };

        name.to_string()
    }

    pub(crate) fn map_filename(&self, filename: &str) -> String {
        if let TypeAliasMap::Map { filename, .. } = self {
            return filename.clone();
        };

        filename.to_string()
    }
}

pub fn get_type_alias_map(item: &Item, target: &Item, ta: &TypeAlias, path: &Path) -> TypeAliasMap {
    let name = item.name.clone().unwrap();

    let filename = item
        .span
        .clone()
        .unwrap()
        .filename
        .into_os_string()
        .into_string()
        .unwrap();

    let target_generics = match &target.inner {
        ItemEnum::Struct(s) => Some(s.generics.clone()),
        ItemEnum::Enum(e) => Some(e.generics.clone()),
        _ => {
            println!("unhandled target generics {:?}", target);
            None
        }
    };

    if let Some(target_generics) = target_generics
        && let Some(args) = path.args.clone()
        && let GenericArgs::AngleBracketed { args, .. } = (*args).clone()
    {
        // struct Abc<'a, B> // target generics
        //    { ... }
        //
        // type Xyz<'a> // type alias generics
        //     = Abc<'a, Option<u64> // type alias path args

        return TypeAliasMap::Map {
            name,
            filename,
            alias_generics: ta.generics.clone(),
            alias_args: get_args_list(args),
            target_params: get_params_list(target_generics),
        };
    };

    TypeAliasMap::None
}

pub fn get_params_list(target_generics: Generics) -> GenericParamsList {
    let mut lifetimes = vec![];
    let mut types = vec![];
    for g in target_generics.params {
        match g.kind {
            GenericParamDefKind::Lifetime { .. } => lifetimes.push(g.name),
            GenericParamDefKind::Type { .. } => types.push(g.name),
            _ => (),
        };
    }

    GenericParamsList { lifetimes, types }
}

pub fn get_args_list(target_generics: Vec<GenericArg>) -> GenericArgsList {
    let mut lifetimes = vec![];
    let mut types = vec![];
    for g in target_generics {
        match g {
            GenericArg::Lifetime(g) => lifetimes.push(g),
            GenericArg::Type(g) => types.push(g),
            _ => (),
        };
    }

    GenericArgsList { lifetimes, types }
}
