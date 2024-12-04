use ggez::{graphics::{Color, DrawParam, Image}, Context, GameError};


pub trait DrawableClass {
    fn get_position(&self) -> (f32, f32); 
    fn get_size(&self) -> (f32, f32); 
    fn get_image(&self) -> Option<Image>;
    fn set_image(&mut self, _: Image);
    fn get_color(&self) -> Color;

    fn draw(&self, ctx: &mut Context,) -> Result<(Image, DrawParam), GameError> {
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
            None => {
                let draw_param = DrawParam::default().dest([x, y]);
                Ok((Image::from_color(ctx, w as u32, h as u32, Some(self.get_color())), draw_param))
            }
        }
    }

    fn load_image_from_file(&mut self, ctx: &mut Context, image_path: &str) -> Result<(), GameError> {
        let image = Image::from_path(ctx, image_path)?;
        self.set_image(image);
        Ok(())
    }
}