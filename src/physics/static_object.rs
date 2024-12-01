use std::collections::HashSet;

use crate::classes::directions::Direction::{self, *};

use super::collision::Collidable;

#[derive(Debug, PartialEq, Clone)]
pub struct StaticObject{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub barriers: HashSet<Direction>
}

impl Default for StaticObject {
    fn default() -> Self {
        StaticObject {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            barriers: HashSet::from([Top])
        }
    }
}

impl Collidable for StaticObject {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }

    fn get_barriers(&self) -> HashSet<Direction> {
        self.barriers.clone()
    }
}
