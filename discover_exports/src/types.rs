use petgraph::graph::NodeIndex;
use quote::ToTokens;
use syn::{
    GenericArgument, Path, PathArguments, PathSegment, Type, punctuated::Punctuated,
    spanned::Spanned,
};

use crate::{
    analysis::Ctx,
    crate_graph::{PathType, full_path},
    process::resolve_path,
    utils::path_segments,
};

pub fn resolve_type_paths(ty: Type, ctx: &Ctx, item_index: NodeIndex) -> Type {
    let mut ty = ty.clone();

    match &mut ty {
        Type::Array(array) => {
            *array.elem = resolve_type_paths(*array.elem.clone(), ctx, item_index);
        }
        Type::BareFn(_) => {}
        Type::Group(_) => {}
        Type::ImplTrait(_) => {}
        Type::Infer(_) => {}
        Type::Macro(_) => {}
        Type::Never(_) => {}
        Type::Paren(_) => {}
        Type::Path(path) => {
            type_path(ctx, item_index, &mut path.path);
        }
        Type::Ptr(ptr) => {
            *ptr.elem = resolve_type_paths(*ptr.elem.clone(), ctx, item_index);
        }
        Type::Reference(reference) => {
            *reference.elem = resolve_type_paths(*reference.elem.clone(), ctx, item_index);
        }
        Type::Slice(slice) => {
            *slice.elem = resolve_type_paths(*slice.elem.clone(), ctx, item_index);
        }
        Type::TraitObject(object) => {
            object.bounds.iter_mut().for_each(|bound| {
                match bound {
                    syn::TypeParamBound::Trait(trait_bound) => {
                        type_path(ctx, item_index, &mut trait_bound.path);
                    }
                    _ => (),
                };
            });
        }
        Type::Tuple(tuple) => {
            tuple.elems.iter_mut().for_each(|elem| {
                *elem = resolve_type_paths(elem.clone(), ctx, item_index);
            });
        }
        Type::Verbatim(_) => todo!(),
        _ => (),
    };

    ty
}

pub fn type_path(ctx: &Ctx, item_index: NodeIndex, path: &mut Path) {
    let segments = path_segments(&path);

    if let Some(last_segment) = path.segments.last() {
        if let Ok(node) = resolve_path(ctx, item_index, &segments)
            && let Ok(full) = full_path(ctx, node, PathType::PublicOnly)
        {
            let mut arguments = last_segment.arguments.clone();

            if let PathArguments::AngleBracketed(arguments) = &mut arguments {
                arguments.args.iter_mut().for_each(|arg| {
                    match arg {
                        GenericArgument::Type(arg_ty) => {
                            *arg = GenericArgument::Type(resolve_type_paths(
                                arg_ty.clone(),
                                ctx,
                                item_index,
                            ));
                        }
                        _ => (),
                    };
                });
            }

            let mut segments = Punctuated::new();
            for ident in full.iter() {
                segments.push(PathSegment {
                    ident: (*ident).clone(),
                    arguments: PathArguments::None,
                });
            }

            path.segments = segments;

            if let Some(last) = path.segments.last_mut() {
                last.arguments = arguments;
            }
        } else {
            log::debug!("Couldn't resolve {:?}", path);
        };
    }
}
