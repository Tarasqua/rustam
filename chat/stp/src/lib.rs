use crate::error::{RecvError, SendError};
use std::io::{Read, Write};

pub mod asnc;
pub mod client;
pub mod error;
pub mod server;

/// # Sends a string to the writer, prefixed with its length as a 4-byte big-endian integer.
fn send_string<Data: AsRef<str>, Writer: Write>(
    data: Data,
    mut writer: Writer,
) -> Result<(), SendError> {
    let bytes = data.as_ref().as_bytes(); // data to bytes
    let len = bytes.len() as u32; // length of the string in bytes
    let len_bytes = len.to_be_bytes(); // length as 4-byte big-endian integer
    writer.write_all(&len_bytes)?; // write the length of the string (4 bytes)
    writer.write_all(bytes)?; // write the string bytes
    Ok(())
}

/// # Receives a string from the reader, prefixed with its length as a 4-byte big-endian integer.
fn recv_string<Reader: Read>(mut reader: Reader) -> Result<String, RecvError> {
    let mut buf = [0; 4]; // 4 bytes prefix buffer
    reader.read_exact(&mut buf)?; // read the length of the string (4 bytes)
    let len = u32::from_be_bytes(buf); // expected length of the string

    let mut buf = vec![0; len as _]; // NOTE: vector with message bytes (must be dynamic since we can't know the exact size of the buffer) (_ is a usize)
    reader.read_exact(&mut buf)?;
    // from_utf8 raises FromUtf8Error which is converted to RecvError::BadEncoding (ignoring in case of Ok)
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

#[cfg(test)]
mod tests {
    use super::{recv_string, send_string};

    // NOTE: generic based implementation allows testing with any Read/Write types: memory buffers instead of actual network sockets

    #[test]
    fn test_send_recv() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        send_string(&data, &mut buf).unwrap();
        let result = recv_string(&buf[..]).unwrap(); // [..] making a slice
        assert_eq!(data, result);
    }

    #[test]
    fn test_send() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        send_string(&data, &mut buf).unwrap();

        let len = u32::from_be_bytes(buf[..4].try_into().unwrap());
        let string_data = String::from_utf8(buf[4..].to_vec()).unwrap();

        assert_eq!(data, string_data);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_recv() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        buf.extend_from_slice(&5_u32.to_be_bytes()); // 5 -> hello is 5 bytes long
        buf.extend_from_slice(data.as_bytes());

        let received = recv_string(&buf[..]).unwrap();
        assert_eq!(data, received);
    }
}
