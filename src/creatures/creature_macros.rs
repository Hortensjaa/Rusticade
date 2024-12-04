#[macro_export]
macro_rules! create_creature {
    ($x:expr, $y:expr) => {
        Creature::new($x, $y, 50.0, 50.0, Vec::new(), 5.0, Box::new(|_, _| Ok(true)))
    };

    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Creature::new($x, $y, $w, $h, Vec::new(), 5.0, Box::new(|_, _| Ok(true)))
    };

    ($x:expr, $y:expr, $moves:expr) => {
        Creature::new($x, $y, 50.0, 50.0, $moves, 5.0, Box::new(|_, _| Ok(true)))
    };

    ($x:expr, $y:expr, $w:expr, $h:expr, $moves:expr) => {
        Creature::new($x, $y, $w, $h, $moves, 5.0, Box::new(|_, _| Ok(true)))
    };
}
