use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use percent_encoding::percent_decode_str;

use crate::error::{Error, Result};

/// An HTTP request.
pub struct Request {
    /// The requested path.
    path: String,
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

        let Ok(path) = percent_decode_str(request_line[1]).decode_utf8() else {
            return Err(Error::RequestPathNotUtf8);
        };

        let mut path = path.to_string();

        if let Some(pair) = path.split_once(|c| c == '#' || c == '?' || c == '&') {
            path = pair.0.to_string();
        }

        let path = path.trim_matches(|c| c == '/' || c == '\\').to_string();

        Ok(Request { path })
    }

    /// Get the requested path.
    pub fn path(&self) -> &str {
        &self.path
    }
}
