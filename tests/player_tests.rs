#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rusticade::config::Config;
    use rusticade::classes::{player::Player, platform::Platform};

    fn mock_config() -> Arc<Config> {
        Arc::new(Config::default())
    }

    #[test]
    fn test_player_creation() {
        let config = mock_config();
        let player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 1.0, 100.0, config);

        assert_eq!(player.hp, 100.0, "Player HP should be 100.0");
        assert_eq!(player.score, 0.0, "Player score should be 0.0");
        assert_eq!(player.pos.x, 0.0, "Player X position should be 0.0");
        assert_eq!(player.pos.y, 0.0, "Player Y position should be 0.0");
    }

    #[test]
    fn test_update_player_position() {
        let config = mock_config();
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 1.0, 100.0, config);
        let platform = Platform::new(0.0, 0.0, 100.0, 10.0); // mock platform

        let result = player.update(&[platform]);

        assert!(result.is_ok(), "Update should succeed");
    }

    #[test]
    fn test_player_move_right() {
        let mut player = Player::new(10.0, 20.0, 50.0, 50.0, 100.0, 300.0, 1.0 / 60.0, 100.0, Arc::new(Config::default()));

        player.move_right().unwrap();
        let _ = player.update(&[]);  

        assert_eq!(player.pos.x, 10.0 + 100.0 * 1.0 / 60.0); 
    }

    #[test]
    fn test_player_move_left() {
        let mut player = Player::new(10.0, 20.0, 50.0, 50.0, 100.0, 300.0, 1.0 / 60.0, 100.0, Arc::new(Config::default()));

        player.move_left().unwrap();
        let _ = player.update(&[]);  

        assert_eq!(player.pos.x, 10.0 - 100.0 * 1.0 / 60.0);  
    }

    #[test]
    fn test_jump() {
        let config = mock_config();
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 1.0, 100.0, config);

        let result = player.jump();

        assert!(result.is_ok(), "Jump should succeed");
    }

    #[test]
    fn test_heal() {
        let config = mock_config();
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 1.0, 50.0, config);

        let result = player.heal(20.0);

        assert!(result.is_ok(), "Healing should succeed");
        assert_eq!(player.hp, 70.0, "Player HP should be 70.0 after healing");
    }

    #[test]
    fn test_take_damage() {
        let config = mock_config();
        let mut player = Player::new(0.0, 0.0, 50.0, 50.0, 5.0, 10.0, 1.0, 100.0, config);

        let result = player.take_damage(30.0);

        assert!(result.is_ok(), "Taking damage should succeed");
        assert_eq!(player.hp, 70.0, "Player HP should be 70.0 after taking damage");
    }

    #[test]
    fn test_add_and_get_property() {
        let mut player = Player::default();

        player.update_property("coins", 50.0).unwrap();

        let value = player.get_property("coins").unwrap();
        assert_eq!(value, 50.0, "The property value should be 50.0");
    }

    #[test]
    fn test_get_nonexistent_property() {
        let player = Player::default();
        let result = player.get_property("stamina");

        assert!(result.is_err(), "Getting a non-existent property should return an error");
    }

    #[test]
    fn test_update_existing_property() {
        let mut player = Player::default();

        player.update_property("coins", 50.0).unwrap();
        
        let value1 = player.get_property("coins").unwrap();
        assert_eq!(value1, 50.0, "The updated property value should be 50.0");

        player.update_property("coins", 100.0).unwrap();

        let value2 = player.get_property("coins").unwrap();
        assert_eq!(value2, 100.0, "The updated property value should be 100.0");
    }

    #[test]
    fn test_get_property_no_properties() {
        let player = Player::default();

        let result = player.get_property("anything");
        assert!(result.is_err(), "Should return an error when no properties exist");
    }

    #[test]
    fn test_add_multiple_properties() {
        let mut player = Player::default();

        player.update_property("coins", 50.0).unwrap();
        player.update_property("stamina", 75.0).unwrap();

        let coins = player.get_property("coins").unwrap();
        let stamina = player.get_property("stamina").unwrap();

        assert_eq!(coins, 50.0, "Coins value should be 50.0");
        assert_eq!(stamina, 75.0, "Stamina value should be 75.0");
    }

    #[test]
    fn test_negative_property_values() {
        let mut player = Player::default();

        player.update_property("health_penalty", -10.0).unwrap();

        let value = player.get_property("health_penalty").unwrap();
        assert_eq!(value, -10.0, "Negative values should be stored correctly");
    }
}
