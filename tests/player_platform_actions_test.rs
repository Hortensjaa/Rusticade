#[cfg(test)]
mod tests {
    use rusticade::classes::{player::*, platform::*, directions::Direction::*};

    // Test, czy platforma wykonuje akcję
    #[test]
    fn test_set_and_do_action() {
        let mut platform = Platform::new(0.0, 0.0, 50.0, 50.0);
        let mut player = Player::default();
        
        // Definiujemy akcję, która zmienia pozycję gracza
        let action = |player: &mut Player| {
            player.physics.x += 10.0;
            Ok(())
        };

        // Dodajemy akcję do platformy w kierunku 'Top'
        platform.set_action(Top, action);
        
        // Sprawdzamy przed wykonaniem akcji
        assert_eq!(player.physics.x, 0.0);
        
        // Wykonujemy akcję
        platform.do_action(&Top, &mut player).unwrap();
        
        // Sprawdzamy po wykonaniu akcji
        assert_eq!(player.physics.x, 10.0);
    }
}
