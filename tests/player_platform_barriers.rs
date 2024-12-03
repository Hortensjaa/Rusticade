#[cfg(test)]
mod tests {
    use rusticade::objects::platform::Platform;
    use rusticade::shared::directions::Direction::*;
    use rusticade::player::player::Player;

    #[test]
    fn test_platform_action_top() {
        let mut player = Player::default();
        player.score = 0.0;
        let mut platform = Platform::default();
        platform.x = 0.0;
        platform.y = 200.0;
        
        platform.set_action(Top, |player| {
            player.score += 10.0;
            Ok(())
        });

        let mut items = vec![];
        let mut creatures = vec![];
        while !player.physics.on_ground {
            let _ = player.update(&[platform.clone()], &mut items, &mut creatures);
        }
        
        assert_eq!(player.score, 10.0); 
    }

    #[test]
    fn test_platform_action_left() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 0.0; 
        player.physics.y = player.get_config().screen_height;
        player.physics.vx = 5.0; 
        player.score = 10.0;

        platform.set_action(Left, |player| {
            player.score *= 2.0;
            Ok(())
        });

        platform.barriers.insert(Left); 
        let mut items = vec![];
        let mut creatures = vec![];

        while player.physics.vx > 0.0 {
            let _ = player.update(&[platform.clone()], &mut items, &mut creatures);
        }

        assert_eq!(player.score, 20.0);
    }

    #[test]
    fn test_platform_action_right() {
        let mut player = Player::default();
        let mut platform = Platform::default();
        platform.x = 100.0;
        platform.y = 550.0;
        player.physics.x = 200.0; 
        player.physics.y = player.get_config().screen_height;
        player.physics.vx = -5.0; 
        player.score = 10.0;

        platform.barriers.insert(Right);

        platform.set_action(Right, |player| {
            player.score *= 2.0;
            Ok(())
        });

        let mut items = vec![]; 
        let mut creatures = vec![];

        while player.physics.vx < 0.0 {
            let _ = player.update(&[platform.clone()], &mut items, &mut creatures);
        }

        assert_eq!(player.score, 20.0);
    }

    #[test]
    fn test_no_action_if_no_collision() {
        let mut player = Player::default();
        player.score = 0.0;
        let mut platform = Platform::default();
        platform.x = 1000.0; 
        platform.y = 1000.0;

        platform.set_action(Top, |player| {
            player.score += 100.0;
            Ok(())
        });

        let mut items = vec![];
        let mut creatures = vec![];
        let _ = player.update(&[platform.clone()], &mut items, &mut creatures);

        assert_eq!(player.score, 0.0); 
    }
}
