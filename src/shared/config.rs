use core::f32;

// configuration structure for game, player and all objects included
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Config {
    // screen size
    pub screen_width: f32,
    pub screen_height: f32,

    // cells size (units of size on the screen)
    pub grid_cell_width: f32,
    pub grid_cell_height: f32,

    // przysics options
    pub gravity: f32,
    pub delta_time: f32,
    pub flying_mode: bool,
    pub max_score: f32,

    // manipulation options
    pub awsd: bool,
    pub arrows: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            screen_width: 800.0,
            screen_height: 600.0,
            grid_cell_width: 50.0,
            grid_cell_height: 50.0,
            gravity: 25.0,
            flying_mode: false,
            delta_time: 1.0 / 40.0,
            max_score: f32::MAX,
            awsd: false,
            arrows: true,
        }
    }
}




