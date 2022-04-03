use std::sync::{Arc, Mutex};
use speedy2d::color::Color;
use crate::animation::Animation;
use crate::Strip;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Firework{
    rocket: FireworkRocket
}

impl Firework{
    fn loc_initialize(&mut self){
        self.rocket = Firework::random_rocket();
    }

    fn random_rocket() -> FireworkRocket{
        let speed_range = (3, 6);
        let speed = rand::thread_rng().gen_range(speed_range.0..speed_range.1);
        let time_to_explode_range = (5, 9);
        FireworkRocket::new(speed, time_to_explode_range)
    }


    pub fn new() -> Firework{
        Firework{
            rocket: Firework::random_rocket()
        }
    }

}

impl Animation for Firework {
    #[allow(unused_variables)]
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        self.loc_initialize();
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        if self.rocket.update(){
            self.loc_initialize();
        }
        let mut strip_lock = strip.lock().unwrap();
        strip_lock.reset();
        if self.rocket.exploded{
            for spark in self.rocket.sparks.iter(){
                if spark.time_to_live > 0{
                    strip_lock.set_pixel(spark.position as usize, spark.color);
                }
            }
        }else{
            strip_lock.set_pixel(self.rocket.position as usize, Color::WHITE);
        }
    }
}

struct FireworkRocket{
    exploded: bool,
    position: u32,
    sparks: Vec<FireworkSpark>,
    speed: u32,
    time_to_explode: u32,
}

impl FireworkRocket{
    fn new(speed: u32, time_to_explode_range: (u32, u32)) -> FireworkRocket{
        let time_to_explode = rand::thread_rng().gen_range(time_to_explode_range.0..time_to_explode_range.1);
        FireworkRocket{
            exploded: false,
            position: 0,
            sparks: Vec::new(),
            speed,
            time_to_explode
        }
    }

    fn update(&mut self) -> bool{
        if self.exploded{
            let mut all_done = false;
            for spark in &mut self.sparks{
                all_done = all_done | spark.update();
            }
            return all_done;
        }else{
            self.position += self.speed;
            self.time_to_explode -= 1;
            let color_selection: Vec<Color> = vec![Color::RED, Color::BLUE, Color::MAGENTA];
            let time_range = (4, 10);
            let speed_range = (-2,4);
            let num_sparks = 10;
            if self.time_to_explode == 0{
                self.exploded = true;
                let mut rng = rand::thread_rng();
                for _ in 0..num_sparks{
                    self.sparks.push(FireworkSpark::new(self.position, color_selection.clone(), time_range, speed_range, &mut rng));
                }
            }
            return false;
        }
    }
}

struct FireworkSpark{
    position: u32,
    color: Color,
    speed: i32,
    time_to_live: u32,
}

impl FireworkSpark {
    fn new(position: u32, color_selection: Vec<Color>, time_range: (u32, u32), speed_range: (i32, i32), rng: &mut ThreadRng) -> FireworkSpark {
        let color = color_selection[rng.gen_range(0..color_selection.len())];
        let time_to_live = rng.gen_range(time_range.0..time_range.1);
        let speed = rng.gen_range(speed_range.0..speed_range.1);
        FireworkSpark {
            position,
            color,
            speed,
            time_to_live,
        }
    }

    fn update(&mut self) -> bool {
        if self.time_to_live == 0{
            self.color = Color::BLACK;
            return true;
        }else{
            self.time_to_live -= 1;
            let new_position = self.position as i32 + self.speed;
            if new_position < 0{
                self.position = 0;
                self.speed = 0;

            }else{
                self.position = new_position as u32;
            }
            return false;
        }
    }
}