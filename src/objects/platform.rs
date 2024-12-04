use std::collections::{HashMap, HashSet};
use ggez::{graphics::{Color, Image}, GameError};

use crate::{player::player::Player, shared::{collidable::Collidable, customisable::Customisable, directions::Direction::{self, *}, drawable::DrawableClass}};
use super::object_graphics::StaticGraphics;


pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub barriers: HashSet<Direction>,
    pub finish_line: bool,
    pub actions: HashMap<Direction, Box<dyn FnMut(&mut Platform, &mut Player) -> Result<(), GameError> + 'static>>,
    pub graphics: StaticGraphics,
    props: HashMap<String, f32>
}

impl Clone for Platform {
    fn clone(&self) -> Self {
        Platform {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            barriers: self.barriers.clone(),
            finish_line: self.finish_line,
            actions: HashMap::new(),
            graphics: self.graphics.clone(),
            props: self.props.clone(),
        }
    }
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

    pub fn set_action(&mut self, direction: Direction, action: Box<dyn FnMut(&mut Platform, &mut Player) -> Result<(), GameError> + 'static>) {
        self.actions.insert(direction, action);
    }

    pub fn do_action(&mut self, direction: &Direction, player: &mut Player) -> Result<(), GameError> {
        if let Some(mut action) = self.actions.remove(direction) {
            let result = action(self, player);
            self.actions.insert(direction.clone(), action);
            result
        } else {
            Ok(())
        }
    }
}

impl DrawableClass for Platform {
    fn get_image(&self) -> Option<Image> {
        self.graphics.basic.clone()
    }

    fn set_image(&mut self, img: Image) {
        self.graphics.basic = Some(img)
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

    fn update_property(&mut self, key: &str, val: f32) {
        self.props.insert(key.to_string(), val);
    }

    fn get_property(&self, key: &str) -> f32{
        self.props
            .get(key)
            .copied()
            .unwrap_or(0.0)
    }
}