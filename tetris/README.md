# Tetris-rs
Written in rust, with the [raylib-rs](https://github.com/deltaphc/raylib-rs) bindings for the [raylib](https://github.com/raysan5/raylib) library. 

![Picture of Tetris Game Over Screen](https://raw.githubusercontent.com/SpicyRicecaker/tetris-rs/master/assets/game_over.jpg)

## Implementation
Board: Used a vector of tetrominos that hold their own coords rather than a global array (for some reason), with width and height just for rendering  
Tetrominos: Struct of vector of coords, with the center being the first coord in the vector  
Collisions: Comparing current focused tetromino with every other tetromino on the board   
Rotations: Used an [offset table](https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works) with indices to center `O` and `I` tetromino rotations as well as take care of wallkicks  
## Helpful Resources
[How to Properly Rotate Tetris Pieces - Game Development Tutorial](https://www.youtube.com/watch?v=yIpk5TJ_uaI&t=1235s) A video explaining how to implement tetromino rotations by Turbo Makes Games
- [SRS](https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works) website mentioned in the above video with offset data and explanations for how to use it
