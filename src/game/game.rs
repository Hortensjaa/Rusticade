use std::sync::Arc;

use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};

use crate::creatures::creature::Creature;
use crate::objects::{platform::Platform, item::Item};
use crate::player::player::Player;
use crate::shared::config::Config;

pub struct Game {
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub items: Vec<Item>,
    pub creatures: Vec<Creature>,
    pub action_before: Option<Box<dyn FnMut(&mut Game) -> GameResult<()> + 'static>>,
    pub action_after: Option<Box<dyn FnMut(&mut Game) -> GameResult<()> + 'static>>,
    config: Arc<Config>
}

impl Game {
    pub fn new(player: Player, config: Arc<Config>) -> GameResult<Self> {
        Ok(Game { player, config, ..Game::default() })
    }

    fn handle_player_input(&mut self, input: KeyCode) -> GameResult {
        match input {
            KeyCode::Up | KeyCode::W => self.player.jump(),
            KeyCode::Left | KeyCode::A => self.player.move_left(),
            KeyCode::Right | KeyCode::D => self.player.move_right(),
            KeyCode::Down | KeyCode::S => self.player.move_down(),
            _ => Ok(()),
        }
    }

    pub fn get_config(&self) -> &Arc<Config> {
        &self.config
    }

    fn update_player(&mut self) -> GameResult<()> {
        self.player.update(&mut self.platforms, &mut self.items, &mut self.creatures)
    }

    fn run_action_before(&mut self) -> GameResult<()> {
        if let Some(mut action) = self.action_before.take() {
            action(self)?; 
            self.action_after = Some(action);
        }
        Ok(())
    }

    fn run_action_after(&mut self) -> GameResult<()> {
        if let Some(mut action) = self.action_after.take() {
            action(self)?; 
            self.action_after = Some(action);
        }
        Ok(())
    }
}

impl EventHandler for Game {
    

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.run_action_before()?;
        self.update_player()?;
        for c in self.creatures.iter_mut() {
            match c.update() {
                Ok(()) => continue, 
                Err(_) => {
                    _ctx.request_quit(); 
                    return Ok(()); 
                }
            }
        }
        self.run_action_after()?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.draw_player(ctx, &mut canvas)?;
        self.draw_platforms(ctx, &mut canvas)?;
        self.draw_items(ctx, &mut canvas)?;
        self.draw_creatures(ctx, &mut canvas)?;

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
                KeyCode::S | KeyCode::Down | KeyCode::W | KeyCode::Up => if self.config.flying_mode {
                    self.player.stop()?
                }
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
            items: Vec::new(),
            creatures: Vec::new(),
            config: Arc::new(Config::default()),
            action_before: None,
            action_after: None
        }
    }
}


