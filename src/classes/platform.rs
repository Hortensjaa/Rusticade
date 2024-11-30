use crate::{classes::player::Player, game::logic::Game, physics::static_object::StaticObject};

use super::directions::Direction;


pub struct Platform {
    // position 
    pub pos: StaticObject,

    // collisions
    top_collision: bool,
    bottom_collision: bool,
    right_collision: bool,
    left_collision: bool,

    // action when touching platform from any direction
    on_top: Option<Box<dyn FnMut(&mut Game, Player)>>,
    on_bottom: Option<Box<dyn FnMut(&mut Game, Player)>>,
    on_right: Option<Box<dyn FnMut(&mut Game, Player)>>,
    on_left: Option<Box<dyn FnMut(&mut Game, Player)>>,
    on_collision: Option<Box<dyn FnMut(&mut Game, Player)>>
}

impl Platform {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        let pos = StaticObject{
            x, y, w, h, 
        };
        Platform{pos, ..Platform::default()}
    }
    
    pub fn set_collision(&mut self, direction: Direction, collision: bool) {
        match direction {
            Direction::Left => {self.left_collision = collision},
            Direction::Right => {self.right_collision = collision},
            Direction::Top => {self.top_collision = collision},
            Direction::Bottom => {self.bottom_collision = collision},
            _ => {}
        }
    }

    pub fn set_action(&mut self, direction: Direction, action: Box<dyn FnMut(&mut Game, Player)>) {
        match direction {
            Direction::Left => {self.on_left = Some(action)},
            Direction::Right => {self.on_right = Some(action)},
            Direction::Top => {self.on_top = Some(action)},
            Direction::Bottom => {self.on_bottom = Some(action)},
            _ => {self.on_collision = Some(action)}
        }
    }
}


impl Default for Platform {
    fn default() -> Self {
        Platform {
            pos: StaticObject::default(),

            top_collision: true,
            bottom_collision: false,
            right_collision: false,
            left_collision: false,

            on_top: None,
            on_bottom: None,
            on_right: None,
            on_left: None,
            on_collision: None
        }
    }
}
