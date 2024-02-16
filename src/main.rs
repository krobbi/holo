mod error;
mod request;
mod response;

use std::{
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use error::Result;
use request::Request;
use response::{Response, Status};

/// The content to respond with for an OK response.
const OK_CONTENT: &[u8] = include_str!("htdocs/index.html").as_bytes();

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
    let response = respond_to_request(&request);
    response.write(&mut stream)?;
    Ok(())
}

/// Respond to an HTTP request with an HTTP response.
fn respond_to_request(request: &Request) -> Response {
    match request.path() {
        "" => Response::ok(OK_CONTENT.to_vec()),
        "sleep" => {
            thread::sleep(Duration::from_secs(5));
            Response::ok(OK_CONTENT.to_vec())
        }
        _ => Response::error(Status::NotFound),
    }
}
