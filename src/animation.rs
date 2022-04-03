#![allow(unused_variables)]
use std::sync::{Arc, Mutex};
use angle::Deg;
use prisma::{FromColor, Hsv, Rgb};
use speedy2d::color::Color;
use crate::Strip;
use paho_mqtt as mqtt;
pub trait Animation{
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>){}
    fn update(&mut self, strip: Arc<Mutex<Strip>>){}
    fn on_message(&mut self, message: mqtt::Message){}
}

pub fn hsv_to_rgb(h: u32, s: f32, v: f32) -> Color {
    let hsv_color = Hsv::new(Deg(h as f32), s, v);
    let rgb_color = Rgb::from_color(&hsv_color);
    Color::from_rgb(rgb_color.red(), rgb_color.green(), rgb_color.blue())
}

pub struct Off{}

impl Off{
    pub fn new() -> Off{
        Off{}
    }
}

impl Animation for Off{
    fn initialize(&mut self, _strip: Arc<Mutex<Strip>>){
        let mut strip = _strip.lock().unwrap();
        strip.reset();
    }
}