use chrono::{DateTime, Utc};
use std::error::Error;
use std::{fmt, fs};
use stp::server::{StpConnection, StpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let addr = // reading from settings/addr or using default value
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr)?;

    // making a new chat
    let mut chat = Chat::default();

    loop {
        let Ok(connection) = server.accept() else {
            continue;
        };
        process_connection(connection, &mut chat);
    }
}

fn process_connection(mut conn: StpConnection, chat: &mut Chat) {
    let address = match conn.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "unknown".to_string(),
    };

    loop {
        // process a single request from the client
        let process_result = conn.process_request(|request| {
            // returning the chat history on "fetch"
            if request == "fetch" {
                return chat.history();
            }

            // appending a message on "append:" prefix
            if let Some(message) = request.strip_prefix("append:") {
                return chat.append(address.clone(), message.into());
            }

            // returning an error for unknown requests
            format!("Unknown request: {}", request)
        });

        if let Err(e) = process_result {
            eprintln!("Error processing request: {}", e);
            break;
        }
    }
}

/// # Chat with messages sequence
#[derive(Default, Clone)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    /// # Returns the chat history as a string
    pub fn history(&self) -> String {
        self.messages.iter().map(|m| m.to_string()).collect()
    }

    /// # Adds a message to the chat
    pub fn append(&mut self, from: String, msg: String) -> String {
        let sent = Utc::now();
        let message = Message { sent, from, msg };
        self.messages.push(message.clone());
        message.to_string()
    }
}

/// # Chat message
#[derive(Debug, Clone)]
pub struct Message {
    sent: DateTime<Utc>,
    from: String,
    msg: String,
}

// INFO: let us use to_string() to convert a message to a string
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{} - {}]: {}", self.sent, self.from, self.msg)
    }
}
