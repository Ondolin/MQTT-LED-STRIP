use angular_units::Deg;
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt as mqtt;
use paho_mqtt::Message;
use prisma::Rgb;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{process, time::Duration};

use crate::{set_animation, PIXEL_NUMBER};

const TOPICS: &[&str] = &[
    "/LED-STRIP/status",
    "/LED-STRIP/brightness",
    "/LED-STRIP/animation/#",
    "/LED-STRIP/parameter",
    "/LED-STRIP/parameter/#",
];
const QOS: &[i32] = &[1, 1, 1, 1, 1];

pub(crate) fn mqtt_setup(
    brightness: Arc<Mutex<f32>>,
    status: Arc<Mutex<u32>>,
    message: Arc<Mutex<Message>>,
    changed: Arc<Mutex<bool>>,
) {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(std::env::var("MQTT_BROKER_ADDRESS").unwrap())
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .client_id("LED-STRIP")
        .finalize();

    // Create the client connection
    let mut mqtt_client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });
    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let mut strm = mqtt_client.get_stream(25);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_5)
            .user_name(std::env::var("MQTT_USERNAME").unwrap())
            .password(std::env::var("MQTT_CLIENT_PASSWORD").unwrap())
            .clean_session(false)
            .finalize();

        // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        mqtt_client.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        mqtt_client.subscribe_many(TOPICS, QOS).await?;

        println!("Subscribed");
        // Just loop on incoming messages.

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                if msg.topic() == TOPICS[0] {
                    if let Ok(x) = msg.payload_str().parse::<u32>() {
                        let mut lock = status.lock().unwrap();
                        *lock = x;
                    }
                } else if msg.topic() == TOPICS[1] {
                    if let Ok(x) = msg.payload_str().parse::<f32>() {
                        let mut lock = brightness.lock().unwrap();
                        *lock = f32::min(f32::max(x as f32 / 100.0, 0.0), 1.0);
                    }
                } else if msg.topic().starts_with(&TOPICS[2][..TOPICS[2].len() - 1]) {
                    let animation_name = msg.topic().split("/").last();

                    if let Some(animation_name) = animation_name {
                        match animation_name.to_lowercase().as_str() {
                            "off" => {
                                use crate::animation::Off;
                                set_animation(Box::new(Off::new()));
                            }
                            "color" => {
                                use crate::animation::SimpleColor;
                                if let Ok(color) = color_from_str(&msg.payload_str()) {
                                    set_animation(Box::new(SimpleColor::new(color)))
                                }
                            }
                            "firework" => {
                                use crate::animation::Firework;
                                set_animation(Box::new(Firework::new()));
                            }
                            "rainbow-full" => {
                                use crate::animation::FullRainbow;
                                set_animation(Box::new(FullRainbow::new(6)));
                            }
                            "rainbow" => {
                                use crate::animation::RainbowFade;
                                set_animation(Box::new(RainbowFade::new(Deg(0.0), Deg(3.0))));
                            }
                            "chase" => {
                                use crate::animation::RainbowChase;
                                set_animation(Box::new(RainbowChase::new(Deg(0.0), 30)));
                            }
                            _ => {}
                        };
                    }
                } else {
                    {
                        let mut lock = message.lock().unwrap();
                        *lock = msg.clone();
                    }
                    {
                        let mut lock = changed.lock().unwrap();
                        *lock = true;
                    }
                }
            } else {
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = mqtt_client.reconnect().await {
                    println!("Error reconnecting: {:?}", err);
                    thread::sleep(Duration::from_millis(1000));
                }
                println!("Reconnected.");
            }
        }
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}

fn color_from_str(string: &str) -> Result<Rgb<u8>, ()> {
    let split = string.replace(" ", "").replace("(", "").replace(")", "");
    let mut split = split.split(",");

    let r = split.next();
    let g = split.next();
    let b = split.next();

    if r.is_some() && g.is_some() && b.is_some() {
        let r = r.unwrap().parse::<u8>();
        let g = g.unwrap().parse::<u8>();
        let b = b.unwrap().parse::<u8>();

        if r.is_ok() && g.is_ok() && b.is_ok() {
            return Ok(Rgb::new(r.unwrap(), g.unwrap(), b.unwrap()));
        }

        println!("Could not parse u8 values for colors.")
    }

    println!("Could not parse 3 color values.");

    Err(())
}
