use self::palette::Palette;

pub mod palette;

pub struct Game {
    pub palette: Palette,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            palette: Palette::default(),
        }
    }
}
