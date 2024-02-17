use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use percent_encoding::percent_decode_str;

use crate::error::{Error, Result};

/// An HTTP request.
pub struct Request {
    /// Whether the request comes from a loopback address.
    loopback: bool,

    /// The requested URL.
    url: String,
}

impl Request {
    /// Read a new HTTP request using a TCP stream.
    pub fn read(stream: &TcpStream) -> Result<Request> {
        let Some(Ok(request_line)) = BufReader::new(stream).lines().next() else {
            return Err(Error::StreamNotHttpRequest);
        };

        let request_line: Vec<&str> = request_line.split_whitespace().collect();

        if request_line.len() != 3 {
            return Err(Error::StreamNotHttpRequest);
        }

        if !request_line[2].starts_with("HTTP/") {
            return Err(Error::StreamNotHttpRequest);
        }

        let loopback = match stream.peer_addr() {
            Ok(address) => address.ip().is_loopback(),
            Err(_) => false,
        };

        let url = normalize_url(request_line[1]);
        Ok(Request { loopback, url })
    }

    /// Get whether the request comes from a loopback address.
    pub fn loopback(&self) -> bool {
        self.loopback
    }

    /// Get the requested path.
    pub fn path(&self) -> &str {
        &self.url
    }
}

/// Normalize a request URL.
fn normalize_url(url: &str) -> String {
    let mut url = match percent_decode_str(url).decode_utf8() {
        Ok(url) => url.to_string(),
        Err(_) => url.to_string(),
    };

    if let Some(pair) = url.split_once(|c| c == '#' || c == '?' || c == '&') {
        url = pair.0.to_string();
    }

    url.replace('\\', "/").trim_matches('/').to_string()
}
