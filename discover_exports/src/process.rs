use std::path::PathBuf;

use crate::{
    AResult, EntryIndex,
    analysis::{
        Analysis, AnalysisConst, AnalysisEdge, AnalysisEntry, AnalysisEnum, AnalysisMod,
        AnalysisStruct, AnalysisTrait, AnalysisTypeAlias,
    },
    crate_graph::{find_neighbor, get_entry, get_entry_mut, item_path, update_edge},
    types::{resolve_type_paths, type_path},
    utils::{krate, path_segments, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{Direction, graph::NodeIndex, visit::Bfs};
use syn::{Fields, Ident, ImplItem, Item, Type, visit_mut::VisitMut};

pub fn parse_crate(
    analysis: &mut Analysis,
    output_path: PathBuf,
    crate_path: PathBuf,
    crate_name: &str,
    dependencies: Vec<(String, NodeIndex)>,
) -> AResult<NodeIndex> {
    write_expanded(&output_path, &crate_path)?;

    let Ok(contents) = std::fs::read_to_string(output_path.clone()) else {
        bail!("Failed to read {:?}", &output_path);
    };

    process_crate(analysis, crate_name, contents, dependencies)
}

pub fn process_crate(
    analysis: &mut Analysis,
    crate_name: &str,
    contents: String,
    dependencies: Vec<(String, NodeIndex)>,
) -> Result<NodeIndex, anyhow::Error> {
    let file = syn::parse_file(&contents)?;

    let root_index = krate(analysis, crate_name, true, file.items)?;

    if let AnalysisEntry::Mod(crate_root) = get_entry_mut(analysis, root_index)?
        && let Some(crate_root) = crate_root.crate_root.as_mut()
    {
        for (name, index) in dependencies {
            crate_root.extern_prelude.insert(name, index);
        }
    }

    process_subtree(analysis, root_index)?;

    Ok(root_index)
}

pub fn process_subtree(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let AnalysisEntry::Mod(module) = get_entry(analysis, parent_mod)? else {
        bail!("Couldn't get node")
    };

    let content = module.content.clone();

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let ident = mod_item.ident.clone();

                let child_mod = analysis
                    .graph
                    .add_node(AnalysisEntry::Mod(AnalysisMod::new(mod_item, None)));

                update_edge(
                    analysis,
                    parent_mod,
                    child_mod,
                    AnalysisEdge::new(true, Some(ident)),
                );

                process_subtree(analysis, child_mod)?;
            }
            Item::Struct(item) => {
                let ident = item.ident.clone();

                let child = analysis
                    .graph
                    .add_node(AnalysisEntry::Struct(AnalysisStruct::new(
                        item.clone(),
                        vec![],
                    )));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );
            }
            Item::Enum(item) => {
                let ident = item.ident.clone();

                let child = analysis
                    .graph
                    .add_node(AnalysisEntry::Enum(AnalysisEnum::new(item.clone(), vec![])));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );

                for variant in &item.variants {
                    let variant_index = analysis.graph.add_node(AnalysisEntry::Variant);

                    update_edge(
                        analysis,
                        child,
                        variant_index,
                        AnalysisEdge::new(true, Some(variant.ident.clone())),
                    );
                }
            }
            Item::Type(item) => {
                let ident = item.ident.clone();

                let child = analysis
                    .graph
                    .add_node(AnalysisEntry::Type(AnalysisTypeAlias::new(item.clone())));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );
            }
            Item::Trait(item) => {
                let ident = item.ident.clone();
                let child = analysis
                    .graph
                    .add_node(AnalysisEntry::Trait(AnalysisTrait::new(item.clone())));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );
            }
            _ => (),
        };
    }

    Ok(())
}

pub fn process_impls(analysis: &mut Analysis) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, analysis.root_index);

    let mut to_add = vec![];

    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisEntry::Mod(module) = get_entry(analysis, node_index)? {
            let content = module.content.clone();
            for item in content {
                if let Item::Impl(item_impl) = &item {
                    let mut item_impl = item_impl.clone();
                    let ty = (*item_impl.self_ty).clone();
                    let ty = resolve_type_paths(ty, analysis, node_index);
                    if let Type::Path(path) = &ty {
                        if let Ok((resolved_index, _)) =
                            resolve_path(analysis, analysis.root_index, &path_segments(&path.path))
                        {
                            item_impl.self_ty = Box::new(ty);

                            let mut resolver = TyResolve {
                                analysis,
                                item_index: node_index,
                            };

                            resolver.visit_item_impl_mut(&mut item_impl);

                            for item in &item_impl.items {
                                if let ImplItem::Const(const_item) = item {
                                    let entry = AnalysisConst::new(const_item.clone());
                                    let ident = const_item.ident.clone();
                                    to_add.push((entry, ident, resolved_index));
                                }
                            }

                            match get_entry_mut(analysis, resolved_index) {
                                Ok(AnalysisEntry::Struct(entry)) => entry.impls.push(item_impl),
                                Ok(AnalysisEntry::Enum(entry)) => entry.impls.push(item_impl),
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }

    for (entry, ident, resolved_index) in to_add.into_iter() {
        let const_index = analysis.graph.add_node(AnalysisEntry::Const(entry));

        update_edge(
            analysis,
            resolved_index,
            const_index,
            AnalysisEdge::new(true, Some(ident)),
        );
    }

    Ok(())
}

struct TyResolve<'a> {
    analysis: &'a Analysis,
    item_index: EntryIndex,
}

impl<'a> VisitMut for TyResolve<'a> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        *ty = resolve_type_paths(ty.clone(), self.analysis, self.item_index);
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        type_path(self.analysis, self.item_index, path);
    }
}

pub fn process_fields(analysis: &mut Analysis) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, analysis.root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisEntry::Struct(entry) = get_entry(analysis, node_index)? {
            let entry = entry.clone();
            if let Fields::Named(fields) = &entry.fields {
                for (field_index, field) in fields.named.iter().enumerate() {
                    let ty = field.ty.clone();
                    let ty = resolve_type_paths(ty, analysis, node_index);

                    if let AnalysisEntry::Struct(entry) = get_entry_mut(analysis, node_index)? {
                        if let Fields::Named(fields) = &mut entry.fields {
                            fields
                                .named
                                .get_mut(field_index)
                                .context("Error updating field")?
                                .ty = ty;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn resolve_path(
    analysis: &Analysis,
    item_index: NodeIndex,
    relative_path: &[Ident],
) -> AResult<(NodeIndex, Vec<Ident>)> {
    let entry = analysis
        .graph
        .node_weight(item_index)
        .context("Couldn't get node")?;

    let module_index = if matches!(entry, AnalysisEntry::Mod(_)) {
        item_index
    } else {
        get_super(analysis, item_index)?
    };

    if relative_path.len() > 1
        && !["core", "fmt", "bitflags", "bytemuck"].contains(&relative_path[0].to_string().as_str())
    {
        let resolve = resolve_path_recurse(analysis, item_index, module_index, relative_path, 0);
        if resolve.is_err() {
            println!(
                "{:?} {:?} {:#?} {:#?}",
                item_path(analysis, item_index),
                relative_path,
                relative_path[0].span().start(),
                resolve_path_recurse(analysis, item_index, module_index, relative_path, 0)
            );
        }
    }

    if let Ok(resolved) = resolve_path_recurse(analysis, item_index, module_index, relative_path, 0)
        && let Ok(path) = item_path(analysis, resolved)
    {
        Ok((resolved, path))
    } else {
        bail!("Couldn't resolve path {:?} {:?}", item_index, relative_path);
    }
}

pub fn resolve_path_recurse(
    analysis: &Analysis,
    self_index: NodeIndex,
    current: NodeIndex,
    relative_path: &[Ident],
    path_segment_index: usize,
) -> AResult<NodeIndex> {
    let Some(path_segment) = relative_path.get(path_segment_index) else {
        return Ok(current);
    };

    let next_index = resolve_next_segment(analysis, self_index, current, path_segment)?;
    resolve_path_recurse(
        analysis,
        self_index,
        next_index,
        relative_path,
        path_segment_index + 1,
    )
}

pub fn resolve_next_segment(
    analysis: &Analysis,
    self_index: NodeIndex,
    current: NodeIndex,
    path_segment: &Ident,
) -> AResult<NodeIndex> {
    if &path_segment.to_string() == "self" {
        Ok(current)
    } else if &path_segment.to_string() == "Self" {
        Ok(self_index)
    } else if &path_segment.to_string() == "crate" {
        Ok(analysis.root_index)
    } else if &path_segment.to_string() == "super" {
        get_super(analysis, current)
    } else if let Some(child_index) = find_neighbor(analysis, current, &path_segment) {
        Ok(child_index)
    } else {
        if let AnalysisEntry::Mod(krate) = get_entry(analysis, analysis.root_index)?
            && let Some(krate_root) = &krate.crate_root
        {
            println!("{:?}", krate_root);
        }

        if let AnalysisEntry::Mod(krate) = get_entry(analysis, analysis.root_index)?
            && let Some(krate_root) = &krate.crate_root
            && let Some(extern_crate) = krate_root.extern_prelude.get(&path_segment.to_string())
        {
            Ok(*extern_crate)
        } else {
            log::debug!("Couldn't resolve path segment {:?}", path_segment);
            bail!("Couldn't resolve path segment {:?}", path_segment);
        }
    }
}

pub fn get_super(analysis: &Analysis, node_index: NodeIndex) -> AResult<NodeIndex> {
    let mut parents = analysis
        .graph
        .neighbors_directed(node_index, Direction::Incoming)
        .detach();

    while let Ok((edge_index, node_index)) =
        parents.next(&analysis.graph).context("Couldn't get parent")
    {
        if let Some(edge) = analysis.graph.edge_weight(edge_index)
            && edge.from_hierarchy
        {
            return Ok(node_index);
        }
    }

    bail!("Couldn't get parent");
}
