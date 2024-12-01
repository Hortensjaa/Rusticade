use std::collections::{HashMap, HashSet};

use ggez::GameError;

use crate::{classes::player::Player, graphics::static_graphics::StaticGraphics, physics::collision::Collidable};

use super::directions::Direction::{self, *};

#[derive(Clone, Debug)]
pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub barriers: HashSet<Direction>,
    pub finish_line: bool,
    pub actions: HashMap<Direction, fn(&mut Player) -> Result<(), GameError>>,
    pub graphics: StaticGraphics
}


impl Platform {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Platform{x, y, w, h, ..Platform::default()}
    }
    
    pub fn set_barrier(&mut self, direction: Direction, locked: bool) {
        if locked {
            self.barriers.insert(direction);
        } else {
            self.barriers.remove(&direction);
        }
    }

    pub fn set_action(&mut self, direction: Direction, action: fn(&mut Player) -> Result<(), GameError>) {
        self.actions.insert(direction, action);
    }

    pub fn do_action(&self, direction: &Direction, player: &mut Player) -> Result<(), GameError> {
        let a = self.actions.get(direction);
        match a {
            Some(action) => action(player),
            None => Ok(())
        }
        
    }
}


impl Default for Platform {
    fn default() -> Self {
        Platform {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            finish_line: false,
            barriers: HashSet::from([Top]),
            graphics: StaticGraphics::default(),
            actions: HashMap::new()
        }
    }
}

impl Collidable for Platform {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }
}
