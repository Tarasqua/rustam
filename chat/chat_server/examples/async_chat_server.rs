use chrono::{DateTime, Utc};
use std::error::Error;
use std::sync::Arc;
use std::{fmt, fs};
use stp::asnc::server::{StpConnection, StpServer};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = // reading from settings/addr or using default value
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr).await?;

    // making a new chat (Arc to share across connections)
    let chat = Arc::new(Chat::default());

    loop {
        let Ok(connection) = server.accept().await else {
            continue;
        };
        tokio::spawn(process_connection(connection, chat.clone()));
    }
}

async fn process_connection(mut conn: StpConnection, chat: Arc<Chat>) {
    loop {
        let chat = chat.clone();

        let address = match conn.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".to_string(),
        };

        // process a single request from the client
        let process_result = conn
            .process_request_async_new(async move |request| {
                // returning the chat history on "fetch"
                if request == "fetch" {
                    return chat.history().await;
                }

                // appending a message on "append:" prefix
                if let Some(message) = request.strip_prefix("append:") {
                    return chat.append(address, message.into()).await;
                }

                // returning an error for unknown requests
                format!("Unknown request: {}", request)
            })
            .await;

        if let Err(e) = process_result {
            eprintln!("Error processing request: {}", e);
            break;
        }
    }
}

/// # Chat with messages sequence
#[derive(Default)]
pub struct Chat {
    messages: RwLock<Vec<Message>>,
}

impl Chat {
    /// # Returns the chat history as a string
    pub async fn history(&self) -> String {
        self.messages
            .read()
            .await
            .iter()
            .map(|m| m.to_string())
            .collect()
    }

    /// # Adds a message to the chat
    pub async fn append(&self, from: String, msg: String) -> String {
        let sent = Utc::now();
        let message = Message { sent, from, msg };
        self.messages.write().await.push(message.clone());
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
