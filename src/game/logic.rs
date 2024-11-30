use std::sync::Arc;

use ggez::event::EventHandler;
use ggez::input::keyboard::KeyCode;
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};

use crate::classes::platform::Platform;
use crate::classes::player::Player;
use crate::config::Config;
use crate::physics::collision::Collidable;

pub struct Game {
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub(super) config: Arc<Config>,
    pub(super) quitted: bool
}

impl Game {
    pub fn new(config: Arc<Config>) -> GameResult<Self> {
        let player = Player::default();
        let platforms = Vec::new();
        Ok(Self { player, platforms, config, quitted: false })
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
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for p in &self.platforms {
            if self.player.pos.is_on_top_of(&p.get_pos()) {
                println!("top")
            }
            if p.get_pos().is_on_top_of(&self.player.pos) {
                println!("down")
            }
            if p.get_pos().is_colliding_with(&self.player.pos) {
                println!("collision")
            }
        }
        if self.quitted {
            self.quit_event(_ctx)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

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

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        Ok(false)
    }
}


