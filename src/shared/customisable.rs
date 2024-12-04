pub trait Customisable {
    fn update_property(&mut self, key: &str, val: f32);
    fn get_property(&self, key: &str) -> f32;
}