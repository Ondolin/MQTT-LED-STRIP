use angular_units::Deg;
use prisma::{FromColor, Hsv, Rgb};

use crate::animation::Animation;
use crate::Strip;
use std::sync::{Arc, Mutex};

pub struct RainbowChase {
    step_size: u16,
    current_color: Hsv<f32, Deg<f32>>,
    current_color_step: u32,
}

impl RainbowChase {
    pub fn new(initial_color_hue: Deg<f32>, step_size: u16) -> RainbowChase {
        RainbowChase {
            step_size,
            current_color: Hsv::new(initial_color_hue, 1.0, 1.0),
            current_color_step: 0,
        }
    }
}

impl Animation for RainbowChase {
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        *self.current_color.hue_mut() += Deg(self.step_size as f32);
        *self.current_color.hue_mut() %= Deg(360.0);
        self.current_color_step = 0;
        {
            let mut strip = strip.lock().unwrap();
            let rgb: Rgb<u8> = Rgb::from_color(&self.current_color).color_cast();
            strip.set_all(rgb);
        }
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        let mut strip = strip.lock().unwrap();

        if self.current_color_step == strip.get_pixel_length() as u32 {
            *self.current_color.hue_mut() += Deg(self.step_size as f32);
            *self.current_color.hue_mut() %= Deg(360.0);
            self.current_color_step = 0;
        }
        self.current_color_step += 1;
        {
            let rgb: Rgb<u8> = Rgb::from_color(&self.current_color).color_cast();
            strip.push_pixel(rgb);
        }
    }
}
