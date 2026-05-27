use anyhow::{Context, bail};
use petgraph::graph::NodeIndex;
use proc_macro2::Span;
use quote::quote as q;
use syn::{
    GenericParam, Ident, ImplItem, ImplItemConst, ItemEnum, ItemImpl, ItemStruct, Lifetime, Path,
    PathArguments, PathSegment, Token, Type,
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
};

use crate::{
    AResult, EntryIndex,
    analysis::{Ctx, EdgeSource},
    analysis_entry::AnalysisEntry,
    crate_graph::{find_neighbor, get_parent, get_path_context},
    types::type_path,
    utils::IsPublic,
};

pub struct TyResolve<'a> {
    pub ctx: &'a Ctx<'a>,
    pub item_index: EntryIndex,
}

impl<'a> VisitMut for TyResolve<'a> {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        type_path(self.ctx, self.item_index, path);

        visit_mut::visit_path_mut(self, path);
    }
}

pub fn resolve_path(ctx: &Ctx, item_index: NodeIndex, relative_path: &Path) -> AResult<NodeIndex> {
    resolve_path_recurse(ctx, item_index, relative_path, 0)
}

pub fn resolve_path_recurse(
    ctx: &Ctx,
    current: NodeIndex,
    relative_path: &Path,
    path_segment_index: usize,
) -> AResult<NodeIndex> {
    let Some(path_segment) = relative_path.segments.get(path_segment_index) else {
        return Ok(current);
    };

    let next_index = resolve_next_segment(ctx, current, path_segment)?;
    resolve_path_recurse(ctx, next_index, relative_path, path_segment_index + 1)
}

pub fn resolve_next_segment(
    ctx: &Ctx,
    current: NodeIndex,
    path_segment: &PathSegment,
) -> AResult<NodeIndex> {
    let ident = &path_segment.ident;

    if *ident == "self" {
        Ok(current)
    } else if *ident == "Self" {
        get_parent(
            ctx,
            current,
            &[EdgeSource::Normal, EdgeSource::LinkToImplItem],
        )
        .map(|p| p.1)
    } else if *ident == "crate" {
        Ok(ctx.crate_root)
    } else if *ident == "super" {
        if let Some(path_context) = get_path_context(ctx, current) {
            get_parent(
                ctx,
                path_context,
                &[EdgeSource::Normal, EdgeSource::ModToImplItem],
            )
            .map(|p| p.1)
        } else {
            bail!("Couldn't resolve super from {:?}", path_segment);
        }
    } else if let Some(path_context) = get_path_context(ctx, current)
        && let Some(child_index) = find_neighbor(ctx, path_context, ident)
    {
        // Neighbors in the source tree
        Ok(child_index)
    } else if let Some(child_index) = find_neighbor(ctx, current, ident) {
        // Associated items
        Ok(child_index)
    } else if let Ok(node) = resolve_prelude(ctx, ident) {
        Ok(node)
    } else {
        let msg = format!(
            "Couldn't resolve segment {:?} from {:?}",
            path_segment, current
        );

        log::debug!("{}", msg);
        bail!(msg);
    }
}

fn resolve_prelude(ctx: &Ctx, path_segment: &Ident) -> AResult<NodeIndex> {
    let root = ctx.crate_root;
    if let AnalysisEntry::Mod(module) = ctx.krate()?
        && let Some(root_of) = &module.root_of_crate
        && root_of == path_segment
    {
        Ok(root)
    } else if let Some(neighbor) = find_neighbor(ctx, root, path_segment)
        && let AnalysisEntry::Mod(entry) = ctx.entry(neighbor)?
        && entry.root_of_crate.is_some()
    {
        Ok(neighbor)
    } else {
        bail!("Couldn't find extern crate {:?}", path_segment);
    }
}

pub fn resolve_enum(ctx: &Ctx, node_index: NodeIndex) -> AResult<ItemEnum> {
    let AnalysisEntry::Enum(enum_item) = ctx.entry(node_index)? else {
        bail!("Can't get enum");
    };

    let enum_item = enum_item.item.clone();

    Ok(enum_item)
}

pub fn resolve_struct(ctx: &Ctx, node_index: NodeIndex) -> AResult<ItemStruct> {
    let AnalysisEntry::Struct(struct_item) = ctx.entry(node_index)? else {
        bail!("Can't get struct");
    };

    let mut struct_item = struct_item.item.clone();
    let mut resolver = TyResolve {
        ctx,
        item_index: node_index,
    };

    resolver.visit_item_struct_mut(&mut struct_item);

    Ok(struct_item)
}

pub fn resolve_type_alias(ctx: &Ctx, node_index: NodeIndex) -> AResult<(ItemStruct, NodeIndex)> {
    let AnalysisEntry::Type(item) = ctx.entry(node_index)? else {
        bail!("Can't get type alias");
    };

    let mut item = item.item.clone();
    let mut resolver = TyResolve {
        ctx,
        item_index: node_index,
    };

    resolver.visit_item_type_mut(&mut item);

    let syn::Type::Path(type_path) = *item.ty else {
        bail!("Can't find struct for type alias.");
    };

    let struct_index = resolve_path(ctx, node_index, &type_path.path)?;
    let ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        ..
    } = resolve_struct(ctx, struct_index)?;

    let mut resolver = AliasGenericsResolver::new(
        ctx,
        &generics,
        &type_path.path.segments.last().unwrap().arguments,
    )?;

    let mut struct_item = ItemStruct {
        attrs,
        vis,
        struct_token: Token![struct](Span::call_site()),
        ident,
        generics: item.generics,
        fields,
        semi_token: None,
    };

    resolver.visit_item_struct_mut(&mut struct_item);

    Ok((struct_item, struct_index))
}

pub struct AliasGenericsResolver<'a> {
    pub ctx: &'a Ctx<'a>,
    pub from_types: Vec<Ident>,
    pub from_lifetimes: Vec<Ident>,
    pub to_types: Vec<Type>,
    pub to_lifetimes: Vec<Lifetime>,
}

impl<'a> AliasGenericsResolver<'a> {
    pub fn new(ctx: &'a Ctx, from: &syn::Generics, to: &syn::PathArguments) -> AResult<Self> {
        let mut from_types = vec![];
        let mut from_lifetimes = vec![];

        for param in from.params.iter() {
            match param {
                GenericParam::Lifetime(param) => from_lifetimes.push(param.lifetime.ident.clone()),
                GenericParam::Type(param) => from_types.push(param.ident.clone()),
                _ => (),
            };
        }

        let mut to_types = vec![];
        let mut to_lifetimes = vec![];

        let PathArguments::AngleBracketed(args) = to else {
            bail!("");
        };

        for arg in args.args.iter() {
            match arg {
                syn::GenericArgument::Lifetime(arg) => to_lifetimes.push(arg.clone()),
                syn::GenericArgument::Type(arg) => to_types.push(arg.clone()),
                _ => (),
            };
        }

        Ok(Self {
            ctx,
            from_types,
            from_lifetimes,
            to_types,
            to_lifetimes,
        })
    }
}

impl<'a> std::fmt::Debug for AliasGenericsResolver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AliasGenericsResolver")
            .field("from_types", &self.from_types)
            .field("from_lifetimes", &self.from_lifetimes)
            .field("to_types", &self.to_types)
            .field("to_lifetimes", &self.to_lifetimes)
            .finish()
    }
}

impl<'a> VisitMut for AliasGenericsResolver<'a> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let Some(position) = self
            .from_types
            .iter()
            .position(|from_ty| *from_ty == q!(#ty).to_string())
        {
            *ty = self.to_types[position].clone();
        }

        visit_mut::visit_type_mut(self, ty);
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if let Some(position) = self
            .from_lifetimes
            .iter()
            .position(|from_lifetime| *from_lifetime == lifetime.ident)
        {
            *lifetime = self.to_lifetimes[position].clone();
        }

        visit_mut::visit_lifetime_mut(self, lifetime);
    }
}

pub fn resolve_impls(ctx: &Ctx, node_index: NodeIndex) -> AResult<Vec<ItemImpl>> {
    let mut items = vec![];

    let mut neighbors = ctx.graph().neighbors(node_index).detach();
    while let Some((edge, neighbor)) = neighbors.next(ctx.graph()) {
        if let Some(edge) = ctx.graph().edge_weight(edge)
            && edge.source == EdgeSource::LinkToImpl
            && let AnalysisEntry::Impl(impl_entry) = ctx.entry(neighbor)?
        {
            let mut item = impl_entry.item.clone();
            item.items.clear();

            let mut neighbors = ctx.graph().neighbors(neighbor).detach();
            while let Some((_, inner_neighbor)) = neighbors.next(ctx.graph()) {
                if let AnalysisEntry::ImplConst(impl_const) = ctx.entry(inner_neighbor)? {
                    let mut inner = impl_const.item.clone();
                    TyResolve {
                        ctx,
                        item_index: inner_neighbor,
                    }
                    .visit_impl_item_const_mut(&mut inner);

                    item.items.push(ImplItem::Const(inner));
                } else if let AnalysisEntry::ImplFn(impl_fn) = ctx.entry(inner_neighbor)? {
                    let mut inner = impl_fn.item.clone();
                    TyResolve {
                        ctx,
                        item_index: inner_neighbor,
                    }
                    .visit_impl_item_fn_mut(&mut inner);

                    item.items.push(ImplItem::Fn(inner));
                }
            }

            items.push(item);
        }
    }

    Ok(items)
}

pub fn resolve_assoc_consts(
    ctx: &Ctx,
    node_index: NodeIndex,
) -> AResult<Vec<(Path, ImplItemConst)>> {
    let mut neighbors = ctx.graph().neighbors(node_index).detach();
    let mut consts = vec![];

    let mut resolver = TyResolve {
        ctx,
        item_index: node_index,
    };

    while let Some((_, neighbor)) = neighbors.next(ctx.graph()) {
        if let AnalysisEntry::ImplConst(c) = ctx.entry(neighbor)? {
            let mut c = c.clone();
            resolver.visit_impl_item_const_mut(&mut c.item);

            let path = get_public_path(ctx, neighbor)?;
            consts.push((path, c.item));
        }
    }

    Ok(consts)
}

pub fn get_public_path(ctx: &Ctx, neighbor: NodeIndex) -> AResult<Path> {
    let path = ctx
        .public_paths
        .get(&neighbor)
        .context("Couldn't get path")?;

    Ok(path.clone())
}

pub fn get_top_level_path(ctx: &Ctx, neighbor: NodeIndex) -> AResult<Path> {
    let path = ctx
        .top_level_paths
        .get(&neighbor)
        .context("Couldn't get path")?;

    Ok(path.clone())
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PathType {
    PublicOnly,
    TopLevelPublicOnly,
}

pub fn calculate_paths(ctx: &mut Ctx) -> AResult<()> {
    let mut current = Path {
        leading_colon: None,
        segments: Punctuated::new(),
    };

    let AnalysisEntry::Mod(module) = ctx.krate()? else {
        bail!("Couldn't get crate name");
    };

    let Some(root_of_crate) = module.root_of_crate.clone() else {
        bail!("Couldn't get crate name");
    };

    current.segments.push(PathSegment {
        ident: root_of_crate,
        arguments: PathArguments::None,
    });

    ctx.public_paths.clear();
    ctx.public_paths.insert(ctx.crate_root, current.clone());
    calculate_paths_recurse(ctx, ctx.crate_root, current.clone(), PathType::PublicOnly);

    ctx.top_level_paths.clear();
    ctx.top_level_paths.insert(ctx.crate_root, current.clone());
    calculate_paths_recurse(
        ctx,
        ctx.crate_root,
        current.clone(),
        PathType::TopLevelPublicOnly,
    );

    Ok(())
}

pub fn calculate_paths_recurse(
    ctx: &mut Ctx,
    node_index: NodeIndex,
    current: Path,
    path_type: PathType,
) {
    let mut neighbors = ctx.graph().neighbors(node_index).detach();
    while let Some((edge_index, neighbor)) = neighbors.next(ctx.graph()) {
        if let Some(edge) = ctx.graph().edge_weight(edge_index)
            && let Some(name) = edge.name.as_ref()
        {
            let entry = ctx.entry(neighbor).unwrap();
            let is_impl = matches!(entry, AnalysisEntry::Impl(_));

            let is_crate = path_type == PathType::TopLevelPublicOnly
                && if let AnalysisEntry::Mod(module) = entry
                    && module.root_of_crate.is_some()
                {
                    true
                } else {
                    false
                };

            let no_visibility = !entry.vis().is_public();

            if !no_visibility && !is_impl && !is_crate {
                let mut next = current.clone();
                next.segments.push(PathSegment {
                    ident: name.clone(),
                    arguments: PathArguments::None,
                });

                let existing_path = if path_type == PathType::PublicOnly {
                    ctx.public_paths.get(&neighbor)
                } else {
                    ctx.top_level_paths.get(&neighbor)
                };

                let shortest_path = match existing_path {
                    Some(existing_path) => {
                        // Prefer paths with fewer segments, or if equal,
                        // shorter string representation
                        if existing_path.segments.len() < next.segments.len()
                            || (existing_path.segments.len() == next.segments.len()
                                && q!(#existing_path).to_string().len()
                                    < q!(#next).to_string().len())
                        {
                            existing_path.clone()
                        } else {
                            next
                        }
                    }
                    None => next,
                };

                if path_type == PathType::PublicOnly {
                    ctx.public_paths.insert(neighbor, shortest_path.clone());
                } else {
                    ctx.top_level_paths.insert(neighbor, shortest_path.clone());
                }

                calculate_paths_recurse(ctx, neighbor, shortest_path.clone(), path_type);
            }
        }
    }
}
