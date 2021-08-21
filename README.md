# Tetris-wgpu-rs

A port of [raylib-tetris](https://github.com/SpicyRicecaker/tetris-rs) -> [wgpu](https://github.com/gfx-rs/wgpu), also using [rodio](https://github.com/RustAudio/rodio) for audio. Api inspiration for `thomas engine` from [ggez](https://github.com/ggez/ggez).

![Picture of Tetris Game Over Screen](https://raw.githubusercontent.com/SpicyRicecaker/tetris-wgpu-rs/master/tetris/resources/game_over.jpg)

## Implementation

Board: Used a vector of tetrominos that hold their own coords rather than a global array (for some reason), with width and height just for rendering  
Tetrominos: Struct of vector of coords, with the center being the first coord in the vector  
Collisions: Comparing current focused tetromino with every other tetromino on the board  
Rotations: Used an [offset table](https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works) with indices to center `O` and `I` tetromino rotations as well as take care of wallkicks

## Helpful Resources

[How to Properly Rotate Tetris Pieces - Game Development Tutorial](https://www.youtube.com/watch?v=yIpk5TJ_uaI&t=1235s) A video explaining how to implement tetromino rotations by Turbo Makes Games

- [SRS](https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works) website mentioned in the above video with offset data and explanations for how to use it

Learned basic wgpu api from [learn-wgpu](https://sotrh.github.io/learn-wgpu/), projections from, [learnopengl](https://learnopengl.com/), batch rendering and engine design from [The Cherno](https://www.youtube.com/playlist?list=PLlrATfBNZ98f5vZ8nJ6UengEkZUMC4fy5)

## Running

Install nightly rust compiler

```rust
rustup install nightly
```

Clone repo & build run

```bash
git clone https://github.com/SpicyRicecaker/tetris-wgpu-rs/tree/master
cd tetris-wgpu-rs
cargo run --release
```
