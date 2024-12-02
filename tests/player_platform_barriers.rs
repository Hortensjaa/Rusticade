#[cfg(test)]
mod tests {
    use rusticade::objects::platform::Platform;
    use rusticade::physics::directions::Direction::*;
    use rusticade::player::player::Player;

    #[test]
    fn test_set_barrier() {
        let mut platform = Platform::new(0.0, 0.0, 50.0, 50.0);
        platform.set_barrier(Left, true);
        assert!(platform.barriers.contains(&Left));
        platform.set_barrier(Left, false);
        assert!(!platform.barriers.contains(&Left));
    }

    #[test]
    fn test_player_bounce_on_top_barrier() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 0.0;
        platform.y = 200.0;
        player.physics.x = 0.0;
        player.physics.y = 100.0;
        while !player.physics.on_ground {
            let _ = player.update(&[platform.clone()]);
        }
        
        assert_eq!(player.physics.y, platform.y - platform.h);
        assert_eq!(player.physics.on_ground, true); 
        assert_eq!(player.physics.vy, 0.0);
    }

    #[test]
    fn test_player_no_bounce_when_no_top_barrier() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 0.0;
        platform.y = 200.0;
        player.physics.x = 0.0;
        player.physics.y = 100.0;
        platform.barriers.remove(&Top);
        while !player.physics.on_ground {
            let _ = player.update(&[platform.clone()]);
        }
        
        assert_eq!(player.physics.y + player.physics.h, player.config.screen_height);
        assert_eq!(player.physics.on_ground, true); 
        assert_eq!(player.physics.vy, 0.0);
    }

    #[test]
    fn test_player_bounce_on_left_barrier() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 0.0; 
        player.physics.y = player.config.screen_height;
        player.physics.vx = 5.0; 

        platform.barriers.insert(Left); 

        while player.physics.vx > 0.0 {
            let _ = player.update(&[platform.clone()]);
        }

        assert_eq!(player.physics.x + player.physics.w, platform.x);
        assert_eq!(player.physics.vx, 0.0); 
    }


    #[test]
    fn test_player_bounce_on_right_barrier() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 200.0; 
        player.physics.y = player.config.screen_height;
        player.physics.vx = -5.0; 

        platform.barriers.insert(Right); 

        while player.physics.vx < 0.0 {
            let _ = player.update(&[platform.clone()]);
        }

        assert_eq!(player.physics.x, platform.x + platform.w);
        assert_eq!(player.physics.vx, 0.0); 
    }

    #[test]
    fn test_player_no_left_barrier() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 0.0; 
        player.physics.y = player.config.screen_height;
        player.physics.vx = 5.0; 

        while player.physics.x + player.physics.w < player.config.screen_width {
            let _ = player.update(&[platform.clone()]);
        }

        assert!(player.physics.x + player.physics.w >= player.config.screen_width);
    }


    #[test]
    fn test_player_no_right_barrier() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 200.0; 
        player.physics.y = player.config.screen_height;
        player.physics.vx = -5.0; 

        
        while player.physics.x > 0.0 {
            let _ = player.update(&[platform.clone()]);
        }

        assert!(player.physics.x <= 0.0);
    }

    #[test]
    fn test_player_bottom_hit() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 0.0;
        platform.y = player.config.screen_height - 100.0;
        player.physics.x = 0.0; 
        player.physics.y = player.config.screen_height;
        player.physics.vy = -100.0;
        platform.barriers.insert(Bottom);
        
        while !player.physics.on_ground {
            assert!(player.physics.y - player.physics.h >= player.config.screen_height - 110.0);
            let _ = player.update(&[platform.clone()]);
            player.physics.vy = -100.0;
        }
    }

    #[test]
    fn test_player_no_bottom_hit() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 0.0;
        platform.y = player.config.screen_height - 100.0;
        player.physics.x = 0.0; 
        player.physics.y = player.config.screen_height;
        player.physics.vy = -100.0;
        
        while !player.physics.on_ground {
            let _ = player.update(&[platform.clone()]);
            player.physics.vy = -100.0;

            if player.physics.y - player.physics.h > player.config.screen_height - 100.0 {
                break;
            }
        }
    }
   
}
