#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

mod spawn {
    use super::super::*;
    use rand::{distributions::Standard, prelude::Distribution, Rng};

    const SPAWN_Y: u32 = 21;
    const JLSTZ_SPAWN_X: u32 = 4;
    const IO_SPAWN_X: u32 = 3;

    impl Distribution<TetrominoType> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
            match rng.gen_range(0..7) {
                0 => TetrominoType::I,
                1 => TetrominoType::J,
                2 => TetrominoType::L,
                3 => TetrominoType::O,
                4 => TetrominoType::S,
                5 => TetrominoType::T,
                6 => TetrominoType::Z,
                _ => {
                    panic!()
                }
            }
        }
    }

    impl TetrominoType {
        pub fn generate_tetromino_rand() -> Tetromino {
            TetrominoType::generate_tetromino_from_type(rand::random())
        }
        /// Function that takes in a tetromino type and returns a spawned tetromino
        /// Important to realize that the first index of reference coords are the center of the tetromino
        /// Doesn't make any sense rn because it's not dependent on board width/height
        pub fn generate_tetromino_from_type(tetromino_type: TetrominoType) -> Tetromino {
            let (reference_coords, spawn_coords) = match tetromino_type {
                TetrominoType::I => (
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(2, 0),
                        Coord::new(3, 0),
                    ],
                    Coord::new(IO_SPAWN_X, SPAWN_Y),
                ),
                TetrominoType::J => (
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(2, 0),
                        Coord::new(0, 1),
                    ],
                    Coord::new(JLSTZ_SPAWN_X, SPAWN_Y),
                ),
                TetrominoType::L => (
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(2, 0),
                        Coord::new(2, 1),
                    ],
                    Coord::new(JLSTZ_SPAWN_X, SPAWN_Y),
                ),
                TetrominoType::O => (
                    vec![
                        Coord::new(0, 0),
                        Coord::new(1, 0),
                        Coord::new(0, 1),
                        Coord::new(1, 1),
                    ],
                    Coord::new(IO_SPAWN_X, SPAWN_Y),
                ),
                TetrominoType::S => (
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(1, 1),
                        Coord::new(2, 1),
                    ],
                    Coord::new(JLSTZ_SPAWN_X, SPAWN_Y),
                ),
                TetrominoType::T => (
                    vec![
                        Coord::new(1, 0),
                        Coord::new(0, 0),
                        Coord::new(1, 1),
                        Coord::new(2, 0),
                    ],
                    Coord::new(JLSTZ_SPAWN_X, SPAWN_Y),
                ),
                TetrominoType::Z => (
                    vec![
                        Coord::new(1, 0),
                        Coord::new(2, 0),
                        Coord::new(0, 1),
                        Coord::new(1, 1),
                    ],
                    Coord::new(5, 22),
                ),
            };
            Tetromino::spawn_tetromino(reference_coords, spawn_coords, tetromino_type)
        }
    }
}
