/*!
# ThomasE
This crate is a 2d framework. Internally, it's a wrapper over [`wgpu`] that takes great
inspiration from [ggez](https://github.com/ggez/ggez) and [raylib](https://github.com/raysan5/raylib).

## Getting Started
Create a struct that implements the [`Runnable`] trait, a context using [`ContextBuilder`],
Then start the game loop with [`main::run`]

```
use thomas::Color;

struct Game {
    playerx: f32,
    playery: f32
}

impl Runnable for Game {
    fn tick(&mut self, _ctx: &mut Context) {
        self.playerx += 1;
        self.playery += 1;
    }
    fn render(&self, ctx: &mut Context) {
        ctx.graphics.clear_background(Color::from_hex("#000000"));
        ctx.graphics.draw_square(playerx, playery, Color::from_hex("#FFFFFF"));
    }
}

let game = Game {playerx: 0.0, playery: 0.0};

let (event_loop, ctx) = thomas::ContextBuilder::new()
    .with_title("Booboo")
    .build();

thomas::main::run(event_loop, ctx, universe);
```
*/

pub mod context;
use audio::Audio;
use context::Context;
pub mod graphics;
mod keyboard;
pub use graphics::frontend;
use image::GenericImageView;
use resource::ResourceManager;
pub use rodio;
pub use winit;
pub mod audio;
mod resource;

use winit::dpi::PhysicalPosition;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use std::path::PathBuf;
use std::time::Instant;
pub use wgpu::Color;

/// Contains parameters that are used by [`main::run`]
pub struct Config {
    pub ticks: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self { ticks: 140 }
    }
}
/// Builder for a [`Context`]
pub struct ContextBuilder {
    title: String,
    margin: f32,
    icon: Option<PathBuf>,
    resource_mgr: PathBuf,
    config: Config,
}

impl ContextBuilder {
    /// ## Defaults
    /// `100.0` px margin
    /// `Game` title
    /// No icon
    /// Default config
    pub fn new() -> Self {
        Self {
            title: String::from("Game"),
            margin: 100.0,
            icon: None,
            resource_mgr: PathBuf::new(),
            config: Config::default(),
        }
    }
    /// Changes title of [`winit::window::Window`]
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    /// Changes ticks in [`Config`]
    pub fn with_ticks(mut self, ticks: u32) -> Self {
        self.config.ticks = ticks;
        self
    }
    /// Changes margin of game window, functionally equivalent to html margin
    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }
    /// Changes icon of window
    pub fn with_icon(mut self, path: PathBuf) -> Self {
        self.icon = Some(path);
        self
    }
    /// Changes root path of resources
    pub fn with_resource_dir(mut self, path: PathBuf) -> Self {
        self.resource_mgr = path;
        self
    }
    /// Creates a [`Context`] and [`EventLoop<()>`] using current settings, consuming the builder
    pub fn build(self) -> (EventLoop<()>, context::Context) {
        // Init logger for errors, etc.
        env_logger::init();

        // Create event loop
        let event_loop = EventLoop::new();

        // Load icon
        let icon = match self.icon {
            Some(icon_path) => {
                let image = image::open(icon_path).expect("Unable to find image");
                Some(
                    winit::window::Icon::from_rgba(image.to_bytes(), image.width(), image.height())
                        .expect("Bad image"),
                )
            }
            None => None,
        };

        // Create window, with `margin`
        let builder = winit::window::WindowBuilder::new()
            .with_title(self.title)
            .with_visible(false)
            .with_window_icon(icon);
        let window = builder.build(&event_loop).unwrap();
        let mut size = window.current_monitor().unwrap().size();
        size.width -= (self.margin * 2.0) as u32;
        size.height -= (self.margin * 2.0) as u32;
        window.set_inner_size(size);
        window.set_outer_position(PhysicalPosition {
            x: self.margin,
            y: self.margin,
        });

        // Init [`wgpu`]
        let graphics = futures::executor::block_on(graphics::backend::State::new(&window));
        // Init keyboard controller
        let keyboard = keyboard::Keyboard::new();

        // After everything's loaded make window visible
        window.set_visible(true);

        let resource_mgr = ResourceManager::new(self.resource_mgr);

        let audio = Audio::new();

        let context = Context {
            graphics,
            keyboard,
            window,
            audio,
            // Doesn't matter if we move here 'cause self is consumed
            config: self.config,
            resource_mgr,
        };

        (event_loop, context)
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A struct with this trait must be passed into [`main::run`]
pub trait Runnable {
    /// Runs every tick, as defined with [`ContextBuidler::with_ticks`]
    /// Put your game logic here
    fn tick(&mut self, ctx: &mut context::Context);

    /// Runs every frame, which matches the refresh rate of whatever device the program
    /// is run on. Use [`Context`]`.graphics.(render)` here
    fn render(&self, ctx: &mut context::Context);
}

/// This module includes the [`main::run`] function
pub mod main {
    use super::*;
    /// Takes in an [`EventLoop`] & [`Context`], both of which are generated from [`ContextBuilder::build`]
    /// Also some sort of state that implements [`Runnable`]
    pub fn run<T: 'static + Runnable>(
        event_loop: EventLoop<()>,
        mut context: Context,
        mut state: T,
    ) {
        // Game "speed" or "update time"
        let ticks_per_second: f64 = context.config.ticks as f64;
        let nanos_per_tick: u128 = (1_000_000_000.0 / ticks_per_second).round() as u128;
        let mut frames = 0;
        let mut average_frames = 0;
        let mut prev_time = Instant::now();
        let mut timer = Instant::now();
        let mut lag: u128 = 0;
        let mut ticks = 0;
        let mut average_ticks = 0;

        // Here's the 'game loop'
        event_loop.run(move |event, _, control_flow| {
            // ControlFlow Poll v. ControlFlow Wait, two different power v. performance cases
            *control_flow = ControlFlow::Poll;

            // world.tick,
            match event {
                Event::WindowEvent { ref event, .. } => {
                    if !context.keyboard.input(event) {
                        match event {
                            WindowEvent::CloseRequested => exit(control_flow),
                            WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => exit(control_flow),
                            WindowEvent::Resized(size) => context.graphics.resize(*size),
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so we have to dereference it twice
                                context.graphics.resize(**new_inner_size);
                            }
                            _ => (),
                        }
                    }
                }
                Event::MainEventsCleared => {
                    let time_passed = prev_time.elapsed();
                    lag += time_passed.as_nanos();
                    prev_time = Instant::now();

                    // So long as time passed is above the designated nanos per fps
                    while lag > nanos_per_tick {
                        state.tick(&mut context);
                        ticks += 1;
                        lag -= nanos_per_tick;
                    }

                    // Unconditionally rerender
                    context.window.request_redraw();
                    frames += 1;

                    if timer.elapsed().as_millis() > 1000 {
                        timer = Instant::now();
                        average_frames = frames;
                        average_ticks = ticks;
                        frames = 0;
                        ticks = 0;
                    }
                }
                Event::RedrawRequested(_) => {
                    state.render(&mut context);

                    // Write fps
                    context.graphics.draw_text(
                        &format!("FPS: {}", average_frames),
                        0.0,
                        0.0,
                        wgpu::Color::GREEN,
                        20.0,
                    );
                    context.graphics.draw_text(
                        &format!("Ticks/s: {}", average_ticks),
                        140.0,
                        0.0,
                        wgpu::Color::GREEN,
                        20.0,
                    );

                    context.graphics.update();
                    match context.graphics.render() {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SurfaceError::Lost) => {
                            context.graphics.resize(*context.graphics.size())
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => exit(control_flow),
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("Err: {:?}", e),
                    };
                }
                _ => (),
            }
        });
    }
    #[inline]
    fn exit(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Exit;
    }
}
