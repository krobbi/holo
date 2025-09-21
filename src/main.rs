mod error;
mod http;
mod page;

use std::process::ExitCode;

use crate::{error::Result, http::Server, page::Page};

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
        let request = match server.try_accept_request() {
            Ok(request) => request,
            Err(error) => {
                error.print();
                continue;
            }
        };

        if let Err(error) = request.try_respond(&Page::Test) {
            error.print();
        }
    }
}
