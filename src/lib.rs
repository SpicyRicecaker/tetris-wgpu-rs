use wgpu_boilerplate::state;
use winit::event::MouseScrollDelta;

pub mod wgpu_boilerplate;

struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn new(x: u32, y: u32) -> Self {
        Coord { x, y }
    }
}

struct Player {
    location: Coord,
    thiccness: u32,
}

impl Player {
    fn tick(&mut self) {
        // If key pressed move
    }
    fn render(&self) {
        // Draw a square at this pos

        // drawSquare(x1, y1, x2, y2);
    }
}

impl Default for Player {
    fn default() -> Self {
        let location = Coord::new(10, 10);
        let thiccness = 5;
        Player {
            location,
            thiccness,
        }
    }
}

struct Config {
    fps: u32,
    w: u32,
    h: u32,
}
pub struct World {
    entities: Vec<Player>,
    scroll_pos: f64,
    cursor_pos: (f64, f64),
}
enum Direction {
    Up,
    Down,
}

impl World {
    fn change_scroll_pos(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.scroll_pos = (self.scroll_pos + 0.1).min(1.0),
            Direction::Down => self.scroll_pos = (self.scroll_pos - 0.1).max(0.0),
        }
    }
    pub fn handle_scroll(&mut self, delta: &MouseScrollDelta) {
        let direction = match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                if *y > 0_f32 {
                    Direction::Up
                } else if *y < 0_f32 {
                    Direction::Down
                } else {
                    return;
                }
            }
            MouseScrollDelta::PixelDelta(pos) => {
                if pos.y > 0_f64 {
                    Direction::Up
                } else if pos.y < 0_f64 {
                    Direction::Down
                } else {
                    return;
                }
            }
        };
        self.change_scroll_pos(direction);
    }
    fn tick(&mut self) {
        // self.entities.iter_mut().for_each(|p| p.tick())
    }
    pub fn render(&self, state: &mut state::State) {
        // self.entities.iter().for_each(|p| p.render())

        // state.render().unwrap();
        self.render_mouse(state);


    }

    fn render_mouse(&self, state: &mut state::State) {
        // Get the fraction of scroll value
        let x_fraction = self.cursor_pos.0 / state.size().width as f64;
        let y_fraction = self.cursor_pos.1 / state.size().height as f64;
        let scroll_fraction = self.scroll_pos / 1.0;

        state
            .render_background(x_fraction, y_fraction, scroll_fraction, 1.0_f64)
            .unwrap();
    }

    /// Get a mutable reference to the world's cursor pos.
    pub fn cursor_pos_mut(&mut self) -> &mut (f64, f64) {
        &mut self.cursor_pos
    }

    /// Get a mutable reference to the world's scroll pos.
    pub fn scroll_pos_mut(&mut self) -> &mut f64 {
        &mut self.scroll_pos
    }
}

impl Default for World {
    fn default() -> Self {
        let players = vec![Player::default()];
        World {
            entities: players,
            scroll_pos: (0.5_f64),
            cursor_pos: (50_f64, 50_f64),
        }
    }
}