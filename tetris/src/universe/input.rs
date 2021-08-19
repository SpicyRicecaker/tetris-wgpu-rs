use thomas::winit::event::VirtualKeyCode;

use super::rotations::rotation_direction::RotationDirection;
use super::{direction::*, Tetromino};
use super::{InputInterface, Universe};

impl InputInterface for Universe {
    fn receive_key(&mut self) {
        for i in 0..self.tetromino_controls.get_queue().len() {
            match self.tetromino_controls.get_queue()[i] {
                VirtualKeyCode::Left => {
                    let dxdy = Tetromino::get_dxdy(Direction::Left);
                    if self.focused_tetromino.within_boundary(dxdy, &self.dim)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            dxdy,
                        )
                    {
                        self.focused_tetromino.move_by(dxdy)
                    }
                }
                VirtualKeyCode::Right => {
                    let dxdy = Tetromino::get_dxdy(Direction::Right);
                    if self.focused_tetromino.within_boundary(dxdy, &self.dim)
                        && !Tetromino::will_collide_all(
                            &self.focused_tetromino,
                            &self.stagnant_tetrominos,
                            dxdy,
                        )
                    {
                        self.focused_tetromino.move_by(dxdy)
                    }
                }
                VirtualKeyCode::Down => {
                    self.fall_focused();
                    self.game.fast_move_down_score()
                }
                VirtualKeyCode::Z => self.rotate_focused(RotationDirection::CounterClockwise),
                VirtualKeyCode::C => self.rotate_focused(RotationDirection::Clockwise),
                VirtualKeyCode::Space => {
                    let lines = self.focused_tetromino.coords()[0].y - self.ghost.coords()[0].y;
                    self.focused_tetromino = self.ghost.clone();
                    self.fall_focused();
                    self.game.hard_move_down_score(lines);
                }
                _ => {}
            }
        }
        self.tetromino_controls.clear_queue();
    }
}
