use std::path::PathBuf;

use crate::{
    AResult,
    analysis::{
        Analysis, AnalysisEdge, AnalysisEntry, AnalysisEnum, AnalysisMod, AnalysisRef,
        AnalysisRefMut, AnalysisStruct, AnalysisTrait, AnalysisTypeAlias, VecPushIndex,
    },
    crate_graph::{find_neighbor, item_path, update_edge},
    types::resolve_type_paths,
    utils::{krate, path_segments, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{Direction, graph::NodeIndex, visit::Bfs};
use syn::{Fields, Ident, Item, Type};

pub fn parse_crate(
    analysis: &mut Analysis,
    output_path: PathBuf,
    crate_path: PathBuf,
    crate_name: &str,
) -> AResult<NodeIndex> {
    write_expanded(&output_path, &crate_path)?;

    let Ok(contents) = std::fs::read_to_string(output_path.clone()) else {
        bail!("Failed to read {:?}", &output_path);
    };

    process_crate(analysis, crate_name, contents)
}

pub fn process_crate(
    analysis: &mut Analysis,
    crate_name: &str,
    contents: String,
) -> Result<NodeIndex, anyhow::Error> {
    let file = syn::parse_file(&contents)?;

    let root_index = krate(analysis, crate_name, true, file.items);

    process_subtree(analysis, root_index)?;

    Ok(root_index)
}

pub fn process_subtree(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let AnalysisRef::Mod(module) = AnalysisEntry::node_index_ref(analysis, parent_mod)? else {
        bail!("Couldn't get node")
    };

    let content = module.content.clone();

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let ident = mod_item.ident.clone();
                let id = analysis
                    .modules
                    .push_index(AnalysisMod::new(mod_item, None));

                let child_mod = analysis.graph.add_node(AnalysisEntry::Mod(id));
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
                let id = analysis
                    .structs
                    .push_index(AnalysisStruct::new(item.clone(), vec![]));

                let child = analysis.graph.add_node(AnalysisEntry::Struct(id));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );
            }
            Item::Enum(item) => {
                let ident = item.ident.clone();
                let id = analysis
                    .enums
                    .push_index(AnalysisEnum::new(item.clone(), vec![]));

                let child = analysis.graph.add_node(AnalysisEntry::Enum(id));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );
            }
            Item::Type(item) => {
                let ident = item.ident.clone();
                let id = analysis
                    .types
                    .push_index(AnalysisTypeAlias::new(item.clone()));

                let child = analysis.graph.add_node(AnalysisEntry::Type(id));

                update_edge(
                    analysis,
                    parent_mod,
                    child,
                    AnalysisEdge::new(true, Some(ident)),
                );
            }
            Item::Trait(item) => {
                let ident = item.ident.clone();
                let id = analysis.traits.push_index(AnalysisTrait::new(item.clone()));
                let child = analysis.graph.add_node(AnalysisEntry::Trait(id));
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

pub fn process_impls(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Mod(module) = AnalysisEntry::node_index_ref(analysis, node_index)? {
            let content = module.content.clone();
            for item in content {
                if let Item::Impl(item_impl) = &item {
                    let mut impl_item = item_impl.clone();
                    let ty = (*impl_item.self_ty).clone();
                    let ty = resolve_type_paths(ty, analysis, root_index, node_index);
                    if let Type::Path(path) = &ty {
                        if let Ok((resolved_index, _)) = resolve_path(
                            analysis,
                            root_index,
                            root_index,
                            &path_segments(&path.path),
                        ) {
                            impl_item.self_ty = Box::new(ty);
                            match AnalysisEntry::node_index_ref_mut(analysis, resolved_index) {
                                Ok(AnalysisRefMut::Struct(entry)) => entry.impls.push(impl_item),
                                Ok(AnalysisRefMut::Enum(entry)) => entry.impls.push(impl_item),
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn process_fields(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Struct(entry) = AnalysisEntry::node_index_ref(analysis, node_index)? {
            let entry = entry.clone();
            if let Fields::Named(fields) = &entry.fields {
                for (field_index, field) in fields.named.iter().enumerate() {
                    let ty = field.ty.clone();
                    let ty = resolve_type_paths(ty, analysis, root_index, node_index);

                    if let AnalysisRefMut::Struct(entry) =
                        AnalysisEntry::node_index_ref_mut(analysis, node_index)?
                    {
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
    root_index: NodeIndex,
    item_index: NodeIndex,
    relative_path: &[&Ident],
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

    if let Ok(resolved) = resolve_path_recurse(analysis, module_index, relative_path, 0)
        && let Ok(path) = item_path(analysis, root_index, resolved)
    {
        Ok((resolved, path))
    } else {
        bail!("Couldn't resolve path {:?} {:?}", item_index, relative_path);
    }
}

pub fn resolve_path_recurse(
    analysis: &Analysis,
    current: NodeIndex,
    relative_path: &[&Ident],
    path_segment_index: usize,
) -> AResult<NodeIndex> {
    let Some(path_segment) = relative_path.get(path_segment_index) else {
        return Ok(current);
    };

    let next_index = resolve_next_segment(analysis, current, path_segment)?;
    resolve_path_recurse(analysis, next_index, relative_path, path_segment_index + 1)
}

pub fn resolve_next_segment(
    analysis: &Analysis,
    current: NodeIndex,
    path_segment: &Ident,
) -> AResult<NodeIndex> {
    if &path_segment.to_string() == "self" {
        Ok(current)
    } else if &path_segment.to_string() == "crate" {
        get_krate(analysis, current)
    } else if &path_segment.to_string() == "super" {
        get_super(analysis, current)
    } else if let Some(child_index) = find_neighbor(analysis, current, &path_segment) {
        Ok(child_index)
    } else {
        let krate_index = get_krate(analysis, current)?;
        if let AnalysisRef::Mod(krate) = AnalysisEntry::node_index_ref(analysis, krate_index)?
            && let Some(krate_root) = &krate.crate_root
            && let Some(extern_crate) = krate_root.extern_prelude.get(&path_segment.to_string())
        {
            Ok(*extern_crate)
        } else {
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

pub fn get_krate(analysis: &Analysis, node_index: NodeIndex) -> AResult<NodeIndex> {
    let mut node_index = node_index;
    loop {
        if let AnalysisRef::Mod(entry) = AnalysisEntry::node_index_ref(analysis, node_index)? {
            if entry.crate_root.is_some() {
                break;
            };
        }

        node_index = get_super(analysis, node_index)?;
    }

    Ok(node_index)
}
