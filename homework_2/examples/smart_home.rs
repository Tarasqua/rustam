//! Example: smart home with a socket and thermometer connected to simulators.
//!
//! Run the simulators first in separate terminals:
//! ```text
//! cargo run --bin socket_simulator -- --addr 127.0.0.1:8080
//! cargo run --bin thermometer_simulator   # reads thermometer_sim.toml
//! ```
//! Then run this example:
//! ```text
//! cargo run --example smart_home
//! ```

use std::time::Duration;

use homework_2::home::SmartHome;
use homework_2::socket::SmartSocket;
use homework_2::thermometer::SmartThermometer;

#[tokio::main]
async fn main() {
    // -------------------------------------------------------------------------
    // 1. Build the smart home
    // -------------------------------------------------------------------------
    let mut home = SmartHome::new("My Smart Home");

    // Create a SmartSocket client pointing at the socket simulator.
    // `new()` only resolves the address — no network connection yet.
    match SmartSocket::new("127.0.0.1:8085", Duration::from_secs(2)) {
        Ok(socket) => {
            home.add_device("Living Room", "Smart Socket", Box::new(socket));
        }
        Err(e) => eprintln!("Failed to create SmartSocket: {e}"),
    }

    // Create a SmartThermometer bound to a local UDP port.
    // This immediately spawns a background task that listens for UDP packets
    // sent by the thermometer simulator.
    match SmartThermometer::new("127.0.0.1:8887").await {
        Ok(thermo) => {
            home.add_device("Living Room", "Thermometer", Box::new(thermo));
        }
        Err(e) => eprintln!("Failed to create SmartThermometer: {e}"),
    }

    // -------------------------------------------------------------------------
    // 2. Interact with individual devices via get_device::<T>()
    //
    //    Devices are stored as Box<dyn SmartDevice> (type-erased trait objects).
    //    get_device::<T>() uses runtime downcasting (Any::downcast_ref) to
    //    recover the concrete type so you can call device-specific methods.
    // -------------------------------------------------------------------------

    // Turn the socket on before reading the report so we get a non-zero wattage.
    // NOTE: turbofish `::<SmartSocket>` is used to specify the type parameter
    if let Some(socket) = home.get_device::<SmartSocket>("Living Room", "Smart Socket") {
        println!("Turning socket ON...");
        match socket.turn_on().await {
            Ok(()) => println!("Socket is now ON."),
            Err(e) => println!("Could not turn socket on (simulator running?): {e}"),
        }
    }

    // Give the thermometer simulator a moment to send its first UDP packet.
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // Read the thermometer directly without going through the house report.
    if let Some(thermo) = home.get_device::<SmartThermometer>("Living Room", "Thermometer") {
        match thermo.temperature().await {
            Ok(t) => println!("Direct temperature read: {t:.1} °C"),
            Err(e) => println!("No temperature yet: {e}"),
        }
    }

    // -------------------------------------------------------------------------
    // 3. Print the full house report
    //
    //    house_report() calls report() on every device concurrently.
    //    Devices that fail (e.g. simulator not running) include an error
    //    description in the report instead of panicking.
    // -------------------------------------------------------------------------
    println!("\n--- House Report ---");
    let report = home.house_report().await;
    println!("{report}");
}
