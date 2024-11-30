mod classes;
mod game;
mod physics;
mod config;

use std::sync::Arc;

use classes::player::Player;
use config::Config;
use ggez::{event, GameResult};
use game::context::create_game_context;
use game::logic::Game;


fn main() -> GameResult {
    let config = Arc::new(Config::default());
    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    let mut game = Game::new(config.clone())?;
    game.add_platform_default_size(100.0, 100.0);
    game.add_platform_default_size(100.0, 200.0);
    game.add_platform_default_size(50.0, 50.0);
    game.add_platform_custom_size(0.0, 550.0, 600.0, 50.0);
    game.add_finish_platform(100.0, 200.0, 100.0, 20.0);

    event::run(ctx, event_loop, game)
}
