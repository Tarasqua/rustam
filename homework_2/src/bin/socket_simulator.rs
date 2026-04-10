//! Smart socket simulator — a TCP server that simulates a smart socket.
//!
//! # Usage
//! ```text
//! cargo run --bin socket_simulator -- --addr 127.0.0.1:8080
//! ```

use std::sync::{Arc, Mutex};

use clap::Parser;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use homework_2::socket::protocol::{TcpCommand, TcpResponse};

/// CLI arguments for the socket simulator.
#[derive(Parser, Debug)]
#[command(about = "Smart socket TCP simulator")]
struct Args {
    /// TCP address to listen on (e.g. 127.0.0.1:8080)
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
}

/// Internal state of the simulated socket.
struct SocketState {
    /// Whether the socket is currently on.
    is_on: bool,
    /// Simulated power consumption in watts (non-zero when on).
    power_watts: f32,
}

impl SocketState {
    fn new() -> Self {
        Self {
            is_on: false,
            power_watts: 0.0,
        }
    }
}

/// Handles a single TCP client connection.
///
/// Reads newline-terminated commands, processes them against shared state,
/// and writes responses back.
async fn handle_client(stream: tokio::net::TcpStream, state: Arc<Mutex<SocketState>>) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
        let response = match TcpCommand::decode(&line) {
            Ok(TcpCommand::TurnOn) => {
                let mut s = state.lock().unwrap();
                s.is_on = true;
                s.power_watts = 220.0;
                println!("[socket_simulator] TURN_ON → OK");
                TcpResponse::Ok
            }
            Ok(TcpCommand::TurnOff) => {
                let mut s = state.lock().unwrap();
                s.is_on = false;
                s.power_watts = 0.0;
                println!("[socket_simulator] TURN_OFF → OK");
                TcpResponse::Ok
            }
            Ok(TcpCommand::PowerQuery) => {
                let s = state.lock().unwrap();
                let w = s.power_watts;
                println!("[socket_simulator] POWER → {w} W");
                TcpResponse::Power(w)
            }
            Err(e) => {
                println!("[socket_simulator] unknown command: {e}");
                TcpResponse::Error(format!("unknown command: {e}"))
            }
        };

        if writer
            .write_all(response.encode().as_bytes())
            .await
            .is_err()
        {
            break;
        }
        line.clear();
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind(&args.addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to {}: {e}", args.addr));

    println!("[socket_simulator] Listening on {}", args.addr);

    let state = Arc::new(Mutex::new(SocketState::new()));

    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                println!("[socket_simulator] New connection from {peer}");
                let state = state.clone();
                tokio::spawn(handle_client(stream, state));
            }
            Err(e) => eprintln!("[socket_simulator] Accept error: {e}"),
        }
    }
}
