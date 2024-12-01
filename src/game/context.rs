use std::sync::Arc;

use ggez::{event::EventLoop, ContextBuilder, GameResult, conf::{WindowMode, WindowSetup}};
use crate::config::Config;

pub fn game_context(
    name: &str,
    author: &str,
    icon: &str,
    maximized: bool,
    config: Arc<Config>
) -> GameResult<(ggez::Context, EventLoop<()>)> {
    let window_setup = WindowSetup::default()
        .title(name)
        .icon(icon);

    let window_mode = WindowMode::default()
        .dimensions(config.screen_width, config.screen_height)
        .maximized(maximized);

    let (ctx, event_loop) = ContextBuilder::new(name, author)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    Ok((ctx, event_loop))
}

macro_rules! create_game_context {
    ($name: literal, $author: literal, $icon: literal, $maximized: expr, $config: expr) => {
        crate::game::context::game_context($name, $author, $icon, $maximized, $config)
    };
    ($name: literal, $author: literal, $icon: literal, $config: expr) => {
        crate::game::context::game_context($name, $author, $icon, false, $config)
    };
    ($name: literal, $author: literal, $config: expr) => {
        crate::game::context::game_context($name, $author, "", false, $config)
    };
}

#[allow(unused_imports)]
pub(crate) use create_game_context;

