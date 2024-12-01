use ggez::graphics::{Color, Image};

use crate::classes::player::Player;

#[derive(Debug, Clone)]
pub struct PlayerGraphics {
    pub color: Color,
    pub basic: Option<Image>,
    pub left: Option<Image>,
    pub right: Option<Image>,
    pub jump: Option<Image>,
    pub jump_left: Option<Image>,
    pub jump_right: Option<Image>,
}

impl Default for PlayerGraphics {
    fn default() -> Self {
        PlayerGraphics {
            color: Color::RED,
            basic: None,
            left: None,
            right: None,
            jump: None,
            jump_left: None,
            jump_right: None,
        }
    }
}

impl Player {
    pub fn choose_image(&mut self) -> Option<Image> {
        let get_image_for_direction = |condition: bool, image: Option<Image>| {
            condition.then(|| image).flatten()
        };

        if self.physics.on_ground {
            get_image_for_direction(self.physics.vx < 0.0, self.graphics.jump_left.clone())
                .or_else(|| get_image_for_direction(self.physics.vx > 0.0, self.graphics.jump_right.clone()))
                .or_else(|| self.graphics.jump.clone())
                .or_else(|| self.graphics.basic.clone())
        } else {
            get_image_for_direction(self.physics.vx < 0.0, self.graphics.left.clone())
                .or_else(|| get_image_for_direction(self.physics.vx > 0.0, self.graphics.right.clone()))
                .or_else(|| self.graphics.basic.clone())
        }
    }
}
