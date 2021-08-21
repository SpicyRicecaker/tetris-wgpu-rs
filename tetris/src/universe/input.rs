use thomas::context::Context;
use thomas::winit::event::VirtualKeyCode;

use crate::tetris_input::InputInterface;

use super::Universe;
use super::rotations::rotation_direction::RotationDirection;
use super::tetromino::{Tetromino, MoveDirection};

impl InputInterface for Universe {
    fn receive_key(&mut self, ctx: &mut Context) {
        for i in 0..self.tetromino_controls.get_queue().len() {
            match self.tetromino_controls.get_queue()[i] {
                VirtualKeyCode::Left => {
                    let dx_dy = Tetromino::get_dxdy(MoveDirection::Left);
                    if self.focused_tetromino.within_boundary(dx_dy, &self.dim)
                        && !self
                            .focused_tetromino
                            .will_collide_diff(&self.stagnant_tetrominos, dx_dy)
                    {
                        self.focused_tetromino.move_by(dx_dy)
                    }
                }
                VirtualKeyCode::Right => {
                    let dx_dy = Tetromino::get_dxdy(MoveDirection::Right);
                    if self.focused_tetromino.within_boundary(dx_dy, &self.dim)
                        && !self
                            .focused_tetromino
                            .will_collide_diff(&self.stagnant_tetrominos, dx_dy)
                    {
                        self.focused_tetromino.move_by(dx_dy)
                    }
                }
                VirtualKeyCode::Down => {
                    self.fall_focused(ctx);
                    self.game.fast_move_down_score()
                }
                VirtualKeyCode::Z => self.rotate_focused(RotationDirection::CounterClockwise),
                VirtualKeyCode::C => self.rotate_focused(RotationDirection::Clockwise),
                VirtualKeyCode::Space => {
                    let lines = self.focused_tetromino.coords()[0].y - self.ghost.coords()[0].y;
                    self.focused_tetromino = self.ghost.clone();
                    self.fall_focused(ctx);
                    self.game.hard_move_down_score(lines);
                }
                _ => {}
            }
        }
        self.tetromino_controls.clear_queue();
    }
}
