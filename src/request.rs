use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

/// An HTTP request.
pub struct Request {
    /// The requested URL.
    url: String,
}

impl Request {
    /// Optionally read a new HTTP request using a TCP stream.
    pub fn read(stream: &TcpStream) -> Option<Request> {
        let Some(Ok(request_line)) = BufReader::new(stream).lines().next() else {
            return None;
        };

        let request_line: Vec<&str> = request_line.split_whitespace().collect();

        if request_line.len() != 3 {
            return None;
        }

        Some(Request {
            url: String::from(request_line[1]),
        })
    }

    /// Get the requested URL.
    pub fn url(&self) -> &str {
        &self.url
    }
}
