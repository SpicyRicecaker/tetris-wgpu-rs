## TODOs

- [ ] Watch wgpu_glyph for update to wgpu 0.10.1

## Wishes

- [ ] Port tetris-rs (via raylib bindings over)
    - [ ] Draw a line w/ two long triangles
    - [ ] Bare copy into tetris crate
        - [ ] Remove all raylib refs
- [ ] Add audio support via rodio
    - [ ] Add audio to context
    - [ ] Implement resource manager


## 0819 Thu

- Updated wgpu from 0.9 => 0.10.1 (got rid of swap chain)
- Refactored code
    - Separated game and engine into their own crates tetris and thomas
        - Require state struct that implements a trait requiring tick and render, to replace main loop
    - Renamed and merged wgpu_boilerplate with graphics
    - Graphics no longer holds any input() methods
- Following GGEZ API
    - Context holds graphics, keyboard, and window (more to come), which gets passed around to tick() and render() methods
