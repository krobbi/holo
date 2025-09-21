use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};

/// An HTTP server.
pub struct Server {
    /// The [`TcpListener`] listening for [`Request`]s over TCP.
    listener: TcpListener,
}

impl Server {
    /// Creates a new `Server`. The returned server is bound to a TCP port and
    /// ready to accept [`Request`]s. The server will be closed when the value
    /// is dropped.
    pub fn new() -> Self {
        const PORT: u16 = 8080;
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, PORT)).unwrap();
        Self { listener }
    }

    /// Accepts a new incoming [`Request`]. The returned request is bound to a
    /// TCP connection and ready for a response. The connection will be closed
    /// when the value is dropped. This function will block the calling thread
    /// until a new TCP connection is established.
    pub fn accept(&self) -> Request<'_> {
        let (stream, client) = self.listener.accept().unwrap();

        Request {
            server: self,
            stream,
            client,
        }
    }
}

/// An HTTP request received from a client.
pub struct Request<'a> {
    /// The [`Server`] that received the `Request`.
    #[expect(dead_code)]
    server: &'a Server,

    /// The [`TcpStream`] for communicating with the client.
    #[expect(dead_code)]
    stream: TcpStream,

    /// The client's TCP/IP address.
    #[expect(dead_code)]
    client: SocketAddr,
}
