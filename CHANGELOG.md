## TODOs

- [x] Watch wgpu_glyph for update to wgpu 0.10.1

## Wishes

- [x] Port tetris-rs (via raylib bindings over)
  - [x] Draw a line w/ two long triangles
  - [x] Bare copy into tetris crate
    - [x] Remove all raylib refs
- [x] Add audio support via rodio
  - [x] Add audio to context
  - [x] Implement resource manager

## 0821 SAT

- Implemented full audio support, looping and single sounds, loaded via resource manager
- Cleaned up old methods using `.any()` and `.for_each()`

Could still optimize some more, like generating all hex types for `color` at runtime, animating piece drops, adding screen-level shader effects for tetris drops, but tbh I'm kinda done with tetris moving onto `hometown` now

## 0819 Thu

- Updated wgpu from 0.9 => 0.10.1 (got rid of swap chain)
- Refactored code
  - Separated game and engine into their own crates tetris and thomas
    - Require state struct that implements a trait requiring tick and render, to replace main loop
  - Renamed and merged wgpu_boilerplate with graphics
  - Graphics no longer holds any input() methods
- Following GGEZ API
  - Context holds graphics, keyboard, and window (more to come), which gets passed around to tick() and render() methods
