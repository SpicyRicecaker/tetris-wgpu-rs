// #![allow(dead_code)]

use std::ops::Range;

use event::{ElementState, VirtualKeyCode, WindowEvent};
use rand::Rng;
use winit::event;

pub mod graphics;
pub mod wgpu_boilerplate;

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
    pub fn tick(&mut self) {
        todo!()
    }
    pub fn render(&self, gfx: &mut graphics::Graphics) {
        gfx.draw_square(
            self.location.x,
            gfx.state.sc_desc.height as f32 - self.location.y,
            self.width,
            graphics::color::Color::new(256, 256, 256, 256),
        );
    }
}
impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            location: Coord { x: 0.0, y: 0.0 },
            width: 50.0,
        }
    }
}
pub struct Player {
    pub location: Coord,
    velocity: f32,
    width: f32,
}

impl Player {
    pub fn tick(&mut self) {
        // If key pressed move
        todo!()
    }
    pub fn render(&self, gfx: &mut graphics::Graphics) {
        gfx.draw_square(
            self.location.x,
            gfx.state.sc_desc.height as f32 - self.location.y,
            self.width,
            graphics::color::Color::new(256, 256, 256, 256),
        );
        // Draw a square at this pos

        gfx.state.font_interface.queue(
            gfx.state.size,
            &format!("Pos: ({}, {})", self.location.x, self.location.y),
            self.location.x,
            gfx.state.sc_desc.height as f32 - self.location.y,
            wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            40.0,
        );
        gfx.state.font_interface.queue(
            gfx.state.size,
            &format!("Zoom: ({})", gfx.state.camera.eye.z),
            self.location.x,
            gfx.state.sc_desc.height as f32 - self.location.y + 40.0,
            wgpu::Color {
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
        let location = Coord::new(500.0, 500.0);
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
    pub fn render(&self, gfx: &mut graphics::Graphics) {
        self.player.render(gfx);
        self.enemies.iter().for_each(|e| e.render(gfx));
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
