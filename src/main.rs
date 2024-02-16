mod config;
mod error;
mod request;
mod response;
mod router;

use std::{
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process,
};

use config::Config;
use error::Result;
use request::Request;
use response::Response;
use router::read_file;

/// Handle errors from the Holo server.
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

/// Run the Holo server.
fn run() -> Result<()> {
    let config = Config::new();
    let port = config.port();
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    eprintln!("Serving files at 'http://localhost:{port}/'...");
    eprintln!("Use 'Ctrl+C' to exit.");

    for stream in listener.incoming() {
        let stream = stream?;

        if let Err(error) = serve(stream, &config) {
            eprintln!("[Error] {error}");
        }
    }

    Ok(())
}

/// Serve a TCP stream.
fn serve(mut stream: TcpStream, config: &Config) -> Result<()> {
    let request = Request::read(&stream)?;
    let response = respond(&request, config);
    response.write(&mut stream)?;
    Ok(())
}

/// Respond to an HTTP request with an HTTP response.
fn respond(request: &Request, config: &Config) -> Response {
    let path = PathBuf::from(request.path());

    let mut response = match read_file(path) {
        Ok(content) => Response::ok(content),
        Err(status) => Response::error(status),
    };

    if config.cross_origin_isolation() {
        response.enable_cross_origin_isolation();
    }

    response
}
