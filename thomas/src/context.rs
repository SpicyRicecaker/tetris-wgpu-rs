use super::graphics::backend::State;
use super::keyboard::Keyboard;

pub struct Context {
    pub graphics: State,
    pub keyboard: Keyboard,
    pub window: winit::window::Window,
    pub config: crate::Config
}