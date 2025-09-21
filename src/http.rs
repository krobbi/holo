use std::{
    fmt::{self, Display, Formatter},
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

use crate::error::{Error, Result};

/// An HTTP response status code.
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Status {
    /// The request succeeded.
    Ok = 200,
}

impl Status {
    /// Returns the `Status`' code.
    fn code(self) -> u16 {
        self as u16
    }

    /// Returns the `Status`' reason phrase.
    fn reason(self) -> &'static str {
        match self {
            Self::Ok => "OK",
        }
    }
}

/// An HTTP server.
pub struct Server {
    /// The [`TcpListener`] listening for [`Request`]s over TCP.
    listener: TcpListener,

    /// The `Server`'s TCP/IP address.
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
    pub fn try_accept_request(&self) -> Result<Request<'_>> {
        let (stream, client) = self.listener.accept().map_err(Error::Connect)?;
        let uri = try_read_request_uri(&stream)?;

        Ok(Request {
            server: self,
            stream,
            client,
            uri,
        })
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const STANDARD_HTTP_PORT: u16 = 80;
        f.write_str("http://")?;

        match self.address.ip() {
            IpAddr::V4(Ipv4Addr::LOCALHOST) => f.write_str("localhost"),
            ip => ip.fmt(f),
        }?;

        match self.address.port() {
            STANDARD_HTTP_PORT => Ok(()),
            port => write!(f, ":{port}"),
        }
    }
}

/// An HTTP request received from a client.
pub struct Request<'a> {
    /// The [`Server`] that received the `Request`.
    #[expect(dead_code)]
    server: &'a Server,

    /// The [`TcpStream`] for communicating with the client.
    stream: TcpStream,

    /// The client's TCP/IP address.
    #[expect(dead_code)]
    client: SocketAddr,

    /// The `Request`'s URI.
    uri: String,
}

impl Request<'_> {
    /// Returns the `Request`'s URI.
    #[expect(dead_code)]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Consumes the `Request` and sends a response to the client.
    pub fn try_respond(mut self, response: &impl Respond) -> Result<()> {
        let status = response.status();
        let packet = format!(
            "HTTP/1.1 {} {}\r\n\
            Connection: close\r\n\
            Cross-Origin-Opener-Policy: same-origin\r\n\
            Cross-Origin-Embedder-Policy: require-corp\r\n\
            Content-Type: text/plain; charset=utf-8\r\n\
            Content-Length: 16\r\n\
            \r\nHello, Holo v2!\n",
            status.code(),
            status.reason()
        );

        self.stream
            .write_all(packet.as_bytes())
            .map_err(Error::ResponseSend)
    }
}

/// A trait for objects which can be sent as an HTTP response.
pub trait Respond {
    /// Returns the HTTP response [`Status`] associated with the object.
    fn status(&self) -> Status;
}

/// Decodes and returns the request URI of an HTTP GET request from a
/// [`TcpStream`].
fn try_read_request_uri(stream: &TcpStream) -> Result<String> {
    const MAX_REQUEST_LEN: u64 = 8192;
    let mut request = String::new();
    BufReader::new(stream.take(MAX_REQUEST_LEN))
        .read_line(&mut request)
        .map_err(Error::RequestRead)?;

    let mut request = request.split(' ');

    let (Some("GET"), Some(uri), Some(protocol), None) = (
        request.next(),
        request.next(),
        request.next(),
        request.next(),
    ) else {
        return Err(Error::RequestNotHttpGet);
    };

    if !protocol.starts_with("HTTP/") {
        return Err(Error::RequestNotHttpGet);
    }

    Ok(trim_query_string(uri).into())
}

/// Returns a URI with any trailing query string removed.
fn trim_query_string(uri: &str) -> &str {
    match uri.split_once('?') {
        None => uri,
        Some((prefix, _)) => prefix,
    }
}
