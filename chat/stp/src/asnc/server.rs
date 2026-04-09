use crate::error::{ConnectionError, RequestError};
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// # An async server for the STP protocol
pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    /// # Creates a new STP server listening on the given address
    pub async fn bind<A>(address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(address).await?;
        Ok(Self { tcp })
    }

    /// # Accepts a new connection from a client performing handshake
    pub async fn accept(&self) -> Result<StpConnection, ConnectionError> {
        let (stream, _) = self.tcp.accept().await?;
        Self::try_handshake(stream).await
    }

    /// # Tries to perform a handshake with the client making sure it sends the correct protocol version:
    /// waiting for the "clnt" message, responding with "serv"
    async fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectionError> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"clnt" {
            Err(ConnectionError::BadHandshake)
        } else {
            stream.write_all(b"serv").await?;
            Ok(StpConnection { stream })
        }
    }
}

/// # A connection to an STP client to handle requests
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    /// # Handles incoming requests from the client and sends responses back
    pub async fn process_request<F>(&mut self, handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(String) -> String,
    {
        let request = super::recv_string(&mut self.stream).await?;
        let response = handler(request);
        super::send_string(&response, &mut self.stream).await?;
        Ok(())
    }

    /// # Handles incoming requests from the client and sends responses back asynchronously
    pub async fn process_request_async<F, Fut>(&mut self, handler: F) -> Result<(), RequestError>
    where
        Fut: Future<Output = String>,
        F: FnOnce(String) -> Fut,
    {
        let request = super::recv_string(&mut self.stream).await?;
        let response = handler(request).await; // handler is a function that returns a Future -> we have to await it
        super::send_string(&response, &mut self.stream).await?;
        Ok(())
    }

    /// # Handles incoming requests from the client and sends responses back asynchronously using an async function
    pub async fn process_request_async_new<F>(&mut self, handler: F) -> Result<(), RequestError>
    where
        F: AsyncFnOnce(String) -> String,
    {
        let request = super::recv_string(&mut self.stream).await?;
        let response = handler(request).await; // handler is a function that returns a Future -> we have to await it
        super::send_string(&response, &mut self.stream).await?;
        Ok(())
    }

    /// # Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
