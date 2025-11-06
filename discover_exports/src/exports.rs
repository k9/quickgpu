// - syn provides tree view
// - this crate resolves paths + visibility
// - structs, enum, trait declarations, and type aliases are top-level exports
// - trait implementations can be anywhere in syn tree, but location doesn't matter
//   - actual link is to type (struct/enum/primitive)
// - fields and consts are also linked to types
//
// - can have two-level structure where structs list:
//   - fields
//   - trait impls
//   - consts, methods (regardless of impl block)
// - downside being trait impls are not one-to-one due to generics
//   - but this is like monomorphisation, where one impl would be copied to multiple types
//
// field path and const path are fully decided by attachment to type
// field type and const type need to be resolved / made crate-relative
//
// Name resolution
//   Ignoring multiple crates for now:
//     - use statements add links as they do now
//     - user code iterates structs
//       - get an exported path for the struct
//         - node is known
//         - calculate path from crate root to item without crate name
//       - get an exported path for a e.g. struct's field type
//         - get node using type path relative to parent
//         - calculate path from crate root to item starting without crate name
//       - get impls for a struct
//         - for each impl, get node using self type path relative to impl parent
//         - any which resolve to node_index count (ignoring generics)
//   Multiple crates:
//     - first crate has no analyzed dependencies
//     - second crate depends on first
//     - are dependencies nodes...
//       - need a way to represent use statement declared in lib::abc to other::X
//       - analysis has crates (map of (name, Crate))
//       - dependencies can add Dependency nodes to root, edges have (dependency_name, node_id) as well as name
//       - extern prelude is Dependency children of root
//     - use statements can add intra-crate edges, also edges with other_crate_node_id
//     - user code iterates structs
//       - using Analysis level iterator
//       - (dependency_name, node) is known
//       - item_path resolves node if crate_name matches
//         - else
//           - get path from root to dependency
//           - get node_ids of edges to dependency_name and resolve each
//       - get an exported path for the struct
//         - node is known
//         - calculate path from crate root to item starting w/ crate name
