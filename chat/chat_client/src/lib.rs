pub mod asnc;

use std::net::ToSocketAddrs;
use stp::client::StpClient;
use stp::error::{ConnectionError, RequestError};

/// # Chat client using STP protocol
pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    /// # Creates a new chat client connected to the given address
    pub fn new<A>(address: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        let stp = StpClient::connect(address)?;
        Ok(Self { stp })
    }

    /// # Requests messages from the server
    pub fn fetch(&mut self) -> Result<String, RequestError> {
        self.stp.send_request("fetch")
    }

    /// # Appends a message to the server
    pub fn append(&mut self, message: &str) -> Result<String, RequestError> {
        let request = format!("append:{}", message);
        self.stp.send_request(request)
    }
}
