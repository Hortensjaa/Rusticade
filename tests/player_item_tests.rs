#[cfg(test)]
mod tests {
    use rusticade::objects::item::Item;
    use rusticade::player::player::Player;

    #[test]
    fn test_item_action_executes() {
        let mut player = Player::default();
        player.physics.x = 10.0;
        player.physics.y = 10.0;

        let item = Item::new(
            10.0,
            10.0,
            10.0,
            10.0,
            |player: &mut Player| {
                player.hp = 200.0; 
                Ok(())
            }
        );

        let mut items = vec![item];
        let mut creatures = vec![]; 

        player.update(&mut vec![], &mut items, &mut creatures).unwrap();

        assert_eq!(player.hp, 200.0);
        assert!(items.is_empty());
    }

    #[test]
    fn test_no_action_if_no_collision() {
        let mut player = Player::default();
        player.physics.x = 100.0;
        player.physics.y = 10.0;

        let item = Item::new(
            10.0,
            10.0,
            10.0,
            10.0,
            |player: &mut Player| {
                player.hp = 200.0; 
                Ok(())
            }
        );

        let mut items = vec![item];
        let mut creatures = vec![]; 

        player.update(&mut vec![], &mut items, &mut creatures).unwrap();
        assert_ne!(player.hp, 200.0);
        assert_eq!(items.len(), 1);
    }
}
