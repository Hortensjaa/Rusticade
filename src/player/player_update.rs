use ggez::GameError;

use crate::objects::{platform::Platform, item::Item};
use crate::shared::{collidable::Collidable, directions::Direction::{self, *}};
use crate::creatures::creature::Creature;
use super::player::Player;

impl Player {
    pub fn update(&mut self, platforms: &[Platform], items: &mut Vec<Item>, creatures: &mut Vec<Creature>) -> Result<(), GameError> {
        if self.hp <= 0.0 || self.score > self.get_config().max_score {
            return Err(GameError::CustomError(String::from("Finish condition")));
        }

        if !self.physics.on_ground {
            self.physics.vy += self.get_config().gravity * self.get_config().delta_time;
        }

        self.physics.x += self.physics.vx * self.get_config().delta_time;
        self.physics.y += self.physics.vy * self.get_config().delta_time;

        let epsilon = self.physics.vy.abs() * self.get_config().delta_time + 0.1;
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

        for i in (0..creatures.len()).rev() {
            let c = &mut creatures[i];
            if self.is_colliding_with(c) {
                let res = c.do_action(self)?;
                match res {
                    true => {}
                    false => {creatures.remove(i);}
                }
            }
        }

        self.physics.x = self.physics.x.clamp(0.0, self.get_config().screen_width - self.physics.w);
        if self.physics.x == self.get_config().screen_width - self.physics.w {
            self.physics.vx = 0.0;
        }

        self.physics.y = self.physics.y.clamp(0.0, self.get_config().screen_height - self.physics.h);
        if self.physics.y == self.get_config().screen_height - self.physics.h {
            self.physics.on_ground = true;
            self.physics.vy = 0.0;
        }
        if self.physics.y == 0.0 {
            self.physics.vy = 0.0;
        }
        Ok(())
    }

    pub fn jump(&mut self) -> Result<(), GameError> {
        let epsilon = self.physics.vy.abs() * self.get_config().delta_time + 1.0;
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