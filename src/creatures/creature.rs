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
    pub moves: Vec<(f32, f32)>,
    pub steps_left: (f32, f32),
    pub speed: f32,
    pub action: fn(&mut Player) -> Result<(), GameError>,
    pub config: Arc<Config>,
    pub graphics: CreatureGraphics
}

fn sgn(value: f32) -> f32 {
    if value > 0.0 {
        1.0
    } else if value < 0.0 {
        -1.0
    } else {
        0.0
    }
}

impl Creature {
    pub fn new(
        x: f32, y: f32, w: f32, h: f32,
        moves: Vec<(f32, f32)>, speed: f32,
        action: fn(&mut Player) -> Result<(), GameError>,
        config: Arc<Config>
    ) -> Self {
        Creature { x, y, w, h, moves, speed, action, config, ..Creature::default() }
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        if self.moves.is_empty() {
            return Ok(());
        }

        if self.steps_left.0.abs() < self.speed * self.config.delta_time / 100.0 
        && self.steps_left.1.abs() < self.speed * self.config.delta_time / 100.0  {
            self.moves.rotate_left(1);
            self.steps_left = self.moves[0];
        }

        let mut delta_x = 0.0;
        let mut scale = 1.0;
        if self.moves[0].0 != 0.0 {
            scale = (self.moves[0].1 / self.moves[0].0).abs();
            delta_x = sgn(self.moves[0].0) * self.speed * self.config.delta_time;
        } 
        
        let delta_y = sgn(self.moves[0].1) * scale * self.speed * self.config.delta_time;
        self.x += delta_x;
        self.y += delta_y;

        self.steps_left.0 -= delta_x;
        self.steps_left.1 -= delta_y;
        
        Ok(())
    }
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            x: 0.0,              
            y: 0.0,              
            w: 50.0,          
            h: 50.0,   
            moves: vec![],         
            speed: 100.0,  
            steps_left: (0.0, 0.0),               
            action: |_| Ok(()),  
            config: Arc::new(Config::default()),
            graphics: CreatureGraphics::default()    
        }
    }
}
