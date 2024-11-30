use ggez::{graphics::{self, Color, DrawMode, Mesh}, Context, GameResult};

use super::logic::Game;


impl Game {

    fn create_player_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.player.pos.x, self.player.pos.y, self.player.pos.w, self.player.pos.h),
            Color::WHITE,
        )
    }

    fn create_platform(&self, ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) -> GameResult<Mesh> {
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(x, y, w, h),
            color,
        )
    }

    pub(super) fn draw_player(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let player_mesh = self.create_player_mesh(ctx)?;
        canvas.draw(&player_mesh, graphics::DrawParam::default());
        Ok(())
    }

    pub(super) fn draw_platforms(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for p in &self.platforms {
            let platform_mesh = self.create_platform(ctx, p.pos.x, p.pos.y, p.pos.w, p.pos.h, Color::CYAN)?;
            canvas.draw(&platform_mesh, graphics::DrawParam::default());
        };
        Ok(())
    }
}