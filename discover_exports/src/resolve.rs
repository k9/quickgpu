use anyhow::{Context, bail};
use petgraph::{algo::astar, graph::NodeIndex, visit::EdgeRef};
use proc_macro2::Span;
use quote::quote as q;
use syn::{
    GenericParam, Ident, ImplItemConst, ItemImpl, ItemStruct, Lifetime, Path, PathArguments,
    PathSegment, Token, Type,
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
};

use crate::{
    AResult, EntryIndex,
    analysis::Ctx,
    analysis_entry::AnalysisEntry,
    crate_graph::{find_neighbor, get_super},
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
    resolve_path_inner(ctx, item_index, relative_path)
}

pub fn resolve_path_inner(
    ctx: &Ctx,
    item_index: NodeIndex,
    relative_path: &Path,
) -> AResult<NodeIndex> {
    let entry = ctx
        .graph()
        .node_weight(item_index)
        .context("Couldn't get node")?;

    let module_index = if matches!(entry, AnalysisEntry::Mod(_)) {
        item_index
    } else {
        get_super(ctx, item_index)?
    };

    resolve_path_recurse(ctx, item_index, module_index, relative_path, 0)
}

pub fn resolve_path_recurse(
    ctx: &Ctx,
    self_index: NodeIndex,
    current: NodeIndex,
    relative_path: &Path,
    path_segment_index: usize,
) -> AResult<NodeIndex> {
    let Some(path_segment) = relative_path.segments.get(path_segment_index) else {
        return Ok(current);
    };

    let next_index = resolve_next_segment(ctx, self_index, current, path_segment)?;
    resolve_path_recurse(
        ctx,
        self_index,
        next_index,
        relative_path,
        path_segment_index + 1,
    )
}

pub fn resolve_next_segment(
    ctx: &Ctx,
    self_index: NodeIndex,
    current: NodeIndex,
    path_segment: &PathSegment,
) -> AResult<NodeIndex> {
    let ident = &path_segment.ident;

    if ident.to_string() == "self" {
        Ok(current)
    } else if ident.to_string() == "Self" {
        Ok(self_index)
    } else if ident.to_string() == "crate" {
        Ok(ctx.crate_root)
    } else if ident.to_string() == "super" {
        get_super(ctx, current)
    } else if let Some(child_index) = find_neighbor(ctx, current, &ident) {
        Ok(child_index)
    } else if let Ok(node) = resolve_prelude(ctx, ident) {
        Ok(node)
    } else {
        bail!("Couldn't resolve segment {:?}", path_segment);
    }
}

fn resolve_prelude(ctx: &Ctx, path_segment: &Ident) -> AResult<NodeIndex> {
    let root = ctx.crate_root;
    if let AnalysisEntry::Mod(module) = ctx.krate()?
        && let Some(root_of) = &module.root_of_crate
        && root_of.to_string() == path_segment.to_string()
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
            .position(|from_ty| q!(#ty).to_string() == from_ty.to_string())
        {
            *ty = self.to_types[position].clone();
        }

        visit_mut::visit_type_mut(self, ty);
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if let Some(position) = self
            .from_lifetimes
            .iter()
            .position(|from_lifetime| lifetime.ident.to_string() == from_lifetime.to_string())
        {
            *lifetime = self.to_lifetimes[position].clone();
        }

        visit_mut::visit_lifetime_mut(self, lifetime);
    }
}

pub fn resolve_impls(ctx: &Ctx, node_index: NodeIndex) -> AResult<Vec<ItemImpl>> {
    let mut neighbors = ctx.graph().neighbors(node_index).detach();
    let mut consts = vec![];

    while let Some((_, neighbor)) = neighbors.next(ctx.graph()) {
        if let AnalysisEntry::Impl(impl_entry) = ctx.entry(neighbor)? {
            let mut item = impl_entry.item.clone();

            TyResolve {
                ctx,
                item_index: neighbor,
            }
            .visit_item_impl_mut(&mut item);

            consts.push(item);
        }
    }

    Ok(consts)
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

            let path = full_path(ctx, neighbor, PathType::PublicOnly)?;

            consts.push((path, c.item));
        }
    }

    Ok(consts)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PathType {
    Any,
    PublicOnly,
    TopLevelPublicOnly,
}

pub fn full_path<'a>(ctx: &'a Ctx, node_index: NodeIndex, path_type: PathType) -> AResult<Path> {
    let (cost, graph_path) = astar(
        ctx.graph(),
        ctx.crate_root,
        |x| x == node_index,
        |e| {
            let entry = ctx.entry(e.target()).unwrap();
            let is_impl = matches!(entry, AnalysisEntry::Impl(_));

            let is_crate = path_type == PathType::TopLevelPublicOnly
                && if let AnalysisEntry::Mod(module) = entry
                    && module.root_of_crate.is_some()
                {
                    true
                } else {
                    false
                };

            let no_visibility = path_type != PathType::Any && !entry.vis().is_public();

            if no_visibility || is_impl || is_crate {
                1
            } else {
                0
            }
        },
        |_| 0,
    )
    .context(format!("Couldn't get path {:?}", node_index))?;

    if cost > 0 {
        bail!("Can't find public path to item");
    }

    let mut segments = Punctuated::new();
    if path_type != PathType::Any {
        let AnalysisEntry::Mod(module) = ctx.krate()? else {
            bail!("Couldn't get crate name");
        };

        segments.push(PathSegment {
            ident: module.root_of_crate.clone().unwrap(),
            arguments: PathArguments::None,
        });
    };

    let mut previous_segment = None;
    for node_index in graph_path.iter() {
        if let Some(from_index) = previous_segment {
            if let Some(edge) = ctx.graph().edges_connecting(from_index, *node_index).next() {
                if let Some(name) = &edge.weight().name {
                    segments.push(PathSegment {
                        ident: name.clone(),
                        arguments: PathArguments::None,
                    });
                }
            }
        }

        previous_segment = Some(*node_index);
    }

    Ok(Path {
        leading_colon: None,
        segments,
    })
}
