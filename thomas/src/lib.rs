pub mod context;
use context::Context;
pub mod graphics;
mod keyboard;
pub use graphics::frontend;
use image::GenericImageView;
pub use winit;

use winit::dpi::PhysicalPosition;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use std::path::PathBuf;
use std::time::Instant;
pub use wgpu::Color;

/// render frames depends on refresh rate, but we can control ticks per second
pub struct Config {
    pub title: String,
    pub ticks: u32,
    pub margin: f32,
    pub icon: Option<PathBuf>,
}

pub struct Porter;
pub trait TrainEngine {
    fn tick(&mut self, ctx: &mut context::Context);

    // Mks includes winit stuff, like mouse events, gfx is purely for drawing
    // Probably don't need mks in render but we'll leave it in here for now
    fn render(&self, ctx: &mut context::Context);
}
impl Porter {
    pub fn build(config: Config) -> (EventLoop<()>, context::Context) {
        env_logger::init();
        // Create event loop
        let event_loop = EventLoop::new();

        // Load icon
        let icon = match &config.icon {
            Some(icon_path) => {
                let image = image::open(icon_path).expect("Unable to find image");
                Some(
                    winit::window::Icon::from_rgba(image.to_bytes(), image.width(), image.height())
                        .expect("Bad image"),
                )
            }
            None => None,
        };

        // Create window
        let builder = winit::window::WindowBuilder::new()
            .with_title(&config.title)
            .with_visible(false)
            .with_window_icon(icon);

        let window = builder.build(&event_loop).unwrap();
        let mut size = window.current_monitor().unwrap().size();
        size.width -= (config.margin * 2.0) as u32;
        size.height -= (config.margin * 2.0) as u32;
        window.set_inner_size(size);
        window.set_outer_position(PhysicalPosition {
            x: config.margin,
            y: config.margin,
        });

        // Init wgpu the whole reason we're playing the game lol
        let graphics = futures::executor::block_on(graphics::backend::State::new(&window));
        let keyboard = keyboard::Keyboard::new();

        window.set_visible(true);

        let context = Context {
            graphics,
            keyboard,
            window,
            config,
        };

        (event_loop, context)
    }
    pub fn run<T: 'static + TrainEngine>(
        event_loop: EventLoop<()>,
        mut context: Context,
        mut state: T,
    ) {
        // Game "speed" or "update time" should be 60
        // But render time should happen regardless of ticks
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
                            WindowEvent::CloseRequested => Self::exit(control_flow),
                            WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => Self::exit(control_flow),
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
                        Err(wgpu::SurfaceError::OutOfMemory) => Self::exit(control_flow),
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("Err: {:?}", e),
                    };
                    // world.render(&mut state);
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
