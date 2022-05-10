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

use crate::strip::Strip;

use dotenv::dotenv;

const FRAMES_PER_SECOND: u32 = 20;

lazy_static! {
    static ref PIXEL_NUMBER: u32 = std::env::var("PIXEL_NUMBER")
        .expect("You need to define the amount of pixels of your LED strip.")
        .parse::<u32>()
        .expect("PIXEL_NUMBER should be an integer.");
    static ref CURRENT_ANIMATION: Mutex<Box<dyn Animation + Send>> =
        Mutex::new(Box::new(Off::new()));
    static ref NEW_ANIMATION: Mutex<Option<Box<dyn Animation + Send>>> = Mutex::new(None);
}

pub fn set_animation(animation: Box<dyn Animation + Send>) {
    let mut lock = NEW_ANIMATION.lock().unwrap();
    *lock = Some(animation);
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
    thread::spawn(move || start_strip(strip_copy, FRAMES_PER_SECOND));

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

fn start_strip(strip: Arc<Mutex<Strip>>, frames_per_second: u32) {
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

    let brightness_clone = brightness.clone();
    loop {
        fps.tick();

        {
            let brightness = brightness_clone.lock().unwrap().clone();
            let mut strip = strip.lock().unwrap();

            strip.set_brightness(brightness.clone());
        }

        let animation_changed = { NEW_ANIMATION.lock().unwrap().is_some() };

        // change the animation
        if animation_changed {
            {
                let mut animation = CURRENT_ANIMATION.lock().unwrap();
                animation.terminate();
            }

            // set the NEW_ANIMATION variable to None
            let mut update_animation: Option<Box<dyn Animation + Send>> = None;
            std::mem::swap(&mut update_animation, &mut *NEW_ANIMATION.lock().unwrap());

            // update the CURRENT_ANIMATION variable
            let mut old_animation = update_animation.unwrap();

            old_animation.initialize(strip.clone());

            std::mem::swap(&mut old_animation, &mut *CURRENT_ANIMATION.lock().unwrap());
        } else {
            let mut current = CURRENT_ANIMATION.lock().unwrap();
            current.update(strip.clone());
        }
    }
}
