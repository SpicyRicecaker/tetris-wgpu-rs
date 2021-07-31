use event::{ElementState, VirtualKeyCode, WindowEvent};
use wgpu_boilerplate::{
    buffers::Vertex,
    state::{self, State},
};
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
    pub fn render(&self, state: &mut State) {
        // Draw a square at this pos

        let x = self.location.x;
        let y = self.location.y;

        let window_height = state.size().height;
        let window_width = state.size().width;

        let l = self.thiccness as f32 / (window_width as f32 / 2.0);

        let gl_y = (y as f32 - window_height as f32 / 2.0) / (window_height as f32 / 2.0);
        let gl_x = (x as f32 - window_width as f32 / 2.0) / (window_width as f32 / 2.0);

        // let vertices_player: &[Vertex] = &[
        //     // Top right
        //     Vertex {
        //         position: [0.1, 0.1, 0.0],
        //         tex_coords: [0.4, 0.00759614],
        //     }, 
        //     // Top left
        //     Vertex {
        //         position: [-0.1, 0.1, 0.0],
        //         tex_coords: [0.0048, 0.43041354],
        //     }, 
        //     // bot left
        //     Vertex {
        //         position: [-0.1, -0.1, 0.0],
        //         tex_coords: [0.28, 0.949],
        //     }, 
        //     // bot right
        //     Vertex {
        //         position: [0.1, -0.1, 0.0],
        //         tex_coords: [0.85967, 0.847329],
        //     },
        // ];
        // convert (x, y) -> (-1, 1) (-1, 1)
        let vertices_player: &[Vertex] = &[
            // Top right
            Vertex {
                position: [gl_x+l, gl_y+l, 0.0],
                tex_coords: [0.4, 0.00759614],
            }, 
            // Top left
            Vertex {
                position: [gl_x-l, gl_y+l, 0.0],
                tex_coords: [0.0048, 0.43041354],
            }, 
            // bot left
            Vertex {
                position: [gl_x-l, gl_y-l, 0.0],
                tex_coords: [0.28, 0.949],
            }, 
            // bot right
            Vertex {
                position: [gl_x+l, gl_y-l, 0.0],
                tex_coords: [0.85967, 0.847329],
            },
        ];

        state.update_buffer(vertices_player);
    }
}

impl Default for Player {
    fn default() -> Self {
        let location = Coord::new(400, 300);
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
