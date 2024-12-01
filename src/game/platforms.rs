use crate::classes::{directions::Direction, platform::Platform, player::Player};

use super::game::Game;

impl Game {

    pub fn add_platform(&mut self, platform: Platform) {
        self.platforms.push(platform);
    }

    pub fn add_platform_custom_size(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let platform = Platform::new(x, y, w, h);
        self.platforms.push(platform);
    }

    pub fn add_platform_default_size(&mut self, x: f32, y: f32) {
        let platform = Platform::new(
            x, y, self.config.grid_cell_width, self.config.grid_cell_height
        );
        self.platforms.push(platform);
    }   
    
    fn finish_action(game: &mut Game, _p: Player) {
            game.quitted = true;
    }

    pub fn add_finish_platform(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let action = Box::new(|game: &mut Game, player: Player| Game::finish_action(game, player));
        let mut platform = Platform::new(x, y, w, h);
        platform.set_action(Direction::Collision, action);
        self.platforms.push(platform);
    }
}