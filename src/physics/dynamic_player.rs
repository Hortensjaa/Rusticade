use std::{collections::HashSet, sync::Arc};

use crate::{classes::directions::Direction::{self, *}, config::Config};

use super::{collision::Collidable, static_object::StaticObject};

#[derive(Debug, PartialEq, Clone)]
pub struct DynamicPlayer {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub on_ground: bool,
    pub speed: f32,
    pub jump: f32,
    pub delta_time: f32,
    barriers: HashSet<Direction>,
    pub config: Arc<Config>
}


impl DynamicPlayer {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32, delta_time: f32, config: Arc<Config>) -> Self {
        let barriers = HashSet::from([Left, Right, Top, Bottom, Collision]);
        DynamicPlayer{x, y, w, h, speed, jump, config, delta_time, barriers, ..DynamicPlayer::default()}
    }

    pub fn update(&mut self, platforms: &[StaticObject]) {
        if !self.on_ground {
            self.vy += self.config.gravity * self.delta_time;
        }

        self.x += self.vx * self.delta_time;
        self.y += self.vy * self.delta_time;

        let epsilon = self.vy.abs() * self.delta_time + 0.1;
        self.on_ground = false;
        for platform in platforms {
            if self.is_on_top_of(platform, epsilon) {
                self.y = platform.get_position().1 - self.h; 
                self.vy = 0.0;
                self.on_ground = true;
                break;
            }
        }

        self.x = self.x.clamp(0.0, self.config.screen_width - self.w);
        self.y = self.y.clamp(0.0, self.config.screen_height - self.h);
        if self.y == self.config.screen_height - self.h {
            self.on_ground = true
        }
    }

    pub fn jump(&mut self) {
        let epsilon = self.vy.abs() * self.delta_time + 1.0;
        if self.on_ground {
            self.vy = -self.jump; 
            self.y -= epsilon;
            self.on_ground = false;
        }
    }

    pub fn move_left(&mut self) {
        self.vx = -self.speed;
    }

    pub fn move_right(&mut self) {
        self.vx = self.speed;
    }

    pub fn stop(&mut self) {
        self.vx = 0.0;
    }
}

impl Default for DynamicPlayer {
    fn default() -> Self {
        DynamicPlayer {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            speed: 100.0,
            jump: 400.0,
            delta_time: 1.0 / 40.0,
            barriers: HashSet::from([Left, Right, Top, Bottom, Collision]),
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

    fn get_barriers(&self) -> HashSet<Direction> {
        self.barriers.clone()
    }
}
