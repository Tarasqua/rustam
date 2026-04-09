use crate::error::{ConnectionError, RequestError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

/// # An async client for the STP protocol
pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    /// # Connects to the server at the given address and checks for STP protocol support
    pub async fn connect<A>(address: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(address).await?;
        Self::try_handshake(stream).await
    }

    /// # Tries to perform an STP handshake with the server:
    /// sends "clnt" bytes and waits for "serv" bytes as a response
    async fn try_handshake(mut stream: TcpStream) -> Result<Self, ConnectionError> {
        stream.write_all(b"clnt").await?;
        let mut buf = [0u8; 4];
        _ = stream.read_exact(&mut buf).await;
        if &buf != b"serv" {
            Err(ConnectionError::BadHandshake)
        } else {
            Ok(Self { stream })
        }
    }

    /// # Sends a request to the server and returns the response
    pub async fn send_request<R: AsRef<str>>(
        &mut self,
        request: R,
    ) -> Result<String, RequestError> {
        super::send_string(request, &mut self.stream).await?;
        let response = super::recv_string(&mut self.stream).await?;
        Ok(response)
    }
}
