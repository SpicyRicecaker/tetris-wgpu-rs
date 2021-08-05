use futures::executor::block_on;
use wgpu_test::wgpu_boilerplate;
use wgpu_test::World;
use wgpu_test::MARGIN;
use wgpu_test::graphics::Graphics;

use winit::dpi::PhysicalPosition;
use winit::window::WindowBuilder;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

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

    // Init wgpu the whole reason we're playing the game lol
    let state = block_on(wgpu_boilerplate::state::State::new(&window));
    let mut gfx = Graphics::new(state);

    // Create stuff
    gfx.state.render_background(0_f64, 0_f64, 0_f64, 0_f64).unwrap();

    window.set_visible(true);

    // Here's the 'game loop'
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow Poll v. ControlFlow Wait, two different power v. performance cases
        *control_flow = ControlFlow::Poll;

        // world.tick,
        match event {
            Event::WindowEvent { ref event, .. } => {
                if !gfx.state.input(event, &mut world) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
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
                // After tick you redraw stuff?
                // Only redraw if you need to? (ui)
                world.tick();

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                world.render(&mut gfx);

                gfx.state.update();
                match gfx.state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => gfx.state.resize(*gfx.state.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                };
                // world.render(&mut state);
            }
            _ => (),
        }
    });
}