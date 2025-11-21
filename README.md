## 🚧 Warning this library is not ready for production, and the API will change
a lot in the coming weeks. Any feedback is appreciated! 🚧

# Overview

`quickgpu` wraps the [wgpu] API allowing users to write shorter, clearer code.
It consists of builders for wgpu structs. As a wrapper library, quickgpu doesn't
manage or own any state after a builder is done building. There's no need to convert
all of your code to quickgpu, you can just use it where it's helpful.


