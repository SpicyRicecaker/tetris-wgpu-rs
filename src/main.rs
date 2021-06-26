use wgpu_test::wgpu_boilerplate;
use wgpu_test::World;

use winit::dpi::LogicalSize;
use winit::dpi::Size;
use winit::window::WindowBuilder;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    // Create event loop
    let event_loop = EventLoop::new();
    // Create window
    let builder = WindowBuilder::new().with_min_inner_size(Size::Logical(LogicalSize {
        width: 1920_f64,
        height: 1080_f64,
    }));
    let window = builder.build(&event_loop).unwrap();

    // Create some mobs
    let world = World::default();

    // Init wgpu the whole reason we're playing the game lol
    wgpu_boilerplate::init(&window);

    // Here's the 'game loop'
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow Poll v. ControlFlow Wait, two different power v. performance cases
        *control_flow = ControlFlow::Poll;

        // world.tick,
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close button pressed");
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                // After tick you redraw stuff?
                // Only redraw if you need to? (ui)
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // world.render
            }
            _ => (),
        }
    });
}
