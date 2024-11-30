use std::sync::Arc;

use ggez::GameError;

use crate::config::Config;

use crate::physics::dynamic_player::DynamicPlayer;
use super::platform::Platform;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub pos: DynamicPlayer,
    pub hp: u16,
    pub score: u16
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32, delta_time: f32, hp: u16, config: Arc<Config>) -> Self {
        Player { pos: DynamicPlayer::new(x, y, w, h, speed, jump, delta_time, config), hp, score: 0}
    }

    pub fn update(&mut self, platforms: &[Platform]) -> Result<(), GameError> {
        self.pos.update( platforms);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), GameError> {
        self.pos.stop();
        Ok(())
    }

    pub fn move_right(&mut self) -> Result<(), GameError> {
        self.pos.move_right();
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), GameError> {
        self.pos.move_left();
        Ok(())
    }

    pub fn jump(&mut self) -> Result<(), GameError>{
        self.pos.jump();
        Ok(())
    }

    pub fn heal(&mut self, points: u16) -> Result<(), GameError>{
        self.hp += points;
        Ok(())
    }

    pub fn take_damage(&mut self, points: u16) -> Result<(), GameError>{
        self.hp -= points;
        Ok(())
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