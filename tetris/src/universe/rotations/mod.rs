use super::*;
pub mod rotation_direction;

/// Tetrominos of type J, L, S, T or Z each have 5 tests, accounting for each of the 4 indices, each with a cartesion coord
pub const JLSTZ_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [1, 0], [0, 0], [-1, 0]],
    [[0, 0], [1, -1], [0, 0], [-1, -1]],
    [[0, 0], [0, 2], [0, 0], [0, 2]],
    [[0, 0], [1, 2], [0, 0], [-1, 2]],
];

/// Tetromino of type  has 5 tests, each with 4 indices, each with a cartesion coord
pub const I_OFFSET_DATA: [[[i32; 2]; 4]; 5] = [
    [[0, 0], [-1, 0], [-1, 1], [0, 1]],
    [[-1, 0], [0, 0], [1, 1], [0, 1]],
    [[2, 0], [0, 0], [-2, 1], [0, -1]],
    [[-1, 0], [0, 1], [1, 0], [0, -1]],
    [[2, 0], [0, -2], [-2, 0], [0, 2]],
];
pub const O_OFFSET_DATA: [[[i32; 2]; 4]; 1] = [[[0, 0], [0, -1], [-1, -1], [-1, 0]]];

impl Universe {
    pub fn rotate_focused(&mut self, rot_direction: RotationDirection) {
        let center_x = self.focused_tetromino.coords()[0].x;
        let center_y = self.focused_tetromino.coords()[0].y;

        let (next_index_diff, m) = match rot_direction {
            RotationDirection::Clockwise => (1, [[0, -1], [1, 0]]),
            RotationDirection::CounterClockwise => (-1, [[0, 1], [-1, 0]]),
        };

        for i in 1..self.focused_tetromino.coords().len() {
            let t = &mut self.focused_tetromino.coords_mut()[i];

            // Get the original coords by subtracting the origin
            // e.g. (1, 1), (1, 0), etc.
            let x = t.x as i32 - center_x as i32;
            let y = t.y as i32 - center_y as i32;
            // Rotate the coords 90 degrees to the left

            let f_x = x * m[0][0] + y * m[1][0];
            let f_y = x * m[0][1] + y * m[1][1];

            // Add the coords back
            t.x = (f_x + center_x as i32) as u32;
            t.y = (f_y + center_y as i32) as u32;
        }

        let offset_data = match self.focused_tetromino.tetromino_type() {
            TetrominoType::J
            | TetrominoType::L
            | TetrominoType::S
            | TetrominoType::T
            | TetrominoType::Z => &JLSTZ_OFFSET_DATA[..],
            TetrominoType::I => &I_OFFSET_DATA[..],
            TetrominoType::O => &O_OFFSET_DATA[..],
        };

        // Try all of the 5 test cases
        for test in offset_data {
            let current_set = test[*self.focused_tetromino().rotation_state().rn() as usize];
            let new_set = test[self
                .focused_tetromino()
                .rotation_state()
                .get_increment(next_index_diff) as usize];
            // Checkout <https://harddrop.com/wiki/SRS#How_Guideline_SRS_Really_Works> for more information on how the offset wallkicks are derived
            // Current - Next
            let dx_dy = [current_set[0] - new_set[0], current_set[1] - new_set[1]];

            // Test collisions
            // First make sure it's in boundaries
            if Tetromino::within_boundary(&self.focused_tetromino, dx_dy, &self.dim)
                && !Tetromino::will_collide_all(
                    &self.focused_tetromino,
                    &self.stagnant_tetrominos,
                    dx_dy,
                )
            {
                // Move tetrimino
                self.focused_tetromino_mut().move_by(dx_dy);
                // Update indice
                self.focused_tetromino_mut()
                    .rotation_state_mut()
                    .increment(next_index_diff);
                // Otherwise need to rotate back
                return;
            }
        }

        // Just rotate back if there is conflict, will show up as nothing happened
        // Good place to add sound as well
        self.rotate_focused(RotationDirection::flip(rot_direction));
    }

}
