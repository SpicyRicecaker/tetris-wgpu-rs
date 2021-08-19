use super::*;

use std::collections::HashSet;

// Collisions
impl Tetromino {
    pub fn will_collide_all(
        t: &Tetromino,
        stagnant_tetrominos: &[Tetromino],
        dxdy: [i32; 2],
    ) -> bool {
        for stagnant_tetromino in stagnant_tetrominos {
            if Tetromino::will_collide(t, stagnant_tetromino, dxdy[0], dxdy[1]) {
                return true;
            }
        }
        false
    }

    fn will_collide(f: &Tetromino, s: &Tetromino, dx: i32, dy: i32) -> bool {
        let mut coords: HashSet<Coord> = HashSet::new();
        for f_coord in f.coords.iter() {
            coords.insert(Coord {
                x: (f_coord.x as i32 + dx) as u32,
                y: (f_coord.y as i32 + dy) as u32,
            });
        }
        for s_coord in s.coords.iter() {
            if coords.contains(s_coord) {
                return true;
            }
        }
        false
    }

    pub fn within_boundary(&self, dx_dy: [i32; 2], dim: &Dimensions) -> bool {
        for coord in self.coords.iter() {
            if !(0..(*dim.w() as i32)).contains(&(coord.x as i32 + dx_dy[0]))
                || !(0..(dim.h() + 4) as i32).contains(&(coord.y as i32 + dx_dy[1]))
            {
                return false;
            }
        }
        true
    }
}
