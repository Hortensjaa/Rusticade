use ggez::graphics::{Color, Image};

#[derive(Debug, Clone)]
pub struct StaticGraphics {
    pub color: Color,
    pub basic: Option<Image>,
}

impl Default for StaticGraphics {
    fn default() -> Self {
        StaticGraphics {
            color: Color::CYAN,
            basic: None,
        }
    }
}
