use crate::Animation;
use crate::Strip;
use std::sync::{Arc, Mutex};

pub struct Off {}

impl Off {
    pub fn new() -> Off {
        Off {}
    }
}

impl Animation for Off {
    fn initialize(&mut self, _strip: Arc<Mutex<Strip>>) {
        let mut strip = _strip.lock().unwrap();
        strip.reset();
    }
}
