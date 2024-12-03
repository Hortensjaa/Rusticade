use ggez::graphics::{Color, Image};

use crate::shared::drawable::DrawableClass;

use super::creature::Creature;

#[derive(Debug, Clone)]
pub struct CreatureGraphics {
    pub color: Color,
    pub basic: Option<Image>,
}

impl Default for CreatureGraphics {
    fn default() -> Self {
        CreatureGraphics {
            color: Color::YELLOW,
            basic: None,
        }
    }
}

impl DrawableClass for Creature {
    fn get_image(&self) -> Option<Image> {
        self.graphics.basic.clone()
    }

    fn get_color(&self) -> Color {
        self.graphics.color
    } 

    fn get_position(&self) -> (f32, f32) {
        (self.physics.x, self.physics.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.physics.w, self.physics.h)
    }
}