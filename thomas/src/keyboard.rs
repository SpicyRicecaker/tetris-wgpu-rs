use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

pub struct Keyboard {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub plus: bool,
    pub minus: bool,
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub r: bool,
    pub z: bool,
    pub c: bool,
    pub space: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            plus: false,
            minus: false,
            w: false,
            s: false,
            a: false,
            d: false,
            r: false,
            z: false,
            c: false,
            space: false,
        }
    }

    pub fn is_pressed(&mut self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Up => self.up,
            VirtualKeyCode::Down => self.down,
            VirtualKeyCode::Left => self.left,
            VirtualKeyCode::Right => self.right,
            VirtualKeyCode::Equals => self.plus,
            VirtualKeyCode::Minus => self.minus,
            VirtualKeyCode::W => self.w,
            VirtualKeyCode::S => self.s,
            VirtualKeyCode::A => self.a,
            VirtualKeyCode::D => self.d,
            VirtualKeyCode::R => self.r,
            VirtualKeyCode::Z => self.z,
            VirtualKeyCode::C => self.c,
            VirtualKeyCode::Space => self.space,
            _ => false,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
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
                    VirtualKeyCode::Equals => {
                        self.plus = is_pressed;
                        true
                    }
                    VirtualKeyCode::Minus => {
                        self.minus = is_pressed;
                        true
                    }
                    VirtualKeyCode::W => {
                        self.w = is_pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.s = is_pressed;
                        true
                    }
                    VirtualKeyCode::A => {
                        self.a = is_pressed;
                        true
                    }
                    VirtualKeyCode::D => {
                        self.d = is_pressed;
                        true
                    }
                    VirtualKeyCode::R => {
                        self.r = is_pressed;
                        true
                    }
                    VirtualKeyCode::Z => {
                        self.z = is_pressed;
                        true
                    }
                    VirtualKeyCode::C => {
                        self.c = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.space = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}
