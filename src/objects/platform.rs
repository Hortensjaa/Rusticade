use std::collections::{HashMap, HashSet};
use ggez::{graphics::{Color, Image}, GameError};

use crate::{player::player::Player, shared::{collidable::Collidable, customisable::Customisable, directions::Direction::{self, *}, drawable::DrawableClass}};
use super::object_graphics::StaticGraphics;


#[derive(Clone, Debug)]
pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub barriers: HashSet<Direction>,
    pub finish_line: bool,
    pub actions: HashMap<Direction, fn(&mut Player) -> Result<(), GameError>>,
    pub graphics: StaticGraphics,
    props: HashMap<String, f32> 
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

impl DrawableClass for Platform {
    fn get_image(&self) -> Option<Image> {
        self.graphics.basic.clone()
    }

    fn get_color(&self) -> Color {
        self.graphics.color
    } 

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
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
            actions: HashMap::new(),
            props: HashMap::new()
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

impl Customisable for Platform {

    fn update_property(&mut self, key: &str, val: f32) -> Result<(), GameError> {
        self.props.insert(key.to_string(), val);
        Ok(())
    }

    fn get_property(&self, key: &str) -> Result<f32, GameError> {
        self.props
            .get(key)
            .copied() 
            .ok_or_else(|| GameError::CustomError(format!("Property '{}' not found", key)))
    }
}