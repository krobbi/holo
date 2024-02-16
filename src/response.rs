use std::{io::Write, net::TcpStream};

use crate::error::Result;

/// An HTTP response status.
pub enum Status {
    /// The request succeeded.
    Ok,

    /// The server cannot find the requested resource.
    NotFound,
}

impl Status {
    /// Get the status code.
    fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::NotFound => 404,
        }
    }

    /// Get the reason phrase.
    fn reason(&self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::NotFound => "Not Found",
        }
    }
}

/// An HTTP response.
pub struct Response {
    /// The response status.
    status: Status,

    /// The content.
    content: Vec<u8>,
}

impl Response {
    /// Create a new HTTP OK response from content.
    pub fn ok(content: Vec<u8>) -> Response {
        let status = Status::Ok;
        Response { status, content }
    }

    /// Create a new HTTP error response from a status.
    pub fn error(status: Status) -> Response {
        let code = status.code();
        let reason = status.reason();

        debug_assert!(
            (400..=599).contains(&code),
            "Status '{code} {reason}' is not an error."
        );

        let content = format!("{code} {reason}\r\n").as_bytes().to_vec();
        Response { status, content }
    }

    /// Write the HTTP response to a TCP stream.
    pub fn write(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_all(&self.to_vec())?;
        Ok(())
    }

    /// Get the header.
    fn header(&self) -> String {
        let status_line = self.status_line();
        let header_fields = self.header_fields();
        format!("{status_line}\r\n{header_fields}\r\n\r\n")
    }

    /// Get the status line.
    fn status_line(&self) -> String {
        let status = &self.status;
        let code = status.code();
        let reason = status.reason();
        format!("HTTP/1.1 {code} {reason}")
    }

    /// Get the header fields.
    fn header_fields(&self) -> String {
        let length = self.content.len();
        format!("Content-Length: {length}")
    }

    /// Convert the HTTP response to a byte vector.
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = self.header().as_bytes().to_vec();
        bytes.append(&mut self.content.clone());
        bytes
    }
}
