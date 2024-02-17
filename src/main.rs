mod config;
mod error;
mod request;
mod response;
mod router;

use std::{
    net::{TcpListener, TcpStream},
    process,
};

use config::Config;
use error::Result;
use request::Request;
use response::Response;
use router::route_request;

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

        if let Err(error) = serve_tcp(stream, &config) {
            eprintln!("[Error] {error}");
        }
    }

    Ok(())
}

/// Serve a TCP stream.
fn serve_tcp(mut stream: TcpStream, config: &Config) -> Result<()> {
    let request = Request::read(&stream)?;
    let response = serve_http(&request, config);
    response.write(&mut stream)?;
    Ok(())
}

/// Serve an HTTP request.
fn serve_http(request: &Request, config: &Config) -> Response {
    let mut response = match route_request(request) {
        Ok(content) => Response::ok(content),
        Err(status) => Response::error(status),
    };

    if config.cross_origin_isolation() {
        response.enable_cross_origin_isolation();
    }

    response
}
