### 🚧 Warning this library is an Alpha release, not ready for production, and the API could change a lot in the coming weeks. Any feedback is appreciated! 🚧

# Overview

`quickgpu` wraps the wgpu API allowing users to write shorter, clearer Rust graphics code.
It consists mainly of builders for wgpu structs. As a wrapper library, quickgpu doesn't
manage or own any state after a builder is done building. There's no need to convert
all of your code to quickgpu, you can just use it where it's helpful.

For an example of the library in use, take a look at [/example/src/scene.rs](/example/src/scene.rs)

## Architecture

This repo is split up into:

- [/quickgpu](/quickgpu) The main crate which wraps wgpu APIs. The main code here is generated as opposed to hand-written.
- [/example](/example) An example of using quickgpu in an app.
- [/xtask](/xtask) Scripts for building and releasing new code. Also includes code for generating the quickgpu bindings using syn and other tools.
- [/discover_exports](/discover_exports) An experimental library for helping to resolve exports from a crate, type aliases, etc, to help with code generation. This crate has a very specific purpose, with only the features needed for this project.
