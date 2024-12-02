use std::collections::HashMap;
use std::sync::Arc;

use ggez::GameError;

use crate::config::Config;

use super::{player_graphics::PlayerGraphics, player_physics::PlayerPhysics};


#[derive(Debug, Clone)]
pub struct Player {
    pub physics: PlayerPhysics,
    pub hp: f32,
    pub score: f32,
    props: HashMap<String, f32>, // properties that might be added by user (e.g. coins, stamina itd.)
    pub config: Arc<Config>,
    pub graphics: PlayerGraphics
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32, hp: f32, config: Arc<Config>) -> Self {
        Player { physics: PlayerPhysics::new(x, y, w, h, speed, jump), config, hp, score: 0.0, ..Default::default()}
    }

    pub fn heal(&mut self, points: f32) -> Result<(), GameError> {
        self.hp += points;
        Ok(())
    }

    pub fn take_damage(&mut self, points: f32) -> Result<(), GameError> {
        self.hp -= points;
        Ok(())
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


macro_rules! create_player {
    ($x:expr, $y:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, 50.0, 50.0, 100.0, 400.0, 1.0 / 40.0, 100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, $w, $h, 100.0, 400.0, 1.0 / 40.0, 100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $hp:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, $w, $h, 100.0, 400.0, 1.0 / 40.0, $hp, $config)
    };

    ($x:expr, $y:expr, $hp:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, 50.0, 50.0, $hp, 400.0, 1.0 / 40.0, $hp, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr,  $speed:expr, $jump:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, $w, $h, 100.0, 400.0, 1.0 / 40.0, 100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $hp:expr,  $speed:expr, $jump:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, $w, $h, 100.0, 400.0, 1.0 / 40.0, $hp, $config)
    };
    
    ($x:expr, $y:expr, $w:expr, $h:expr, $speed:expr, $jump:expr, $hp:expr, $config:expr) => {
        crate::classes::player::Player::new($x, $y, $w, $h, $speed, $jump, $hp, $config)
    };
}

#[allow(unused_imports)]
pub(crate) use create_player;

