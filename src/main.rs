mod game;
mod shared;
mod objects;
mod creatures;
mod player;

use std::sync::Arc;

use creatures::creature::Creature;
use creatures::creature_macros::create_creature;
use objects::item::Item;
use objects::platform::Platform;
use shared::directions::Direction::*;
use player::player::{Player};
use player::player_macros::{create_player};
use shared::config::Config;
use ggez::{event, GameResult, GameError};
use game::context::create_game_context;
use game::game::Game;


fn main() -> GameResult {
    let config = Arc::new(Config {gravity: 0.0, ..Config::default()});

    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    let mut superplatform = Platform::new(250.0, 320.0, 80.0, 80.0);
    superplatform.set_barrier(Bottom, true);
    superplatform.set_action(Bottom, |_p: &mut Player| { 
        _p.take_damage(10.0);
        Ok(println!("aua moja głowa, hp: {}", _p.hp)) 
    });
    fn action_top(p: &mut Player) -> Result<(), GameError> {
        p.physics.y -= 100.0;
        Ok(())
    }
    superplatform.set_action(Top, action_top);


    let mut player = create_player!(0.0, 0.0, config.clone());
    let mut platform = Platform::default();
    platform.x = 100.0;
    platform.y = 550.0;
    player.physics.x = 200.0; 
    player.physics.y = player.get_config().screen_height;
    player.physics.speed = 5.0;
    player.physics.jump = 12.0;

    platform.barriers.insert(Right);
    platform.barriers.insert(Left);

    let creature_action = |_p: &mut Player| {println!("auuuu"); Ok(true)};

    let moves = Vec::from([(0.0, -50.0), (-100.0, 0.0), (0.0, 50.0), (100.0, 0.0)]);
    let creature = Creature::new(600.0, player.get_config().screen_height/2.0, 20.0, 20.0, 
        moves, 5.0, creature_action);
    let mut game = Game::new(player, config)?;
    game.add_creature(creature);
    game.add_platform(platform);
    game.add_platform(superplatform);
    game.add_item_default_size(100.0, 100.0, |_p: &mut Player| { Ok(println!("o nie jestem kółkiem")) });
    game.add_platform_default_size(100.0, 250.0);
    game.add_platform_default_size(50.0, 50.0);
    game.add_platform_default_size(200.0, 500.0);
    let item = Item::new(600.0, 500.0, 100.0, 30.0, |_p: &mut Player| { Ok(println!("o sorki jednak elipsą"))});
    game.add_item(item);

    event::run(ctx, event_loop, game)
}
