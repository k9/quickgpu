use petgraph::graph::NodeIndex;

pub use crate::analysis::{
    Analysis, AnalysisEdge, AnalysisEnum, AnalysisStruct, AnalysisTypeAlias,
};
use crate::crate_graph::keep_only_public;
pub use crate::process::parse_crate;
use crate::process::{process_fields, process_impls};
use crate::use_statements::process_use_statements;

pub mod analysis;
pub mod crate_graph;
mod exports;
mod process;
pub mod types;
mod use_statements;
pub mod utils;

type AResult<T> = anyhow::Result<T>;
pub type EntryIndex = NodeIndex;

pub fn discover(analysis: &mut Analysis) -> AResult<()> {
    discover_paths(analysis)?;
    process_impls(analysis)?;
    process_fields(analysis)?;
    keep_only_public(analysis)?;
    Ok(())
}

fn discover_paths(analysis: &mut Analysis) -> AResult<()> {
    let mut num_edges: usize = analysis.graph.edge_count();
    loop {
        process_use_statements(analysis, analysis.root_index)?;
        let new_num_edges = analysis.graph.edge_count();
        if num_edges == new_num_edges {
            break;
        } else {
            num_edges = new_num_edges;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        analysis::{Analysis, AnalysisEntry},
        crate_graph::{filter_map_nodes, for_each_node, get_entry, node_ident, print_dot},
        process::{parse_crate, process_crate},
        utils::{path_from_string, path_string, relative_path},
    };

    use quote::quote as q;
    use syn::{ImplItem, ImplItemConst};

    use super::discover;

    fn test_workspace() -> Analysis {
        let mut analysis = Analysis::default();

        analysis.root_index = parse_crate(
            &mut analysis,
            relative_path("../expanded/wgpu_types.rs"),
            relative_path("test_workspace/test_lib_types"),
            "test_lib_types",
            vec![],
        )
        .unwrap();
        discover(&mut analysis).unwrap();

        analysis.root_index = parse_crate(
            &mut analysis,
            relative_path("../expanded/wgpu.rs"),
            relative_path("test_workspace/test_lib"),
            "test_lib",
            vec![],
        )
        .unwrap();
        discover(&mut analysis).unwrap();

        analysis
    }

    #[test]
    fn test_discover() {
        let analysis = test_workspace();

        print_dot(&analysis);

        assert_eq!(
            filter_map_nodes(&analysis, |index| matches!(
                get_entry(&analysis, index).unwrap(),
                AnalysisEntry::Struct(_)
            )
            .then_some(index))
            .count(),
            7
        );

        let mut consts: HashMap<String, Vec<String>> = HashMap::new();
        for_each_node(&analysis, |index| {
            if let AnalysisEntry::Struct(entry) = get_entry(&analysis, index).unwrap() {
                consts.insert(
                    node_ident(&analysis, index).unwrap().to_string(),
                    struct_consts(entry)
                        .iter()
                        .map(|c| c.ident.to_string())
                        .collect::<Vec<_>>(),
                );
            }
        });

        assert_eq!(consts.len(), 7);

        let abc_consts = consts.get("A").unwrap();
        assert_eq!(abc_consts.len(), 0);

        let abc_consts = consts.get("Abc").unwrap();
        assert_eq!(abc_consts.len(), 2);
        assert!(abc_consts.contains(&"XYZ".to_string()));
        assert!(abc_consts.contains(&"ZZZZ".to_string()));
    }

    fn struct_consts(entry: &crate::AnalysisStruct) -> Vec<&ImplItemConst> {
        let mut consts = vec![];
        for impl_item in &entry.impls {
            for item in &impl_item.items {
                if let ImplItem::Const(c) = item {
                    consts.push(c);
                };
            }
        }

        consts
    }

    #[test]
    fn resolve_path() {
        let mut analysis = test_workspace();
        let root_index = analysis.root_index.clone();

        let resolution =
            super::process::resolve_path(&mut analysis, root_index, &path_from_string("abc::ZZ"))
                .unwrap();

        assert_eq!(path_string(&resolution.1), "test_lib::abc::ZZ");

        let abc = super::process::resolve_path(
            &mut analysis,
            root_index,
            &path_from_string("crate::abc"),
        )
        .unwrap();

        assert_eq!(path_string(&abc.1), "test_lib::abc");

        let resolution =
            super::process::resolve_path(&mut analysis, abc.0, &path_from_string("super")).unwrap();

        assert_eq!(path_string(&resolution.1), "test_lib");

        let resolution =
            super::process::resolve_path(&mut analysis, abc.0, &path_from_string("crate")).unwrap();

        assert_eq!(path_string(&resolution.1), "test_lib");

        assert_eq!(
            path_string(
                &super::process::resolve_path(
                    &mut analysis,
                    abc.0,
                    &path_from_string("super::tlt::counters::CounterA")
                )
                .unwrap()
                .1
            ),
            "test_lib::tlt::CounterA"
        );

        assert_eq!(
            path_string(
                &super::process::resolve_path(
                    &mut analysis,
                    abc.0,
                    &path_from_string("test_lib::tlt::counters::CounterA")
                )
                .unwrap()
                .1
            ),
            "test_lib::tlt::CounterA"
        );
    }

    #[test]
    fn crate_relative() {
        let mut analysis = Analysis::default();

        let extra = process_crate(
            &mut analysis,
            "extra",
            q!(
                pub mod B {
                    pub const Z: crate::A = crate::A {};
                }
            )
            .to_string(),
            vec![],
        )
        .unwrap();
        discover(&mut analysis).unwrap();

        let root_index = process_crate(
            &mut analysis,
            "base",
            q!(
                extern crate extra as ext;
                pub struct A {}
            )
            .to_string(),
            vec![("extra".to_string(), extra)],
        )
        .unwrap();
        analysis.root_index = root_index;
        discover(&mut analysis).unwrap();

        assert_eq!(
            path_string(
                &super::process::resolve_path(
                    &mut analysis,
                    root_index,
                    &path_from_string("base::ext")
                )
                .unwrap()
                .1
            ),
            "base::ext"
        );
    }

    #[test]
    fn type_alias() {
        let mut analysis = Analysis::default();

        process_crate(
            &mut analysis,
            "extra",
            q!(
                pub struct A {}
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        let root_index = process_crate(
            &mut analysis,
            "base",
            q!(
                extern crate extra as ext;
                pub mod api {
                    pub use ext::A as ABase;
                    pub type A = ABase;
                }
                pub use api::*;
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        analysis.root_index = root_index;

        discover(&mut analysis).unwrap();

        let mut structs = filter_map_nodes(&analysis, |node_index| {
            if let Ok(AnalysisEntry::Struct(_)) = get_entry(&analysis, node_index) {
                Some(node_index)
            } else {
                None
            }
        });

        assert_eq!(
            node_ident(&analysis, structs.next().unwrap())
                .unwrap()
                .to_string(),
            "ABase"
        );

        assert!(structs.next().is_none());

        /*let mut types = filter_nodes(&analysis, root_index, |node_index| {
            if let Ok(AnalysisRef::Type(_)) = AnalysisEntry::node_index_ref(&analysis, node_index) {
                Some(node_index)
            } else {
                None
            }
        });


        assert_eq!(
            node_ident(&analysis, root_index, types.next().unwrap())
                .unwrap()
                .to_string(),
            "A"
        );

        assert!(structs.next().is_none());*/
    }
}
