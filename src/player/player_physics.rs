use ggez::GameError;

use crate::objects::{platform::Platform, item::Item};
use crate::utils::{collidable::Collidable, directions::Direction::{self, *}};

use super::player::Player;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPhysics {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub on_ground: bool,
    pub speed: f32,
    pub jump: f32
}

impl PlayerPhysics {
        pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32) -> Self {
        PlayerPhysics{x, y, w, h, speed, jump, ..PlayerPhysics::default()}
    }
}

impl Player {
    pub fn update(&mut self, platforms: &[Platform], items: &mut Vec<Item>) -> Result<(), GameError> {
        if self.hp <= 0.0 || self.score > self.config.max_score {
            return Err(GameError::CustomError(String::from("Finish condition")));
        }

        if !self.physics.on_ground {
            self.physics.vy += self.config.gravity * self.config.delta_time;
        }

        self.physics.x += self.physics.vx * self.config.delta_time;
        self.physics.y += self.physics.vy * self.config.delta_time;

        let epsilon = self.physics.vy.abs() * self.config.delta_time + 0.1;
        self.physics.on_ground = false;
        for platform in platforms {
            if self.is_on_top_of(platform, epsilon) {
                if  platform.barriers.contains(&Top) {
                    self.physics.y = platform.y - self.physics.h; 
                    self.physics.vy = 0.0;
                    self.physics.on_ground = true;
                }
                platform.do_action(&Top, self)?;
                break;
            }
            if self.is_at_bottom_of(platform, epsilon) {
                if platform.barriers.contains(&Direction::Bottom) {
                    self.physics.y = platform.y + platform.h + 1.0; 
                    self.physics.vy = 0.0;
                }
                platform.do_action(&Bottom, self)?;
                break;
            }
            if self.is_touching_left_of(platform, epsilon) || self.is_colliding_from_left(platform) {
                if platform.barriers.contains(&Direction::Left) {
                    self.physics.x = platform.x - self.physics.w; 
                    self.physics.vx = 0.0;
                }
                platform.do_action(&Left, self)?;
                break;
            }
            if self.is_touching_right_of(platform, epsilon) || self.is_colliding_from_right(platform) {
                if platform.barriers.contains(&Direction::Right) {
                    self.physics.x = platform.x + platform.w; 
                    self.physics.vx = 0.0;
                }
                platform.do_action(&Right, self)?;
                break;
            }
        }

        for i in (0..items.len()).rev() {
            let item = &mut items[i];
            if self.is_colliding_with(item) {
                item.do_action(self)?;
                items.remove(i);
            }
        }

        self.physics.x = self.physics.x.clamp(0.0, self.config.screen_width - self.physics.w);
        if self.physics.x == self.config.screen_width - self.physics.w {
            self.physics.vx = 0.0;
        }

        self.physics.y = self.physics.y.clamp(0.0, self.config.screen_height - self.physics.h);
        if self.physics.y == self.config.screen_height - self.physics.h {
            self.physics.on_ground = true;
            self.physics.vy = 0.0;
        }
        if self.physics.y == 0.0 {
            self.physics.vy = 0.0;
        }
        Ok(())
    }

    pub fn jump(&mut self) -> Result<(), GameError> {
        let epsilon = self.physics.vy.abs() * self.config.delta_time + 1.0;
        if self.physics.on_ground {
            self.physics.vy = -self.physics.jump; 
            self.physics.y -= epsilon;
            self.physics.on_ground = false;
        }
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), GameError>  {
        self.physics.vx = -self.physics.speed;
        Ok(())
    }

    pub fn move_right(&mut self) -> Result<(), GameError>  {
        self.physics.vx = self.physics.speed;
        Ok(())
    }

    pub fn stop(&mut self)  -> Result<(), GameError>  {
        self.physics.vx = 0.0;
        Ok(())
    }
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        PlayerPhysics {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            speed: 100.0,
            jump: 400.0,
        }
    }
}

impl Collidable for Player {
    fn get_position(&self) -> (f32, f32) {
        (self.physics.x, self.physics.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.physics.w, self.physics.h)
    }
}
