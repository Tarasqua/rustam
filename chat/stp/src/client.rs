use crate::error::{ConnectionError, RequestError};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// # A client for the STP protocol
pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    /// # Connects to the server at the given address and checks for STP protocol support
    pub fn connect<A>(address: A) -> Result<Self, ConnectionError>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(address)?;
        Self::try_handshake(stream)
    }

    /// # Tries to perform an STP handshake with the server:
    /// sends "clnt" bytes and waits for "serv" bytes as a response
    fn try_handshake(mut stream: TcpStream) -> Result<Self, ConnectionError> {
        stream.write_all(b"clnt")?;
        let mut buf = [0u8; 4];
        _ = stream.read_exact(&mut buf);
        if &buf != b"serv" {
            Err(ConnectionError::BadHandshake)
        } else {
            Ok(Self { stream })
        }
    }

    /// # Sends a request to the server and returns the response
    pub fn send_request<R: AsRef<str>>(&mut self, request: R) -> Result<String, RequestError> {
        crate::send_string(request, &mut self.stream)?;
        let response = crate::recv_string(&mut self.stream)?;
        Ok(response)
    }
}
