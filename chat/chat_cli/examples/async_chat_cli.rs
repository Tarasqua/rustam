use chat_client::asnc::ChatClient;
use std::error::Error;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = get_server_addr().await;

    // INFO: Reading command line arguments
    let mut cli_args = std::env::args().skip(1); // NOTE: skip the first arg (the program name)
    let Some(action) = cli_args.next() else {
        return Err(String::from("No action provided, use 'append' or 'fetch'").into());
    };

    println!("Performing action: {action}...");

    // INFO: Connecting to the chat server
    let mut client = ChatClient::new(address).await?;

    if action == "fetch" {
        // INFO: Fetching chat history
        let chat_history = client.fetch().await?;
        println!("Chat history:");
        println!("{}", chat_history);
        return Ok(());
    }

    if action == "append" {
        // INFO: Appending a message
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        client.append(&msg).await?;
        return Ok(());
    }

    Err(String::from("Unknown action, use 'append' or 'fetch'").into())
}

/// # Reads the server address from the settings file
async fn get_server_addr() -> String {
    fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
