pub struct Background {
    pub color: wgpu::Color,
    pub should_clear: bool,
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
            color: wgpu::Color::TRANSPARENT,
            should_clear: false,
        }
    }
}
