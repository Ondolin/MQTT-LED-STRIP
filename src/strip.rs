#![allow(dead_code)]

use prisma::Rgb;

#[derive(Clone)]
pub struct Strip {
    pixels: Vec<Rgb<u8>>,
    width: usize,
    shut_down: bool,
    brightness: f32,
}

lazy_static! {
    static ref BLACK: Rgb<u8> = Rgb::new(0, 0, 0);
    static ref WHITE: Rgb<u8> = Rgb::new(255, 255, 255);
}

fn get_pixel_brightness(color: Rgb<u8>, brightness: f32) -> Rgb<u8> {
    Rgb::new(
        ((color.red() as f32 * brightness) as u32 % 255) as u8,
        ((color.green() as f32 * brightness) as u32 % 255) as u8,
        ((color.blue() as f32 * brightness) as u32 % 255) as u8,
    )
}

impl Strip {
    pub fn new(width: usize) -> Strip {
        Strip {
            pixels: vec![*BLACK; width],
            width,
            shut_down: false,
            brightness: 1.0,
        }
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
    }

    pub fn get_pixel_length(&self) -> usize {
        self.pixels.len()
    }

    pub fn shutdown(&mut self) {
        self.shut_down = true;
    }

    pub fn get_pixels(&self) -> Vec<Rgb<u8>> {
        if self.shut_down {
            vec![*BLACK; self.width]
        } else {
            self.pixels.clone()
        }
    }

    #[cfg(not(feature = "simulate"))]
    pub fn get_led_stip_pixels(&self) -> Vec<u8> {
        use ws2818_rgb_led_spi_driver::encoding::encode_rgb;
        let mut pixels = Vec::new();

        for pixel in &self.pixels {
            let pixel_with_brightness = get_pixel_brightness(pixel.clone(), self.brightness);

            let encoded_rgb = if self.shut_down {
                encode_rgb(0, 0, 0)
            } else {
                encode_rgb(
                    pixel_with_brightness.red(),
                    pixel_with_brightness.green(),
                    pixel_with_brightness.blue(),
                )
            };

            pixels.extend_from_slice(&encoded_rgb);
        }

        pixels
    }

    pub fn get_width(&self) -> usize {
        self.width.clone()
    }

    pub fn reset(&mut self) {
        self.pixels = vec![*BLACK; self.width];
    }

    pub fn set_all(&mut self, color: Rgb<u8>) {
        self.pixels = vec![color; self.width];
    }

    pub fn set_pixel(&mut self, x: usize, color: Rgb<u8>) {
        if x >= self.width {
            return;
        }
        self.pixels[x] = color;
    }

    pub fn get_pixel(&self, x: usize) -> Rgb<u8> {
        if self.shut_down {
            return *BLACK;
        }
        if x >= self.width {
            return *WHITE;
        }
        self.pixels[x]
    }

    pub fn push_pixel(&mut self, color: Rgb<u8>) {
        self.pixels.splice(0..0, vec![color]);
        self.pixels.remove(self.pixels.len() - 1);
    }
}
