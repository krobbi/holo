mod content;
mod status;

pub use content::Content;
pub use status::Status;

use std::{
    collections::HashMap,
    io::{self, Write},
    net::TcpStream,
    path::PathBuf,
};

/// An HTTP response.
pub struct Response {
    /// The response status.
    status: Status,

    /// The header fields.
    fields: HashMap<&'static str, String>,

    /// The content.
    content: Content,
}

impl Response {
    /// Create a new HTTP OK response from content.
    pub fn ok(content: Content) -> Response {
        let status = Status::Ok;
        Response::new(status, content)
    }

    /// Create a new HTTP error response from a status.
    pub fn error(status: Status) -> Response {
        let code = status.code();
        let reason = status.reason();

        debug_assert!(
            (400..=599).contains(&code),
            "Status '{code} {reason}' is not an error."
        );

        let content = Content::new(
            PathBuf::from(format!("{code}.txt")),
            format!("{code} {reason}\r\n").as_bytes().to_vec(),
        );
        Response::new(status, content)
    }

    /// Enable cross-origin isolation for the HTTP response.
    pub fn enable_cross_origin_isolation(&mut self) {
        self.insert_field("Cross-Origin-Embedder-Policy", "require-corp".to_string());
        self.insert_field("Cross-Origin-Opener-Policy", "same-origin".to_string());
    }

    /// Write the HTTP response to a TCP stream.
    pub fn write(&self, stream: &mut TcpStream) -> io::Result<()> {
        stream.write_all(&self.to_vec())
    }

    /// Create a new HTTP response from a status and content.
    fn new(status: Status, content: Content) -> Response {
        let fields = HashMap::new();
        let mut response = Response {
            status,
            fields,
            content,
        };
        response.insert_field("Connection", "close".to_string());
        response.insert_field("Content-Length", response.content.len().to_string());

        if let Some(mime) = response.content.mime() {
            response.insert_field("Content-Type", mime.essence_str().to_string());
        }

        response
    }

    /// Insert a header field into the HTTP response.
    fn insert_field(&mut self, key: &'static str, value: String) {
        self.fields.insert(key, value);
    }

    /// Get the HTTP response as a byte vector.
    fn to_vec(&self) -> Vec<u8> {
        let status = &self.status;
        let code = status.code();
        let reason = status.reason();
        let mut data = format!("HTTP/1.1 {code} {reason}\r\n");

        for (key, value) in &self.fields {
            data.push_str(format!("{key}: {value}\r\n").as_str());
        }

        data.push_str("\r\n");
        let mut data = data.as_bytes().to_vec();
        data.extend_from_slice(self.content.as_bytes());
        data
    }
}
