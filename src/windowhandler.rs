use crate::Strip;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::sync::{Arc, Mutex};

pub struct StripWindowHandler {
    strip: Arc<Mutex<Strip>>,
    pixel_size: u32,
}

impl StripWindowHandler {
    pub fn new(strip: Arc<Mutex<Strip>>, pixel_size: u32) -> StripWindowHandler {
        StripWindowHandler { strip, pixel_size }
    }
}

impl WindowHandler for StripWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        {
            let loc_strip = self.strip.lock().unwrap();
            for i in 0..loc_strip.get_pixel_length() {
                let top = Vector2::new(i as f32 * self.pixel_size as f32, 0.0);
                let bottom = Vector2::new(
                    (i + 1) as f32 * self.pixel_size as f32,
                    self.pixel_size as f32,
                );
                let rect = Rectangle::new(top, bottom);

                let rgb = loc_strip.get_pixel(i);
                let speedy_rgb = Color::from_int_rgb(rgb.red(), rgb.green(), rgb.blue());

                graphics.draw_rectangle(rect, speedy_rgb);
            }
        }
        helper.request_redraw()
    }
}
