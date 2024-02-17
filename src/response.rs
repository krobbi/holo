mod content;
mod status;

pub use content::Content;
pub use status::Status;

use std::{
    collections::HashMap,
    io::{self, Write},
    net::TcpStream,
};

/// An HTTP response.
pub struct Response {
    /// The content.
    content: Content,

    /// The header fields.
    fields: HashMap<&'static str, String>,
}

impl Response {
    /// Create a new HTTP response from content.
    pub fn new(content: Content) -> Response {
        let fields = HashMap::new();
        let mut response = Response { content, fields };
        response.insert_field("Connection", "close".to_string());
        response.insert_field("Content-Length", response.content.data().len().to_string());

        if let Some(mime) = response.content.mime() {
            response.insert_field("Content-Type", mime.to_string());
        }

        response
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

    /// Insert a header field into the HTTP response.
    fn insert_field(&mut self, key: &'static str, value: String) {
        self.fields.insert(key, value);
    }

    /// Get the HTTP response as a byte vector.
    fn to_vec(&self) -> Vec<u8> {
        let status = self.content.status();
        let code = status.code();
        let reason = status.reason();
        let mut data = format!("HTTP/1.1 {code} {reason}\r\n");

        for (key, value) in &self.fields {
            data.push_str(format!("{key}: {value}\r\n").as_str());
        }

        data.push_str("\r\n");
        let mut data = data.as_bytes().to_vec();
        data.extend_from_slice(self.content.data());
        data
    }
}
