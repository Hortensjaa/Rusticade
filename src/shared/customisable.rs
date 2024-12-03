use ggez::GameError;

pub trait Customisable {
    fn update_property(&mut self, key: &str, val: f32) -> Result<(), GameError>;
    fn get_property(&self, key: &str) -> Result<f32, GameError>;
}