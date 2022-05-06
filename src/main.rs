#[macro_use]
extern crate lazy_static;

mod animation;
mod mqtt;
mod strip;

#[cfg(feature = "simulate")]
mod windowhandler;
//mod audio_visualizer;

extern crate angular_units as angle;
extern crate fps_clock;

use angle::Deg;
use prisma::Rgb;

use std::sync::{Arc, Mutex};
use std::{process, thread};

use paho_mqtt::Message;

#[cfg(feature = "simulate")]
use speedy2d::dimen::Vector2;

use ctrlc;

#[cfg(not(feature = "simulate"))]
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
#[cfg(not(feature = "simulate"))]
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;

use crate::animation::Animation;
use crate::animation::Off;
// use crate::beat_detection_reciever::BeatDetector;
// use crate::fireworks::Firework;
// use crate::full_rainbow::FullRainbow;
// use crate::rainbow_chase::RainbowChase;
use crate::animation::RainbowFade;
use crate::animation::SimpleColor;
//use crate::audio_visualizer::AudioVisualizer;

use crate::strip::Strip;

use dotenv::dotenv;

const FRAMES_PER_SECOND: u32 = 20;

lazy_static! {
    static ref PIXEL_NUMBER: u32 = std::env::var("PIXEL_NUMBER")
        .expect("You need to define the amount of pixels of your LED strip.")
        .parse::<u32>()
        .expect("PIXEL_NUMBER should be an integer.");
}

#[cfg(feature = "simulate")]
const PIXEL_SIZE: u32 = 30; // only important if use_window is true

fn main() {
    dotenv().ok();

    std::env::var("MQTT_BROKER_ADDRESS").expect("You need to specify an MQTT_BROKER_ADRESS!");
    std::env::var("MQTT_USERNAME").expect("You need to specify an MQTT_USERNAME!");
    std::env::var("MQTT_CLIENT_PASSWORD").expect("You need to specify an MQTT_CLIENT_PASSWORD!");

    // initialize everything
    let strip = Arc::new(Mutex::new(Strip::new(*PIXEL_NUMBER as usize)));
    let strip_copy = strip.clone();

    // animation thread
    thread::spawn(move || {
        let animations: Vec<Box<dyn Animation>> = vec![
            Box::new(Off::new()),
            // Box::new(RainbowChase::new(0, 30, *PIXEL_NUMBER)),
            Box::new(RainbowFade::new(Deg(0.0), Deg(3.0))),
            //Box::new(FullRainbow::new(6)),
            // Box::new(Firework::new()),
            Box::new(SimpleColor::new(Rgb::new(255, 0, 0))),
            // Box::new(BeatDetector::new()),
            //Box::new(AudioVisualizer::new()),
        ];
        start_strip(strip_copy, FRAMES_PER_SECOND, animations);
    });

    // setup ctrlc handling
    let ctrl_strip_copy = strip.clone();
    ctrlc::set_handler(move || {
        {
            let mut lock = ctrl_strip_copy.lock().unwrap();
            lock.shutdown();
        }
        println!("\nShutting down...");
        thread::sleep(std::time::Duration::from_millis(
            (1500.0 / FRAMES_PER_SECOND as f32).ceil() as u64,
        ));
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    #[cfg(feature = "simulate")]
    {
        // display thread
        let window = speedy2d::Window::new_centered(
            "Strip",
            Vector2::new(*PIXEL_NUMBER * PIXEL_SIZE as u32, PIXEL_SIZE as u32),
        )
        .unwrap();
        let stripwindowhandler = windowhandler::StripWindowHandler::new(strip, PIXEL_SIZE);
        window.run_loop(stripwindowhandler);
    }

    #[cfg(not(feature = "simulate"))]
    {
        // use neopixel
        let mut fps = fps_clock::FpsClock::new(FRAMES_PER_SECOND);
        let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
        loop {
            let local_strip;
            {
                let strip = strip.lock().unwrap();
                local_strip = (*strip).clone();
            }
            let spi_encoded_rgb_bits = local_strip.get_led_stip_pixels();
            adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
            fps.tick();
        }
    }
}

fn start_strip(
    strip: Arc<Mutex<Strip>>,
    frames_per_second: u32,
    mut animations: Vec<Box<dyn Animation>>,
) {
    let message_mutex: Arc<Mutex<Message>> = Arc::new(Mutex::new(Message::default()));
    let message_clone = message_mutex.clone();

    let message_has_changed: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let message_has_changed_clone = message_has_changed.clone();

    let animation_index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let animation_index_clone = animation_index.clone();

    let brightness: Arc<Mutex<f32>> = Arc::new(Mutex::new(1.0));
    let brightness_clone = brightness.clone();

    let strip = strip.clone();

    thread::spawn(move || {
        mqtt::mqtt_setup(
            brightness_clone,
            animation_index_clone,
            message_clone,
            message_has_changed_clone,
        );
    });

    let mut fps = fps_clock::FpsClock::new(frames_per_second);
    let mut prev_status: u32 = u32::MAX;

    let brightness_clone = brightness.clone();
    loop {
        fps.tick();

        {
            let brightness = brightness_clone.lock().unwrap().clone();
            let mut strip = strip.lock().unwrap();

            strip.set_brightness(brightness.clone());
        }

        let local_status;
        {
            let animation_index = animation_index.lock().unwrap();
            local_status = animation_index.clone();
        }

        // The animation number exeeds the amount of animations
        if local_status >= animations.len() as u32 {
            prev_status = u32::MAX;
            {
                let mut strip_lock = strip.lock().unwrap();
                strip_lock.reset();
            }
            continue;
        }

        // The anuimation has changed. Terminate old one -> Start new one
        if local_status != prev_status {
            if prev_status != u32::MAX {
                animations[prev_status as usize].terminate();
            }
            prev_status = local_status;
            animations[local_status as usize].initialize(strip.clone());
        }

        let has_changed;
        {
            let mut changed_lock = message_has_changed.lock().unwrap();
            has_changed = *changed_lock;
            *changed_lock = false;
        }
        if has_changed {
            let lock = message_mutex.lock().unwrap();
            animations[local_status as usize].on_message(lock.clone());
        }
        animations[local_status as usize].update(strip.clone());
    }
}
