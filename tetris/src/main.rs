// Prevent popup of console window
// Don't know how to declare it only for release builds...
// #![windows_subsystem = "windows"]

use tetris::universe::Universe;

fn main() {
    let universe = Universe::default();

    let (event_loop, ctx) = thomas::ContextBuilder::new()
        .with_title(universe.config.title())
        .with_ticks(*universe.config.ticks())
        .build();

    thomas::main::run(event_loop, ctx, universe);
}
