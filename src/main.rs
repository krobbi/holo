mod http;

use crate::http::Server;

/// Runs Holo.
fn main() {
    let server = Server::new();
    println!("Serving files...");
    println!("Use 'Ctrl+C' to exit.");

    loop {
        let _request = server.accept();
        println!("Request accepted!");
    }
}
