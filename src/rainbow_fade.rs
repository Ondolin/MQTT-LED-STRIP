use std::sync::{Arc, Mutex};
use crate::animation::{Animation, hsv_to_rgb};
use crate::Strip;

pub struct RainbowFade {
    current_color_hue: u16,
    initial_color_hue: u16,
    step_size: u16,
}

impl RainbowFade {
    pub fn new(initial_color_hue: u16, step_size: u16) -> RainbowFade {
        RainbowFade {
            current_color_hue: 0,
            initial_color_hue,
            step_size,
        }
    }
}

impl Animation for RainbowFade {
    #[allow(unused)]
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        self.current_color_hue = self.initial_color_hue;
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        self.current_color_hue += self.step_size;
        self.current_color_hue %= 360;
        strip.lock().unwrap().set_all(hsv_to_rgb(self.current_color_hue as u32, 1.0, 1.0));
    }
}