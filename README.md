# TODO

Right now graphics -> context, and wgpu_boilerplate isn't actually just wgpu, it includes winit, which we need to handle keyboard/mouse events. Therefore, we need a struct like the following

```rust
struct Context {
    Graphics {
        (Drawing Methods),
        Camera,
        Backend
    },
    Keyboard,
    Mouse,
}
```

Which entails something like renaming `wgpu-boilerplate` -> `context`, and renaming `state` -> `graphics`, merging the two to have one struct with pub and private methods

TL;DR `Graphics` should never hold `input` lol
