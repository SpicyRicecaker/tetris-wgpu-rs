use std::ops::Range;

use event::{ElementState, VirtualKeyCode, WindowEvent};
use rand::Rng;
use wgpu::Color;
use wgpu_boilerplate::{
    buffers::Vertex,
    state::{self, State},
};
use winit::event;

pub mod wgpu_boilerplate;
pub mod graphics;

pub const MARGIN: f32 = 100.0;
pub const WORLD_WIDTH: f32 = 1920.0 - MARGIN;
pub const WORLD_HEIGHT: f32 = 1080.0 - MARGIN;

#[derive(Debug)]
pub struct Coord {
    x: f32,
    y: f32,
}

impl Coord {
    fn new(x: f32, y: f32) -> Self {
        Coord { x, y }
    }
    fn rand(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Coord {
            x: rng.gen_range(x_range),
            y: rng.gen_range(y_range),
        }
    }
}

pub struct Enemy {
    pub location: Coord,
    width: f32,
}

impl Enemy {
    pub fn new(location: Coord) -> Self {
        Enemy {
            location,
            ..Default::default()
        }
    }
}
impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            location: Coord { x: 0.0, y: 0.0 },
            width: 5.0,
        }
    }
}
pub struct Player {
    pub location: Coord,
    velocity: f32,
    width: f32,
}

impl Player {
    fn tick(&mut self) {
        // If key pressed move
    }
    pub fn render(&self, state: &mut State) {
        // Draw a square at this pos

        let x = self.location.x;
        let y = self.location.y;

        let window_height = state.size().height;
        let window_width = state.size().width;

        // let l = self.thiccness as f32 / (window_width as f32 / 2.0);
        let l = 0.1;

        let gl_y = (y as f32 - window_height as f32 / 2.0) / (window_height as f32 / 2.0);
        let gl_x = (x as f32 - window_width as f32 / 2.0) / (window_width as f32 / 2.0);

        state.font_interface.queue(
            state.size,
            &format!("Pos: ({}, {})", gl_x, gl_y),
            x as f32,
            (window_height as f32 - y) as f32,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            40.0,
        );
        state.font_interface.queue(
            state.size,
            &format!("Zoom: ({})", state.camera.eye.z),
            x as f32,
            (window_height as f32 - y - 50.0) as f32,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            40.0,
        );

    }
}

impl Default for Player {
    fn default() -> Self {
        let location = Coord::new(0.0, 0.0);
        let width = 50.0;
        let velocity = 10.0;
        Player {
            location,
            width,
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
    pub enemies: Vec<Enemy>,
    pub controller: Controller,
    pub width: f32,
    pub height: f32,
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
        let x_range = 0.0..WORLD_WIDTH;
        let y_range = 0.0..WORLD_HEIGHT;
        World {
            player: Player::default(),
            controller: Controller::default(),
            enemies: vec![
                Enemy {
                    location: Coord::rand(x_range.clone(), y_range.clone()),
                    ..Enemy::default()
                },
                Enemy {
                    location: Coord::rand(x_range.clone(), y_range.clone()),
                    ..Enemy::default()
                },
                Enemy {
                    location: Coord::rand(x_range, y_range),
                    ..Enemy::default()
                },
            ],
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
        }
    }
}
