use std::sync::Arc;

use crate::config::Config;
use crate::classes::player::Player;

#[derive(Debug, PartialEq, Clone)]
pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub effect: fn(Player) -> (),
    config: Arc<Config>
}

impl Platform {
    pub fn new(x: f32, y: f32, w: f32, h: f32, effect: fn(Player) -> (), config: Arc<Config>) -> Self {
        Platform{x, y, w, h, effect, config}
    }
}


pub fn no_effect(_player: Player) {}

impl Default for Platform {
    fn default() -> Self {
        Platform {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            effect: no_effect,
            config: Arc::new(Config::default())
        }
    }
}

macro_rules! create_platform {
    ($x: expr, $y: expr, $w: expr, $h: expr, $effect: expr, $config: expr) => {
        crate::classes::platform::Platform::new(
            $x, $y, $w, $h, $effect, $config
        )
    };
    ($x: expr, $y: expr, $w: expr, $h: expr, $config: expr) => {
        crate::classes::platform::Platform::new(
            $x, $y, $w, $h, crate::classes::platform::no_effect, $config
        )
    };
    ($x: expr, $y: expr, $effect: expr, $config: expr) => {
        crate::classes::platform::Platform::new(
            $x, $y, 50.0, 50.0, $effect, $config
        )
    };
    ($x: expr, $y: expr, $config: expr) => {
        crate::classes::platform::Platform::new(
            $x, $y, 50.0, 50.0, crate::classes::platform::no_effect, $config
        )
    };
}

pub(crate) use create_platform;