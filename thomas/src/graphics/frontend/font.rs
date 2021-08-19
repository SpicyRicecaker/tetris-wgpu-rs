use super::State;
use wgpu_glyph::{ab_glyph, Section, Text};

impl State {
    pub fn load_font(&mut self, path: &str) -> Result<(), std::io::Error> {
        let buffer = std::fs::read(path)?;
        let font = ab_glyph::FontArc::try_from_vec(buffer).unwrap();
        self.font_interface.add_font(font);

        Ok(())
    }
    #[inline]
    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, color: wgpu::Color, scale: f32) {
        self.font_interface.queue(Section {
            screen_position: (x, y),
            text: vec![Text::new(text)
                .with_color([
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ])
                .with_scale(scale)],
            ..Section::default()
        });
    }
}
