macro_rules! create_player {
    ($x:expr, $y:expr, $config:expr) => {
        crate::player::player::Player::new($x, $y, 50.0, 50.0, 100.0, 400.0, 100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $config:expr) => {
        crate::player::player::Player::new($x, $y, $w, $h, 100.0, 400.0,  100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $hp:expr, $config:expr) => {
        crate::player::player::Player::new($x, $y, $w, $h, 100.0, 400.0, $hp, $config)
    };

    ($x:expr, $y:expr, $hp:expr, $config:expr) => {
        crate::player::player::Player::new($x, $y, 50.0, 50.0, $hp, 400.0, $hp, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr,  $speed:expr, $jump:expr, $config:expr) => {
        crate::player::player::Player::new($x, $y, $w, $h, 100.0, 400.0, 100.0, $config)
    };
    
    ($x:expr, $y:expr, $w:expr, $h:expr, $speed:expr, $jump:expr, $hp:expr, $config:expr) => {
        crate::player::player::Player::new($x, $y, $w, $h, $speed, $jump, $hp, $config)
    };
}

#[allow(unused_imports)]
pub(crate) use create_player;


#[cfg(test)]
mod tests {
    use crate::{player::player::Player, shared::config::Config};

    use super::*;
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

