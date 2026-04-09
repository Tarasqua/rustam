use crate::error::{ConnectionError, RequestError};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

/// # A server for the STP protocol
pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    /// # Creates a new STP server listening on the given address
    pub fn bind<A>(address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(address)?;
        Ok(Self { tcp })
    }

    /// # Accepts a new connection from a client performing handshake
    pub fn accept(&self) -> Result<StpConnection, ConnectionError> {
        let (stream, _) = self.tcp.accept()?;
        Self::try_handshake(stream)
    }

    /// # Tries to perform a handshake with the client making sure it sends the correct protocol version:
    /// waiting for the "clnt" message, responding with "serv"
    fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectionError> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            Err(ConnectionError::BadHandshake)
        } else {
            stream.write_all(b"serv")?;
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
    pub fn process_request<F>(&mut self, handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(String) -> String,
    {
        let request = super::recv_string(&mut self.stream)?;
        let response = handler(request);
        super::send_string(&response, &mut self.stream)?;
        Ok(())
    }

    /// # Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
