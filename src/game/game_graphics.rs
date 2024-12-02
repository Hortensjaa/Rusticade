use ggez::{graphics, Context, GameResult};

use crate::{game::game::Game, utils::drawable::DrawableClass};


impl Game {

    pub fn draw_player(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        match self.player.draw() {
            Ok((img, draw_param)) => {
                canvas.draw(&img, draw_param);
            }
            _ => {
                let mesh = self.player.draw_rectangle(ctx)?;
                canvas.draw(&mesh, graphics::DrawParam::default());
            }
        }
        Ok(())
    }    

    pub fn draw_platforms(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for p in &self.platforms {
            match p.draw() {
                Ok((img, draw_param)) => {
                    canvas.draw(&img, draw_param);
                }
                _ => {
                    let mesh = p.draw_rectangle(ctx)?;
                    canvas.draw(&mesh, graphics::DrawParam::default());
                }
            }
        };
        Ok(())
    }

    pub fn draw_items(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for item in &self.items {
            match item.draw() {
                Ok((img, draw_param)) => {
                    canvas.draw(&img, draw_param);
                }
                _ => {
                    let mesh = item.draw_ellipse(ctx)?;
                    canvas.draw(&mesh, graphics::DrawParam::default());
                }
            }
        };
        Ok(())
    }
}
