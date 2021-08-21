use super::*;

use std::collections::HashSet;

// Collisions
impl Tetromino {
    pub fn will_collide_towards(&self, tetrominos: &[Tetromino], direction: MoveDirection) -> bool {
        let dx_dy = Tetromino::get_dxdy(direction);
        tetrominos.iter().any(|t| self.will_collide(t, dx_dy))
    }

    pub fn will_collide_diff(&self, tetrominos: &[Tetromino], dx_dy: [i32; 2]) -> bool {
        tetrominos.iter().any(|t| self.will_collide(t, dx_dy))
    }

    fn will_collide(&self, t: &Tetromino, dx_dy: [i32; 2]) -> bool {
        let mut coords: HashSet<Coord> = HashSet::new();
        self.coords.iter().for_each(|c| {
            coords.insert(Coord {
                x: (c.x as i32 + dx_dy[0]) as u32,
                y: (c.y as i32 + dx_dy[1]) as u32,
            });
        });
        t.coords.iter().any(|c| coords.contains(c))
    }

    pub fn within_boundary(&self, dx_dy: [i32; 2], dim: &Dimensions) -> bool {
        // Are there no cases in which the range does not include the coords ? true : false
        !self.coords.iter().any(|c| {
            !((0..(*dim.w() as i32)).contains(&(c.x as i32 + dx_dy[0]))
                && (0..(dim.h() + 4) as i32).contains(&(c.y as i32 + dx_dy[1])))
        })
    }
}
