use std::{fs, path::Path};

use ggez::{graphics::{Color, Image}, Context, GameError};

use crate::shared::drawable::DrawableClass;

use super::player::Player;

#[derive(Debug, Clone)]
pub struct PlayerGraphics {
    pub color: Color,
    pub basic: Option<Image>,
    pub current_frame: usize,
    pub frame_time: f32,
    pub time_since_last_frame: f32,
    pub walk_left: Vec<Image>,
    pub walk_right: Vec<Image>,
    pub run_left: Vec<Image>,
    pub run_right: Vec<Image>,
    pub jump_left: Vec<Image>,
    pub jump_right: Vec<Image>,
    pub fall_left: Vec<Image>,
    pub fall_right: Vec<Image>,
}

impl Default for PlayerGraphics {
    fn default() -> Self {
        PlayerGraphics {
            color: Color::WHITE,
            basic: None,
            current_frame: 0,
            frame_time: 0.1,
            time_since_last_frame: 0.0,
            walk_left: vec![],
            walk_right: vec![],
            run_left: vec![],
            run_right: vec![],
            jump_left: vec![],
            jump_right: vec![],
            fall_left: vec![],
            fall_right: vec![],
        }
    }
}


impl PlayerGraphics {
    fn load_images_from_dir(&self, ctx: &mut Context, dir_path: &str) -> ggez::GameResult<Vec<Image>> {
        let mut images = Vec::new();
        let base_path = Path::new("./examples/resources");
        let formatted_path = format!("./examples/resources{}", dir_path);
        let full_path = Path::new(&formatted_path);
    
        if full_path.exists() && full_path.is_dir() {
            for entry in fs::read_dir(full_path)? {
                let entry = entry?;
                let file_path = entry.path();
                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "png" {
                            let relative_path = file_path
                                .strip_prefix(base_path)
                                .expect("Failed to create relative path")
                                .to_str()
                                .expect("Failed to convert path to str");
                            let img = Image::from_path(ctx, format!("/{}", relative_path))?;
                            images.push(img);
                        }
                    }
                }
            }
        }
    
        Ok(images)
    }
    

    pub fn load_graphics(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.basic = if Path::new("./examples/resources/basic.png").exists() {
            Some(Image::from_path(ctx, "/basic.png")?)
        } else {
            None
        };
        self.walk_left = self.load_images_from_dir(ctx, "/left/walk")?;
        self.walk_right = self.load_images_from_dir(ctx, "/right/walk")?;
        self.jump_left = self.load_images_from_dir(ctx, "/left/jump")?;
        self.jump_right = self.load_images_from_dir(ctx, "/right/jump")?;
        self.run_left = self.load_images_from_dir(ctx, "/left/run")?;
        self.run_right = self.load_images_from_dir(ctx, "/right/run")?;
        self.run_left = self.load_images_from_dir(ctx, "/left/fall")?;
        self.run_right = self.load_images_from_dir(ctx, "/right/fall")?;
        Ok(())
    }


}

impl DrawableClass for Player {
    fn get_image(&self) -> Option<Image> {
        // stand
        if self.physics.vx == 0.0 && self.physics.vy == 0.0 {
            self.graphics.basic.clone()
        
        // jump
        } else if self.physics.vy < 0.0 {

            // right jump
            if self.physics.vx > 0.0 {
                if !self.graphics.jump_right.is_empty() {
                    self.graphics.jump_right.get(self.graphics.current_frame % self.graphics.jump_right.len()).cloned()
                } else {
                    self.graphics.basic.clone()
                }
            
            // left jump
            } else {
                if !self.graphics.jump_left.is_empty() {
                    self.graphics.jump_left.get(self.graphics.current_frame % self.graphics.jump_left.len()).cloned()
                } else {
                    self.graphics.basic.clone()
                }
            }
        
        // fall
        } else if self.physics.vy > 0.0 {

            // right jump
            if self.physics.vx > 0.0 {
                if !self.graphics.fall_right.is_empty() {
                    self.graphics.fall_right.get(self.graphics.current_frame % self.graphics.fall_right.len()).cloned()
                } else {
                    self.graphics.basic.clone()
                }
            
            // left jump
            } else {
                if !self.graphics.fall_left.is_empty() {
                    self.graphics.fall_left.get(self.graphics.current_frame % self.graphics.fall_left.len()).cloned()
                } else {
                    self.graphics.basic.clone()
                }
            }

        // run right   
        } else if self.physics.vx > 5.0 {
            if !self.graphics.run_right.is_empty() {
                self.graphics.run_right.get(self.graphics.current_frame % self.graphics.run_right.len()).cloned()
            } else {
                self.graphics.basic.clone()
            }

        // walk right    
        } else if self.physics.vx > 0.0 {
            if !self.graphics.walk_right.is_empty() {
                self.graphics.walk_right.get(self.graphics.current_frame % self.graphics.walk_right.len()).cloned()
            } else {
                self.graphics.basic.clone()
            }

        // run left    
        } else if self.physics.vx < -5.0 {
            if !self.graphics.run_left.is_empty() {
                self.graphics.run_left.get(self.graphics.current_frame % self.graphics.run_left.len()).cloned()
            } else {
                self.graphics.basic.clone()
            }

        // walk left    
        } else {
            if !self.graphics.walk_left.is_empty() {
                self.graphics.walk_left.get(self.graphics.current_frame % self.graphics.walk_left.len()).cloned()
            } else {
                self.graphics.basic.clone()
            }

        }
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

    fn set_image(&mut self, img: Image) {
        self.graphics.basic = Some(img)
    }
}
