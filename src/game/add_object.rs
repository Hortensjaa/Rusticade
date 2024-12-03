use ggez::GameError;

use crate::creatures::creature::Creature;
use crate::player::player::Player;
use crate::objects::{platform::Platform, item::Item};
use crate::shared::directions::Direction;

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
            x, y, self.get_config().grid_cell_width, self.get_config().grid_cell_height
        );
        self.platforms.push(platform);
    }   

    pub fn add_finish_platform(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let mut platform = Platform::new(x, y, w, h);
        platform.set_action(Direction::Top, |_p: &mut Player| {
            Err(GameError::CustomError(String::from("Finish platform action")))
        });
        self.platforms.push(platform);
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn add_item_custom_size(&mut self, x: f32, y: f32, w: f32, h: f32, action: fn(&mut Player) -> Result<(), GameError>) {
        let item = Item::new(
            x, y, w, h, action
        );
        self.items.push(item);
    }

    pub fn add_item_default_size(&mut self, x: f32, y: f32, action: fn(&mut Player) -> Result<(), GameError>) {
        let item = Item::new(
            x, y, self.get_config().grid_cell_width, self.get_config().grid_cell_height, action
        );
        self.items.push(item);
    }    

    pub fn add_creature(&mut self, creature: Creature) {
        self.creatures.push(creature);
    }
}