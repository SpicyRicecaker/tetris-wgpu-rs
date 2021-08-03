use crate::wgpu_boilerplate::{buffers::Vertex, state::State};

mod color;

struct Graphics {
    state: State,
}

impl Graphics {
    /// Takes in top left coordinate of square, and width
    fn draw_square(&mut self, x: f32, y: f32, width: f32, color: color::Color) {
        // We're allowed to pass in coords straight from our game, since our view matrix
        // will take care of transforming coords (hopefully)
        // Z is always 0 for a 2d game

        // convert (x, y) -> (-1, 1) (-1, 1)
        let vertices: &[Vertex] = &[
            // Top left, 1
            Vertex {
                position: [x, y, 0.0],
            },
            // Top right, 2
            Vertex {
                position: [x + width, y, 0.0],
            },
            // Bot left, 3
            Vertex {
                position: [x, y + width, 0.0],
            },
            // bot right, 4
            Vertex {
                position: [x + width, y + width, 0.0],
            },
        ];
        let indices: &[u16] = &[
            1, 2, 3, // Top triangle
            2, 3, 4, // Bot triangle
            0, // padding
        ];

        let indices = self.state.update_buffer(vertices);
    }
}
