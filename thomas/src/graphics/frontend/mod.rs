pub mod camera_controller;
pub mod color;
pub mod font;

use std::f32::consts::PI;

use super::backend::{buffers::Vertex, State};
use color::Color;

impl State {
    /// Takes in top left coordinate of square, width, and a `color::Color`
    pub fn draw_square(&mut self, x: f32, y: f32, width: f32, color: Color) {
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

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
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
                position: [x, y + height, 0.0],
                color,
            },
            // bot right, 3
            Vertex {
                position: [x + width, y + height, 0.0],
                color,
            },
        ];

        let indices = &[
            0, 2, 3, // Top triangle
            3, 1, 0, // Bot triangle
        ];

        self.push_shape(vertices, indices);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
        let color = wgpu::Color::from(color);
        let color = [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32,
        ];
        // Get angle of line
        let angle = ((y2 - y1) / (x2 - x1)).atan();
        // Get perpendicular upper angle of line
        let pangle = angle + PI / 2.0;
        let r = thickness / 2.0;
        // Get diffs
        let pdx = pangle.cos() * r;
        let pdy = pangle.sin() * r;

        let vertices = &[
            // Top left, 0
            Vertex {
                position: [x2 + pdx, y2 + pdy, 0.0],
                color,
            },
            // Top right, 1
            Vertex {
                position: [x1 + pdx, y1 + pdy, 0.0],
                color,
            },
            // bot right, 3
            Vertex {
                position: [x2 - pdx, y2 - pdy, 0.0],
                color,
            },
            // Bot left, 2
            Vertex {
                position: [x1 - pdx, y1 - pdy, 0.0],
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
        let len = self.vertices.len() as u16;

        // Not sure which implementation is better/faster
        // indices.iter_mut().map(|i| *i += len);
        // self.state.indices.extend_from_slice(indices);
        // The reason is because while for_each avoids iterating over the
        // array twice, push() might increase/decrease array len
        // Need to benchmark

        indices.iter().for_each(|i| {
            self.indices.push(*i + len);
        });

        self.vertices.extend_from_slice(vertices);
    }

    pub fn clear_background(&mut self, color: color::Color) {
        self.background.clear(wgpu::Color::from(color));
    }
}
