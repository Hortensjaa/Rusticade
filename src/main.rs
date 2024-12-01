mod classes;
mod game;
mod physics;
mod config;

use std::sync::Arc;

use classes::platform::Platform;
use classes::directions::Direction::*;
use classes::player::{create_player, Player};
use config::Config;
use ggez::{event, GameResult};
use game::context::create_game_context;
use game::game::Game;


fn main() -> GameResult {
    let config = Arc::new(Config::default());
    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    let player = create_player!(0.0, 0.0, 50.0, 50.0, config.clone());
    let mut game = Game::new(player, config.clone())?;
    let mut superplatform = Platform::new(250.0, 320.0, 80.0, 80.0);
    superplatform.set_barrier(Bottom, true);
    superplatform.set_action(Bottom, |_p: &mut Player| { Ok(println!("aua moja głowa")) });

    game.add_platform(superplatform);
    game.add_platform_default_size(100.0, 100.0);
    game.add_platform_default_size(100.0, 250.0);
    game.add_platform_default_size(50.0, 50.0);
    game.add_platform_default_size(200.0, 500.0);
    game.add_platform_custom_size(0.0, 550.0, 600.0, 50.0);
    game.add_finish_platform(600.0, 500.0, 100.0, 200.0);

    event::run(ctx, event_loop, game)
}
