use petgraph::graph::NodeIndex;

pub use crate::analysis::{AnalysisEdge, AnalysisEnum, AnalysisStruct, AnalysisTypeAlias};

pub mod analysis;
pub mod crate_graph;
mod exports;
mod process;
pub mod types;
mod use_statements;
pub mod utils;

type AResult<T> = anyhow::Result<T>;
pub type EntryIndex = NodeIndex;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        EntryIndex,
        analysis::{Analysis, AnalysisEntry, Ctx},
        crate_graph::{filter_map_nodes, for_each_node, full_path, node_ident, print_dot},
        process::{parse_crate, process_crate},
        utils::{path_from_string, path_string, relative_path},
    };

    use quote::quote as q;
    use syn::{ImplItem, ImplItemConst};

    use super::process::resolve_path;

    fn test_workspace<'a>(analysis: &'a mut Analysis) -> Ctx<'a> {
        parse_crate(
            analysis,
            relative_path("../expanded/wgpu_types.rs"),
            relative_path("test_workspace/test_lib_types"),
            "test_lib_types",
            vec![],
        )
        .unwrap();

        let ctx = parse_crate(
            analysis,
            relative_path("../expanded/wgpu.rs"),
            relative_path("test_workspace/test_lib"),
            "test_lib",
            vec![],
        )
        .unwrap();

        ctx
    }

    #[test]
    fn test_discover() {
        let mut analysis = Analysis::default();
        let ctx = test_workspace(&mut analysis);

        print_dot(&ctx).unwrap();

        assert_eq!(
            filter_map_nodes(&ctx, |index| matches!(
                ctx.entry(index).unwrap(),
                AnalysisEntry::Struct(_)
            )
            .then_some(index))
            .unwrap()
            .count(),
            7
        );

        let mut consts: HashMap<String, Vec<String>> = HashMap::new();
        for_each_node(&ctx, |index| {
            if let AnalysisEntry::Struct(entry) = ctx.entry(index).unwrap() {
                consts.insert(
                    node_ident(&ctx, index).unwrap().to_string(),
                    struct_consts(entry)
                        .iter()
                        .map(|c| c.ident.to_string())
                        .collect::<Vec<_>>(),
                );
            }
        })
        .unwrap();

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
    fn test_resolve_path() {
        let mut analysis = Analysis::default();
        let mut ctx = test_workspace(&mut analysis);

        let (_, path) = resolve_full(&mut ctx, None, "abc::ZZ");
        assert_eq!(&path, "test_lib::abc::ZZ");

        let (abc_index, abc) = resolve_full(&mut ctx, None, "crate::abc");
        assert_eq!(&abc, "test_lib::abc");

        let (_, resolution) = resolve_full(&mut ctx, Some(abc_index), "super");
        assert_eq!(&resolution, "test_lib");

        let (_, resolution) = resolve_full(&mut ctx, Some(abc_index), "crate");
        assert_eq!(&resolution, "test_lib");

        assert_eq!(
            &resolve_full(&mut ctx, Some(abc_index), "super::tlt::counters::CounterA").1,
            "test_lib::tlt::CounterA"
        );

        assert_eq!(
            &resolve_full(
                &mut ctx,
                Some(abc_index),
                "test_lib::tlt::counters::CounterA"
            )
            .1,
            "test_lib::tlt::CounterA"
        );
    }

    fn resolve_full(
        ctx: &Ctx<'_>,
        from: Option<EntryIndex>,
        relative_path: &str,
    ) -> (EntryIndex, String) {
        let from = from.unwrap_or_else(|| ctx.crate_root.clone());
        let resolution = resolve_path(ctx, from, &path_from_string(&relative_path)).unwrap();
        (
            resolution,
            path_string(&full_path(&*ctx, resolution).unwrap()),
        )
    }

    #[test]
    fn one_crate() {
        let mut analysis = Analysis::default();

        let mut ctx = process_crate(
            &mut analysis,
            "base",
            q!(
                pub struct A {}
                pub mod inner {
                    pub mod inner2 {
                        pub enum E {
                            X,
                            Y,
                        }
                    }
                }
                pub mod inner3 {
                    pub use super::inner::inner2::E;
                    pub use crate::inner;
                }
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        let (a_node, path) = resolve_full(&mut ctx, None, "A");
        assert_eq!(&path, "A");
        assert_eq!(&resolve_full(&mut ctx, None, "crate::A").1, "A");
        assert_eq!(&resolve_full(&mut ctx, Some(a_node), "inner").1, "inner");

        let (inner2_node, path) = resolve_full(&mut ctx, Some(a_node), "inner::inner2");
        assert_eq!(path, "inner::inner2");
        assert_eq!(
            &resolve_full(&mut ctx, Some(inner2_node), "super").1,
            "inner"
        );
        assert_eq!(
            &resolve_full(&mut ctx, Some(inner2_node), "E::X").1,
            "inner3::E::X"
        );
        assert_eq!(
            &resolve_full(&mut ctx, None, "crate::inner3::inner").1,
            "inner"
        );
    }

    #[test]
    fn crate_relative() {
        let mut analysis = Analysis::default();

        process_crate(
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

        let ctx = process_crate(
            &mut analysis,
            "base",
            q!(
                extern crate extra as ext;
                pub struct A {}
            )
            .to_string(),
            vec!["extra".to_string()],
        )
        .unwrap();

        assert_eq!(&resolve_full(&ctx, None, "base::ext").1, "base::ext");
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

        let ctx = process_crate(
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

        let mut structs = filter_map_nodes(&ctx, |node_index| {
            if let Ok(AnalysisEntry::Struct(_)) = ctx.entry(node_index) {
                Some(node_index)
            } else {
                None
            }
        })
        .unwrap();

        assert_eq!(
            node_ident(&ctx, structs.next().unwrap())
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
