mod error;
mod request;
mod response;
mod router;

use std::{
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process,
};

use error::Result;
use request::Request;
use response::Response;
use router::read_file;

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
    let path = PathBuf::from(request.path());

    match read_file(&path) {
        Ok(content) => Response::ok(content),
        Err(status) => Response::error(status),
    }
}
