use std::path::PathBuf;

use crate::{
    AResult,
    analysis::{
        Analysis, AnalysisEdge, AnalysisEnum, AnalysisItem, AnalysisRef, AnalysisRefMut,
        AnalysisStruct, AnalysisTypeAlias, VecPushIndex,
    },
    utils::{dummy_module, write_expanded},
};
use anyhow::{Context, bail};
use petgraph::{Direction, graph::NodeIndex, visit::Bfs};
use syn::{Item, Type, Visibility};

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
    let root_index = analysis.graph.add_node(AnalysisItem::Mod(id));

    analysis.crates.insert(crate_name.to_string(), root_index);

    process_subtree(analysis, root_index)?;

    Ok(root_index)
}

pub fn process_subtree(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let AnalysisRef::Mod(module) = AnalysisItem::node_index_ref(analysis, parent_mod)? else {
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
                let child_mod = analysis.graph.add_node(AnalysisItem::Mod(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child_mod, AnalysisEdge::Normal);

                process_subtree(analysis, child_mod)?;
            }
            Item::Struct(struct_item) => {
                let id = analysis.structs.push_index(AnalysisStruct {
                    item: struct_item.clone(),
                    impls: vec![],
                });

                let child = analysis.graph.add_node(AnalysisItem::Struct(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            Item::Enum(enum_item) => {
                let id = analysis.enums.push_index(AnalysisEnum {
                    item: enum_item.clone(),
                    impls: vec![],
                });

                let child = analysis.graph.add_node(AnalysisItem::Enum(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            Item::Type(type_item) => {
                let id = analysis.types.push_index(AnalysisTypeAlias {
                    item: type_item.clone(),
                    impls: vec![],
                });
                let child = analysis.graph.add_node(AnalysisItem::Type(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            Item::Impl(type_impl) => {
                let id = analysis.impls.push_index(type_impl.clone());
                let child = analysis.graph.add_node(AnalysisItem::Impl(id));
                analysis
                    .graph
                    .update_edge(parent_mod, child, AnalysisEdge::Normal);
            }
            _ => (),
        };
    }

    Ok(())
}

pub fn process_impls(analysis: &mut Analysis, parent_mod: NodeIndex) -> AResult<()> {
    let mut bfs = Bfs::new(&analysis.graph, parent_mod);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let AnalysisRef::Impl(item) = AnalysisItem::node_index_ref(analysis, node_index)? {
            let item = item.clone();
            if let Type::Path(path) = *item.self_ty.clone() {
                let parent = analysis
                    .graph
                    .neighbors_directed(node_index, Direction::Incoming)
                    .next()
                    .context("Couldn't get parent")?;

                let ty_path = path
                    .path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>();

                if ty_path.len() > 1 {
                    println!(
                        "Multi-segment impl paths not supported yet: {:?} for {:?}",
                        item.trait_.map(|x| x
                            .1
                            .segments
                            .iter()
                            .map(|segment| segment.ident.to_string())
                            .collect::<Vec<_>>()),
                        ty_path
                    );
                    continue;
                }

                let ty_name = &ty_path[0];

                let siblings = analysis
                    .graph
                    .neighbors_directed(parent, Direction::Outgoing)
                    .collect::<Vec<_>>();

                siblings.iter().any(|sibling| {
                    let sibling = sibling.clone();
                    let Ok(node_ref) = AnalysisItem::node_index_ref_mut(analysis, sibling) else {
                        return false;
                    };

                    match node_ref {
                        AnalysisRefMut::Struct(analysis_struct) => {
                            if &analysis_struct.item.ident.to_string() == ty_name {
                                analysis_struct.impls.push(item.clone());
                                return true;
                            }
                        }
                        AnalysisRefMut::Enum(analysis_enum) => {
                            if &analysis_enum.item.ident.to_string() == ty_name {
                                analysis_enum.impls.push(item.clone());
                                return true;
                            }
                        }
                        _ => (),
                    };

                    false
                });
            }
        }
    }

    Ok(())
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
        if let Some(item) = analysis.graph.node_weight(neighbor) {
            match item.get_ref(analysis) {
                Ok(AnalysisRef::Struct(struct_item)) => {
                    if matches!(struct_item.item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Type(struct_item)) => {
                    if matches!(struct_item.item.vis, Visibility::Public(_)) {
                        to_keep.push(neighbor);
                    }
                }
                Ok(AnalysisRef::Mod(mod_item)) => {
                    if matches!(mod_item.vis, Visibility::Public(_)) {
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
