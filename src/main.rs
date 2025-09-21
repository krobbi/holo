mod error;
mod http;

use std::process::ExitCode;

use crate::{error::Result, http::Server};

/// Runs Holo and returns an [`ExitCode`].
fn main() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.print();
            ExitCode::FAILURE
        }
    }
}

/// Runs Holo.
fn try_run() -> Result<()> {
    let server = Server::try_new()?;
    println!("Serving files at '{server}'...");
    println!("Use 'Ctrl+C' to exit.");

    loop {
        match server.try_accept_request() {
            Ok(request) => println!("Requested URI: '{}'", request.uri().escape_default()),
            Err(error) => error.print(),
        }
    }
}
