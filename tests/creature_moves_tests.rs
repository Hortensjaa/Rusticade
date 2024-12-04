#[cfg(test)]
mod tests {
    use rusticade::creatures::creature::Creature;

    fn test_creature_moves_returns_to_start(vec: Vec<(f32, f32)>, steps: i32) {

        let mut creature = Creature::new(
            0.0, 0.0, 10.0, 10.0,
            vec, 50.0, Box::new(|_, _| Ok(true)),
        );

        for _ in 0..steps {
            creature.update().unwrap();
            if creature.physics.x.abs() <= creature.physics.vx.abs() / 2.0 && creature.physics.y.abs() <= creature.physics.vy.abs() / 2.0 {
                break;
            }
        }

        assert!((creature.physics.x).abs() <= creature.physics.vy.abs() / 2.0, "Creature did not return to starting x position");
        assert!((creature.physics.y).abs() <= creature.physics.vx.abs() / 2.0, "Creature did not return to starting y position");
    }

    #[test]
    fn test_creature_moves_returns_to_start_case1() {
        test_creature_moves_returns_to_start(vec![(100.0, 0.0), (-100.0, 0.0)], 200);
    }

    #[test]
    fn test_creature_moves_returns_to_start_case2() {
        test_creature_moves_returns_to_start(vec![(0.0, 50.0), (0.0, -50.0)], 200);
    }

    #[test]
    fn test_creature_moves_returns_to_start_case3() {
        test_creature_moves_returns_to_start(vec![(100.0, 50.0), (-100.0, -50.0)], 200);
    }

    #[test]
    fn test_creature_moves_returns_to_start_case4() {
        test_creature_moves_returns_to_start(vec![(100.0, -50.0), (-100.0, 50.0)], 200);
    }

    #[test]
    fn test_creature_moves_returns_to_start_case5() {
        test_creature_moves_returns_to_start(vec![(0.0, -50.0), (-100.0, 0.0), (0.0, 50.0), (100.0, 0.0)], 600);
    }
}
