pub mod circular_num;
pub mod collisions;
pub mod coord;
pub mod tetromino_type;

use circular_num::*;
use coord::*;
use tetromino_type::*;
use thomas::context::Context;

use super::universe::{color::ColorPalette, Dimensions};
use super::Config;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}
/// Built off tetromino coords
#[derive(Clone)]
pub struct Tetromino {
    coords: Vec<Coord>,
    tetromino_type: TetrominoType,
    rotation_state: CircularNum,
}

impl Tetromino {
    /// Generates a tetromino, given a set of coords, a type
    /// The center of the tetromino, as well as the location it should be spawned in
    pub fn spawn_tetromino(
        // List of coords
        reference_coords: Vec<Coord>,
        // Real center is where to spawn the tetromino
        spawn_coords: Coord,
        // Type of tetromino
        tetromino_type: TetrominoType,
    ) -> Tetromino {
        // Generate real coords from reference coords
        let coords = reference_coords
            .iter()
            .map(|coord| {
                let dx: i32 = coord.x as i32 - reference_coords[0].x as i32;
                let dy: i32 = coord.y as i32 - reference_coords[0].y as i32;
                Coord::new(
                    (spawn_coords.x as i32 + dx) as u32,
                    (spawn_coords.y as i32 + dy) as u32,
                )
            })
            .collect();

        Tetromino {
            coords,
            tetromino_type,
            rotation_state: CircularNum::default(),
        }
    }

    /// Gives true pixel value,
    /// since graphics use 4th quadrant instead of 1st
    pub fn reversed_coord_y(canvas_y: u32, coord_y: u32, dy: u32) -> i32 {
        (canvas_y - (coord_y * dy)) as i32
    }

    pub fn render(
        &self,
        ctx: &mut Context,
        config: &Config,
        dim: &Dimensions,
        color_palette: &ColorPalette,
    ) {
        let dy = config.h() / *dim.h() as f32;
        let dx = config.actual_w() / *dim.w() as f32;

        // For every coord in the tetromino (4 coords in total)
        for (_idx, coord) in self.coords.iter().enumerate() {
            if coord.y >= *dim.h() {
                continue;
            }
            // Figure out what this means in terms of real coords
            ctx.graphics.draw_rectangle(
                config.canvas_l() + coord.x as f32 * dx,
                config.h() - (coord.y as f32 + 1.0) * dy,
                dx,
                dy,
                color_palette.color_for(self.tetromino_type),
            )
        }
    }

    pub fn render_alpha(
        &self,
        ctx: &mut Context,
        config: &Config,
        dim: &Dimensions,
        color_palette: &ColorPalette,
    ) {
        let dy: f32 = config.h() / *dim.h() as f32;
        let dx: f32 = config.actual_w() / *dim.w() as f32;

        // For every coord in the tetromino (4 coords in total)
        for (_idx, coord) in self.coords.iter().enumerate() {
            if coord.y >= *dim.h() {
                continue;
            }
            // Figure out what this means in terms of real coords
            ctx.graphics.draw_rectangle(
                config.canvas_l() + coord.x as f32 * dx,
                config.h() - (coord.y as f32 + 1.0) * dy,
                dx,
                dy,
                color_palette.color_for(self.tetromino_type).fade(0.4),
            )
        }
    }

    pub fn get_dxdy(direction: MoveDirection) -> [i32; 2] {
        match direction {
            MoveDirection::Down => [0, -1],
            MoveDirection::Up => [0, 1],
            MoveDirection::Left => [-1, 0],
            MoveDirection::Right => [1, 0],
            MoveDirection::None => [0, 0],
        }
    }

    pub fn move_by(&mut self, dx_dy: [i32; 2]) {
        // Moves all real coords
        self.coords.iter_mut().for_each(|c| {
            c.x = (c.x as i32 + dx_dy[0]) as u32;
            c.y = (c.y as i32 + dx_dy[1]) as u32;
        });
    }
}

// Getters and setters
impl Tetromino {
    /// Get a mutable reference to the tetromino's rotation state.
    pub fn rotation_state_mut(&mut self) -> &mut CircularNum {
        &mut self.rotation_state
    }

    /// Get a reference to the tetromino's tetromino type.
    pub fn tetromino_type(&self) -> &TetrominoType {
        &self.tetromino_type
    }

    /// Get a reference to the tetromino's rotation state.
    pub fn rotation_state(&self) -> &CircularNum {
        &self.rotation_state
    }

    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }
    pub fn coords_mut(&mut self) -> &mut Vec<Coord> {
        &mut self.coords
    }
}
