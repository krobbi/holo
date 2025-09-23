mod config;
mod error;
mod http;
mod page;
mod router;

use std::process::ExitCode;

use crate::{config::Config, error::Result, http::Server};

/// Runs Holo and returns an [`ExitCode`].
fn main() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.print();
            error.exit_code()
        }
    }
}

/// Runs Holo.
fn try_run() -> Result<()> {
    let config = Config::try_new()?;
    let server = Server::try_new(&config)?;
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

        let page = router::find_page(&request);

        if let Err(error) = request.try_respond(&page) {
            error.print();
        }
    }
}
