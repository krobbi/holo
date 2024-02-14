use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// The content to respond with for an OK response.
const OK_CONTENT: &str = include_str!("htdocs/index.html");

/// The content to respond with for a not found response.
const NOT_FOUND_CONTENT: &str = include_str!("htdocs/404.html");

/// Listen for and handle TCP streams.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

/// Respond to an HTTP request.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", OK_CONTENT),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", OK_CONTENT)
        }
        _ => ("HTTP/1.1 404 Not Found", NOT_FOUND_CONTENT),
    };

    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
