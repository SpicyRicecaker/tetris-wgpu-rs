const LETTERBOX_RATIO: f32 = 9.0 / 32.0;

pub struct Dimensions {
    w: f32,
    h: f32,
    actual_w: f32,
    canvas_l: f32,
    canvas_r: f32,
}

impl Dimensions {
    fn new(w: f32, h: f32) -> Self {
        // Actual tetris board is the area left after taking away the letterbox ratio
        let actual_w = w * LETTERBOX_RATIO;
        // Calculate the postiion at the left point at which the board starts
        let canvas_l = (w - actual_w) / 2.0;
        // Calculate the postiion at the right point at which the board ends
        let canvas_r = canvas_l + actual_w;
        Self {
            w, 
            h,
            actual_w,
            canvas_l,
            canvas_r
        }
    }

}

pub struct Config {
    ticks: u32,
    title: String,
    pub dimensions: Dimensions,
}

impl Config {
    pub fn new(ticks: u32, w: f32, h: f32, title: String) -> Self {
        let dimensions = Dimensions::new(w, h);
        Config {
            ticks,
            title,
            dimensions
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

    /// Resizes the board
    pub fn resize(&mut self, w: f32, h: f32) {
        self.dimensions = Dimensions::new(w, h);
    }

    /// Get a reference to the config's h.
    pub fn h(&self) -> &f32 {
        &self.dimensions.h
    }

    /// Get a reference to the config's w
    pub fn w(&self) -> &f32 {
        &self.dimensions.w
    }

    /// Get a reference to the config's actual w.
    pub fn actual_w(&self) -> &f32 {
        &self.dimensions.actual_w
    }

    /// Get a reference to the config's canvas l.
    pub fn canvas_l(&self) -> &f32 {
        &self.dimensions.canvas_l
    }

    /// Get a reference to the config's canvas r.
    pub fn canvas_r(&self) -> &f32 {
        &self.dimensions.canvas_r
    }

}
impl Default for Config {
    fn default() -> Self {
        Self::new(60, 1600.0, 900.0, String::from("Tetris"))
    }
}
