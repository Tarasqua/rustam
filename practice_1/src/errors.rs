use core::fmt;
use std::error::Error;

fn main() {
    let data = download();

    // match data {
    //     Ok(_) => todo!(),
    //     Err(DownloadError::ConnectionError(_)) => todo!("retry connection"),
    //     Err(_) => todo!(),
    // }

    match &data {
        Ok(_) => todo!(),
        Err(e) => log_error(e),
    }

    print!("{:?}", data)
}

fn log_error(e: &impl Error) {
    eprintln!("Error: {:?}", e);
    println!("Error: {}", e);

    println!("\t Caused by: {:?}", e.source());
}

fn download() -> Result<Data, DownloadError> {
    // let connection = match connect() {
    //     Ok(conn) => conn,
    //     Err(e) => return Err(DownloadError::ConnectionError(e)),
    // };
    let connection = connect()?;

    // let data = match load(connection) {
    //     Ok(data) => data,
    //     Err(e) => return Err(e.into()),
    // };
    // let data = load(connection).map_err(DownloadError::LoadError)?;
    let data = load(connection)?;

    Ok(data)
}

#[derive(Debug)]
enum DownloadError {
    ConnectionError(ConnectionError),
    LoadError(LoadError),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::ConnectionError(e) => write!(f, "Connection error: {:?}", e),
            DownloadError::LoadError(e) => write!(f, "Load error: {:?}", e),
        }
    }
}

impl Error for DownloadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DownloadError::ConnectionError(e) => Some(e),
            DownloadError::LoadError(e) => Some(e),
        }
    }
}

impl From<ConnectionError> for DownloadError {
    fn from(e: ConnectionError) -> Self {
        DownloadError::ConnectionError(e)
    }
}

fn connect() -> Result<Connection, ConnectionError> {
    let smth = false;
    if smth {
        Ok(Connection)
    } else {
        Err(ConnectionError::BadAddress)
    }
}

#[derive(Debug)]
enum ConnectionError {
    BadAddress,
    TimedOut,
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::BadAddress => write!(f, "Bad address"),
            ConnectionError::TimedOut => write!(f, "Timed out"),
        }
    }
}

impl Error for ConnectionError {}

fn load(_connection: Connection) -> Result<Data, LoadError> {
    let condition = true;
    if condition {
        Ok(Data {})
    } else {
        Err(LoadError::AccessDenied)
    }
}

#[derive(Debug)]
enum LoadError {
    AccessDenied,
    NotFound,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::AccessDenied => write!(f, "Access denied"),
            LoadError::NotFound => write!(f, "Not found"),
        }
    }
}

impl Error for LoadError {}

impl From<LoadError> for DownloadError {
    fn from(value: LoadError) -> Self {
        DownloadError::LoadError(value)
    }
}

#[derive(Debug)]
struct Data;

struct Connection;
