use futures::executor::block_on;
use wgpu_boilerplate::state::State;
use wgpu_test::wgpu_boilerplate;
use wgpu_test::World;

use winit::dpi::LogicalSize;
use winit::dpi::Size;
use winit::window::WindowBuilder;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    // Create event loop
    let event_loop = EventLoop::new();
    // Create window
    let builder = WindowBuilder::new()
        .with_min_inner_size(Size::Logical(LogicalSize {
            width: 1920_f64,
            height: 1080_f64,
        }))
        .with_visible(false);
    let window = builder.build(&event_loop).unwrap();

    // Create some mobs
    let mut world = World::default();

    // Init wgpu the whole reason we're playing the game lol
    let mut state = block_on(wgpu_boilerplate::state::State::new(&window));
    state.render_background(0_f64, 0_f64, 0_f64, 0_f64).unwrap();

    window.set_visible(true);

    // Here's the 'game loop'
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow Poll v. ControlFlow Wait, two different power v. performance cases
        *control_flow = ControlFlow::Poll;

        // world.tick,
        match event {
            Event::WindowEvent { ref event, .. } => {
                if !state.input(event, &mut world) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => {
                            handle_input(&mut state, input, control_flow)
                        }
                        WindowEvent::Resized(size) => state.resize(*size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => (),
                    }
                }
            }
            Event::MainEventsCleared => {
                // After tick you redraw stuff?
                // Only redraw if you need to? (ui)
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                world.tick();
                state.update(&mut world);
                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(*state.size()),
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

fn handle_input(state: &mut State, input: &KeyboardInput, control_flow: &mut ControlFlow) {
    match input {
        KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        } => *control_flow = ControlFlow::Exit,
        KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Space),
            ..
        } => {
            // change render pipeline
            // state.swap_render_pipeline();
            // change buffers
            state.swap_buffers();
        }
        _ => {}
    }
}
