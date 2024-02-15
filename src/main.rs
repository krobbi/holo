mod error;
mod request;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use error::Result;
use request::Request;

/// The content to respond with for an OK response.
const OK_CONTENT: &str = include_str!("htdocs/index.html");

/// The content to respond with for a not found response.
const NOT_FOUND_CONTENT: &str = include_str!("htdocs/404.html");

/// Handle errors from the Holo server.
fn main() {
    if let Err(error) = serve() {
        eprintln!("{error}");
        process::exit(1);
    }
}

/// Run the Holo server.
fn serve() -> Result<()> {
    eprintln!("Holo - Basic HTTP server for local hosting.\nUse Ctrl+C to exit.");
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        if let Err(error) = serve_stream(stream) {
            eprintln!("[Error] {error}");
        }
    }

    Ok(())
}

/// Serve a TCP stream.
fn serve_stream(mut stream: TcpStream) -> Result<()> {
    let request = Request::read(&stream)?;
    let (status_line, content) = respond_to_request(&request);
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

/// Create an HTTP response using an HTTP request.
fn respond_to_request(request: &Request) -> (&'static str, &'static str) {
    match request.url() {
        "/" => ("HTTP/1.1 200 OK", OK_CONTENT),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", OK_CONTENT)
        }
        _ => ("HTTP/1.1 404 Not Found", NOT_FOUND_CONTENT),
    }
}
