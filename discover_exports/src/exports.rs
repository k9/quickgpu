use anyhow::Context;
use petgraph::{algo::astar, graph::NodeIndex, visit::Bfs};

use crate::{
    AResult, Analysis,
    analysis::{
        AnalysisEdge, AnalysisEnum, AnalysisItem, AnalysisRef, AnalysisStruct, AnalysisTypeAlias,
    },
};

#[derive(Debug, Clone)]
pub struct Exported<T> {
    pub path: Vec<String>,
    pub item: T,
}

fn sort_shortest_first<T>(exported_list: &mut Vec<Exported<T>>) {
    exported_list.sort_by(|a, b| a.path.join("").cmp(&b.path.join("")));
    exported_list.sort_by(|a, b| a.path.len().cmp(&b.path.len()));
}

#[derive(Debug, Default)]
pub struct ExportedItems {
    pub structs: Vec<Exported<AnalysisStruct>>,
    pub enums: Vec<Exported<AnalysisEnum>>,
    pub types: Vec<Exported<AnalysisTypeAlias>>,
}

pub fn list_exports<'a>(analysis: &'a Analysis, root_index: NodeIndex) -> AResult<ExportedItems> {
    let mut exports = ExportedItems::default();
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let Some(path) = item_path(analysis, root_index, node_index)? {
            match AnalysisItem::node_index_ref(analysis, node_index)? {
                AnalysisRef::Struct(node) => {
                    exports.structs.push(Exported {
                        path,
                        item: node.clone(),
                    });
                }
                AnalysisRef::Enum(node) => {
                    exports.enums.push(Exported {
                        path,
                        item: node.clone(),
                    });
                }
                AnalysisRef::Type(node) => {
                    exports.types.push(Exported {
                        path,
                        item: node.clone(),
                    });
                }
                _ => (),
            };
        }
    }

    sort_shortest_first(&mut exports.structs);
    sort_shortest_first(&mut exports.enums);
    sort_shortest_first(&mut exports.types);

    Ok(exports)
}

pub fn item_path<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    node_index: NodeIndex,
) -> AResult<Option<Vec<String>>> {
    let graph_path = astar(
        &analysis.graph,
        root_index,
        |x| x == node_index,
        |_| 1,
        |_| 0,
    )
    .context("Couldn't get path")?;

    let mut path = vec![];
    let mut previous_segment = None;
    for segment_index in graph_path.1 {
        let name = match AnalysisItem::node_index_ref(analysis, segment_index)? {
            AnalysisRef::Struct(struct_item) => Some(struct_item.item.ident.to_string()),
            AnalysisRef::Enum(enum_item) => Some(enum_item.item.ident.to_string()),
            AnalysisRef::Type(type_item) => Some(type_item.item.ident.to_string()),
            AnalysisRef::Impl(_) => None,
            AnalysisRef::Mod(mod_item) => Some(mod_item.ident.to_string()),
        };

        if let Some(mut name) = name {
            if let Some(previous_segment) = previous_segment
                && let Some(edge) = analysis.graph.find_edge(previous_segment, previous_segment)
                && let Some(AnalysisEdge::Rename(edge)) = analysis.graph.edge_weight(edge)
            {
                name = edge.clone();
            };

            path.push(name);

            previous_segment = Some(segment_index);
        }
    }

    Ok(Some(path))
}
