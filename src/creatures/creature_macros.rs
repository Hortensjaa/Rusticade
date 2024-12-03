macro_rules! create_creature {
    ($x:expr, $y:expr, $config:expr) => {
        crate::creatures::creature::Creature::new($x, $y, 50.0, 50.0, Vec::new(), 200.0, |_| Ok(true), $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $config:expr) => {
        crate::creatures::creature::Creature::new($x, $y, $w, $h, Vec::new(), 200.0, |_| Ok(true), $config)
    };

    ($x:expr, $y:expr, $moves:expr, $config:expr) => {
        crate::creatures::creature::Creature::new($x, $y, 50.0, 50.0, $moves, 200.0, |_| Ok(true), $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $moves:expr, $config:expr) => {
        crate::creatures::creature::Creature::new($x, $y, $w, $h, $moves, 200.0, |_| Ok(true), $config)
    };
}

#[allow(unused_imports)]
pub(crate) use create_creature;


#[cfg(test)]
mod tests {
    use crate::{creatures::creature::Creature, shared::config::Config};

    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_create_creature_default_size_empty_moves() {
        let config = Arc::new(Config::default());
        let creature = create_creature!(100.0, 200.0, config.clone());

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 50.0);
        assert_eq!(creature.physics.h, 50.0);
        assert!(creature.physics.moves.is_empty());
    }

    #[test]
    fn test_create_creature_custom_size_empty_moves() {
        let config = Arc::new(Config::default());
        let creature = create_creature!(100.0, 200.0, 75.0, 100.0, config.clone());

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 75.0);
        assert_eq!(creature.physics.h, 100.0);
        assert!(creature.physics.moves.is_empty());
    }

    #[test]
    fn test_create_creature_default_size_with_moves() {
        let config = Arc::new(Config::default());
        let moves = vec![(0.0, -100.0), (-200.0, 0.0)];
        let mut m = moves.clone();
        m.rotate_right(1);
        let creature = Creature::new(100.0, 200.0, 50.0, 50.0, moves.clone(), 200.0, |_| Ok(true), config.clone());

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 50.0);
        assert_eq!(creature.physics.h, 50.0);
        assert_eq!(creature.physics.moves, m);
    }

    #[test]
    fn test_create_creature_custom_size_with_moves() {
        let config = Arc::new(Config::default());
        let moves = vec![(0.0, -100.0), (-200.0, 0.0)];
        let creature = Creature::new(100.0, 200.0, 75.0, 100.0, moves.clone(), 200.0, |_| Ok(true), config.clone());

        let mut m = moves.clone();
        m.rotate_right(1);
        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 75.0);
        assert_eq!(creature.physics.h, 100.0);
        assert_eq!(creature.physics.moves, m);
    }
}

