use stp::asnc::client::StpClient;
use stp::error::{ConnectionError, RequestError};
use tokio::net::ToSocketAddrs;

/// # Async chat client using STP protocol
pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    /// # Creates a new chat client connected to the given address
    pub async fn new<A>(address: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        let stp = StpClient::connect(address).await?;
        Ok(Self { stp })
    }

    /// # Requests messages from the server
    pub async fn fetch(&mut self) -> Result<String, RequestError> {
        self.stp.send_request("fetch").await
    }

    /// # Appends a message to the server
    pub async fn append(&mut self, message: &str) -> Result<String, RequestError> {
        let request = format!("append:{}", message);
        self.stp.send_request(request).await
    }
}
