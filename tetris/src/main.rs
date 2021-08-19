use tetris::universe::Universe;

fn main() {
    let universe = Universe::default();

    let (event_loop, ctx) = thomas::Porter::build(thomas::Config {
        title: universe.config.title().to_string(),
        ticks: *universe.config.ticks(),
        margin: 100.0,
    });

    thomas::Porter::run(event_loop, ctx, universe);
}
