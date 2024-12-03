use std::collections::HashMap;
use std::sync::Arc;

use ggez::GameError;

use crate::shared::collidable::Collidable;
use crate::shared::customisable::Customisable;
use crate::{shared::config::Config, player::player::Player};

use super::creature_graphics::CreatureGraphics;
use super::creature_physics::CreaturePhysics;


#[derive(Clone, Debug)]
pub struct Creature {
    pub physics: CreaturePhysics,
    pub action: fn(&mut Player) -> Result<bool, GameError>,
    pub graphics: CreatureGraphics,
    props: HashMap<String, f32>, 
    config: Arc<Config>,
}

impl Creature {
    pub fn new(
        x: f32, y: f32, w: f32, h: f32,
        moves: Vec<(f32, f32)>, speed: f32,
        action: fn(&mut Player) -> Result<bool, GameError>,
        config: Arc<Config>
    ) -> Self {    
        Creature { 
            physics: CreaturePhysics::new(x, y, w, h, moves, speed), 
            action, config, props: HashMap::new(),
            graphics: CreatureGraphics::default()
        }
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        if self.physics.moves.is_empty() {
            return Ok(());
        }

        if self.physics.steps_left.0.abs() <= (self.physics.vx).abs()/2.0 
        && self.physics.steps_left.1.abs() <= (self.physics.vy).abs()/2.0 {
            self.physics.moves.rotate_left(1);
            self.physics.steps_left = self.physics.moves[0];
            self.physics.tg = if self.physics.moves[0].0 != 0.0 {
                self.physics.moves[0].1 / self.physics.moves[0].0
            } else {
                if self.physics.moves[0].1 > 0.0 { f32::INFINITY } else { f32::NEG_INFINITY }
            };
        
            let magnitude = (self.physics.moves[0].0.powi(2) + self.physics.moves[0].1.powi(2)).sqrt();
            self.physics.vx = self.physics.moves[0].0 / magnitude * self.physics.speed * self.config.delta_time;
            self.physics.vy = self.physics.moves[0].1 / magnitude * self.physics.speed * self.config.delta_time;
        }
        
        self.physics.x += self.physics.vx;
        self.physics.y += self.physics.vy;
        
        self.physics.steps_left.0 -= self.physics.vx;
        self.physics.steps_left.1 -= self.physics.vy;
        Ok(())
        
    }

    pub fn do_action(&self, player: &mut Player) -> Result<bool, GameError> {
        (self.action)(player)  
    }
}



impl Collidable for Creature {
    fn get_position(&self) -> (f32, f32) {
        (self.physics.x, self.physics.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.physics.w, self.physics.h)
    }
}

impl Customisable for Creature {

    fn update_property(&mut self, key: &str, val: f32) -> Result<(), GameError> {
        self.props.insert(key.to_string(), val);
        Ok(())
    }

    fn get_property(&self, key: &str) -> Result<f32, GameError> {
        self.props
            .get(key)
            .copied() 
            .ok_or_else(|| GameError::CustomError(format!("Property '{}' not found", key)))
    }
}
