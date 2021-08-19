pub struct Config {
    ticks: u32,
    w: f32,
    h: f32,
    title: String,
    actual_w: f32,
    canvas_l: f32,
    canvas_r: f32,
}

impl Config {
    pub fn new(ticks: u32, w: f32, h: f32, title: String) -> Self {
        let actual_w = w * (9.0 / 32.0);
        let canvas_l = (w - actual_w) / 2.0;
        let canvas_r = canvas_l + actual_w;

        Config {
            ticks,
            w,
            h,
            title,
            actual_w,
            canvas_l,
            canvas_r,
        }
    }

    /// Get a reference to the config's ticks
    pub fn ticks(&self) -> &u32 {
        &self.ticks
    }

    /// Get a reference to the config's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get a reference to the config's h.
    pub fn h(&self) -> &f32 {
        &self.h
    }

    /// Get a reference to the config's w
    pub fn w(&self) -> &f32 {
        &self.w
    }

    /// Get a reference to the config's actual w.
    pub fn actual_w(&self) -> &f32 {
        &self.actual_w
    }

    /// Get a reference to the config's canvas l.
    pub fn canvas_l(&self) -> &f32 {
        &self.canvas_l
    }

    /// Get a reference to the config's canvas r.
    pub fn canvas_r(&self) -> &f32 {
        &self.canvas_r
    }
}
impl Default for Config {
    fn default() -> Self {
        Self::new(60, 1600.0, 900.0, String::from("Tetris"))
    }
}
