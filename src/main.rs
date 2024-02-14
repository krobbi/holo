mod request;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use request::Request;

/// The content to respond with for an OK response.
const OK_CONTENT: &str = include_str!("htdocs/index.html");

/// The content to respond with for a not found response.
const NOT_FOUND_CONTENT: &str = include_str!("htdocs/404.html");

/// Listen for and handle TCP streams.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}

/// Handle a TCP stream.
fn handle_stream(mut stream: TcpStream) {
    let Some(request) = Request::read(&stream) else {
        return;
    };

    let (status_line, content) = respond(&request);
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

/// Create an HTTP response using an HTTP request.
fn respond(request: &Request) -> (&'static str, &'static str) {
    match request.url() {
        "/" => ("HTTP/1.1 200 OK", OK_CONTENT),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", OK_CONTENT)
        }
        _ => ("HTTP/1.1 404 Not Found", NOT_FOUND_CONTENT),
    }
}
