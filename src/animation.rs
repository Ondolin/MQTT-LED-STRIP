use crate::Strip;
use paho_mqtt as mqtt;
use std::sync::{Arc, Mutex};

pub trait Animation {
    fn initialize(&mut self, _strip: Arc<Mutex<Strip>>, _brightness: f32) {}
    fn update(&mut self, _strip: Arc<Mutex<Strip>>, _brightness: f32) {}
    fn on_message(&mut self, _message: mqtt::Message) {}
    fn terminate(&mut self) {}
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
