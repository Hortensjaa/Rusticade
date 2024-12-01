#[cfg(test)]
mod tests {
    use rusticade::classes::{player::Player, platform::Platform};
    use rusticade::game::game::Game;
    use rusticade::classes::directions::Direction;

    #[test]
    fn test_default_platform() {
        let platform = Platform::default();
        assert_eq!(platform.physics.x, 0.0);
        assert_eq!(platform.physics.y, 0.0);
        assert_eq!(platform.physics.w, 50.0);
        assert_eq!(platform.physics.h, 50.0);

        assert!(platform.on_top.is_none());
        assert!(platform.on_bottom.is_none());
        assert!(platform.on_right.is_none());
        assert!(platform.on_left.is_none());
        assert!(platform.on_collision.is_none());
    }

    #[test]
    fn test_new_platform() {
        let platform = Platform::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(platform.physics.x, 10.0);
        assert_eq!(platform.physics.y, 20.0);
        assert_eq!(platform.physics.w, 100.0);
        assert_eq!(platform.physics.h, 50.0);
    }

    #[test]
    fn test_set_action_top() {
        let mut platform = Platform::default();
        let mut game = Game::default(); 
        let player = Player::default(); 

        use std::rc::Rc;
        use std::cell::RefCell;

        let action_called = Rc::new(RefCell::new(false));
        let action_called_clone = Rc::clone(&action_called);
        platform.set_action(Direction::Top, Box::new(move |_game: &mut Game, _player: Player| {
            *action_called_clone.borrow_mut() = true;
        }));

        if let Some(action) = platform.on_top.as_mut() {
            action(&mut game, player);
        }

        assert!(*action_called.borrow());
    }

    #[test]
    fn test_set_action_collision() {
        let mut platform = Platform::default();
        let mut game = Game::default();
        let player = Player::default();

        use std::rc::Rc;
        use std::cell::RefCell;

        let collision_called = Rc::new(RefCell::new(false));
        let collision_called_clone = Rc::clone(&collision_called);
        platform.set_action(Direction::Collision, Box::new(move |_game: &mut Game, _player: Player| {
            *collision_called_clone.borrow_mut() = true;
        }));

        if let Some(action) = platform.on_collision.as_mut() {
            action(&mut game, player);
        }

        assert!(*collision_called.borrow());
    }
}
