#[derive(Debug, PartialEq, Clone)]
pub struct PlayerPhysics {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vx: f32,
    pub vy: f32,
    pub on_ground: bool,
    pub speed: f32,
    pub jump: f32,
    pub attacked: bool
}

impl PlayerPhysics {
        pub fn new(x: f32, y: f32, w: f32, h: f32, speed: f32, jump: f32) -> Self {
        PlayerPhysics{x, y, w, h, speed, jump, ..PlayerPhysics::default()}
    }
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        PlayerPhysics {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            attacked: false,
            speed: 100.0,
            jump: 400.0,
        }
    }
}

