macro_rules! create_creature {
    ($x:expr, $y:expr) => {
        crate::creatures::creature::Creature::new($x, $y, 50.0, 50.0, Vec::new(), 5.0, |_| Ok(true))
    };

    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        crate::creatures::creature::Creature::new($x, $y, $w, $h, Vec::new(), 5.0, |_| Ok(true))
    };

    ($x:expr, $y:expr, $moves:expr) => {
        crate::creatures::creature::Creature::new($x, $y, 50.0, 50.0, $moves, 5.0, |_| Ok(true))
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $moves:expr) => {
        crate::creatures::creature::Creature::new($x, $y, $w, $h, $moves, 5.0, |_| Ok(true))
    };
}

#[allow(unused_imports)]
pub(crate) use create_creature;


#[cfg(test)]
mod tests {
    use crate::creatures::creature::Creature;

    use super::*;

    #[test]
    fn test_create_creature_default_size_empty_moves() {
        let creature = create_creature!(100.0, 200.0);

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 50.0);
        assert_eq!(creature.physics.h, 50.0);
        assert!(creature.physics.moves.is_empty());
    }

    #[test]
    fn test_create_creature_custom_size_empty_moves() {
        let creature = create_creature!(100.0, 200.0, 75.0, 100.0);

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 75.0);
        assert_eq!(creature.physics.h, 100.0);
        assert!(creature.physics.moves.is_empty());
    }

    #[test]
    fn test_create_creature_default_size_with_moves() {
        let moves = vec![(0.0, -100.0), (-200.0, 0.0)];
        let mut m = moves.clone();
        m.rotate_right(1);
        let creature = Creature::new(100.0, 200.0, 50.0, 50.0, moves.clone(), 200.0, |_| Ok(true));

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 50.0);
        assert_eq!(creature.physics.h, 50.0);
        assert_eq!(creature.physics.moves, m);
    }

    #[test]
    fn test_create_creature_custom_size_with_moves() {
        let moves = vec![(0.0, -100.0), (-200.0, 0.0)];
        let creature = Creature::new(100.0, 200.0, 75.0, 100.0, moves.clone(), 200.0, |_| Ok(true));

        let mut m = moves.clone();
        m.rotate_right(1);
        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 75.0);
        assert_eq!(creature.physics.h, 100.0);
        assert_eq!(creature.physics.moves, m);
    }
}

