mod game;
mod utils;
mod config;
mod objects;
mod creatures;
mod player;

use std::sync::Arc;

use creatures::creature::Creature;
use objects::item::Item;
use objects::platform::Platform;
use utils::directions::Direction::*;
use player::player::{create_player, Player};
use config::Config;
use ggez::{event, GameResult, GameError};
use game::context::create_game_context;
use game::game::Game;


fn main() -> GameResult {
    let config = Arc::new(Config {gravity: 100.0, ..Config::default()});
    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    // let mut superplatform = Platform::new(250.0, 320.0, 80.0, 80.0);
    // superplatform.set_barrier(Bottom, true);
    // superplatform.set_action(Bottom, |_p: &mut Player| { 
    //     _p.take_damage(10.0);
    //     Ok(println!("aua moja głowa, hp: {}", _p.hp)) 
    // });
    // fn action_top(p: &mut Player) -> Result<(), GameError> {
    //     p.physics.y -= 100.0;
    //     Ok(())
    // }
    // superplatform.set_action(Top, action_top);


    let mut player = create_player!(0.0, 0.0, config.clone());
    let mut platform = Platform::default();
    platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 200.0; 
        player.physics.y = player.config.screen_height;

        platform.barriers.insert(Right);
        platform.barriers.insert(Left);
    let moves = Vec::from([(100.0, 0.0), (0.0, 50.0), (-100.0, 0.0), (0.0, -50.0)]);
    // let moves = Vec::from([(100.0, 50.0), (-100.0, -50.0)]);
    // let moves = Vec::from([(100.0, -50.0), (-100.0, 50.0)]);
    let mut creature = Creature {x: 200.0, y: 200.0, moves, ..Creature::default()};
    creature.speed = 1000.0;
    let mut game = Game::new(player, config)?;
    game.add_creature(creature);
    // game.add_platform(platform);
    // game.add_platform(superplatform);
    // game.add_item_default_size(100.0, 100.0, |_p: &mut Player| { Ok(println!("o nie jestem kółkiem")) });
    // game.add_platform_default_size(100.0, 250.0);
    // game.add_platform_default_size(50.0, 50.0);
    // game.add_platform_default_size(200.0, 500.0);
    // let item = Item::new(600.0, 500.0, 100.0, 30.0, |_p: &mut Player| { Ok(println!("o sorki jednak elipsą"))});
    // game.add_item(item);

    event::run(ctx, event_loop, game)
}
