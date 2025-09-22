use std::{
    fmt::{self, Display, Formatter, Write as _},
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

use crate::{
    config::Config,
    error::{Error, Result},
};

/// An HTTP response status code.
#[derive(Clone, Copy, Default)]
#[repr(u16)]
pub enum Status {
    /// The request succeeded.
    #[default]
    Ok = 200,

    /// The client does not have access rights to the content.
    Forbidden = 403,
}

impl Status {
    /// Returns the `Status`' code.
    pub fn code(self) -> u16 {
        self as u16
    }

    /// Returns the `Status`' reason phrase.
    pub fn reason(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Forbidden => "Forbidden",
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
    /// Creates a new `Server` from configuration data. The returned server is
    /// bound to a TCP port and ready to accept [`Request`]s. The server will be
    /// closed when the value is dropped.
    pub fn try_new(config: &Config) -> Result<Self> {
        let listener =
            TcpListener::bind((Ipv4Addr::LOCALHOST, config.port())).map_err(Error::ServerOpen)?;

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
    client: SocketAddr,

    /// The `Request`'s URI.
    uri: String,
}

impl Request<'_> {
    /// Returns whether the `Request` was sent from the host machine.
    pub fn is_local(&self) -> bool {
        self.client.ip().is_loopback()
    }

    /// Returns the `Request`'s URI.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Consumes the `Request` and sends a response to the client.
    pub fn try_respond(mut self, response: &impl Respond) -> Result<()> {
        let status = response.status();
        let mut packet = format!(
            "HTTP/1.1 {} {}\r\n\
            Connection: close\r\n\
            Cross-Origin-Opener-Policy: same-origin\r\n\
            Cross-Origin-Embedder-Policy: require-corp\r\n",
            status.code(),
            status.reason()
        );

        if let Some(location) = response.location() {
            let _ = write!(packet, "Location: {}\r\n", location.as_ref());
        }

        if let Some(media_type) = response.media_type() {
            let _ = write!(packet, "Content-Type: {}\r\n", media_type.as_ref());
        }

        let body = response.body();
        let body = body.as_ref();
        let _ = write!(packet, "Content-Length: {}\r\n\r\n", body.len());
        let mut packet = packet.into_bytes();
        packet.extend_from_slice(body);
        self.stream.write_all(&packet).map_err(Error::ResponseSend)
    }
}

/// A trait for objects which can be sent as an HTTP response.
pub trait Respond {
    /// Returns the HTTP response [`Status`] associated with the object.
    fn status(&self) -> Status {
        Status::default()
    }

    /// Returns the HTTP location header field associated with the object.
    /// Returns [`None`] if the object should not be sent with a location header
    /// field.
    fn location(&self) -> Option<impl AsRef<str>> {
        None::<&str>
    }

    /// Returns the media type associated with the object. Returns [`None`] if
    /// the object has no known media type.
    fn media_type(&self) -> Option<impl AsRef<str>> {
        None::<&str>
    }

    /// Returns the HTTP message body associated with the object.
    fn body(&self) -> impl AsRef<[u8]>;
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

    Ok(decode_uri(trim_query_string(uri)))
}

/// Returns a URI with any trailing query string removed.
fn trim_query_string(uri: &str) -> &str {
    match uri.split_once('?') {
        None => uri,
        Some((prefix, _)) => prefix,
    }
}

/// Returns a URI with any percent encoding decoded.
fn decode_uri(uri: &str) -> String {
    percent_encoding::percent_decode_str(uri)
        .decode_utf8_lossy()
        .into()
}
