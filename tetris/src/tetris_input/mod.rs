use thomas::{context::Context, winit::event::VirtualKeyCode};
// Utils for holding a key
mod utils;

use utils::*;
// The framework that keyboard input and keys are built on

pub trait InputInterface {
    fn receive_key(&mut self);
}
// Our implementation of tetrominos
pub struct TetrominoControls {
    // Not sure if fallrate really fits the agenda here
    controlled_keys: Vec<ControlledKey>,
    queue: Vec<VirtualKeyCode>,
}

// This implementation isn't gonna work, if we have for example more functions that we want the keys to do than move the tetromino
impl TetrominoControls {
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    pub fn get_queue(&self) -> &[VirtualKeyCode] {
        &self.queue
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        for controlled_key in self.controlled_keys.iter_mut() {
            if controlled_key.tick(ctx) {
                self.queue.push(controlled_key.key)
            }
        }
    }
}

impl Default for TetrominoControls {
    fn default() -> Self {
        let controlled_keys = vec![
            ControlledKey {
                key: VirtualKeyCode::Left,
                repeat: Repeat { delay: 8, rate: 4 },
                ..Default::default()
            },
            ControlledKey {
                key: VirtualKeyCode::Right,
                repeat: Repeat { delay: 8, rate: 4 },
                ..Default::default()
            },
            ControlledKey {
                key: VirtualKeyCode::Down,
                repeat: Repeat { delay: 0, rate: 4 },
                ..Default::default()
            },
            ControlledKey {
                key: VirtualKeyCode::Z,
                repeat: Repeat { delay: 8, rate: 8 },
                ..Default::default()
            },
            ControlledKey {
                key: VirtualKeyCode::C,
                repeat: Repeat { delay: 8, rate: 8 },
                ..Default::default()
            },
            ControlledKey {
                key: VirtualKeyCode::Space,
                repeat: Repeat { delay: 8, rate: 8 },
                ..Default::default()
            },
        ];
        TetrominoControls {
            controlled_keys,
            queue: Vec::new(),
        }
    }
}
