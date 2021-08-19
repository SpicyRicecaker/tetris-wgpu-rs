#[cfg(test)]
mod test {
    mod movement {
        use crate::{
            tetromino::{
                coord::Coord, direction::Direction, tetromino_type::TetrominoType, Tetromino,
            },
            universe::Universe,
        };

        #[test]
        fn test_move_down() {
            let mut tetromino = Tetromino::spawn_tetromino(
                vec![
                    Coord::new(1, 0),
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 22),
                TetrominoType::T,
            );
            tetromino.move_by(Tetromino::get_dxdy(Direction::Down));

            let right_real_coords = vec![
                Coord { x: 4, y: 21 },
                Coord { x: 5, y: 22 },
                Coord { x: 5, y: 21 },
                Coord { x: 6, y: 21 },
            ];

            dbg!(&right_real_coords, tetromino.coords());

            for idx in (0..4).into_iter() {
                assert_eq!(right_real_coords.get(idx), tetromino.coords().get(idx))
            }
        }
        #[test]
        fn test_move_down_3() {
            let mut tetromino = Tetromino::spawn_tetromino(
                vec![
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(1, 0),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 22),
                TetrominoType::T,
            );
            tetromino.move_by(Tetromino::get_dxdy(Direction::Left));
            tetromino.move_by(Tetromino::get_dxdy(Direction::Left));
            tetromino.move_by(Tetromino::get_dxdy(Direction::Left));

            let right_real_coords = vec![
                Coord { x: 4, y: 19 },
                Coord { x: 5, y: 20 },
                Coord { x: 5, y: 19 },
                Coord { x: 6, y: 19 },
            ];

            dbg!(&right_real_coords, tetromino.coords());

            for idx in (0..4).into_iter() {
                assert_eq!(right_real_coords.get(idx), tetromino.coords().get(idx))
            }
        }

        #[test]
        fn test_increment_arr() {
            let mut arr = [0_u32; 5];
            let test = [0, 1, 1, 1, 1];
            Universe::change_arr_from_idx(&mut arr, 1, 1);
            assert_eq!(arr, test);
            dbg!(arr);
        }
    }

    mod spawn {
        use crate::{
            tetromino::{
                coord::Coord, direction::Direction, tetromino_type::TetrominoType, Tetromino,
            },
            universe::Dimensions,
        };

        #[test]
        fn test_t_tetromino_spawn() {
            let tetromino = Tetromino::spawn_tetromino(
                vec![
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(1, 0),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 22),
                TetrominoType::T,
            );
            let right_real_coords = vec![
                Coord { x: 4, y: 22 },
                Coord { x: 5, y: 23 },
                Coord { x: 5, y: 22 },
                Coord { x: 6, y: 22 },
            ];
            dbg!(&right_real_coords, tetromino.coords());

            for idx in (0..4).into_iter() {
                assert_eq!(right_real_coords.get(idx), tetromino.coords().get(idx))
            }
        }
        #[test]
        fn test_boundary_positive() {
            let tetromino = Tetromino::spawn_tetromino(
                vec![
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(1, 0),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 10),
                TetrominoType::T,
            );
            assert!(tetromino.within_boundary(
                Tetromino::get_dxdy(Direction::Down),
                &Dimensions { w: 10, h: 20 }
            ));
        }

        #[test]
        fn test_boundary_negative() {
            let tetromino = Tetromino::spawn_tetromino(
                vec![
                    Coord::new(0, 0),
                    Coord::new(1, 1),
                    Coord::new(1, 0),
                    Coord::new(2, 0),
                ],
                Coord::new(5, 0),
                TetrominoType::T,
            );
            assert!(tetromino.within_boundary(
                Tetromino::get_dxdy(Direction::Down),
                &Dimensions { w: 10, h: 20 }
            ));
        }
    }
}
