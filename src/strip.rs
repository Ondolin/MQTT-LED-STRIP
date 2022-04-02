#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::shape::Rectangle;
use speedy2d::dimen::Vector2;

#[derive(Clone)]
pub struct Strip{
    pixels: Vec<Color>,
    width: usize,
    pixel_size: usize,
}

impl Strip {
    pub fn new(width: usize, pixel_size: usize) -> Strip {
        Strip {
            pixels: vec![Color::from_rgb(0.0, 0.0, 0.0); width],
            width,
            pixel_size,
        }
    }

    pub fn get_pixels(&self) -> &Vec<Color> {
        &self.pixels
    }


    pub fn get_width(&self) -> usize {
        self.width.clone()
    }

    pub fn reset(&mut self) {
        self.pixels = vec![Color::from_rgb(0.0, 0.0, 0.0); self.width];
    }

    pub fn set_all(&mut self, color: Color) {
        self.pixels = vec![color; self.width];
    }

    pub fn set_pixel(&mut self, x: usize, color: Color) {
        self.pixels[x] = color;
    }

    pub fn get_pixel(&self, x: usize) -> Color {
        self.pixels[x]
    }

    pub fn push_pixel(&mut self, color: Color) {
        self.pixels.splice(0..0, vec![color]);
        self.pixels.remove(self.pixels.len() - 1);
    }
}

pub struct StripWindowHandler{
    strip: Arc<Mutex<Strip>>,
}

impl StripWindowHandler {
    pub fn new(strip: Arc<Mutex<Strip>>) -> StripWindowHandler {
        StripWindowHandler {
            strip,
        }
    }
}

impl WindowHandler for StripWindowHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        {
            let loc_strip = self.strip.lock().unwrap();
            for i in 0..loc_strip.pixels.len() {
                let top = Vector2::new(i as f32 * loc_strip.pixel_size as f32, 0.0);
                let bottom = Vector2::new((i+1) as f32 * loc_strip.pixel_size as f32, loc_strip.pixel_size as f32);
                let rect = Rectangle::new(top, bottom);
                graphics.draw_rectangle(rect, loc_strip.get_pixel(i));
            }
        }
        helper.request_redraw()
    }
}