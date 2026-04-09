use std::error::Error;
use std::{fmt, io};

/// # Connection error
#[derive(Debug)]
pub enum ConnectionError {
    /// # Bad handshake error
    BadHandshake,
    /// # IO error
    Io(io::Error),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadHandshake => write!(f, "bad handshake"),
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

// # io::Error -> ConnectionError (.into(), ?)
impl From<io::Error> for ConnectionError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

// # Error source for ConnectionError (for error inspection)
impl Error for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::BadHandshake => None,
        }
    }
}

/// # Sending message error
#[derive(Debug)]
pub enum SendError {
    /// # Internal IO error
    Io(io::Error),
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for SendError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error for SendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let Self::Io(e) = self;
        Some(e)
        // match self {
        //     Self::Io(e) => Some(e),
        // }
    }
}

/// # Receiving message error
#[derive(Debug)]
pub enum RecvError {
    /// # Invalid encoding of the received string
    BadEncoding,

    /// # Internal IO error
    Io(io::Error),
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecvError::BadEncoding => write!(f, "bad encoding"),
            RecvError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for RecvError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error for RecvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RecvError::Io(e) => Some(e),
            RecvError::BadEncoding => None,
        }
    }
}

/// # Error during data exchange with the servers
#[derive(Debug)]
pub enum RequestError {
    /// # Send error
    Send(SendError),

    /// # Receive error
    Recv(RecvError),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Send(e) => write!(f, "send error: {e}"),
            RequestError::Recv(e) => write!(f, "recv error: {e}"),
        }
    }
}

impl From<SendError> for RequestError {
    fn from(value: SendError) -> Self {
        Self::Send(value)
    }
}

impl From<RecvError> for RequestError {
    fn from(value: RecvError) -> Self {
        Self::Recv(value)
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RequestError::Send(e) => Some(e),
            RequestError::Recv(e) => Some(e),
        }
    }
}
