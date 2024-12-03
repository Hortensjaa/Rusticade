use std::collections::HashMap;
use std::sync::Arc;

use ggez::GameError;

use crate::shared::{config::Config, collidable::Collidable};

use super::{player_graphics::PlayerGraphics, player_physics::PlayerPhysics};


#[derive(Debug, Clone)]
pub struct Player {
    pub physics: PlayerPhysics,
    pub hp: f32,
    pub score: f32,
    pub graphics: PlayerGraphics,
    props: HashMap<String, f32>,
    config: Arc<Config>,
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32, hp: f32, config: Arc<Config>) -> Self {
        Player { physics: PlayerPhysics::new(x, y, w, h, speed, jump), config, hp, score: 0.0, ..Default::default()}
    }

    pub fn heal(&mut self, points: f32) {
        self.hp += points;
    }

    pub fn take_damage(&mut self, points: f32){
        self.hp -= points;
    }

    pub fn add_score(&mut self, points: f32) {
        self.score += points;
    }

    pub fn reduce_score(&mut self, points: f32){
        self.score -= points;
    }

    pub fn update_property(&mut self, key: &str, val: f32) -> Result<(), GameError> {
        self.props.insert(key.to_string(), val);
        Ok(())
    }

    pub fn get_property(&self, key: &str) -> Result<f32, GameError> {
        self.props
            .get(key)
            .copied() 
            .ok_or_else(|| GameError::CustomError(format!("Property '{}' not found", key)))
    }

    pub fn get_config(&self) -> &Arc<Config> {
        &self.config
    }

}

impl Default for Player {
    fn default() -> Self {
        Player {
            physics: PlayerPhysics::default(),
            hp: 100.0,
            score: 0.0,
            props: HashMap::new(),
            config: Arc::new(Config::default()),
            graphics: PlayerGraphics::default()
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

