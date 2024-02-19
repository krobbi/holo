use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

/// An HTTP request.
pub struct Request {
    /// Whether the request comes from a loopback address.
    loopback: bool,

    /// The request URL.
    url: String,
}

impl Request {
    /// Receive a new request using a TCP connection.
    pub fn receive(stream: &TcpStream) -> Option<Request> {
        let Some(Ok(request_line)) = BufReader::new(stream).lines().next() else {
            return None;
        };

        let request_line: Vec<&str> = request_line.split_whitespace().collect();

        if request_line.len() != 3 || !request_line[2].starts_with("HTTP/") {
            return None;
        }

        let loopback = match stream.peer_addr() {
            Ok(address) => address.ip().is_loopback(),
            Err(_) => false,
        };

        let url = strip_query(request_line[1]).to_string();
        Some(Request { loopback, url })
    }

    /// Get whether the request comes from a loopback address.
    pub fn loopback(&self) -> bool {
        self.loopback
    }

    /// Get the request URL.
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// Strip a query string from a request URL.
fn strip_query(url: &str) -> &str {
    match url.split_once('?') {
        Some((prefix, _)) => prefix,
        None => url,
    }
}
