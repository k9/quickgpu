use anyhow::{Context, bail};
use petgraph::{algo::astar, graph::NodeIndex, visit::Bfs};
use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::{
    AResult, Analysis,
    analysis::{AnalysisEdge, AnalysisItem, AnalysisRef},
};

#[derive(Debug, Clone)]
pub struct ExportedItem {
    pub path: Vec<String>,
    pub span: Span,
}

pub fn list_exports<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    filter: impl Fn(AnalysisRef<'a>) -> bool,
) -> AResult<Vec<ExportedItem>> {
    let mut exports = vec![];
    let mut bfs = Bfs::new(&analysis.graph, root_index);
    while let Some(node_index) = bfs.next(&analysis.graph) {
        if let Some(export_item) = list_export_item(analysis, root_index, node_index, &filter)? {
            exports.push(export_item);
        };
    }

    exports.sort_by(|a, b| a.path.join("").cmp(&b.path.join("")));
    exports.sort_by(|a, b| a.path.len().cmp(&b.path.len()));

    Ok(exports)
}

fn list_export_item<'a>(
    analysis: &'a Analysis,
    root_index: NodeIndex,
    node_index: NodeIndex,
    filter: &impl Fn(AnalysisRef<'a>) -> bool,
) -> AResult<Option<ExportedItem>> {
    if !filter(AnalysisItem::node_index_ref(analysis, node_index)?) {
        return Ok(None);
    };

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
    let mut span = None;
    for segment_index in graph_path.1 {
        let name = match AnalysisItem::node_index_ref(analysis, segment_index)? {
            AnalysisRef::Struct(struct_item) => {
                span = Some(struct_item.item.span());
                Some(struct_item.item.ident.to_string())
            }
            AnalysisRef::Enum(enum_item) => {
                span = Some(enum_item.item.span());
                Some(enum_item.item.ident.to_string())
            }
            AnalysisRef::Type(type_item) => {
                span = Some(type_item.span());
                Some(type_item.ident.to_string())
            }
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

    let Some(span) = span else {
        bail!("No span for exported item");
    };

    Ok(Some(ExportedItem { path, span }))
}
