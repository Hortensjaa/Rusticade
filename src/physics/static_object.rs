use super::collision::Collidable;

#[derive(Debug, PartialEq, Clone)]
pub struct StaticObject{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    // collisions
    pub top_collision: bool,
    pub bottom_collision: bool,
    pub right_collision: bool,
    pub left_collision: bool,
}

impl Default for StaticObject {
    fn default() -> Self {
        StaticObject {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            top_collision: true,
            bottom_collision: false,
            right_collision: false,
            left_collision: false,
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
}
