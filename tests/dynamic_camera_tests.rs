#[cfg(test)]
mod test {
    use rusticade::physics::dynamic_player::*;
    use rusticade::physics::static_object::StaticObject;

    // Helper function to create a default player for testing
    fn create_default_player() -> DynamicPlayer {
        DynamicPlayer::default()
    }

    fn create_default_platform() -> StaticObject {
        StaticObject::default()
    }

    // Test movement
    #[test]
    fn test_move_right() {
        let mut player = create_default_player();
        player.x = 100.0;
        player.speed = 100.0;
        player.move_right();
        assert_eq!(player.vx, player.speed);

        player.update(&[]);
        assert_eq!(player.x, 100.0 + player.speed * player.delta_time);
    }

    #[test]
    fn test_move_left() {
        let mut player = create_default_player();
        player.x = 100.0;
        player.speed = 100.0;
        player.move_left();
        assert_eq!(player.vx, -player.speed);

        player.update(&[]);
        assert_eq!(player.x, 100.0 - player.speed * player.delta_time);
    }

    #[test]
    fn test_stop() {
        let mut player = create_default_player();
        player.x = 100.0;
        player.speed = 100.0;
        player.move_right();
        player.update(&[]);

        player.stop();
        player.update(&[]);

        assert_eq!(player.vx, 0.0);
        assert_eq!(player.x, 100.0 + player.speed * player.delta_time); // No change after stop
    }

    // Test jumping
    #[test]
    fn test_jump() {
        let mut player = create_default_player();
        player.on_ground = true;
        player.jump();

        assert_eq!(player.vy, -player.jump);
        assert_eq!(player.on_ground, false);
    }

    // Test landing on platform
    #[test]
    fn test_landing_on_platform() {
        let mut player = create_default_player();
        let mut platform = create_default_platform();
        platform.x = 0.0;
        platform.y = 200.0;
        player.x = 0.0;
        player.y = 100.0; // Start above the platform
        println!("{:#?}", player);
        while !player.on_ground {
            player.update(&[platform.clone()]);
        }
        
        assert_eq!(player.y, platform.clone().y - platform.clone().h); // Player should land on top of platform
        assert_eq!(player.on_ground, true); // Player should be marked as on ground
    }

    // Test gravity effect
    #[test]
    fn test_gravity_effect() {
        let mut player = create_default_player();
        player.vy = 0.0; // Initial vertical speed
        player.update(&[]);

        // After update, vy should increase due to gravity (assuming gravity is negative)
        assert!(player.vy > 0.0);
    }

    // Test that the player does not fall off the screen
    #[test]
    fn test_player_clamp_position() {
        let mut player = create_default_player();
        player.x = 1000.0; // Move player outside the screen to the right
        player.y = 1000.0; // Move player outside the screen to the bottom

        player.update(&[]);

        assert_eq!(player.x, player.config.screen_width - player.w); // Player should be clamped to the right edge
        assert_eq!(player.y, player.config.screen_height - player.h); // Player should be clamped to the bottom edge
    }

    // Test default values for DynamicPlayer
    #[test]
    fn test_default_dynamic_player() {
        let player = DynamicPlayer::default();

        assert_eq!(player.x, 0.0);
        assert_eq!(player.y, 0.0);
        assert_eq!(player.w, 50.0);
        assert_eq!(player.h, 50.0);
        assert_eq!(player.vx, 0.0);
        assert_eq!(player.vy, 0.0);
        assert_eq!(player.speed, 100.0);
        assert_eq!(player.jump, 400.0);
        assert_eq!(player.delta_time, 1.0 / 40.0);
    }
}
