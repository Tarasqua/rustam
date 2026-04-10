//! Smart thermometer simulator — sends random temperature UDP packets.
//!
//! # Usage
//! Create a `thermometer_sim.toml` file:
//! ```toml
//! udp_target  = "127.0.0.1:8888"
//! interval_ms = 1000
//! ```
//! Then run:
//! ```text
//! cargo run --bin thermometer_simulator
//! ```

use std::time::Duration;

use rand::Rng;
use serde::Deserialize;
use tokio::net::UdpSocket;
use tokio::time::sleep;

use homework_2::thermometer::packet::UdpTemperaturePacket;

/// Configuration loaded from `thermometer_sim.toml`.
#[derive(Deserialize, Debug)]
struct Config {
    /// UDP destination address (e.g. "127.0.0.1:8888").
    udp_target: String,
    /// Interval between packets in milliseconds.
    interval_ms: u64,
}

/// Loads and parses the TOML configuration file.
fn load_config(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Cannot read {path}: {e}"))?;
    toml::from_str(&content).map_err(|e| format!("Invalid TOML in {path}: {e}"))
}

/// Generates a random temperature in the range [−50.0, +150.0] °C.
fn random_temperature() -> f32 {
    rand::thread_rng().gen_range(-50.0_f32..=150.0_f32)
}

#[tokio::main]
async fn main() {
    let config = match load_config("thermometer_sim.toml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[thermometer_simulator] Config error: {e}");
            std::process::exit(1);
        }
    };

    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .unwrap_or_else(|e| panic!("Failed to bind UDP socket: {e}"));

    println!(
        "[thermometer_simulator] Sending to {} every {} ms",
        config.udp_target, config.interval_ms
    );

    let interval = Duration::from_millis(config.interval_ms);

    loop {
        let temp = random_temperature();
        let packet = UdpTemperaturePacket(temp);

        match socket.send_to(&packet.encode(), &config.udp_target).await {
            Ok(_) => println!("[thermometer_simulator] Sent {:.1} °C", temp),
            Err(e) => eprintln!("[thermometer_simulator] Send error: {e}"),
        }

        sleep(interval).await;
    }
}
