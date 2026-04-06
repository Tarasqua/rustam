fn main() {
    let a = 2;
}

fn from_conn_err_to_dowload_err(conn_err: ConnectionError) -> DownloadError {
    // my_into(conn_err)
    conn_err.my_into()
}

trait MyInto<U> {
    fn my_into(self) -> U;
}

impl<T, U> MyInto<U> for T
where
    U: From<T>,
{
    fn my_into(self) -> U {
        U::from(self)
    }
}

fn my_into<T, U>(from: T) -> U
where
    U: From<T>, // INFO: U can be converted from T
{
    U::from(from)
}

enum DownloadError {
    ConnectionError(ConnectionError),
}

impl From<ConnectionError> for DownloadError {
    fn from(value: ConnectionError) -> Self {
        DownloadError::ConnectionError(value)
    }
}

enum ConnectionError {
    BadAddress,
    TimedOut,
}
