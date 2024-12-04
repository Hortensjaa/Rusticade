
#[cfg(test)]
mod tests {
    use rusticade::{create_player, player::player::Player, shared::config::Config};

    use std::sync::Arc;

    fn get_default_player() -> Player {
        Player::default()
    }

    #[test]
    fn test_create_player_default_size() {
        let config = Arc::new(Config::default());
        let player = create_player!(100.0, 200.0, config.clone());

        assert_eq!(player.physics.x, 100.0);
        assert_eq!(player.physics.y, 200.0);
        assert_eq!(player.physics.w, 50.0);
        assert_eq!(player.physics.h, 50.0);
        assert_eq!(player.hp, 100.0);
        assert_eq!(player.physics.speed, get_default_player().physics.speed);
        assert_eq!(player.physics.jump, get_default_player().physics.jump);
    }

    #[test]
    fn test_create_player_custom_size() {
        let config = Arc::new(Config::default());
        let player = create_player!(100.0, 200.0, 75.0, 100.0, config.clone());

        assert_eq!(player.physics.x, 100.0);
        assert_eq!(player.physics.y, 200.0);
        assert_eq!(player.physics.w, 75.0);
        assert_eq!(player.physics.h, 100.0);
        assert_eq!(player.hp, 100.0);
        assert_eq!(player.physics.speed, get_default_player().physics.speed);
        assert_eq!(player.physics.jump, get_default_player().physics.jump);
    }

    #[test]
    fn test_create_player_custom_hp() {
        let config = Arc::new(Config::default());
        let player = create_player!(100.0, 200.0, 75.0, 100.0, 150.0, config.clone());

        assert_eq!(player.physics.x, 100.0);
        assert_eq!(player.physics.y, 200.0);
        assert_eq!(player.physics.w, 75.0);
        assert_eq!(player.physics.h, 100.0);
        assert_eq!(player.hp, 150.0);
        assert_eq!(player.physics.speed, get_default_player().physics.speed);
        assert_eq!(player.physics.jump, get_default_player().physics.jump);
    }

    #[test]
    fn test_create_player_custom_hp_no_speed_jump() {
        let config = Arc::new(Config::default());
        let player = create_player!(100.0, 200.0, 75.0, 100.0, 150.0, config.clone());

        assert_eq!(player.physics.x, 100.0);
        assert_eq!(player.physics.y, 200.0);
        assert_eq!(player.physics.w, 75.0);
        assert_eq!(player.physics.h, 100.0);
        assert_eq!(player.hp, 150.0);
        assert_eq!(player.physics.speed, get_default_player().physics.speed);
        assert_eq!(player.physics.jump, get_default_player().physics.jump);
    }

    #[test]
    fn test_create_player_full_custom() {
        let config = Arc::new(Config::default());
        let player = create_player!(100.0, 200.0, 75.0, 100.0, 150.0, 250.0, 120.0, config.clone());

        assert_eq!(player.physics.x, 100.0);
        assert_eq!(player.physics.y, 200.0);
        assert_eq!(player.physics.w, 75.0);
        assert_eq!(player.physics.h, 100.0);
        assert_eq!(player.hp, 120.0);
        assert_eq!(player.physics.speed, 150.0);
        assert_eq!(player.physics.jump, 250.0);
    }
}
