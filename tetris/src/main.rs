use std::env;
use std::path::PathBuf;
use tetris::{config::Config, universe::Universe};

fn main() {
    let config = Config::default();

    #[cfg(debug_assertions)]
    let resource_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "resources"].iter().collect();
    #[cfg(not(debug_assertions))]
    let resource_dir = tetris::prod::res();

    // Thinking of maybe moving this icon dir to be ctx.window.set_icon after initializing the resource dir
    let mut icon_dir = resource_dir.clone();
    icon_dir.push("icon.ico");

    let (event_loop, mut ctx) = thomas::ContextBuilder::new()
        .with_title(config.title())
        .with_ticks(*config.ticks())
        .with_resource_dir(resource_dir)
        .with_icon(icon_dir)
        .build();

    let mut universe = Universe::new(&mut ctx, config);
    // We're going to update universe config with window size
    // TODO support updating window size on resize window
    universe.config.resize(
        ctx.graphics.size.width as f32,
        ctx.graphics.size.height as f32,
    );

    thomas::main::run(event_loop, ctx, universe);
}
