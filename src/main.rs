use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let port = "4000";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Do something with the request
    let req = String::from_utf8_lossy(&buffer[..]);

    // Build response
    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("index.html").unwrap();

    let res = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    // Send response
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

