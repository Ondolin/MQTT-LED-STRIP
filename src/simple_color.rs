use std::sync::{Arc, Mutex};
use paho_mqtt::Message;
use speedy2d::color::Color;
use crate::animation::Animation;
use crate::Strip;

pub(crate) struct SimpleColor{
    color: Color,
}

impl SimpleColor{
    pub fn new(color: Color) -> SimpleColor{
        SimpleColor{
            color
        }
    }
}

impl Animation for SimpleColor{
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>, brightness: f32) {
        let mut lock = strip.lock().unwrap();
        lock.set_all(Color::from_rgb(self.color.r() * brightness, self.color.g() * brightness, self.color.b() * brightness));
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>, brightness: f32) {
        let mut lock = strip.lock().unwrap();
        lock.set_all(Color::from_rgb(self.color.r() * brightness, self.color.g() * brightness, self.color.b() * brightness));
    }

    fn on_message(&mut self, message: Message) {
        let body = message.payload_str();
        let color_rgb: Vec<&str> = body.split("/").collect();
        let r: u16;
        let g: u16;
        let b: u16;
        if color_rgb.len() != 3{
            println!("Invalid color format; expected format: r/g/b; Code 0");
            return;
        }
        if let Ok(r_str) = color_rgb[0].parse::<u16>(){
            r = r_str;
        }else{
            println!("Invalid color format; expected format: r/g/b; Code 1");
            return;
        }
        if let Ok(g_str) = color_rgb[1].parse::<u16>(){
            g = g_str;
        }else{
            println!("Invalid color format; expected format: r/g/b; Code 2");
            return;
        }
        if let Ok(b_str) = color_rgb[2].parse::<u16>(){
            b = b_str;
        }else{
            println!("Invalid color format; expected format: r/g/b; Code 3");
            return;
        }
        self.color = Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    }
}