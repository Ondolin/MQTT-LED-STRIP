use crate::{Animation, Strip};
use paho_mqtt::Message;
use speedy2d::color::Color;
use std::sync::{Arc, Mutex};

pub struct BeatDetector {
    percentage: u32,
    color: Color,
}

impl BeatDetector {
    pub fn new() -> Self {
        BeatDetector {
            percentage: 0,
            color: Color::GREEN,
        }
    }
}

impl Animation for BeatDetector {
    fn update(&mut self, strip: Arc<Mutex<Strip>>, brightness: f32) {
        let mut strip = strip.lock().unwrap();
        strip.reset();
        let pixels_to_fill = ((self.percentage as f32) * strip.get_pixel_length() as f32) as u32;

        let actual_color = Color::from_rgb(
            self.color.r() * brightness,
            self.color.g() * brightness,
            self.color.b() * brightness,
        );

        for i in 0..pixels_to_fill {
            strip.set_pixel(i as usize, actual_color);
        }
    }

    fn on_message(&mut self, message: Message) {
        if message.topic().contains("beat") {
            self.percentage = message.payload_str().parse::<u32>().unwrap();
        }
    }
}
