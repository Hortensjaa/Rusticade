#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rusticade::{config::Config, creatures::creature::Creature};

    fn test_creature_moves_returns_to_start(vec: Vec<(f32, f32)>, steps: i32) {
        let config = Arc::new(Config {
            delta_time: 0.1,
            ..Default::default()
        });
        let mut creature = Creature::new(
            0.0, 0.0, 10.0, 10.0,
            vec, 50.0, |_| Ok(()),config,
        );

        for _ in 0..steps {
            creature.update().unwrap();
            if creature.x.abs() <= creature.vx.abs() / 2.0 && creature.y.abs() <= creature.vy.abs() / 2.0 {
                break;
            }
        }

        assert!((creature.x).abs() <= creature.vy.abs() / 2.0, "Creature did not return to starting x position");
        assert!((creature.y).abs() <= creature.vx.abs() / 2.0, "Creature did not return to starting y position");
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
