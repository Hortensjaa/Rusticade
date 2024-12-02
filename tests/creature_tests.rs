#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rusticade::{config::Config, creatures::creature::Creature};

    fn test_creature_moves_returns_to_start(vec: Vec<(f32, f32)>, steps: i32) {
        let config = Arc::new(Config {
            delta_time: 0.1,
            ..Default::default()
        });
        let mut creature = Creature {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 10.0,
            action: |_| Ok(()),
            moves: vec,
            speed: 50.0,
            config,
            ..Default::default()
        };

        for _ in 0..steps {
            creature.update().unwrap();
        }

        assert!((creature.x - 0.0).abs() < 1e-5, "Creature did not return to starting x position");
        assert!((creature.y - 0.0).abs() < 1e-5, "Creature did not return to starting y position");
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
}
