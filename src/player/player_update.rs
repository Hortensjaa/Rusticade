use ggez::GameError;

use crate::objects::{platform::Platform, item::Item};
use crate::shared::{collidable::Collidable, directions::Direction::*};
use crate::creatures::creature::Creature;
use super::player::Player;

impl Player {
    pub fn update(&mut self, platforms: &mut Vec<Platform>, items: &mut Vec<Item>, creatures: &mut Vec<Creature>) -> Result<(), GameError> {
        self.check_finish_conditions()?;
        self.apply_gravity();
        self.update_position();
        self.handle_platform_collisions(platforms)?;
        self.handle_item_collisions(items)?;
        self.handle_creature_collisions(creatures)?;
        self.clamp_position();
        self.graphics.time_since_last_frame += 1.0/60.0;
        if self.graphics.time_since_last_frame > self.graphics.frame_time {
            self.graphics.time_since_last_frame = 0.0;
            self.graphics.current_frame += 1;
        }
        Ok(())
    }
    
    fn check_finish_conditions(&self) -> Result<(), GameError> {
        if self.hp <= 0.0 {
            return Err(GameError::CustomError(String::from("Oh, you lost all your hp - try again")));
        }
        if self.score > self.get_config().max_score {
            return Err(GameError::CustomError(format!("Win! You score {} points", self.score)));
        }
        Ok(())
    }
    
    fn apply_gravity(&mut self) {
        if !self.physics.on_ground && !self.get_config().flying_mode {
            self.physics.vy += self.get_config().gravity / 100.0;
        }
    }
    
    fn update_position(&mut self) {
        self.physics.x += self.physics.vx;
        self.physics.y += self.physics.vy;
        self.physics.on_ground = false;
    }
    
    fn handle_platform_collisions(&mut self, platforms: &mut Vec<Platform>) -> Result<(), GameError> {
        for platform in platforms {
            if self.is_on_top_of(platform, self.physics.vy.abs()) {
                self.resolve_top_collision(platform)?;
                break;
            }
            if self.is_at_bottom_of(platform, self.physics.vy.abs()) {
                self.resolve_bottom_collision(platform)?;
                break;
            }
            if self.is_touching_left_of(platform, self.physics.vx.abs()) {
                self.resolve_left_collision(platform)?;
                break;
            }
            if self.is_touching_right_of(platform, self.physics.vx.abs()) {
                self.resolve_right_collision(platform)?;
                break;
            }
        }
        Ok(())
    }
    
    fn resolve_top_collision(&mut self, platform: &mut Platform) -> Result<(), GameError> {
        if platform.barriers.contains(&Top) {
            self.physics.y = platform.y - self.physics.h - 0.1;
            self.physics.vy = self.physics.vy.min(0.0);
            self.physics.on_ground = true;
        }
        platform.do_action(&Top, self)?;
        Ok(())
    }
    
    fn resolve_bottom_collision(&mut self, platform: &mut Platform) -> Result<(), GameError> {
        if platform.barriers.contains(&Bottom) {
            self.physics.y = platform.y + platform.h + 0.1;
            self.physics.vy = self.physics.vy.max(0.0);
        }
        platform.do_action(&Bottom, self)?;
        Ok(())
    }
    
    fn resolve_left_collision(&mut self, platform: &mut Platform) -> Result<(), GameError> {
        if platform.barriers.contains(&Left) {
            self.physics.x = platform.x - self.physics.w - 0.1;
            self.physics.vx = 0.0;
        }
        platform.do_action(&Left, self)?;
        Ok(())
    }
    
    fn resolve_right_collision(&mut self, platform: &mut Platform) -> Result<(), GameError> {
        if platform.barriers.contains(&Right) {
            self.physics.x = platform.x + platform.w + 0.1;
            self.physics.vx = 0.0;
        }
        platform.do_action(&Right, self)?;
        Ok(())
    }
    
    fn handle_item_collisions(&mut self, items: &mut Vec<Item>) -> Result<(), GameError> {
        for i in (0..items.len()).rev() {
            let item = &mut items[i];
            if self.is_colliding_with(item) {
                item.do_action(self)?;
                items.remove(i);
            }
        }
        Ok(())
    }
    
    fn handle_creature_collisions(&mut self, creatures: &mut Vec<Creature>) -> Result<(), GameError> {
        for i in (0..creatures.len()).rev() {
            let creature = &mut creatures[i];
            if self.is_colliding_with(creature) {
                if !creature.triggered {
                    let res = creature.do_action(self)?;
                    creature.triggered = true;
                    if !res {
                        creatures.remove(i);
                    }
                }
            } else {
                creature.triggered = false;
            }
        }
        Ok(())
    }
    
    fn clamp_position(&mut self) {
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
    }
    

    pub fn jump(&mut self) -> Result<(), GameError> {
        if !self.get_config().flying_mode {
            if self.physics.on_ground {
                self.physics.vy = -self.physics.jump; 
                self.physics.y += self.physics.vy;
                self.physics.on_ground = false;
            }
        } else {
            self.physics.vy = -self.physics.speed;
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

    pub fn move_down(&mut self) -> Result<(), GameError>  {
        if !self.physics.on_ground && self.get_config().flying_mode {
            self.physics.vy = self.physics.speed;
        }
        Ok(())
    }

    pub fn stop(&mut self)  -> Result<(), GameError>  {
        self.physics.vx = 0.0;
        if self.get_config().flying_mode {
            self.physics.vy = 0.0;
        }
        Ok(())
    }
}