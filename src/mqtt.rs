use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt as mqtt;
use std::{process, time::Duration};
use std::sync::{Arc, Mutex};
use std::thread;
use paho_mqtt::Message;

const TOPICS: &[&str] = &["/Lumibaer/status", "/Lumibaer/brightness", "/Lumibaer/parameter", "/Lumibaer/parameter/#"];
const QOS: &[i32] = &[1, 1, 1, 1];

pub(crate) fn mqtt_setup(brightness: Arc<Mutex<f32>>, status: Arc<Mutex<u32>>, message: Arc<Mutex<Message>>, changed: Arc<Mutex<bool>>) {
    let host = "tcp://localhost:1883".to_string();
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("Lumibaer")
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
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
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
                if msg.topic() == TOPICS[0]{
                    if let Ok(x) = msg.payload_str().parse::<u32>() {
                        let mut lock = status.lock().unwrap();
                        *lock = x;
                    }
                }else if msg.topic() == TOPICS[1]{
                    if let Ok(x) = msg.payload_str().parse::<i32>() {
                        let mut lock = brightness.lock().unwrap();
                        *lock = f32::min(f32::max(x as f32 / 100.0, 0.0), 1.0);
                    }
                }else{
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
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = mqtt_client.reconnect().await {
                    println!("Error reconnecting: {:?}", err);
                    thread::sleep(Duration::from_millis(1000));
                }
            }
        }
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
}
