use std::sync::Arc;
use ggez::{event, GameResult};

use rusticade::{create_game_context, create_creature, create_player};
use rusticade::creatures::creature::Creature;
use rusticade::objects::{item::Item, platform::Platform};
use rusticade::shared::{directions::Direction::*, config::Config};
use rusticade::player::player::Player;
use rusticade::game::game::Game;


pub fn main() -> GameResult {
// configuration
let config = Arc::new(Config::default());
let (mut ctx, event_loop) = create_game_context!("Test generatora", "Nadal Julia Kulczycka", config.clone())?;
// creating player object
let mut player = create_player!(100.30, 100.50, config.clone());
let mut game = Game::new(player, config.clone())?;
let platform2 = Platform::new(150.00, 350.00, 50.00, 50.00);
game.add_platform(platform2);
let platform1 = Platform::new(50.00, 50.00, 50.00, 50.00);
game.add_platform(platform1);
event::run(ctx, event_loop, game)
}
