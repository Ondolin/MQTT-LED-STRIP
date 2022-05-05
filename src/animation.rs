#![allow(unused_variables)]
use crate::Strip;
use angle::Deg;
use paho_mqtt as mqtt;
use prisma::{FromColor, Hsv, Rgb};
use speedy2d::color::Color;
use std::sync::{Arc, Mutex};
pub trait Animation {
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>, brightness: f32) {}
    fn update(&mut self, strip: Arc<Mutex<Strip>>, brightness: f32) {}
    fn on_message(&mut self, message: mqtt::Message) {}
    fn terminate(&mut self) {}
}

pub fn hsv_to_rgb(h: u32, s: f32, v: f32) -> Color {
    let hsv_color = Hsv::new(Deg(h as f32), s, v);
    let rgb_color = Rgb::from_color(&hsv_color);
    Color::from_rgb(rgb_color.red(), rgb_color.green(), rgb_color.blue())
}

pub struct Off {}

impl Off {
    pub fn new() -> Off {
        Off {}
    }
}

impl Animation for Off {
    fn initialize(&mut self, _strip: Arc<Mutex<Strip>>, _brightness: f32) {
        let mut strip = _strip.lock().unwrap();
        strip.reset();
    }
}
