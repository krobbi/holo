use std::net::{Ipv4Addr, TcpListener};

/// An HTTP server.
pub struct Server {
    /// The [`TcpListener`] listening for HTTP requests over TCP.
    #[expect(dead_code)]
    listener: TcpListener,
}

impl Server {
    /// Creates a new `Server`. The returned server is bound to a TCP port and
    /// ready to accept HTTP requests. The server will be closed when the value
    /// is dropped.
    pub fn new() -> Self {
        const PORT: u16 = 8080;
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, PORT)).unwrap();
        Self { listener }
    }
}
