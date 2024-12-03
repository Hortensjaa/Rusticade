use ggez::{graphics::{self, Color, DrawMode, DrawParam, Image, Mesh}, Context, GameError};


pub trait DrawableClass {
    fn get_position(&self) -> (f32, f32); 
    fn get_size(&self) -> (f32, f32); 
    fn get_image(&self) -> Option<Image>;
    fn get_color(&self) -> Color;

    fn draw(&self) -> Result<(Image, DrawParam), GameError> {
        let (x, y) = self.get_position();
        let (w, h) = self.get_size();

        match self.get_image() {
            Some(img) => {
                let scale_x = w / img.width() as f32;
                let scale_y = h / img.height() as f32;
                let draw_param = DrawParam::default()
                    .dest([x, y]) 
                    .scale([scale_x, scale_y]); 
                Ok((img, draw_param))
            }
            None => Err(GameError::CustomError("Can't find".to_string()))
        }
    }

    fn draw_rectangle(&self, ctx: &mut Context) -> Result<Mesh, GameError> {
        let (x, y) = self.get_position();
        let (w, h) = self.get_size();

        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(x, y, w, h),
            self.get_color(),
        )
    }

    fn draw_ellipse(&self, ctx: &mut Context) -> Result<Mesh, GameError> {
        let (x, y) = self.get_position();
        let (w, h) = self.get_size();

        Mesh::new_ellipse(
            ctx,
            DrawMode::fill(),
            [x, y],
            w,
            h, 
            0.1,
            self.get_color(),
        )
    }
}