# Overview

`quickgpu` wraps the `wgpu` API
allowing users to write shorter,
clearer Rust graphics code.
It consists mainly
of builders for `wgpu` structs.
As a wrapper library,
`quickgpu` doesn't manage or own
any state after
a builder is done building.
There's no need to convert
all of your code to `quickgpu`,
you can just use it where it's helpful.

For an example of the library in use,
take a look at [/example/src/scene.rs](https://github.com/k9/quickgpu/blob/main/example/src/scene.rs)

## Architecture

Published crates:

- [/quickgpu](https://github.com/k9/quickgpu/blob/main/quickgpu)
  Enable the appropriate feature,
  such as `v29`,
  to enable the API matching `wgpu` v29

Internal / example crates:
  
- [/example](https://github.com/k9/quickgpu/blob/main/example)
  An example of using quickgpu
  in an app.
- [/bunnymark](https://github.com/k9/quickgpu/blob/main/bunnymark)
  Port of wgpu bunnymark
  to make sure performance is comparable.
- [/xtask](https://github.com/k9/quickgpu/blob/main/xtask)
  Code which generates
  `bon` builders by analyzing wgpu
  using `syn` and other tools.
  Also includes scripts
  for building and releasing new code.
- [/discover_exports](https://github.com/k9/quickgpu/blob/main/discover_exports)
  An experimental library
  for helping to resolve exports from a crate,
  type aliases, etc, to help with code generation.
  `discover_exports` isn't a general solution,
  it only has the features needed for this project.
  Note that this crate is not a user-facing part
  of `quickgpu`, it just helps generates the bindings.

### Tech Decisions

- **All structs** `quickgpu`'s goal is that for any `wgpu` struct `SomeStruct`, as long at doesn't contain private fields, you can use a builder by typing `some_struct()`. This means even structs with zero or one fields have builders. This way developers don't have to memorize which structs have builders.
- **Nested trait** Nested builders shouldn't need to call `build()`, so a builder which builds `SomeStruct` will implement a custom `NestedSomeStruct` trait. This was chosen over using `Into`, so quickgpu doesn't need to worry about implications of other `Into` conversions unrelated to nested builders.

### Next steps

- Look for other simple ways to wrap `wgpu` concepts

### Building

To run `example`, `cd` into `example` and run `cargo run`.

To run the `bunnymark` benchmark,
`cd` into `bunnymark` and run `cargo run`.
Press space to create more bunnies.

To re-generate quickgpu bindings, run `cargo xtask build`
from the repo root.
