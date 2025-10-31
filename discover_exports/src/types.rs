use petgraph::graph::NodeIndex;
use quote::ToTokens;
use syn::{
    GenericArgument, PathArguments, PathSegment, Type, punctuated::Punctuated, spanned::Spanned,
};

use crate::{Analysis, process::resolve_path, utils::path_segments};

pub fn resolve_type_paths(
    ty: Type,
    analysis: &Analysis,
    root_index: NodeIndex,
    item_index: NodeIndex,
) -> Type {
    let mut ty = ty.clone();

    match &mut ty {
        Type::Array(array) => {
            *array.elem = resolve_type_paths(*array.elem.clone(), analysis, root_index, item_index);
        }
        Type::BareFn(_) => todo!(),
        Type::Group(_) => todo!(),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(_) => todo!(),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Path(path) => {
            type_path(analysis, root_index, item_index, &mut path.path);
        }
        Type::Ptr(ptr) => {
            *ptr.elem = resolve_type_paths(*ptr.elem.clone(), analysis, root_index, item_index);
        }
        Type::Reference(reference) => {
            *reference.elem =
                resolve_type_paths(*reference.elem.clone(), analysis, root_index, item_index);
        }
        Type::Slice(slice) => {
            *slice.elem = resolve_type_paths(*slice.elem.clone(), analysis, root_index, item_index);
        }
        Type::TraitObject(object) => {
            object.bounds.iter_mut().for_each(|bound| {
                match bound {
                    syn::TypeParamBound::Trait(trait_bound) => {
                        type_path(analysis, root_index, item_index, &mut trait_bound.path);
                    }
                    _ => (),
                };
            });
        }
        Type::Tuple(tuple) => {
            tuple.elems.iter_mut().for_each(|elem| {
                *elem = resolve_type_paths(elem.clone(), analysis, root_index, item_index);
            });
        }
        Type::Verbatim(_) => todo!(),
        _ => (),
    };

    ty
}

fn type_path(
    analysis: &Analysis,
    root_index: NodeIndex,
    item_index: NodeIndex,
    path: &mut syn::Path,
) {
    let segments = path_segments(&path);

    if let Some(last_segment) = path.segments.last() {
        let mut arguments = last_segment.arguments.clone();

        if let PathArguments::AngleBracketed(arguments) = &mut arguments {
            arguments.args.iter_mut().for_each(|arg| {
                match arg {
                    GenericArgument::Type(arg_ty) => {
                        *arg = GenericArgument::Type(resolve_type_paths(
                            arg_ty.clone(),
                            analysis,
                            root_index,
                            item_index,
                        ));
                    }
                    _ => (),
                };
            });
        }

        match resolve_path(analysis, root_index, item_index, &segments) {
            Ok((_, resolved)) => {
                let mut segments = Punctuated::new();
                for ident in resolved.iter() {
                    segments.push(PathSegment {
                        ident: (*ident).clone(),
                        arguments: PathArguments::None,
                    });
                }

                path.segments = segments;
            }
            Err(_) => {
                let last_segment = last_segment.ident.to_string();
                let last_segment = last_segment.as_str();
                if ![
                    "f32", "f64", "i32", "u8", "u32", "u64", "usize", "bool", "String", "str",
                ]
                .contains(&last_segment)
                    && last_segment.len() > 1
                    && arguments.is_none()
                {
                    log::debug!(
                        "Couldn't resolve type {} {:?} {:?}",
                        path.into_token_stream(),
                        path.span().start(),
                        path.span().file()
                    );
                }
            }
        };

        if let Some(last) = path.segments.last_mut() {
            last.arguments = arguments;
        }
    }
}
