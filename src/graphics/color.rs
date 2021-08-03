pub struct Color { 
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32
}

impl Color {
    fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        Color {
            r,
            g, 
            b, 
            a
        }
    }
}