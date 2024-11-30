#[derive(Debug, PartialEq, Clone)]
pub struct DynamicObject{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Default for DynamicObject {
    fn default() -> Self {
        DynamicObject {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0
        }
    }
}
