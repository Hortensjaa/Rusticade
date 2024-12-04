use std::collections::HashMap;
use std::sync::Arc;

use crate::shared::{collidable::Collidable, config::Config, customisable::Customisable};

use super::{player_graphics::PlayerGraphics, player_physics::PlayerPhysics};


#[derive(Debug, Clone)]
pub struct Player {
    pub physics: PlayerPhysics,
    pub hp: f32,
    pub score: f32,
    pub graphics: PlayerGraphics,
    properties: HashMap<String, f32>,
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
            properties: HashMap::new(),
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

impl Customisable for Player {

    fn update_property(&mut self, key: &str, val: f32) {
        self.properties.insert(key.to_string(), val);
    }

    fn get_property(&self, key: &str) -> f32 {
        self.properties
            .get(key)
            .copied()
            .unwrap_or(0.0)
    }
}

