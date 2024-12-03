#[derive(Clone, Debug)]
pub struct CreaturePhysics {
    pub x: f32,
    pub y: f32,
    pub w: f32, 
    pub h: f32,
    pub tg: f32,
    pub vx: f32,
    pub vy: f32,
    pub moves: Vec<(f32, f32)>,
    pub steps_left: (f32, f32),
    pub speed: f32,
}

impl CreaturePhysics {
    pub fn new( x: f32, y: f32, w: f32, h: f32, moves: Vec<(f32, f32)>, speed: f32) -> Self {    
        let mut m = moves.clone();
        if !m.is_empty() {
           m.rotate_right(1); 
        }
        CreaturePhysics { x, y, w, h, tg: 0.0, vx: 0.0, vy: 0.0, moves: m, speed, steps_left: (0.0, 0.0) }
    }
}