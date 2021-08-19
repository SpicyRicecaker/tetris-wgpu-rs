use tetris::game::Game;
use tetris::World;

use thomas::context::Context;
use thomas::Config;
use thomas::Porter;
use thomas::frontend::camera_controller::CameraController;

/// The idea is something like
/// ```rust
/// #[game_loop]
/// // Where state contains something like controls
/// fn run(state: &State, gfx: &Graphics) {
///     // tick(state)
///     // render(gfx)
///     
/// }
/// ```
/// The problem is that if we want other variables in that scope
/// Like a `game_config`, `world`, or anything like that, then
/// we would have to purposely pass it into the function, making it
/// not very extensible at all
///
/// The solution?
/// We require user to define a state object that holds all their info, then we take this object
/// in as a requirement to our loops
/// Basically how `ggez` does it @ https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs

fn main() {
    // Idk what the diff between world and game is but watevs
    let world = World::default();
    let game = Game::default();
    // Builtin, should probably put in docs or something
    let camera_controller = CameraController::new(0.2);


    let config = Config {
        title: String::from("Tetris"),
        margin: tetris::MARGIN,
    };

    let state = State { world, game, camera_controller };

    // Setup config, as well as load resources like textuers and sound in the future
    let (event_loop, ctx) = Porter::build(config);

    Porter::run(event_loop, ctx, state);
}

struct State {
    world: World,
    game: Game,
    camera_controller: CameraController
}

impl thomas::TrainEngine for State {
    fn tick(&mut self, ctx: &mut Context) {
        self.camera_controller
            .tick(ctx);

        self.world.tick(ctx, &self.game);
    }

    // Mks includes winit stuff, like mouse events, gfx is purely for drawing
    // Probably don't need mks in render but we'll leave it in here for now
    fn render(&self, ctx: &mut Context) {
        self.world.render(ctx, &self.game);
    }
}
