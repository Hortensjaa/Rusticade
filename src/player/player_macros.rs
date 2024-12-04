#[macro_export]
macro_rules! create_player {
    ($x:expr, $y:expr, $config:expr) => {
        Player::new($x, $y, 50.0, 50.0, 100.0, 400.0, 100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $config:expr) => {
        Player::new($x, $y, $w, $h, 100.0, 400.0,  100.0, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $hp:expr, $config:expr) => {
        Player::new($x, $y, $w, $h, 100.0, 400.0, $hp, $config)
    };

    ($x:expr, $y:expr, $hp:expr, $config:expr) => {
        Player::new($x, $y, 50.0, 50.0, $hp, 400.0, $hp, $config)
    };

    ($x:expr, $y:expr, $w:expr, $h:expr,  $speed:expr, $jump:expr, $config:expr) => {
        Player::new($x, $y, $w, $h, 100.0, 400.0, 100.0, $config)
    };
    
    ($x:expr, $y:expr, $w:expr, $h:expr, $speed:expr, $jump:expr, $hp:expr, $config:expr) => {
        Player::new($x, $y, $w, $h, $speed, $jump, $hp, $config)
    };
}
