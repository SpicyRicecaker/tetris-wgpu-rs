use crate::wgpu_boilerplate::{
    buffers::Vertex,
    state::{buffer_queue::Shape, State},
};

pub mod color;

pub struct Graphics {
    pub state: State,
}

impl Graphics {
    pub fn new(state: State) -> Self {
        Graphics { state }
    }
    /// Takes in top left coordinate of square, and width
    pub fn draw_square(&mut self, x: f32, y: f32, width: f32, color: color::Color) {
        // We're allowed to pass in coords straight from our game, since our view matrix
        // will take care of transforming coords (hopefully)
        // Z is always 0 for a 2d game
        let shape = Shape {
            vertices: vec![
                Vertex {
                    position: [x, y, 0.0],
                },
                // Top right, 1
                Vertex {
                    position: [x + width, y, 0.0],
                },
                // Bot left, 2
                Vertex {
                    position: [x, y + width, 0.0],
                },
                // bot right, 3
                Vertex {
                    position: [x + width, y + width, 0.0],
                },
            ],
            indices: vec![
                0, 2, 3, // Top triangle
                3, 1, 0, // Bot triangle
            ],
        };

        self.state.buffer_queue.push_back(shape)
    }
    pub fn clear_background(&mut self, color: color::Color) {
        self.state.background.clear(color.into());
    }
}
