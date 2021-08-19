use thomas::graphics::frontend::color::Color;

use crate::tetromino::tetromino_type::TetrominoType;

pub struct ColorPalette {
    line: Color,
    grid: Color,
    j: Color,
    l: Color,
    s: Color,
    t: Color,
    z: Color,
    i: Color,
    o: Color,
}

impl ColorPalette {
    pub fn color_for(&self, tetrimino_type: TetrominoType) -> Color {
        match tetrimino_type {
            TetrominoType::I => self.i,
            TetrominoType::J => self.j,
            TetrominoType::L => self.l,
            TetrominoType::O => self.o,
            TetrominoType::S => self.s,
            TetrominoType::T => self.t,
            TetrominoType::Z => self.z,
        }
    }

    /// Get a color palette's line color.
    pub fn line(&self) -> Color {
        self.line
    }

    /// Get a color palette's grid color.
    pub fn grid(&self) -> Color {
        self.grid
    }

}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette {
            grid: Color::from_hex("211A1E").unwrap(),
            line: Color::from_hex("3A5683").unwrap(),
            j: Color::from_hex("5BC0EB").unwrap(),
            l: Color::from_hex("FDE74C").unwrap(),
            s: Color::from_hex("9BC53D").unwrap(),
            t: Color::from_hex("C3423F").unwrap(),
            z: Color::from_hex("4C6085").unwrap(),
            i: Color::from_hex("34344A").unwrap(),
            o: Color::from_hex("D4BEBE").unwrap(),
        }
    }
}
