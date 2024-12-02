mod game;
mod physics;
mod config;
mod objects;
mod player;

use std::sync::Arc;

use objects::platform::Platform;
use physics::directions::Direction::*;
use player::player::Player;
use config::Config;
use ggez::{event, GameResult, GameError};
use game::context::create_game_context;
use game::game::Game;


fn main() -> GameResult {
    let config = Arc::new(Config::default());
    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    let mut superplatform = Platform::new(250.0, 320.0, 80.0, 80.0);
    superplatform.set_barrier(Bottom, true);
    superplatform.set_action(Bottom, |_p: &mut Player| { Ok(println!("aua moja głowa")) });
    fn action_top(p: &mut Player) -> Result<(), GameError> {
        p.physics.y -= 100.0;
        Ok(())
    }
    superplatform.set_action(Top, action_top);


    let mut player = Player::default();
    let mut platform = Platform::default();
    platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 200.0; 
        player.physics.y = player.config.screen_height;
        // player.physics.vx = -5.0; 

        platform.barriers.insert(Right);
        platform.barriers.insert(Left);
    let mut game = Game::new(player, config.clone())?;
    game.add_platform(platform);
    game.add_platform(superplatform);
    game.add_platform_default_size(100.0, 100.0);
    game.add_platform_default_size(100.0, 250.0);
    game.add_platform_default_size(50.0, 50.0);
    game.add_platform_default_size(200.0, 500.0);
    // game.add_platform_custom_size(0.0, 550.0, 600.0, 50.0);
    // game.add_finish_platform(600.0, 500.0, 100.0, 200.0);

    event::run(ctx, event_loop, game)
}
