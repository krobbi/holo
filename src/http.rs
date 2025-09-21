use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};

use crate::error::{Error, Result};

/// An HTTP server.
pub struct Server {
    /// The [`TcpListener`] listening for [`Request`]s over TCP.
    listener: TcpListener,

    /// The `Server`'s TCP/IP address.
    #[expect(dead_code)]
    address: SocketAddr,
}

impl Server {
    /// Creates a new `Server`. The returned server is bound to a TCP port and
    /// ready to accept [`Request`]s. The server will be closed when the value
    /// is dropped.
    pub fn try_new() -> Result<Self> {
        const PORT: u16 = 8080;
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, PORT)).map_err(Error::ServerOpen)?;
        let address = listener.local_addr().map_err(Error::ServerAddressQuery)?;
        Ok(Self { listener, address })
    }

    /// Accepts a new incoming [`Request`]. The returned request is bound to a
    /// TCP connection and ready for a response. The connection will be closed
    /// when the value is dropped. This function will block the calling thread
    /// until a new TCP connection is established.
    pub fn try_accept(&self) -> Result<Request<'_>> {
        let (stream, client) = self.listener.accept().map_err(Error::Connect)?;

        Ok(Request {
            server: self,
            stream,
            client,
        })
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
