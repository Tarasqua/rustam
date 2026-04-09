pub mod client;
pub mod server;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::error::{RecvError, SendError};

/// # Sends a string data, prefixed with its length as a 4-byte big-endian integer.
pub async fn send_string<Data: AsRef<str>, W: AsyncWriteExt + Unpin>(
    // Unpin guarantees that the writer is not moved during the await operation
    data: Data,
    writer: &mut W,
) -> Result<(), SendError> {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

/// # Receives a string from the reader, prefixed with its length as a 4-byte big-endian integer.
pub async fn recv_string<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<String, RecvError> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}
