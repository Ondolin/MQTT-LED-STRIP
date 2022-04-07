mod strip;
mod mqtt;
mod animation;
mod windowhandler;
mod rainbow_chase;
mod rainbow_fade;
mod full_rainbow;
mod fireworks;
mod simple_color;
//mod audio_visualizer;

extern crate fps_clock;
extern crate angular_units as angle;

use std::sync::{Arc, Mutex};
use std::{thread, process};

use paho_mqtt::Message;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;

use ctrlc;

use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

use crate::animation::Animation;
use crate::animation::Off;
use crate::full_rainbow::FullRainbow;
use crate::rainbow_chase::RainbowChase;
use crate::rainbow_fade::RainbowFade;
use crate::fireworks::Firework;
use crate::simple_color::SimpleColor;
use crate::strip::Strip;
//use crate::audio_visualizer::AudioVisualizer;



fn main() {
    // global parameter
    let pixel_size = 30; // only important if use_window is true
    let num_pixel = 77;
    let use_window = true;
    let frames_per_second: u32 = 20;
    let start_status = 0;
    // edit the animations down below

    // initialize everything
    let strip = Arc::new(Mutex::new(strip::Strip::new(num_pixel)));
    let strip_copy = strip.clone();

    // animation thread
    thread::spawn(move || {
        let animations: Vec<Box<dyn Animation>> =
            vec![
                Box::new(Off::new()),
                Box::new(RainbowChase::new(0, 30, num_pixel as u32)),
                Box::new(RainbowFade::new(0, 3)),
                Box::new(FullRainbow::new(6)),
                Box::new(Firework::new()),
                Box::new(SimpleColor::new(Color::from_rgb(1.0, 0.0, 0.0))),
                //Box::new(AudioVisualizer::new()),
            ];
        animation(strip_copy, frames_per_second, animations, start_status);
    });

    // setup ctrlc handling
    let ctrl_strip_copy = strip.clone();
    let fps_copy = frames_per_second;
    ctrlc::set_handler(move || {
        {
            let mut lock = ctrl_strip_copy.lock().unwrap();
            lock.shutdown();
        }
        println!("\nShutting down...");
        thread::sleep(std::time::Duration::from_millis((1500.0 / fps_copy as f32).ceil() as u64));
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");


    if use_window {
        // display thread
        let window = speedy2d::Window::new_centered("Strip", Vector2::new(num_pixel as u32 * pixel_size as u32, pixel_size as u32)).unwrap();
        let stripwindowhandler = windowhandler::StripWindowHandler::new(strip, pixel_size);
        window.run_loop(stripwindowhandler);
    }
    else{
        // use neopixel
        let mut fps = fps_clock::FpsClock::new(frames_per_second);
        let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
        loop{
            let local_strip;
            {
                let strip = strip.lock().unwrap();
                local_strip = (*strip).clone();
            }
            let mut spi_encoded_rgb_bits = vec![];
            for pixel in local_strip.get_pixels().iter(){
                let rgb = encode_rgb((pixel.r() * 255.0).floor() as u8 , (pixel.g() * 255.0).floor() as u8, (pixel.b() * 255.0).floor() as u8);
                spi_encoded_rgb_bits.extend_from_slice(&rgb);
            }
            adapter.write_encoded_rgb(&spi_encoded_rgb_bits).unwrap();
            fps.tick();
        }
    }
}

fn animation(strip: Arc<Mutex<Strip>>, frames_per_second: u32, mut animations: Vec<Box<dyn Animation>>, start_status: u32) {
    let message_mutex: Arc<Mutex<Message>> = Arc::new(Mutex::new(Message::default()));
    let message_clone = message_mutex.clone();

    let new_message: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let new_message_clone = new_message.clone();

    let status: Arc<Mutex<u32>> = Arc::new(Mutex::new(start_status));
    let status_copy = status.clone();
    thread::spawn(move || {
        mqtt::mqtt_setup(status_copy, message_clone, new_message_clone);
    });
    let mut fps = fps_clock::FpsClock::new(frames_per_second);
    let mut prev_status: u32 = u32::MAX;
    loop {
        fps.tick();
        let local_status;
        {
            let lock = status.lock().unwrap();
            local_status = lock.clone();
        }
        if local_status >= animations.len() as u32 {
            prev_status = u32::MAX;
            {
                let mut strip_lock = strip.lock().unwrap();
                strip_lock.reset();
            }
            continue;
        }
        if local_status != prev_status {
            if prev_status != u32::MAX {
                animations[prev_status as usize].terminate();
            }
            prev_status = local_status;
            animations[local_status as usize].initialize(strip.clone());
        }
        let has_changed;
        {
            let mut changed_lock = new_message.lock().unwrap();
            has_changed = *changed_lock;
            *changed_lock = false;
        }
        if has_changed{
            let lock = message_mutex.lock().unwrap();
            animations[local_status as usize].on_message(lock.clone());
        }
        animations[local_status as usize].update(strip.clone());
    }
}
