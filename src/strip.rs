#![allow(dead_code)]

use speedy2d::color::Color;

#[derive(Clone)]
pub struct Strip {
    pixels: Vec<Color>,
    width: usize,
    shut_down: bool,
}

impl Strip {
    pub fn new(width: usize) -> Strip {
        Strip {
            pixels: vec![Color::BLACK; width],
            width,
            shut_down: false,
        }
    }

    pub fn get_pixel_length(&self) -> usize {
        self.pixels.len()
    }

    pub fn shutdown(&mut self) {
        self.shut_down = true;
    }

    pub fn get_pixels(&self) -> Vec<Color> {
        if self.shut_down {
            vec![Color::BLACK; self.width]
        } else {
            self.pixels.clone()
        }
    }

    pub fn get_width(&self) -> usize {
        self.width.clone()
    }

    pub fn reset(&mut self) {
        self.pixels = vec![Color::BLACK; self.width];
    }

    pub fn set_all(&mut self, color: Color) {
        self.pixels = vec![color; self.width];
    }

    pub fn set_pixel(&mut self, x: usize, color: Color) {
        if x >= self.width {
            return;
        }
        self.pixels[x] = color;
    }

    pub fn get_pixel(&self, x: usize) -> Color {
        if self.shut_down {
            return Color::BLACK;
        }
        if x >= self.width {
            return Color::WHITE;
        }
        self.pixels[x]
    }

    pub fn push_pixel(&mut self, color: Color) {
        self.pixels.splice(0..0, vec![color]);
        self.pixels.remove(self.pixels.len() - 1);
    }
}
