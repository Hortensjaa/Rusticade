use std::collections::HashSet;

use rusticade::{classes::directions::Direction::{self, *}, physics::collision::*};

#[derive(Debug)]
struct TestObject {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Collidable for TestObject {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.w, self.h)
    }

    fn get_barriers(&self) -> HashSet<Direction> {
        HashSet::from([Top, Left])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_touching_x() {
        let obj1 = TestObject { x: 0.0, y: 0.0, w: 50.0, h: 50.0 };
        let obj2 = TestObject { x: 50.05, y: 0.0, w: 50.0, h: 50.0 };
        
        assert!(obj1.is_touching_left_of(&obj2, 0.1));
        assert!(!obj1.is_touching_left_of(&obj2, 0.01));
        assert!(!obj2.is_touching_left_of(&obj1, 0.1));
        
        assert!(obj2.is_touching_right_of(&obj1, 0.1));
        assert!(!obj2.is_touching_right_of(&obj1, 0.01));
        assert!(!obj1.is_touching_right_of(&obj2, 0.1));
    }

    #[test]
    fn test_is_at_y() {
        let obj1 = TestObject { x: 0.0, y: 50.05, w: 50.0, h: 50.0 };
        let obj2 = TestObject { x: 0.0, y: 0.0, w: 50.0, h: 50.0 };

        assert!(obj1.is_at_bottom_of(&obj2, 0.1));
        assert!(!obj1.is_at_bottom_of(&obj2, 0.01)); 
        assert!(!obj2.is_at_bottom_of(&obj1, 0.1));
    }

    #[test]
    fn test_is_on_top_of() {
        let obj1 = TestObject { x: 0.0, y: 0.0, w: 50.0, h: 50.0 };
        let obj2 = TestObject { x: 0.0, y: 50.05, w: 50.0, h: 50.0 };

        assert!(obj1.is_on_top_of(&obj2, 0.1));
        assert!(!obj1.is_on_top_of(&obj2, 0.01)); 
        assert!(!obj2.is_on_top_of(&obj1, 0.1));
    }

    #[test]
    fn test_is_colliding_with() {
        let obj1 = TestObject { x: 0.0, y: 0.0, w: 50.0, h: 50.0 };
        let obj2 = TestObject { x: 25.0, y: 25.0, w: 50.0, h: 50.0 };
        let obj3 = TestObject { x: 100.0, y: 100.0, w: 50.0, h: 50.0 };

        assert!(obj1.is_colliding_with(&obj2));
        assert!(!obj1.is_colliding_with(&obj3));
    }

    #[test]
    fn test_combined_collisions() {
        let obj1 = TestObject { x: 0.0, y: 0.0, w: 50.0, h: 50.0 };
        let obj2 = TestObject { x: 50.0, y: 0.0, w: 50.0, h: 50.0 };
        let obj3 = TestObject { x: 0.0, y: 50.0, w: 50.0, h: 50.0 };

        assert!(obj1.is_touching_left_of(&obj2, 0.1));
        assert!(obj3.is_at_bottom_of(&obj1, 0.1));
        assert!(obj1.is_on_top_of(&obj3, 0.1));
    }
}
