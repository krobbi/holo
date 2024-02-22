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

/// Handle errors from Holo.
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

/// Run Holo.
fn run() -> io::Result<()> {
    let config = Config::new();
    let port = config.port();
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    eprintln!("Serving files at 'http://localhost:{port}/'...");
    eprintln!("Use 'Ctrl+C' to exit.");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream, &config)?;
    }

    Ok(())
}

/// Handle a TCP connection.
fn handle_connection(mut stream: TcpStream, config: &Config) -> io::Result<()> {
    let Some(request) = Request::receive(&stream) else {
        return Ok(());
    };

    let response = server::respond(&request, config);
    response.send(&mut stream, config)
}
