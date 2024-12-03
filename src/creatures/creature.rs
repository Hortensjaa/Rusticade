use std::sync::Arc;

use ggez::GameError;

use crate::{config::Config, player::player::Player};

use super::creature_graphics::CreatureGraphics;


#[derive(Clone, Debug)]
pub struct Creature {
    pub x: f32,
    pub y: f32,
    pub w: f32, 
    pub h: f32,
    pub tg: f32,
    pub vx: f32,
    pub vy: f32,
    pub moves: Vec<(f32, f32)>,
    pub steps_left: (f32, f32),
    pub speed: f32,
    pub action: fn(&mut Player) -> Result<(), GameError>,
    pub config: Arc<Config>,
    pub graphics: CreatureGraphics
}

impl Creature {
    pub fn new(
        x: f32, y: f32, w: f32, h: f32,
        moves: Vec<(f32, f32)>, speed: f32,
        action: fn(&mut Player) -> Result<(), GameError>,
        config: Arc<Config>
    ) -> Self {    
        let mut m = moves.clone();
        m.rotate_right(1);
        Creature { 
            x, y, w, h, tg: 0.0, vx: 0.0, vy: 0.0,
            moves: m, speed, steps_left: (0.0, 0.0), action, config, 
            graphics: CreatureGraphics::default()
        }
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        if self.moves.is_empty() {
            return Ok(());
        }

    
        if self.steps_left.0.abs() <= (self.vx).abs()/2.0 && self.steps_left.1.abs() <= (self.vy).abs()/2.0 {
            self.moves.rotate_left(1);
            self.steps_left = self.moves[0];
            self.tg = if self.moves[0].0 != 0.0 {
                self.moves[0].1 / self.moves[0].0
            } else {
                if self.moves[0].1 > 0.0 { f32::INFINITY } else { f32::NEG_INFINITY }
            };
        
            let magnitude = (self.moves[0].0.powi(2) + self.moves[0].1.powi(2)).sqrt();
            self.vx = self.moves[0].0 / magnitude * self.speed * self.config.delta_time;
            self.vy = self.moves[0].1 / magnitude * self.speed * self.config.delta_time;
        }
        
        self.x += self.vx;
        self.y += self.vy;
        
        self.steps_left.0 -= self.vx;
        self.steps_left.1 -= self.vy;
        Ok(())
        
    }
    
    
}


