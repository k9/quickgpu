use std::collections::HashSet;

use petgraph::graph::NodeIndex;

pub use crate::analysis::{
    Analysis, AnalysisEdge, AnalysisEnum, AnalysisStruct, AnalysisTypeAlias,
};
pub use crate::exports::list_exports;
pub use crate::exports::{Exported, ExportedItems};
pub use crate::process::parse_crate;
use crate::process::{keep_only_pub, process_impls};
use crate::use_statements::process_use_statements;

mod analysis;
mod crate_graph;
mod exports;
mod process;
mod use_statements;
mod utils;

type AResult<T> = anyhow::Result<T>;

pub fn discover(mut analysis: Analysis, root_index: NodeIndex) -> AResult<ExportedItems> {
    let mut skipped_mods = HashSet::new();
    let mut num_edges: usize = analysis.graph.edge_count();
    loop {
        process_use_statements(&mut analysis, root_index, &mut skipped_mods)?;
        let new_num_edges = analysis.graph.edge_count();
        if num_edges == new_num_edges {
            break;
        } else {
            num_edges = new_num_edges;
        }
    }

    process_impls(&mut analysis, root_index)?;

    println!(
        "skipped mod names: {}",
        skipped_mods.into_iter().collect::<Vec<_>>().join(", ")
    );

    keep_only_pub(&mut analysis, root_index)?;
    list_exports(&analysis, root_index)
}

#[cfg(test)]
mod test {
    use crate::{
        analysis::{Analysis, AnalysisEdge},
        process::parse_crate,
        utils::relative_path,
    };

    #[test]
    fn discover() {
        let mut analysis = Analysis::default();

        let root_index = parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu.rs"),
            relative_path("test_workspace/test_lib"),
            "crate",
        )
        .unwrap();

        let root_types_index = parse_crate(
            &mut analysis,
            relative_path("expanded/wgpu_types.rs"),
            relative_path("test_workspace/test_lib_types"),
            "tlt",
        )
        .unwrap();

        analysis
            .graph
            .update_edge(root_index, root_types_index, AnalysisEdge::Normal);

        let exports = super::discover(analysis, root_index).unwrap();
        assert_eq!(exports.structs.len(), 8);
        assert_eq!(exports.types.len(), 1);
    }
}
