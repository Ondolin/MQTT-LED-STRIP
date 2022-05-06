use angular_units::Deg;
use prisma::{FromColor, Hsv, Rgb};

use crate::animation::Animation;
use crate::Strip;
use std::sync::{Arc, Mutex};

pub struct FullRainbow {
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
        let mut current_hue = self.offset as f32;
        let mut strip = strip.lock().unwrap();
        let increment = 360.0 / strip.get_pixel_length() as f32;

        for i in 0..strip.get_pixel_length() {
            let hsv: Hsv<f32, Deg<f32>> = Hsv::new(Deg(current_hue), 1.0, 1.0);
            let rgb: Rgb<u8> = Rgb::from_color(&hsv).color_cast();

            strip.set_pixel(i as usize, rgb);
            current_hue += increment;
            current_hue %= 360.0;
        }
    }
}
