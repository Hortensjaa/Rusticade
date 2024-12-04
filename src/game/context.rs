use std::sync::Arc;

use ggez::{event::EventLoop, ContextBuilder, GameResult, conf::{WindowMode, WindowSetup}};
use crate::shared::config::Config;

pub fn game_context(
    name: &str,
    author: &str,       
    res_path: &str,     // custom resource path
    icon: &str,         // game icon
    maximized: bool,    // windows in full size
    config: Arc<Config> // context for game loop
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
        .add_resource_path("./examples/resources")
        .add_resource_path(res_path)
        .build()?;

    Ok((ctx, event_loop))
}

#[macro_export]
macro_rules! create_game_context {
    ($name: literal, $author: literal, $res_path: literal, $icon: literal, $maximized: expr, $config: expr) => {
        rusticade::game::game_context($name, $author, $res_path, $icon, $maximized, $config)
    };
    ($name: literal, $author: literal, $res_path: literal, $icon: literal, $config: expr) => {
        rusticade::game::game_context($name, $author, $res_path, $icon, false, $config)
    };
    ($name: literal, $author: literal, $res_path: literal, $config: expr) => {
        rusticade::game::context::game_context($name, $author, $res_path, "", false, $config)
    };
    ($name: literal, $author: literal, $config: expr) => {
        rusticade::game::context::game_context($name, $author, "", "", false, $config)
    };
}


