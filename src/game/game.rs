use std::sync::Arc;

use ggez::event::EventHandler;
use ggez::input::keyboard::KeyCode;
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::{Context, GameResult};

use crate::classes::platform::{create_platform, Platform};
use crate::classes::player::Player;
use crate::config::Config;

pub struct Game {
    player: Player,
    platforms: Vec<Platform>,
    config: Arc<Config>
}

impl Game {
    pub fn new(config: Arc<Config>) -> GameResult<Self> {
        let player = Player::default();
        let platforms = Vec::new();
        Ok(Self { player, platforms, config })
    }

    pub fn add_platform(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let platform = create_platform!(x, y, w, h, self.config.clone());
        self.platforms.push(platform);
    }

    pub fn add_platform_default_size(&mut self, x: f32, y: f32) {
        let platform = create_platform!(x, y, self.config.clone());
        self.platforms.push(platform);
    }

    pub fn add_platform_with_effect(&mut self, x: f32, y: f32, effect: fn(Player) -> ()) {
        let platform = create_platform!(x, y, effect, self.config.clone());
        self.platforms.push(platform);
    }

    fn handle_player_input(&mut self, input: KeyCode) -> GameResult {
        match input {
            KeyCode::Up | KeyCode::W => self.player.jump(),
            KeyCode::Down | KeyCode::S => self.player.fall(),
            KeyCode::Left | KeyCode::A => self.player.move_left(),
            KeyCode::Right | KeyCode::D => self.player.move_right(),
            _ => Ok(()),
        }
    }

    fn create_player_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.player.pos.x, self.player.pos.y, self.player.pos.w, self.player.pos.h),
            Color::RED,
        )
    }

    fn create_platform_mesh(&self, ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) -> GameResult<Mesh> {
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(x, y, w, h),
            Color::GREEN,
        )
    }

    fn draw_player(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let player_mesh = self.create_player_mesh(ctx)?;
        canvas.draw(&player_mesh, graphics::DrawParam::default());
        Ok(())
    }

    fn draw_platforms(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for p in &self.platforms {
            let platform_mesh = self.create_platform_mesh(ctx, p.x, p.y, p.w, p.h)?;
            canvas.draw(&platform_mesh, graphics::DrawParam::default());
        }
        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.draw_player(ctx, &mut canvas)?;
        self.draw_platforms(ctx, &mut canvas)?;

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if let Some(key) = input.keycode {
            self.handle_player_input(key)?;
        }
        Ok(())
    }
}


