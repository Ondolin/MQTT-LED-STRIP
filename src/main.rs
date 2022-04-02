mod strip;
mod mqtt;
mod animation;
mod windowhandler;
mod rainbow_chase;
mod rainbow_fade;
mod full_rainbow;

use crate::strip::Strip;

extern crate fps_clock;
extern crate angular_units as angle;

use speedy2d::dimen::Vector2;

use std::sync::{Arc, Mutex};
use std::thread;
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;

use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;
use crate::animation::Off;
use crate::full_rainbow::FullRainbow;
use crate::rainbow_chase::RainbowChase;
use crate::rainbow_fade::RainbowFade;

fn main() {
    // global parameter
    let pixel_size = 30;
    let num_pixel = 50;
    let use_window = true;
    let neopixel_frames_per_second = 5;

    // initialize everything
    let strip = Arc::new(Mutex::new(strip::Strip::new(num_pixel, pixel_size)));
    let strip_copy = strip.clone();

    // animation thread
    thread::spawn(move || {
        animation(strip_copy);
    });


    if use_window {
        // display thread
        let window = speedy2d::Window::new_centered("Strip", Vector2::new(num_pixel as u32 * pixel_size as u32, pixel_size as u32)).unwrap();
        let stripwindowhandler = windowhandler::StripWindowHandler::new(strip);
        window.run_loop(stripwindowhandler);
    }
    else{
        // use neopixel
        let mut fps = fps_clock::FpsClock::new(neopixel_frames_per_second);
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

fn animation(strip: Arc<Mutex<Strip>>) {
    let start_status = 3;
    let frames_per_second = 5;


    let width;
    {
        let strip = strip.lock().unwrap();
        width = strip.get_width();
    }

    let status: Arc<Mutex<u32>> = Arc::new(Mutex::new(start_status));

    let status_copy = status.clone();
    thread::spawn(move || {
        mqtt::mqtt_setup(&status_copy);
    });
    let mut fps = fps_clock::FpsClock::new(frames_per_second);
    let mut prev_status: u32 = u32::MAX;
    let mut animations: Vec<Box<dyn animation::Animation>> =
        vec![
            Box::new(Off::new()),
            Box::new(RainbowChase::new(0, 30, width as u32)),
            Box::new(RainbowFade::new(0, 3)),
            Box::new(FullRainbow::new(6))
        ];
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
            prev_status = local_status;
            animations[local_status as usize].initialize(strip.clone());
        }
        animations[local_status as usize].update(strip.clone());
    }
}
