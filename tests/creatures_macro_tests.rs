#[cfg(test)]
mod tests {
    use rusticade::{creatures::creature::Creature, create_creature};


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
        let creature = Creature::new(100.0, 200.0, 50.0, 50.0, moves.clone(), 200.0, Box::new(|_, _| Ok(true)));

        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 50.0);
        assert_eq!(creature.physics.h, 50.0);
        assert_eq!(creature.physics.moves, m);
    }

    #[test]
    fn test_create_creature_custom_size_with_moves() {
        let moves = vec![(0.0, -100.0), (-200.0, 0.0)];
        let creature = Creature::new(100.0, 200.0, 75.0, 100.0, moves.clone(), 200.0, Box::new(|_, _| Ok(true)));

        let mut m = moves.clone();
        m.rotate_right(1);
        assert_eq!(creature.physics.x, 100.0);
        assert_eq!(creature.physics.y, 200.0);
        assert_eq!(creature.physics.w, 75.0);
        assert_eq!(creature.physics.h, 100.0);
        assert_eq!(creature.physics.moves, m);
    }
}