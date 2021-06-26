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
}

impl World {
    fn tick(&mut self) {
        self.entities.iter_mut().for_each(|p| p.tick())
    }
    fn render(&self) {
        self.entities.iter().for_each(|p| p.render())
    }
}

impl Default for World {
    fn default() -> Self {
        let players = vec![Player::default()];
        World { entities: players }
    }
}