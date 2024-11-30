use crate::classes::{platform::Platform, player::Player};

use super::logic::Game;

impl Game {
    pub fn add_platform(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let platform = Platform::new_normal(x, y, w, h);
        self.platforms.push(platform);
    }

    pub fn add_platform_default_size(&mut self, x: f32, y: f32) {
        let platform = Platform::new_normal(
            x, y, self.config.grid_cell_width, self.config.grid_cell_height
        );
        self.platforms.push(platform);
    }

    pub fn add_platform_with_effect(&mut self, x: f32, y: f32, effect: fn(Player) -> ()) {
        let platform = Platform::new_special(
            x, y, self.config.grid_cell_width, self.config.grid_cell_height, effect
        );
        self.platforms.push(platform);
    }        
    
    fn finish_action(game: &mut Game, _p: Player) {
            game.quitted = true;
    }

    pub fn add_finish_platform(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let platform = Platform::new_finish(
            x, y, w, h, 
            Box::new(|game: &mut Game, player: Player| Game::finish_action(game, player))
        );
        self.platforms.push(platform);
    }
}