use ggez::graphics::{Color, Image};
use ggez::GameError;

use crate::utils::collidable::Collidable;
use crate::player::player::Player;
use crate::utils::drawable::DrawableClass;
use super::object_graphics::StaticGraphics;


#[derive(Clone, Debug)]
pub struct Item {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub action: fn(&mut Player) -> Result<(), GameError>,
    pub graphics: StaticGraphics
}

impl Item {
    pub fn new(x: f32, y: f32, w: f32, h: f32, action:fn(&mut Player) -> Result<(), GameError>) -> Self {
        Item{x, y, w, h, action, ..Item::default()}
    }
    
    pub fn set_action(&mut self, action: fn(&mut Player) -> Result<(), GameError>) {
        self.action = action;
    }
    
    pub fn do_action(&self, player: &mut Player) -> Result<(), GameError> {
        (self.action)(player)  
    }
}


impl Default for Item {
    fn default() -> Self {
        Item {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            graphics: StaticGraphics::default(),
            action: |_p: &mut Player| Ok(())
        }
    }
}

impl Collidable for Item {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
}

impl DrawableClass for Item {
    fn get_image(&self) -> Option<Image> {
        self.graphics.basic.clone()
    }

    fn get_color(&self) -> Color {
        self.graphics.color
    } 

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
}
