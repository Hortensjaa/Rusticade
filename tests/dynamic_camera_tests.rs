#[cfg(test)]
mod test {
    use rusticade::objects::platform::Platform;
    use rusticade::player::player::Player;

    // Helper function to create a default player for testing
    fn create_default_player() -> Player {
        Player::default()
    }

    fn create_default_platform() -> Platform {
        Platform::default()
    }

    // Test movement
    #[test]
    fn test_move_right() {
        let mut player = create_default_player();
        player.physics.x = 100.0;
        player.physics.speed = 100.0;
        let _ = player.move_right();
        assert_eq!(player.physics.vx, player.physics.speed);

        let _ = player.update(&[]);
        assert_eq!(player.physics.x, 100.0 + player.physics.speed * player.config.delta_time);
    }

    #[test]
    fn test_move_left() {
        let mut player = create_default_player();
        player.physics.x = 100.0;
        player.physics.speed = 100.0;
        let _ = player.move_left();
        assert_eq!(player.physics.vx, -player.physics.speed);

        let _ = player.update(&[]);
        assert_eq!(player.physics.x, 100.0 - player.physics.speed * player.config.delta_time);
    }

    #[test]
    fn test_stop() {
        let mut player = create_default_player();
        player.physics.x = 100.0;
        player.physics.speed = 100.0;
        let _ = player.move_right();
        let _ = player.update(&[]);

        let _ = player.stop();
        let _ = player.update(&[]);

        assert_eq!(player.physics.vx, 0.0);
        assert_eq!(player.physics.x, 100.0 + player.physics.speed * player.config.delta_time); // No change after stop
    }

    // Test jumping
    #[test]
    fn test_jump() {
        let mut player = create_default_player();
        player.physics.on_ground = true;
        let _ = player.jump();

        assert_eq!(player.physics.vy, -player.physics.jump);
        assert_eq!(player.physics.on_ground, false);
    }

    // Test landing on platform
    #[test]
    fn test_landing_on_platform() {
        let mut player = create_default_player();
        let mut platform = create_default_platform();
        platform.x = 0.0;
        platform.y = 200.0;
        player.physics.x = 0.0;
        player.physics.y = 100.0; // Start above the platform
        while !player.physics.on_ground {
            let _ = player.update(&[platform.clone()]);
        }
        
        assert_eq!(player.physics.y, platform.y - platform.h); // Player should land on top of platform
        assert_eq!(player.physics.on_ground, true); // Player should be marked as on ground
    }

    // Test gravity effect
    #[test]
    fn test_gravity_effect() {
        let mut player = create_default_player();
        player.physics.vy = 0.0; // Initial vertical speed
        let _ = player.update(&[]);

        // After update, vy should increase due to gravity (assuming gravity is negative)
        assert!(player.physics.vy > 0.0);
    }

    // Test that the player does not fall off the screen
    #[test]
    fn test_player_clamp_position() {
        let mut player = create_default_player();
        player.physics.x = 1000.0; // Move player outside the screen to the right
        player.physics.y = 1000.0; // Move player outside the screen to the bottom

        let _ = player.update(&[]);

        assert_eq!(player.physics.x, player.config.screen_width - player.physics.w); // Player should be clamped to the right edge
        assert_eq!(player.physics.y, player.config.screen_height - player.physics.h); // Player should be clamped to the bottom edge
    }

    // Test default values for DynamicPlayer
    #[test]
    fn test_default_dynamic_player() {
        let player = Player::default();

        assert_eq!(player.physics.x, 0.0);
        assert_eq!(player.physics.y, 0.0);
        assert_eq!(player.physics.w, 50.0);
        assert_eq!(player.physics.h, 50.0);
        assert_eq!(player.physics.vx, 0.0);
        assert_eq!(player.physics.vy, 0.0);
        assert_eq!(player.physics.speed, 100.0);
        assert_eq!(player.physics.jump, 400.0);
    }
}
