use std::path::PathBuf;

use crate::{
    AResult,
    analysis::{
        Analysis, AnalysisEdge, AnalysisEntry, AnalysisEnum, AnalysisRef, AnalysisRefMut,
        AnalysisStruct, AnalysisTypeAlias, VecPushIndex, find_neighbor,
    },
    exports::item_path,
    utils::{dummy_module, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{Direction, graph::NodeIndex, visit::Bfs};
use syn::{Ident, Item, Type, Visibility};

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

    let file = syn::parse_file(&contents)?;

    let crate_root = dummy_module(crate_name, file.items);
    let id = analysis.modules.push_index(crate_root);
    let root_index = analysis.graph.add_node(AnalysisEntry::Mod(id));

    analysis.crates.insert(crate_name.to_string(), root_index);

    process_subtree(analysis, root_index)?;

    Ok(root_index)
}

pub fn process_subtree(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let AnalysisRef::Mod(module) = AnalysisEntry::node_index_ref(analysis, parent_mod)? else {
        bail!("Couldn't get node")
    };

    let Some((_, content)) = &module.content else {
        bail!("Couldn't get node")
    };

    let content = content.clone();

    for item in content.into_iter() {
        match item {
            Item::Mod(mod_item) => {
                let id = analysis.modules.push_index(mod_item);
                let child_mod = analysis.graph.add_node(AnalysisEntry::Mod(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child_mod, AnalysisEdge::new(false, None));

                process_subtree(analysis, child_mod)?;
            }
            Item::Struct(struct_item) => {
                let id = analysis.structs.push_index(AnalysisStruct {
                    item: struct_item.clone(),
                    impls: vec![],
                    path: vec![],
                });

                let child = analysis.graph.add_node(AnalysisEntry::Struct(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::new(false, None));
            }
            Item::Enum(enum_item) => {
                let id = analysis.enums.push_index(AnalysisEnum {
                    item: enum_item.clone(),
                    impls: vec![],
                    path: vec![],
                });

                let child = analysis.graph.add_node(AnalysisEntry::Enum(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::new(false, None));
            }
            Item::Type(type_item) => {
                let id = analysis.types.push_index(AnalysisTypeAlias {
                    item: type_item.clone(),
                    impls: vec![],
                    path: vec![],
                });
                let child = analysis.graph.add_node(AnalysisEntry::Type(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::new(false, None));
            }
            Item::Impl(type_impl) => {
                let id = analysis.impls.push_index(type_impl.clone());
                let child = analysis.graph.add_node(AnalysisEntry::Impl(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::new(false, None));
            }
            _ => (),
        };
    }

    Ok(())
}

pub fn process_impls(analysis: &mut Analysis, root_index: NodeIndex) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Impl(entry) = AnalysisEntry::node_index_ref(analysis, node_index)? {
            let entry = entry.clone();
            if let Type::Path(path) = *entry.self_ty.clone() {
                let ty_path = path
                    .path
                    .segments
                    .iter()
                    .map(|seg| &seg.ident)
                    .collect::<Vec<_>>();

                if let Ok((resolved, _)) = resolve_path(analysis, root_index, node_index, &ty_path)
                {
                    match AnalysisEntry::node_index_ref_mut(analysis, resolved)? {
                        AnalysisRefMut::Struct(analysis_struct) => {
                            analysis_struct.impls.push(entry.clone());
                        }
                        AnalysisRefMut::Enum(analysis_enum) => {
                            analysis_enum.impls.push(entry.clone());
                        }
                        _ => (),
                    };
                };
            }
        }
    }

    Ok(())
}

pub fn resolve_path(
    analysis: &mut Analysis,
    root_index: NodeIndex,
    item_index: NodeIndex,
    relative_path: &[&Ident],
) -> AResult<(NodeIndex, Vec<String>)> {
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
        && let Some(path) = item_path(analysis, root_index, resolved)?
    {
        Ok((resolved, path))
    } else {
        bail!("Couldn't resolve path {:?}", relative_path);
    }
}

pub fn resolve_path_recurse(
    analysis: &mut Analysis,
    current: NodeIndex,
    relative_path: &[&Ident],
    path_segment_index: usize,
) -> AResult<NodeIndex> {
    let Some(path_segment) = relative_path.get(path_segment_index) else {
        return Ok(current);
    };

    if let Some(krate) = analysis.crates.get(&path_segment.to_string()) {
        resolve_path_recurse(analysis, *krate, relative_path, path_segment_index + 1)
    } else if &path_segment.to_string() == "super" {
        let parent = get_super(analysis, current)?;
        resolve_path_recurse(analysis, parent, relative_path, path_segment_index + 1)
    } else {
        if let Some(child_index) = find_neighbor(analysis, current, &path_segment) {
            resolve_path_recurse(analysis, child_index, relative_path, path_segment_index + 1)
        } else {
            bail!("Can't resove path {:?}", relative_path);
        }
    }
}

pub fn get_super(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<NodeIndex> {
    let mut parents = analysis
        .graph
        .neighbors_directed(node_index, Direction::Incoming)
        .detach();

    while let Ok((edge_index, node_index)) =
        parents.next(&analysis.graph).context("Couldn't get parent")
    {
        if let Some(edge) = analysis.graph.edge_weight(edge_index)
            && edge.from_use_statement == false
        {
            return Ok(node_index);
        }
    }

    bail!("Couldn't get parent");
}

pub fn keep_only_pub(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<()> {
    let to_keep = keep_only_gather(analysis, node_index)?;
    let node_indices = analysis.graph.node_indices().collect::<Vec<_>>();
    for node_index in node_indices {
        if !to_keep.contains(&node_index) {
            analysis.graph.remove_node(node_index);
        }
    }

    Ok(())
}

fn keep_only_gather(analysis: &mut Analysis, node_index: NodeIndex) -> AResult<Vec<NodeIndex>> {
    let mut to_keep = vec![node_index];
    let neighbors = analysis.graph.neighbors(node_index).collect::<Vec<_>>();

    for neighbor in neighbors {
        if let Some(entry) = analysis.graph.node_weight(neighbor) {
            match entry.get_ref(analysis) {
                Ok(AnalysisRef::Struct(struct_entry)) => {
                    if matches!(struct_entry.item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Type(struct_entry)) => {
                    if matches!(struct_entry.item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Mod(mod_entry)) => {
                    if matches!(mod_entry.vis, Visibility::Public(_)) {
                        let mut descendants = keep_only_gather(analysis, neighbor)?;
                        to_keep.append(&mut descendants);
                    }
                }
                _ => (),
            }
        }
    }

    Ok(to_keep)
}
