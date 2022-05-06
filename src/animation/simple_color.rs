use crate::animation::Animation;
use crate::Strip;
use paho_mqtt::Message;
use prisma::Rgb;
use std::sync::{Arc, Mutex};

pub struct SimpleColor {
    color: Rgb<u8>,
}

impl SimpleColor {
    pub fn new(color: Rgb<u8>) -> SimpleColor {
        SimpleColor { color }
    }
}

impl Animation for SimpleColor {
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        let mut lock = strip.lock().unwrap();
        lock.set_all(self.color);
    }

    fn on_message(&mut self, message: Message) {
        let body = message.payload_str();
        let color_rgb: Vec<&str> = body.split("/").collect();
        let r: u8;
        let g: u8;
        let b: u8;
        if color_rgb.len() != 3 {
            println!("Invalid color format; expected format: r/g/b; Code 0");
            return;
        }
        if let Ok(r_str) = color_rgb[0].parse::<u8>() {
            r = r_str;
        } else {
            println!("Invalid color format; expected format: r/g/b; Code 1");
            return;
        }
        if let Ok(g_str) = color_rgb[1].parse::<u8>() {
            g = g_str;
        } else {
            println!("Invalid color format; expected format: r/g/b; Code 2");
            return;
        }
        if let Ok(b_str) = color_rgb[2].parse::<u8>() {
            b = b_str;
        } else {
            println!("Invalid color format; expected format: r/g/b; Code 3");
            return;
        }
        self.color = Rgb::new(r, g, b);
    }
}
