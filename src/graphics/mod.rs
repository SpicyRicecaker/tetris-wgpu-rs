use crate::wgpu_boilerplate::{buffers::Vertex, state::State};

pub mod color;

pub struct Graphics {
    pub state: State,
}

impl Graphics {
    pub fn new(state: State) -> Self {
        Graphics { state }
    }
    /// Takes in top left coordinate of square, width, and a `color::Color`
    pub fn draw_square(&mut self, x: f32, y: f32, width: f32, color: color::Color) {
        let color = wgpu::Color::from(color);
        let color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        // We're allowed to pass in coords straight from our game, since our view matrix
        // will take care of transforming coords

        // Z is always 0 for a 2d game
        let vertices = &[
            // Top left, 0
            Vertex {
                position: [x, y, 0.0],
                color,
            },
            // Top right, 1
            Vertex {
                position: [x + width, y, 0.0],
                color,
            },
            // Bot left, 2
            Vertex {
                position: [x, y + width, 0.0],
                color,
            },
            // bot right, 3
            Vertex {
                position: [x + width, y + width, 0.0],
                color,
            },
        ];

        let indices = &[
            0, 2, 3, // Top triangle
            3, 1, 0, // Bot triangle
        ];

        self.push_shape(vertices, indices);
    }

    /// Pushes a shape into the vector of shapes. These shapes are copied into the vertex and index buffer
    /// in the `render()` function, to be batch rendered.
    /// Internally, updates `num_indices` and `num_vertices`, as well as converts `indices` on shape based off of previous `num_indices`
    pub fn push_shape(&mut self, vertices: &[Vertex], indices: &[u16]) {
        let len = self.state.vertices.len() as u16;

        // Not sure which implementation is better/faster
        // indices.iter_mut().map(|i| *i += len);
        // self.state.indices.extend_from_slice(indices);
        // The reason is because while for_each avoids iterating over the
        // array twice, push() might increase/decrease array len
        // Need to benchmark

        indices.iter().for_each(|i| {
            self.state.indices.push(*i + len);
        });

        self.state.vertices.extend_from_slice(vertices);
    }

    pub fn clear_background(&mut self, color: color::Color) {
        self.state.background.clear(wgpu::Color::from(color));
    }
}
