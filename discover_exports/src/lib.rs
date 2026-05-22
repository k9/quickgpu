use petgraph::graph::NodeIndex;

pub mod analysis;
pub mod analysis_entry;
pub mod crate_graph;
pub mod process;
pub mod resolve;
pub mod types;
mod use_statements;
pub mod utils;

type AResult<T> = anyhow::Result<T>;
pub type EntryIndex = NodeIndex;

#[cfg(test)]
mod test {
    use crate::{
        EntryIndex,
        analysis::{Analysis, Ctx},
        analysis_entry::AnalysisEntry,
        crate_graph::{filter_map_nodes, print_dot},
        process::{parse_crate, process_crate},
        resolve::{PathType, get_public_path, get_top_level_path, resolve_impls, resolve_path},
        utils::{path_from_string, path_refs_string, relative_path},
    };

    use quote::quote as q;
    use syn::{Expr, ImplItem, Stmt};

    fn test_workspace<'a>(analysis: &'a mut Analysis) -> Ctx<'a> {
        parse_crate(
            analysis,
            relative_path("../expanded/wgpu_types.rs"),
            relative_path("test_workspace/test_lib_types"),
            "test_lib_types",
            vec![],
        )
        .unwrap();

        parse_crate(
            analysis,
            relative_path("../expanded/wgpu.rs"),
            relative_path("test_workspace/test_lib"),
            "test_lib",
            vec!["test_lib_types".to_string()],
        )
        .unwrap()
    }

    #[test]
    fn test_discover() {
        let mut analysis = Analysis::default();
        let ctx = test_workspace(&mut analysis);

        assert_eq!(
            filter_map_nodes(
                &ctx,
                |(index, _)| matches!(ctx.entry(index).unwrap(), AnalysisEntry::Struct(_))
                    .then_some(index),
                PathType::PublicOnly
            )
            .unwrap()
            .count(),
            7
        );
    }

    #[test]
    fn test_resolve_path() {
        let mut analysis = Analysis::default();
        let ctx = test_workspace(&mut analysis);

        let (_, path) = resolve_top_level(&ctx, None, "abc::ZZ");
        assert_eq!(&path, "test_lib::abc::ZZ");

        let (abc_index, abc) = resolve_top_level(&ctx, None, "crate::abc");
        assert_eq!(&abc, "test_lib::abc");

        let (_, resolution) = resolve_top_level(&ctx, Some(abc_index), "super");
        assert_eq!(&resolution, "test_lib");

        let (_, resolution) = resolve_top_level(&ctx, Some(abc_index), "crate");
        assert_eq!(&resolution, "test_lib");

        assert_eq!(
            &resolve_public(&ctx, Some(abc_index), "super::tlt::counters::CounterA").1,
            "test_lib::tlt::CounterA"
        );

        assert_eq!(
            &resolve_public(&ctx, Some(abc_index), "tlt::counters::CounterA").1,
            "test_lib::tlt::CounterA"
        );
    }

    fn resolve_top_level(
        ctx: &Ctx<'_>,
        from: Option<EntryIndex>,
        relative_path: &str,
    ) -> (EntryIndex, String) {
        let from = from.unwrap_or(ctx.crate_root);
        let resolution = resolve_path(ctx, from, &path_from_string(relative_path)).unwrap();
        let path = get_top_level_path(ctx, resolution).unwrap();

        (resolution, path_refs_string(&path))
    }

    fn resolve_public(
        ctx: &Ctx<'_>,
        from: Option<EntryIndex>,
        relative_path: &str,
    ) -> (EntryIndex, String) {
        let from = from.unwrap_or(ctx.crate_root);
        let resolution = resolve_path(ctx, from, &path_from_string(relative_path)).unwrap();
        let path = get_public_path(ctx, resolution).unwrap();

        (resolution, path_refs_string(&path))
    }

    #[test]
    fn one_crate() {
        let mut analysis = Analysis::default();

        let ctx = process_crate(
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

        let (a_node, path) = resolve_top_level(&ctx, None, "A");
        assert_eq!(&path, "base::A");
        assert_eq!(&resolve_top_level(&ctx, None, "crate::A").1, "base::A");
        assert_eq!(
            &resolve_top_level(&ctx, Some(a_node), "inner").1,
            "base::inner"
        );

        let (inner2_node, path) = resolve_top_level(&ctx, Some(a_node), "base::inner::inner2");
        assert_eq!(path, "base::inner::inner2");
        assert_eq!(
            &resolve_top_level(&ctx, Some(inner2_node), "super").1,
            "base::inner"
        );
        assert_eq!(
            &resolve_top_level(&ctx, Some(inner2_node), "E::X").1,
            "base::inner3::E::X"
        );
        assert_eq!(
            &resolve_top_level(&ctx, None, "crate::inner3::inner").1,
            "base::inner"
        );
    }

    #[test]
    fn impls() {
        let mut analysis = Analysis::default();

        let ctx = process_crate(
            &mut analysis,
            "base",
            q!(
                pub struct A {
                    pub b: B,
                }

                pub mod inner {
                    pub struct B {}
                    impl Default for crate::A {
                        pub fn default() {
                            crate::A { b: B::default() }
                        }
                    }
                }

                pub mod impls {
                    impl Default for crate::inner::B {
                        fn default() {
                            B {}
                        }
                    }

                    impl super::inner::B {
                        pub const Y: u32 = 5;
                    }
                    const C: crate::inner::B = crate::inner::B {};
                }
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        let (index, path) = resolve_top_level(&ctx, None, "crate::A");
        assert_eq!(&path, "base::A");

        let impl_ = &resolve_impls(&ctx, index).unwrap()[0];
        let ImplItem::Fn(item) = &impl_.items[0] else {
            panic!("Expected function");
        };

        let Stmt::Expr(expr, _) = &item.block.stmts[0] else {
            panic!("Expected expr");
        };

        let Expr::Struct(expr) = &expr else {
            panic!("Expected struct");
        };

        let expr = &expr.fields[0].expr;

        assert_eq!(
            q!(base::inner::B::default()).to_string(),
            q!(#expr).to_string()
        );

        assert_eq!(
            &resolve_top_level(&ctx, None, "crate::inner::B::Y").1,
            "base::inner::B::Y"
        );
    }

    #[test]
    fn resolve_item() {
        let mut analysis = Analysis::default();

        let ctx = process_crate(
            &mut analysis,
            "base",
            q!(
                pub enum Other {}
                pub mod abc {
                    pub struct Abc {
                        pub a: super::Other,
                    }

                    pub mod def {
                        pub struct Ghi {
                            pub g: crate::Other,
                        }

                        impl super::Abc {
                            pub const C: u32 = 5;
                        }
                    }
                }
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        print_dot(&ctx).unwrap();
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
                    pub struct Inner {}
                }
                pub use B::Inner;
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

        assert_eq!(&resolve_public(&ctx, None, "base::ext").1, "base::ext");
        assert_eq!(
            &resolve_public(&ctx, None, "base::ext::B::Inner").1,
            "base::ext::Inner"
        );
    }

    #[test]
    fn top_level() {
        let mut analysis = Analysis::default();

        process_crate(
            &mut analysis,
            "extra",
            q!(
                pub struct B {}
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

        let from = ctx.crate_root;
        let resolution = resolve_path(&ctx, from, &path_from_string("A")).unwrap();
        assert!(get_public_path(&ctx, resolution).is_ok());
        assert!(get_top_level_path(&ctx, resolution).is_ok());

        let resolution = resolve_path(&ctx, from, &path_from_string("ext::B")).unwrap();
        assert!(get_public_path(&ctx, resolution).is_ok());
        assert!(get_top_level_path(&ctx, resolution).is_err());
    }

    #[test]
    fn type_alias() {
        let mut analysis = Analysis::default();

        process_crate(
            &mut analysis,
            "extra",
            q!(
                pub struct A<'a, 'bc, T, U> {
                    pub f: &'bc T,
                    pub f2: &'a U,
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
                pub mod api {
                    pub use ext::A as ABase;
                    pub type A<'b, U> = ABase<'b, 'static, u32, U>;
                }
                pub use api::*;
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        let mut structs = filter_map_nodes(
            &ctx,
            |(node_index, _)| {
                if let Ok(AnalysisEntry::Struct(_)) = ctx.entry(node_index) {
                    Some(node_index)
                } else {
                    None
                }
            },
            PathType::PublicOnly,
        )
        .unwrap();

        assert_eq!(
            path_refs_string(&get_public_path(&ctx, structs.next().unwrap()).unwrap()),
            "base::ABase"
        );

        assert!(structs.next().is_none());

        let mut types = filter_map_nodes(
            &ctx,
            |(node_index, _)| {
                if let Ok(AnalysisEntry::Type(_)) = ctx.entry(node_index) {
                    Some(node_index)
                } else {
                    None
                }
            },
            PathType::PublicOnly,
        )
        .unwrap();

        assert!(structs.next().is_none());

        let type_alias = types.next().unwrap();
        assert_eq!(
            path_refs_string(&get_public_path(&ctx, type_alias).unwrap()),
            "base::A"
        );
    }

    #[test]
    fn resolve_default_self() {
        let mut analysis = Analysis::default();

        let ctx = process_crate(
            &mut analysis,
            "base",
            q!(
                pub mod a {
                    pub struct A {}
                }
                pub mod b {
                    const SUPER_CONST: u32 = 5;
                    pub mod c {
                        impl crate::a::A {
                            pub const START: Self = Self {};
                            pub const OTHER: u32 = super::SUPER_CONST;
                        }
                    }
                }
            )
            .to_string(),
            vec![],
        )
        .unwrap();

        print_dot(&ctx).unwrap();

        let (index, _path) = resolve_top_level(&ctx, None, "crate::a::A");
        let impl_ = &resolve_impls(&ctx, index).unwrap()[0];
        if let ImplItem::Const(c) = &impl_.items[0] {
            assert!(!q!(#c).to_string().contains("Self"));
        };
    }
}
