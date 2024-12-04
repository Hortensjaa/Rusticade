#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ggez::GameError;
    use rusticade::player::player::Player;
    use rusticade::creatures::creature::Creature;
    use rusticade::shared::collidable::Collidable;
    use rusticade::shared::config::Config;

    fn dummy_action(_: &mut Creature, player: &mut Player) -> Result<bool, GameError> {
        player.hp -= 10.0;
        Ok(true)
    }

    #[test]
    fn test_creature_action_triggered_once() {
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 20.0, 5.0, 100.0,
             Arc::new(Config {gravity: 0.0, ..Config::default()}));
        let creature = Creature::new(
            60.0,
            0.0,
            50.0,
            50.0,
            vec![],
            0.0,
            Box::new(dummy_action),
        );

        let mut creatures = vec![creature];

        // before collison
        assert_eq!(player.hp, 100.0);
        assert!(!creatures[0].triggered);

        // first collision
        let _ = player.move_right();
        let _ = player.update(&mut vec![], &mut vec![], &mut creatures);
        assert!(player.is_colliding_with(&creatures[0]));
        assert_eq!(player.hp, 90.0); 

        // next update - no action
        let _ = player.update(&mut vec![], &mut vec![], &mut creatures);
        assert!(player.is_colliding_with(&creatures[0]));
        assert_eq!(player.hp, 90.0); 

        // go out of collision
        let _ = player.move_left();
        let _ = player.update(&mut vec![], &mut vec![], &mut creatures);
        let _ = player.move_left();
        let _ = player.update(&mut vec![], &mut vec![], &mut creatures);
        assert!(!player.is_colliding_with(&creatures[0]));

        // going back to collision
        let _ = player.move_right();
        let _ = player.update(&mut vec![], &mut vec![], &mut creatures);
        assert!(player.is_colliding_with(&creatures[0]));
        assert_eq!(player.hp, 80.0); // action done again
    }

}
