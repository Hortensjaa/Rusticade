use crate::{classes::player::Player, game::logic::Game, physics::dynamic_object::DynamicObject};


pub enum Platform {
    Normal(DynamicObject),
    Special(DynamicObject, fn(Player) -> ()),
    Finish(DynamicObject, Box<dyn FnMut(&mut Game, Player)>)
}

impl Platform {
    pub fn new_normal(x: f32, y: f32, w: f32, h: f32) -> Self {
        let pos = DynamicObject{
            x, y, w, h, 
        };
        Platform::Normal(pos)
    }
    
    pub fn new_special(x: f32, y: f32, w: f32, h: f32, effect: fn(Player) -> ()) -> Self {
        let pos = DynamicObject{
            x, y, w, h, 
        };
        Platform::Special(pos, effect)
    }

    pub fn new_finish(x: f32, y: f32, w: f32, h: f32, effect: Box<dyn FnMut(&mut Game, Player)>) -> Self {
        let pos = DynamicObject{
            x, y, w, h, 
        };
        Platform::Finish(pos, effect)
    }
}


impl Default for Platform {
    fn default() -> Self {
        Platform::Normal(DynamicObject::default()) 
    }
}
