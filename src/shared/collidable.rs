pub trait Collidable {
    fn get_position(&self) -> (f32, f32); 
    fn get_size(&self) -> (f32, f32); 

    fn left(&self) -> f32 {
        let (x1, _y1) = self.get_position();
        x1
    }

    fn right(&self) -> f32 {
        let (x1, _y1) = self.get_position();
        let (w1, _h1) = self.get_size();
        x1 + w1
    }

    fn top(&self) -> f32 {
        let (_x1, y1) = self.get_position();
        y1
    }

    fn bottom(&self) -> f32 {
        let (_x1, y1) = self.get_position();
        let (_w1, h1) = self.get_size();
        y1 - h1
    }

    fn center_x(&self) -> f32 {
        let (x1, _y1) = self.get_position();
        let (w1, _h1) = self.get_size();
        x1 + w1/2.0
    }

    fn center_y(&self) -> f32 {
        let (_x1, y1) = self.get_position();
        let (_w1, h1) = self.get_size();
        y1 - h1/2.0
    }

    fn is_colliding_with(&self, other: &dyn Collidable) -> bool {
        let (x1, y1) = self.get_position();
        let (w1, h1) = self.get_size();
        let (x2, y2) = other.get_position();
        let (w2, h2) = other.get_size();

        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    fn is_on_top_of(&self, other: &dyn Collidable, epsilon: f32) -> bool {
        let (x1, y1) = self.get_position();
        let (w1, h1) = self.get_size();
        let (x2, y2) = other.get_position();
        let (w2, _h2) = other.get_size();

        let vertically_aligned = ((y1 + h1) - y2).abs() <= epsilon;
        let horizontally_overlapping = x1 < x2 + w2 && x1 + w1 > x2;
        
        vertically_aligned && horizontally_overlapping
    }

    fn is_at_bottom_of(&self, other: &dyn Collidable, epsilon: f32) -> bool {
        let (x1, y1) = self.get_position();
        let (w1, _h1) = self.get_size();
        let (x2, y2) = other.get_position();
        let (w2, _h2) = other.get_size();
    
        let vertically_aligned = (y1 - (y2 + _h2)).abs() <= epsilon;
        let horizontally_overlapping = x1 < x2 + w2 && x1 + w1 > x2;

        vertically_aligned && horizontally_overlapping
    }

    fn is_touching_left_of(&self, other: &dyn Collidable, epsilon: f32) -> bool {
        let (x1, y1) = self.get_position();
        let (w1, h1) = self.get_size();
        let (x2, y2) = other.get_position();
        let (_w2, h2) = other.get_size();

        let horizontally_aligned = ((x1 + w1) - x2).abs() <= epsilon;
        let vertically_overlapping = y1 < y2 + h2 && y1 + h1 > y2;

        horizontally_aligned && vertically_overlapping
    }

    fn is_touching_right_of(&self, other: &dyn Collidable, epsilon: f32) -> bool {
        let (x1, y1) = self.get_position();
        let (_w1, h1) = self.get_size();
        let (x2, y2) = other.get_position();
        let (w2, h2) = other.get_size();
    
        let horizontally_aligned = (x1 - (w2 + x2)).abs() <= epsilon;
        let vertically_overlapping = y1 < y2 + h2 && y1 + h1 > y2;
    
        horizontally_aligned && vertically_overlapping
    }

    fn is_colliding_from_left(&self, other: &dyn Collidable) -> bool {
        self.is_colliding_with(other) && self.center_x() < other.center_x()
    }

    fn is_colliding_from_right(&self, other: &dyn Collidable) -> bool {
        self.is_colliding_with(other) && self.center_x() > other.center_x()
    }
}