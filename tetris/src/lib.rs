// #![allow(dead_code)]

use rand::prelude::*;
use std::ops::Range;
use thomas::context::Context;

pub mod game;

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
    pub fn tick(&mut self, ctx: &mut Context) {
        self.location.y -= 1.0;
    }
    pub fn render(&self, ctx: &mut Context, game: &crate::game::Game) {
        ctx.graphics.draw_square(
            self.location.x,
            ctx.graphics.size.height as f32 - self.location.y,
            self.width,
            game.palette.l,
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
    pub fn tick(&mut self, ctx: &mut Context) {
        // If key pressed move
        if ctx.keyboard.up {
            self.location.y += self.velocity;
        }
        if ctx.keyboard.down {
            self.location.y -= self.velocity;
        }
        if ctx.keyboard.left {
            self.location.x -= self.velocity;
        }
        if ctx.keyboard.right {
            self.location.x += self.velocity;
        }
    }
    pub fn render(&self, ctx: &mut Context, game: &crate::game::Game) {
        ctx.graphics.draw_square(
            self.location.x,
            ctx.graphics.size.height as f32 - self.location.y,
            self.width,
            game.palette.l,
        );
        // Draw a square at this pos

        ctx.graphics.font_interface.queue(
            ctx.graphics.size,
            &format!("Pos: ({}, {})", self.location.x, self.location.y),
            self.location.x,
            ctx.graphics.size.height as f32 - self.location.y - 40.0,
            thomas::Color::from(game.palette.fg),
            40.0,
        );
        ctx.graphics.font_interface.queue(
            ctx.graphics.size,
            &format!("Zoom: ({})", ctx.graphics.camera.eye.z),
            self.location.x,
            ctx.graphics.size.height as f32 - self.location.y - 80.0,
            thomas::Color::from(game.palette.fg),
            40.0,
        );
    }
}

impl Default for Player {
    fn default() -> Self {
        let location = Coord::new(500.0, 500.0);
        let width = 50.0;
        let velocity = 5.0;
        Player {
            location,
            width,
            velocity,
        }
    }
}

pub struct World {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub width: f32,
    pub height: f32,
}

impl World {
    pub fn tick(&mut self, ctx: &mut Context, game: &game::Game) {
        self.player.tick(ctx);
        // self.enemies.iter_mut().for_each(|e| e.tick(ctx));
    }
    pub fn render(&self, ctx: &mut Context, game: &game::Game) {
        // Create debug line
        ctx.graphics.draw_line(0.0, 0.0, 500.0, 500.0, 50.0, game.palette.l);
        // Render player and all enemies
        self.enemies.iter().for_each(|e| e.render(ctx, game));
        self.player.render(ctx, game);
    }
}

impl Default for World {
    fn default() -> Self {
        let x_range = 0.0..WORLD_WIDTH;
        let y_range = 0.0..WORLD_HEIGHT;
        World {
            player: Player::default(),
            enemies: vec![
                Enemy {
                    location: Coord { x: 100.0, y: 100.0 },
                    ..Enemy::default()
                },
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
