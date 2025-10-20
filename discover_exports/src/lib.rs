use std::collections::HashSet;

use petgraph::graph::NodeIndex;

use crate::analyze::{AnalysisRef, process_impls, process_use_statements};
use crate::analyze::{keep_only_pub, list_exports};

mod analyze;
mod crate_graph;
mod utils;

pub use analyze::{Analysis, AnalysisEdge, ExportedItem, parse_crate};

type AResult<T> = anyhow::Result<T>;

#[derive(Default, Debug)]
pub struct Exports {
    pub structs: Vec<ExportedItem>,
    pub types: Vec<ExportedItem>,
}

pub fn discover(mut analysis: Analysis, root_index: NodeIndex) -> AResult<Exports> {
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

    let mut exports = Exports::default();

    for export in list_exports(&analysis, root_index, |analysis_ref| {
        matches!(analysis_ref, AnalysisRef::Struct(_))
    })? {
        exports.structs.push(export);
    }

    println!("\ntype aliases");
    for export in list_exports(&analysis, root_index, |analysis_ref| {
        matches!(analysis_ref, AnalysisRef::Type(_))
    })? {
        exports.types.push(export);
    }

    Ok(exports)
}

#[cfg(test)]
mod test {
    use crate::{
        analyze::{Analysis, AnalysisEdge, parse_crate},
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
