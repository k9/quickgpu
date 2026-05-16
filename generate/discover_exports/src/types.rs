use petgraph::graph::NodeIndex;
use syn::{Path, PathArguments, PathSegment, punctuated::Punctuated};

use crate::{
    analysis::Ctx,
    resolve::{get_public_path, resolve_path},
};

pub fn type_path(ctx: &Ctx, item_index: NodeIndex, path: &mut Path) {
    if let Some(last_segment) = path.segments.last() {
        if let Ok(node) = resolve_path(ctx, item_index, path)
            && let Ok(full) = get_public_path(ctx, node)
        {
            let arguments = last_segment.arguments.clone();
            let mut segments = Punctuated::new();

            for segment in full.segments {
                segments.push(PathSegment {
                    ident: segment.ident,
                    arguments: PathArguments::None,
                });
            }

            path.segments = segments;

            if let Some(last) = path.segments.last_mut() {
                last.arguments = arguments;
            }
        } else {
            log::debug!("Couldn't resolve {:?}", path);
        };
    }
}
