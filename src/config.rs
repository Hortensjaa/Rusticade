use core::f32;

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
    pub allow_flying: bool,
    pub max_score: f32,

    // camera options (fixed means always focusing on player)
    pub fixed_x: bool,
    pub fixed_y: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            screen_width: 800.0,
            screen_height: 600.0,
            grid_cell_width: 50.0,
            grid_cell_height: 50.0,
            gravity: 400.0,
            allow_flying: false,
            delta_time: 1.0 / 40.0,
            max_score: f32::MAX,
            fixed_x: false,
            fixed_y: false,
        }
    }
}




