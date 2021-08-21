// Single, double, triple, tetris, based off of gameboy
const SCORE: [u32; 4] = [40, 100, 300, 1200];
// Speeds for levels 3-20, based off of gameboy
const FRAMES_PER_FALL: [u32; 21] = [
    53, 49, 45, 41, 37, 33, 28, 22, 17, 11, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 3,
];
const LVL_CAP: u32 = 20;
const LINES_PER_LEVEL: u32 = 10;

pub struct Game {
    // Internal game tick
    ticks: u32,
    // Game running
    running: bool,
    // Score
    lines_cleared: u32,
    // level
    level: u32,
    // score
    score: u32,
}
impl Game {
    /// Get a reference to the game's running.
    pub fn running(&self) -> &bool {
        &self.running
    }

    /// Get a reference to the universe's ticks.
    pub fn ticks(&self) -> &u32 {
        &self.ticks
    }

    /// Get a mutable reference to the universe's ticks.
    pub fn ticks_mut(&mut self) -> &mut u32 {
        &mut self.ticks
    }

    /// Get a reference to the game's level.
    pub fn level(&self) -> &u32 {
        &self.level
    }

    /// Get a reference to the game's score.
    pub fn score(&self) -> &u32 {
        &self.score
    }
}

impl Game {
    /// Sets running state to false
    pub fn pause(&mut self) {
        self.running = false;
    }
    /// Sets running state to true
    pub fn resume(&mut self) {
        self.running = true;
    }

    /// Updates score, # of lines cleared, and level
    pub fn update(&mut self, lines_cleared: u32) {
        // First update score
        self.update_score(lines_cleared);
        // Next update lines cleared
        self.lines_cleared += lines_cleared;
        // Next update levels, based on lines cleared
        self.update_level();
    }

    /// Should never get called with 0 probably
    fn update_score(&mut self, lines_cleared: u32) {
        // Update score depending on lines cleared
        self.score += SCORE[(lines_cleared - 1) as usize];
    }

    /// Changes level based on self. num of lines cleared
    fn update_level(&mut self) {
        // level goes up every 10 lines, capped at 20
        self.level =
            ((self.lines_cleared as f32 / LINES_PER_LEVEL as f32).floor() as u32).min(LVL_CAP);
    }

    pub fn fast_move_down_score(&mut self) {
        self.score += self.level + 1
    }

    pub fn hard_move_down_score(&mut self, lines: u32) {
        self.score += (self.level + 2) * lines;
    }

    /// Increments tick by 1
    pub fn tick(&mut self) {
        if self.ticks >= FRAMES_PER_FALL[self.level as usize] {
            self.reset_ticks();
        }

        self.ticks += 1;
    }

    /// Resets ticks to 0
    fn reset_ticks(&mut self) {
        self.ticks = 0;
    }

    /// Determines whether to fall tetrimino
    pub fn should_fall(&mut self) -> bool {
        self.ticks % FRAMES_PER_FALL[self.level as usize] == 0
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            ticks: 0,
            running: true,
            lines_cleared: 0,
            level: 0,
            score: 0,
        }
    }
}
