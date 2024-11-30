use std::sync::Arc;

use ggez::GameError;

use crate::config::Config;

use super::super::physics::dynamic_player::DynamicPlayer;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub pos: DynamicPlayer,
    pub hp: u16,
    pub score: u16
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32, hp: u16, config: Arc<Config>) -> Self {
        Player { pos: DynamicPlayer::new(x, y, w, h, speed, jump, config), hp, score: 0}
    }

    pub fn move_right(&mut self) -> Result<(), GameError>{
        self.pos.move_right(1.0);
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), GameError> {
        self.pos.move_left(1.0);
        Ok(())
    }

    pub fn jump(&mut self) -> Result<(), GameError>{
        self.pos.jump(1.0);
        Ok(())
    }

    pub fn fall(&mut self) -> Result<(), GameError>{
        self.pos.fall(1.0);
        Ok(())
    }

    fn heal(&mut self) -> Result<(), GameError>{
        unimplemented!()
    }

    fn take_damage(&mut self) -> Result<(), GameError>{
        unimplemented!()
    }

}

impl Default for Player {
    fn default() -> Self {
        Player {
            pos: DynamicPlayer::default(),
            hp: 100,
            score: 0
        }
    }
}