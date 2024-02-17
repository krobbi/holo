mod config;
mod request;
mod response;
mod server;

use std::{
    io,
    net::{TcpListener, TcpStream},
    process,
};

use config::Config;
use request::Request;
use response::Response;

/// Handle errors from the Holo server.
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

/// Run the Holo server.
fn run() -> io::Result<()> {
    let config = Config::new();
    let port = config.port();
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    eprintln!("Serving files at 'http://localhost:{port}/'...");
    eprintln!("Use 'Ctrl+C' to exit.");

    for stream in listener.incoming() {
        let stream = stream?;
        serve_tcp(stream, &config)?;
    }

    Ok(())
}

/// Serve a TCP stream.
fn serve_tcp(mut stream: TcpStream, config: &Config) -> io::Result<()> {
    let Some(request) = Request::read(&stream) else {
        return Ok(());
    };

    let response = serve_http(&request, config);
    response.write(&mut stream)
}

/// Serve an HTTP request.
fn serve_http(request: &Request, config: &Config) -> Response {
    let content = server::serve_content(request);
    let mut response = Response::new(content);

    if config.cross_origin_isolation() {
        response.enable_cross_origin_isolation();
    }

    response
}
