use ggez::GameError;

use crate::player::player::Player;
use super::static_graphics::StaticGraphics;


#[derive(Clone, Debug)]
pub struct Item {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub actions: fn(&mut Player) -> Result<(), GameError>,
    pub graphics: StaticGraphics
}