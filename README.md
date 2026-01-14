### 🚧 Warning this library is not ready for production use 🚧

Bugs need to be fixed, and the API could change
a lot in the coming weeks.
Any feedback is appreciated!

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
  Supports wgpu 28
- [/quickgpu27](https://github.com/k9/quickgpu/blob/main/quickgpu27)
  Supports wgpu 27

Internal / example crates:
  
- [/example](https://github.com/k9/quickgpu/blob/main/example)
  An example of using quickgpu
  in an app.
- [/bunnymark](https://github.com/k9/quickgpu/blob/main/bunnymark)
  Quick port of wgpu bunnymark
  to make sure performance is comparable.
- [/xtask](https://github.com/k9/quickgpu/blob/main/xtask)
  Code which generates
  he quickgpu bindings
  using `syn` and other tools.
  Also includes scripts
  or building and releasing new code.
- [/discover_exports](https://github.com/k9/quickgpu/blob/main/discover_exports)
  An experimental library
  or helping to resolve exports from a crate,
  type aliases, etc, to help with code generation.
  `discover_exports` isn't a general solution,
  it only has the features needed for this project.
  Lots of assumptions,
  unnecessary use of `clone`,
  and inefficient algrorithms.
  However, this crate is not a user-facing part
  of `quickgpu`, it just helps generates the bindings.

### Tech Decisions

- **All structs** `quickgpu`'s goal is that for any `wgpu` struct `SomeStruct`, as long at doesn't contain private fields, you can use a builder by typing `some_struct()`. This means even structs with zero or one fields have builders. This way developers don't have to memorize which structs have builders.
- **Custom builders** For most of quickgpu's development, quickgpu used the excellent `bon` to generate builders. However, figuring out some tricky issues involving trait bounds etc made me realize it would be quicker to write custom builders for now. I'm happy to discuss going back to using a builder library though.
- **Nested trait** Nested builders shouldn't need to call `build()`, so all builders implement a custom `Nested` trait. This was chosen over using `Into`, so quickgpu doesn't need to worry about implications of other `Into` conversions unrelated to nested builders.

### Next steps

- More tests and benchmarks to ensure `quickgpu` has minimal cost over `wgpu`
- Better code documentation
- Add convenience methods beyond simple builders

### Building

To run `example`, `cd` into `example` and run `cargo run`.
To run the `bunnymark` benchmark,
`cd` into `bunnymark` and run `cargo run`.
Press space to create more bunnies.
To re-generate quickgpu bindings, run `cargo xtask build`
from the repo root.
