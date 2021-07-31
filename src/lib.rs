use event::{ElementState, VirtualKeyCode, WindowEvent};
use wgpu_boilerplate::state;
use winit::event;

pub mod wgpu_boilerplate;

#[derive(Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn new(x: u32, y: u32) -> Self {
        Coord { x, y }
    }
}

pub struct Player {
    pub location: Coord,
    velocity: u32,
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
        let location = Coord::new(900, 900);
        let thiccness = 5;
        let velocity = 10;
        Player {
            location,
            thiccness,
            velocity,
        }
    }
}

pub struct Controller {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
impl Default for Controller {
    fn default() -> Self {
        Controller {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
impl Controller {
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Up => {
                        self.up = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.down = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.left = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.right = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

struct Config {
    fps: u32,
    w: u32,
    h: u32,
}
pub struct World {
    pub player: Player,
    pub controller: Controller,
}
enum Direction {
    Up,
    Down,
}

impl World {
    pub fn tick(&mut self) {
        if self.controller.up {
            self.player.location.y += self.player.velocity;
        }
        if self.controller.down {
            self.player.location.y -= self.player.velocity;
        }
        if self.controller.left {
            self.player.location.x -= self.player.velocity;
        }
        if self.controller.right {
            self.player.location.x += self.player.velocity;
        }
    }
    pub fn render(&self, state: &mut state::State) {
        state.render().unwrap();
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            player: Player::default(),
            controller: Controller::default(),
        }
    }
}
