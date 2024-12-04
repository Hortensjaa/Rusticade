use std::sync::Arc;
use ggez::{event, GameResult};

use rusticade::shared::customisable::Customisable;
use rusticade::{create_game_context, create_creature, create_player};
use rusticade::creatures::creature::Creature;
use rusticade::objects::{item::Item, platform::Platform};
use rusticade::shared::{directions::Direction::*, config::Config};
use rusticade::player::player::Player;
use rusticade::game::game::Game;


pub fn main() -> GameResult {
    // function that will be used in every iteration of game loop
    let wind_from_west = |game: &mut Game|{
        game.player.physics.x += 1.0; 
        Ok(())
    };

    // configuration
    let config = Arc::new(Config::default());
    let (ctx, event_loop) = create_game_context!("Moja gra", "Julia Kulczycka", config.clone())?;

    // creating player
    let mut player = create_player!(200.0, config.screen_height - 50.0, config.clone());
    // modifying player
    player.physics.speed = 5.0;
    player.physics.jump = 12.0;


    // generationg platforms
    // first platform with barriers on top and sides
    let mut platform1 = Platform::new(100.0, 550.0, 150.0, 20.0);
    platform1.set_barrier(Left, true);
    platform1.set_barrier(Right, true);

    // platform with counter of hits
    let mut counter_platform = Platform::new(300.0, 450.0, 100.0, 20.0);
    let _ = counter_platform.update_property("counter", 0.0);
    counter_platform.set_action(Top, Box::new(|p: &mut Platform, _: &mut Player| {
        let c = p.get_property("counter");
        p.update_property("counter", c + 1.0);
        print!("Counter set to {}", c + 1.0);
        Ok(())
    }));

    // bounce platform, working like trampoline
    let mut bounce_platform = Platform::new(250.0, 320.0, 80.0, 20.0);
    bounce_platform.set_action(Top, Box::new(|_: &mut Platform, p: &mut Player| {
        p.physics.y -= 10.0;
        p.physics.vy -= 10.0;
        Ok(())
    }));

    // hit head platform - if player hit head too many times, game will end
    let mut hit_platform = Platform::new(350.0, 320.0, 80.0, 20.0);
    hit_platform.set_barrier(Bottom, true);
    hit_platform.set_action(Bottom, Box::new(|_: &mut Platform, p: &mut Player| {
        p.take_damage(10.0);
        Ok(println!("Ouch, my head - {} hp left", p.hp))
    }));

    // platforms don't have initiallised moveset like creatures, but we can add them move logic
    let mut moving_platform = Platform::new(350.0, config.screen_height - 100.0, 20.0, 100.0);
    moving_platform.set_action(Left, Box::new(|p: &mut Platform, _: &mut Player| {
        p.x += 10.0;
        Ok(println!("Im moving!: {} {}", p.x, p.y))
    }));

    // creating items
    let bonus = Item::new(100.0, 500.0, 30.0, 30.0, |p: &mut Player| {
        p.add_score(10.0);
        Ok(println!("Earned 10 points"))
    });
    let bomb = Item::new(400.0, 400.0, 40.0, 40.0, |p: &mut Player| {
        p.take_damage(20.0);
        p.physics.vx += 10.0;
        p.physics.vy += 10.0;
        Ok(println!("It was a bomb, {} hp left!", p.hp))
    });

    // Generating creatures
    // this one will consume players hp and then die, because function return false
    let creature1_moves = vec![(0.0, -50.0), (-100.0, 0.0), (0.0, 50.0), (100.0, 0.0)];
    let creature1 = Creature::new(
        600.0, config.screen_height / 2.0, 50.0, 50.0, creature1_moves, 5.0, Box::new(|_: &mut Creature, p: &mut Player| {
        p.hp -= 5.0;
        println!("Ouch, this creature bit me - good thing it's gone");
        Ok(false)
    }));

    // this creature will hit player and change his move direction
    let creature2_moves = vec![(100.0, 50.0), (-100.0, -50.0)];
    let creature2 = Creature::new(200.0, 300.0, 20.0, 20.0, creature2_moves, 3.0, 
        Box::new(|_: &mut Creature,_p: &mut Player| {
        println!("Why did you hit me??");
        _p.physics.vx -= 5.0;
        Ok(true)
    }));

    // creature also can not move (or can start moving after trigger)
    let mut creature3 = create_creature!(100.0, 200.0, 75.0, 100.0);
    creature3.action = Box::new(|c: &mut Creature, _: &mut Player| {
        println!("Why didn't you let me rest?");
        c.physics.moves = Vec::from([(20.0, -10.0), (-20.0, 10.0)]);
        Ok(true)
    });

    // creating game object and adding objects
    let mut game = Game::new(player, config.clone())?;

    game.add_platform(platform1);
    game.add_platform(counter_platform);
    game.add_platform(bounce_platform);
    game.add_platform(hit_platform);
    game.add_platform(moving_platform);
    game.add_platform_default_size(500.0, 500.0); // also we can add platform directly to game

    game.add_item(bonus);
    game.add_item(bomb);

    game.add_creature(creature1);
    game.add_creature(creature2);
    game.add_creature(creature3);

    game.action_before = Some(Box::new(wind_from_west)); // this action will happen every time at the beginning of game loop

    event::run(ctx, event_loop, game)
}
