use std::sync::{Arc, Mutex};
use paho_mqtt::Message;
use crate::animation::{Animation, hsv_to_rgb};
use crate::Strip;

pub struct RainbowChase {
    initial_color_hue: u16,
    step_size: u16,
    current_color_hue: u16,
    current_color_step: u32,
    width: u32,
}

impl RainbowChase{
    pub fn new(initial_color_hue: u16, step_size: u16, width: u32) -> RainbowChase {
        RainbowChase {
            initial_color_hue,
            step_size,
            current_color_hue: initial_color_hue,
            current_color_step: 0,
            width,
        }
    }
}

impl Animation for RainbowChase{
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        self.current_color_hue = (self.initial_color_hue + self.step_size) % 360;
        self.current_color_step = 0;
        {
            let mut strip = strip.lock().unwrap();
            strip.set_all(hsv_to_rgb(self.initial_color_hue as u32, 1.0, 1.0));
        }
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        if self.current_color_step == self.width {
            self.current_color_hue = (self.current_color_hue + self.step_size) % 360;
            self.current_color_step = 0;
        }
        self.current_color_step += 1;
        {
            let mut strip = strip.lock().unwrap();
            strip.push_pixel(hsv_to_rgb(self.current_color_hue as u32, 1.0, 1.0));
        }
    }

    #[allow(unused_variables)]
    fn on_message(&mut self, message: Message) {
    }
}