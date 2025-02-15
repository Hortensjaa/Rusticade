#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rusticade::shared::config::Config;
    use rusticade::player::player::Player;
    use rusticade::shared::customisable::Customisable;

    fn mock_config() -> Arc<Config> {
        Arc::new(Config::default())
    }

    #[test]
    fn test_player_creation() {
        let config = mock_config();
        let player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 100.0, config);

        assert_eq!(player.hp, 100.0, "Player HP should be 100.0");
        assert_eq!(player.score, 0.0, "Player score should be 0.0");
        assert_eq!(player.physics.x, 0.0, "Player X position should be 0.0");
        assert_eq!(player.physics.y, 0.0, "Player Y position should be 0.0");
    }

    #[test]
    fn test_update_player_position() {
        let config = mock_config();
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 100.0, config);
        let result = player.update(&mut vec![], &mut vec![], &mut vec![]);

        assert!(result.is_ok(), "Update should succeed");
    }

    #[test]
    fn test_player_move_right() {
        let mut player = Player::new(10.0, 20.0, 50.0, 50.0, 100.0, 300.0, 100.0, Arc::new(Config::default()));

        player.move_right().unwrap();
        let _ = player.update(&mut vec![], &mut vec![], &mut vec![]);  

        assert!(player.physics.x > 10.0); 
    }

    #[test]
    fn test_player_move_left() {
        let mut player = Player::new(10.0, 20.0, 50.0, 50.0, 100.0, 300.0, 100.0, Arc::new(Config::default()));

        player.move_left().unwrap();
        let _ = player.update(&mut vec![], &mut vec![], &mut vec![]);  

        assert!(player.physics.x < 10.0);  
    }

    #[test]
    fn test_jump() {
        let config = mock_config();
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 100.0, config);

        let result = player.jump();

        assert!(result.is_ok(), "Jump should succeed");
    }

    #[test]
    fn test_heal() {
        let mut player = Player::default();
        player.hp = 50.0;

        player.heal(20.0);

        assert_eq!(player.hp, 70.0, "Player HP should be 70.0 after healing");
    }

    #[test]
    fn test_take_damage() {
        let mut player = Player::default();
        player.hp = 100.0;

         player.take_damage(30.0);

        assert_eq!(player.hp, 70.0, "Player HP should be 70.0 after taking damage");
    }

    #[test]
    fn test_add_and_get_property() {
        let mut player = Player::default();

        player.update_property("coins", 50.0);

        let value = player.get_property("coins");
        assert_eq!(value, 50.0, "The property value should be 50.0");
    }

    #[test]
    fn test_get_nonexistent_property() {
        let player = Player::default();
        let result = player.get_property("stamina");
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_update_existing_property() {
        let mut player = Player::default();

        player.update_property("coins", 50.0);
        
        let value1 = player.get_property("coins");
        assert_eq!(value1, 50.0, "The updated property value should be 50.0");

        player.update_property("coins", 100.0);

        let value2 = player.get_property("coins");
        assert_eq!(value2, 100.0, "The updated property value should be 100.0");
    }

    #[test]
    fn test_get_property_no_properties() {
        let player = Player::default();
        let result = player.get_property("anything");
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_add_multiple_properties() {
        let mut player = Player::default();

        player.update_property("coins", 50.0);
        player.update_property("stamina", 75.0);

        let coins = player.get_property("coins");
        let stamina = player.get_property("stamina");

        assert_eq!(coins, 50.0, "Coins value should be 50.0");
        assert_eq!(stamina, 75.0, "Stamina value should be 75.0");
    }

    #[test]
    fn test_negative_property_values() {
        let mut player = Player::default();

        player.update_property("health_penalty", -10.0);

        let value = player.get_property("health_penalty");
        assert_eq!(value, -10.0, "Negative values should be stored correctly");
    }
}
