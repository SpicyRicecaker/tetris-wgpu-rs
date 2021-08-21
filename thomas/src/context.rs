use super::graphics::backend::State;
use super::keyboard::Keyboard;
use super::resource::ResourceManager;
use super::audio::Audio;

pub struct Context {
    pub graphics: State,
    pub keyboard: Keyboard,
    pub audio: Audio,
    pub window: winit::window::Window,
    pub resource_mgr: ResourceManager,
    pub config: crate::Config
}