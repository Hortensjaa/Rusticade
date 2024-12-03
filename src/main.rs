mod game;
mod shared;
mod objects;
mod creatures;
mod player;

use std::sync::Arc;
use ggez::{event, GameResult, GameError};

use creatures::creature::Creature;
use objects::{item::Item, platform::Platform};
use shared::{directions::Direction::*, config::Config};
use player::{player::Player, player_macros::create_player};
use game::context::create_game_context;
use game::game::Game;

fn main() -> GameResult {
    // let config = Arc::new(Config::default());
    let config = Arc::new(Config{flying_mode: false, ..Config::default()});
    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    // Tworzenie gracza
    let mut player = create_player!(200.0, config.screen_height - 50.0, config.clone());
    player.physics.speed = 5.0;
    player.physics.jump = 12.0;

    // Tworzenie platform
    let mut platform1 = Platform::new(100.0, 550.0, 150.0, 20.0);
    platform1.set_barrier(Left, true);
    platform1.set_barrier(Right, true);

    let mut platform2 = Platform::new(300.0, 450.0, 100.0, 20.0);

    let mut bounce_platform = Platform::new(250.0, 320.0, 80.0, 20.0);
    bounce_platform.set_barrier(Bottom, true);
    bounce_platform.set_action(Bottom, |_p: &mut Player| {
        _p.take_damage(10.0);
        Ok(println!("aua moja głowa, hp: {}", _p.hp))
    });
    bounce_platform.set_action(Top, |p: &mut Player| {
        p.physics.y -= 100.0;
        Ok(())
    });

    // Tworzenie przedmiotów
    let mut item1 = Item::new(100.0, 500.0, 30.0, 30.0, |_p: &mut Player| {
        Ok(println!("Znalazłeś monetę!"))
    });
    let mut item2 = Item::new(400.0, 400.0, 40.0, 40.0, |_p: &mut Player| {
        Ok(println!("Znalazłeś bonus!"))
    });

    // Tworzenie stworzeń
    let creature1_moves = vec![(0.0, -50.0), (-100.0, 0.0), (0.0, 50.0), (100.0, 0.0)];
    let mut creature1 = Creature::new(600.0, config.screen_height / 2.0, 50.0, 50.0, creature1_moves, 5.0, |_p: &mut Player| {
        println!("auuuu! Stworzenie cię ugryzło i umarło.");
        Ok(false)
    });

    let creature2_moves = vec![(50.0, 0.0), (0.0, -50.0), (-50.0, 0.0), (0.0, 50.0)];
    let mut creature2 = Creature::new(200.0, 300.0, 20.0, 20.0, creature2_moves, 3.0, |_p: &mut Player| {
        println!("Stworzenie cię zepchnęło!");
        _p.physics.vx -= 5.0;
        Ok(true)
    });

    // Tworzenie gry i dodawanie elementów
    let mut game = Game::new(player, config.clone())?;
    game.add_platform(platform1);
    game.add_platform(platform2);
    game.add_platform(bounce_platform);
    game.add_item(item1);
    game.add_item(item2);
    game.add_creature(creature1);
    game.add_creature(creature2);

    event::run(ctx, event_loop, game)
}
