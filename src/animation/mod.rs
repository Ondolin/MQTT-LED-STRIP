mod off;
// mod beat_detection_reciever;
mod fireworks;
mod full_rainbow;
mod rainbow_chase;
mod rainbow_fade;
mod simple_color;

use crate::strip::Strip;
use std::sync::{Arc, Mutex};

pub use off::Off;
// pub use beat_detection_reciever::BeatDetector;
pub use fireworks::Firework;
pub use full_rainbow::FullRainbow;
pub use rainbow_chase::RainbowChase;
pub use rainbow_fade::RainbowFade;
pub use simple_color::SimpleColor;

pub trait Animation {
    fn initialize(&mut self, _strip: Arc<Mutex<Strip>>) {}
    fn update(&mut self, _strip: Arc<Mutex<Strip>>) {}
    fn on_message(&mut self, _message: paho_mqtt::Message) {}
    fn terminate(&mut self) {}
}
