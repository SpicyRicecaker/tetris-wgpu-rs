use crate::graphics::color::Color;

pub struct Palette {
    pub bg: Color,
    pub fg: Color,
    pub j: Color,
    pub l: Color,
    pub s: Color,
    pub t: Color,
    pub z: Color,
    pub i: Color,
    pub o: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            bg: Color::from_hex("211A1E").unwrap(),
            fg: Color::from_hex("3A5683").unwrap(),
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
