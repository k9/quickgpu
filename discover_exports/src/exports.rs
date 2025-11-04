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
// Returns the shortest path from root to a node
