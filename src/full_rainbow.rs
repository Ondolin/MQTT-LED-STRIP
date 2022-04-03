use std::sync::{Arc, Mutex};
use crate::animation::{Animation, hsv_to_rgb};
use crate::Strip;

pub struct FullRainbow{
    offset: u32,
    step_size: u32,
}

impl FullRainbow {
    pub fn new(step_size: u32) -> FullRainbow {
        FullRainbow {
            offset: 0,
            step_size,
        }
    }
}

impl Animation for FullRainbow {
    #[allow(unused_variables)]
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        self.offset = 0;
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        self.offset += self.step_size;
        self.offset %= 360;
        let mut current_hue = self.offset as f64;
        let mut strip = strip.lock().unwrap();
        let increment = 360.0 / strip.get_pixel_length() as f64;
        for i in 0..strip.get_pixel_length() {
            strip.set_pixel(i as usize, hsv_to_rgb(current_hue.floor() as u32, 1.0, 1.0));
            current_hue += increment;
            current_hue %= 360.0;
        }
    }
}