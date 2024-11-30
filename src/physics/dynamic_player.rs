use std::sync::Arc;

use crate::config::Config;

use super::collision::Collidable;

#[derive(Debug, PartialEq, Clone)]
pub struct DynamicPlayer {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    speed: f32,
    jump: f32,
    config: Arc<Config>
}


impl DynamicPlayer {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32, config: Arc<Config>) -> Self {
        DynamicPlayer{x, y, w, h, speed, jump, config}
    }

    pub fn move_right(&mut self, steps: f32) {
        self.x = (self.x + steps * self.speed).min(self.config.screen_width - self.w);
    }

    pub fn move_left(&mut self, steps: f32) {
        self.x = (self.x - steps * self.speed).max(0.0);
    }

    pub fn jump(&mut self, steps: f32) {
        self.y = (self.y - steps * self.jump).max(0.0); // to be changed
    }

    pub fn fall(&mut self, steps: f32) {
        self.y = (self.y + steps * self.config.gravity).min(self.config.screen_height as f32 - self.h); // to be deleted
    }

    pub fn _change_speed(&mut self, multiplier: f32) {
        self.speed *= multiplier;
    }

    pub fn _change_jump(&mut self, multiplier: f32) {
        self.jump *= multiplier;
    }
}

impl Default for DynamicPlayer {
    fn default() -> Self {
        DynamicPlayer {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            speed: 25.0,
            jump: 25.0,
            config: Arc::new(Config::default())
        }
    }
}

impl Collidable for DynamicPlayer {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
}
