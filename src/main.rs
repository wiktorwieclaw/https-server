use http_server::ThreadPool;
use std::{
    fs,
    io::{self, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(10);
    for stream in listener.incoming().take(2) {
        pool.execute(|| {
            stream
                .and_then(handle_connection)
                .unwrap_or_else(|err| eprintln!("{err}"));
        })
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let response = make_response(status_line, filename)?;
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn make_response(status_line: &str, filename: &str) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    Ok(response)
}
