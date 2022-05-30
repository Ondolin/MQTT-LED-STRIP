use prisma::Rgb;

use crate::animation::Animation;
use crate::Strip;
use std::{sync::{Arc, Mutex}, time::{SystemTime, Duration}};

pub struct Timer {
    target_time: Duration,
    start_time: SystemTime,
}

impl Timer {
    pub fn new(timer: u64) -> Timer {
        Timer {
            target_time: Duration::from_secs(timer),
            start_time: SystemTime::now()
        }
    }
}

impl Animation for Timer {
    fn update(&mut self, strip: Arc<Mutex<Strip>>) {

        let amount_leds = {
            strip.lock().unwrap().get_width()
        };

        let percentage_past = 1.0 - (self.start_time.elapsed().unwrap().as_millis() as f32 / self.target_time.as_millis() as f32);

        if percentage_past < 0.0 {
            return;
        }

        let last_active_led = (amount_leds as f32 * percentage_past) as usize;

        for i in 0..amount_leds {
            let mut strip = strip.lock().unwrap();
            if i < last_active_led {
                strip.set_pixel(i, Rgb::new(245, 0, 0));
            } else {
                strip.set_pixel(i, Rgb::new(0, 0, 0));
            }
        }

    }
}
