use server::ThreadPool;
use std::{
    fs,
    io::{BufReader, Error, ErrorKind, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt::init();
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();
    info!("Listening on {address}");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // if let Err(e) = handle_connection(stream) {
        //     error!("Error: {e:#?}");
        // };

        // thread::spawn(|| {
        //     if let Err(e) = handle_connection(stream) {
        //         error!("Error: {e:#?}");
        //     }
        // });

        pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                error!("Error: {e:#?}");
            }
        });
    }

    info!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&stream);
    // INFO: the first ? takes care of Result<String, Error> из Option. If None, it returns an error. The second ? extracts the String from the Result.
    let request_line = buf_reader
        .lines()
        .next()
        .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "Empty request"))??;

    // let http_request: Vec<String> = buf_reader
    //     .lines()
    //     .take_while(|line| match line {
    //         Ok(line) => !line.is_empty(),
    //         Err(_) => true,
    //     })
    //     .collect::<std::io::Result<Vec<String>>>()?;

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "sosal.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    // INFO: &request_line[..] since we have a match on a slice + match doesn’t do automatic referencing and dereferencing, like the equality method does
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "sosal.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sosal.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}
