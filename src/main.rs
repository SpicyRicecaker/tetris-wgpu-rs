use futures::executor::block_on;
use game::Game;
use tetris_wgpu_rs::game;
use tetris_wgpu_rs::graphics::Graphics;
use tetris_wgpu_rs::wgpu_boilerplate;
use tetris_wgpu_rs::World;
use tetris_wgpu_rs::MARGIN;

use winit::dpi::PhysicalPosition;
use winit::window::WindowBuilder;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use std::time::Instant;

fn main() {
    env_logger::init();
    // Create event loop
    let event_loop = EventLoop::new();
    // Create window
    let builder = WindowBuilder::new()
        .with_title("Tetris")
        .with_visible(false);
    let window = builder.build(&event_loop).unwrap();
    let mut size = window.current_monitor().unwrap().size();
    size.width -= (MARGIN * 2.0) as u32;
    size.height -= (MARGIN * 2.0) as u32;
    window.set_inner_size(size);
    window.set_outer_position(PhysicalPosition {
        x: MARGIN,
        y: MARGIN,
    });

    // Create some mobs
    let mut world = World::default();
    let game = Game::default();

    // Init wgpu the whole reason we're playing the game lol
    let state = block_on(wgpu_boilerplate::state::State::new(&window));
    let mut gfx = Graphics::new(state);

    // Create stuff
    // gfx.state.render_background(0_f64, 0_f64, 0_f64, 0_f64).unwrap();

    window.set_visible(true);

    // Game "speed" or "update time" should be 60
    // But render time should happen regardless of ticks
    let ticks_per_second: f64 = 144.0;
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
                if !gfx.state.input(event, &mut world) {
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
                        WindowEvent::Resized(size) => gfx.state.resize(*size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            gfx.state.resize(**new_inner_size);
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
                    world.tick();
                    ticks += 1;
                    lag -= nanos_per_tick;
                }

                // Unconditionally rerender
                window.request_redraw();
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
                // First clear background
                gfx.clear_background(game.palette.bg);
                // Write fps
                gfx.state.font_interface.queue(
                    gfx.state.size,
                    &format!("FPS: {}", average_frames),
                    0.0,
                    0.0,
                    wgpu::Color::from(game.palette.fg),
                    20.0,
                );
                gfx.state.font_interface.queue(
                    gfx.state.size,
                    &format!("Ticks/s: {}", average_ticks),
                    140.0,
                    0.0,
                    wgpu::Color::from(game.palette.fg),
                    20.0,
                );

                world.render(&mut gfx, &game);

                gfx.state.update();
                match gfx.state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => gfx.state.resize(*gfx.state.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("Err: {:?}", e),
                };
                // world.render(&mut state);
            }
            _ => (),
        }
    });
}

fn exit(control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Exit;
}
