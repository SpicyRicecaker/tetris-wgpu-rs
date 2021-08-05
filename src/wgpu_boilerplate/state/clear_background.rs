pub struct Background {
    color: wgpu::Color,
    should_clear: bool,
}

impl Background {
    pub fn clear(&mut self, color: wgpu::Color) {
        self.should_clear = true;
        self.color = color;
    }
    pub fn reset(&mut self) {
        self.should_clear = false;
    }
}

impl Default for Background {
    fn default() -> Self {
        Background {
            color: wgpu::Color::BLACK,
            should_clear: false,
        }
    }
}
