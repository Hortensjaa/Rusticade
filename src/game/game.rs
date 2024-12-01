use std::sync::Arc;

use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};

use crate::classes::platform::Platform;
use crate::classes::player::Player;
use crate::config::Config;

pub struct Game {
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub(super) config: Arc<Config>
}

impl Game {
    pub fn new(player: Player, config: Arc<Config>) -> GameResult<Self> {
        let platforms = Vec::new();
        Ok(Self { player, platforms, config })
    }

    fn handle_player_input(&mut self, input: KeyCode) -> GameResult {
        match input {
            KeyCode::Up | KeyCode::W => self.player.jump(),
            KeyCode::Left | KeyCode::A => self.player.move_left(),
            KeyCode::Right | KeyCode::D => self.player.move_right(),
            _ => Ok(()),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let e = self.player.update(&self.platforms);
        match e {
            Ok(()) => Ok(()),
            _ => {
                _ctx.request_quit();
                Ok(())
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.draw_player(ctx, &mut canvas)?;
        self.draw_platforms(ctx, &mut canvas)?;

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: ggez::input::keyboard::KeyInput, _repeat: bool) -> GameResult {
        if let Some(key) = input.keycode {
            self.handle_player_input(key)?;
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(key) = input.keycode {
            match key {
                KeyCode::Left | KeyCode::A | KeyCode::Right | KeyCode::D => self.player.stop()?,
                _ => (),
            };
        }
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        Ok(false)
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player: Player::default(),
            platforms: Vec::new(),
            config: Arc::new(Config::default())
        }
    }
}


