use prisma::{FromColor, Hsv, Rgb};

use crate::animation::Animation;
use crate::Strip;
use std::sync::{Arc, Mutex};

use angle::Deg;

pub struct RainbowFade {
    current_color: Hsv<f32, Deg<f32>>,
    initial_color: Hsv<f32, Deg<f32>>,
    step_size: Deg<f32>,
}

impl RainbowFade {
    pub fn new(initial_color_hue: Deg<f32>, step_size: Deg<f32>) -> RainbowFade {
        RainbowFade {
            current_color: Hsv::new(initial_color_hue, 1.0, 1.0),
            initial_color: Hsv::new(initial_color_hue, 1.0, 1.0),
            step_size,
        }
    }
}

impl Animation for RainbowFade {
    fn initialize(&mut self, _strip: Arc<Mutex<Strip>>) {
        self.current_color = self.initial_color;
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        *self.current_color.hue_mut() += self.step_size;
        *self.current_color.hue_mut() %= Deg(360.0);

        let rgb: Rgb<u8> = Rgb::from_color(&self.current_color).color_cast();

        strip.lock().unwrap().set_all(rgb)
    }
}
