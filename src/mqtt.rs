use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt as mqtt;
use std::{process, time::Duration};
use std::sync::{Arc, Mutex};
use std::thread;

const TOPICS: &[&str] = &["/Lumibaer/status"];
const QOS: &[i32] = &[1];

pub(crate) fn mqtt_setup(status: &Arc<Mutex<u32>>) {
    let host = "tcp://localhost:1883".to_string();
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("Lumibaer")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });
    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .finalize();

        // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        println!("Subscribed");
        // Just loop on incoming messages.

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                if let Ok(x) = msg.payload_str().parse::<u32>() {
                    let mut lock = (*status).lock().unwrap();
                    *lock = x;
                    println!("Set new status: {}", x);
                }
                println!("Status {}", msg);
                println!("{}", msg.to_string());
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
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
