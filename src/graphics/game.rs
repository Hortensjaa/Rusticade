use ggez::{graphics::{self, DrawMode, DrawParam, Mesh}, Context, GameResult};

use crate::game::game::Game;


impl Game {

    pub fn draw_player(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        match self.player.clone().choose_image() {
            Some(img) => {
                let scale_x = self.player.physics.w / img.width() as f32;
                let scale_y = self.player.physics.h / img.height() as f32;
                let draw_param = DrawParam::default()
                    .dest([self.player.physics.x, self.player.physics.y]) 
                    .scale([scale_x, scale_y]); 
                canvas.draw(&img, draw_param);
            }
            None => {
                let mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    graphics::Rect::new(self.player.physics.x, self.player.physics.y, self.player.physics.w, self.player.physics.h),
                    self.player.graphics.color,
                )?;
                canvas.draw(&mesh, graphics::DrawParam::default());
            }
        }
        Ok(())
    }    

    pub fn draw_platforms(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for p in &self.platforms {
            match p.graphics.basic.clone() {
                Some(img) => {
                    let scale_x = p.w / img.width() as f32;
                    let scale_y = p.h / img.height() as f32;
                    let draw_param = DrawParam::default()
                        .dest([p.x, p.y]) 
                        .scale([scale_x, scale_y]); 
                    canvas.draw(&img, draw_param);
                }
                None => {
                    let mesh = Mesh::new_rectangle(
                        ctx,
                        DrawMode::fill(),
                        graphics::Rect::new(p.x, p.y, p.w, p.h),
                        p.graphics.color,
                    )?;
                    canvas.draw(&mesh, graphics::DrawParam::default());
                }
            }
        };
        Ok(())
    }
}
