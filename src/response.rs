mod packet;
mod page;
mod status;

pub use page::Page;
pub use status::Status;

use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::config::Config;

use packet::Packet;

/// An HTTP response.
pub struct Response {
    /// The page.
    page: Page,
}

impl Response {
    /// Create a new response from a page.
    pub fn from_page(page: Page) -> Response {
        Response { page }
    }

    /// Consume and send the response to a TCP connection.
    pub fn send(self, stream: &mut TcpStream, config: &Config) -> io::Result<()> {
        let mut packet = Vec::new();

        {
            let status = self.page.status();
            let code = status.code();
            let reason = status.reason();
            packet.append_status_line(code, reason);
        }

        packet.append_field("Connection", "close");

        if !config.cors() {
            packet.append_field("Cross-Origin-Embedder-Policy", "require-corp");
            packet.append_field("Cross-Origin-Opener-Policy", "same-origin");
        }

        if let Page::Redirect(url) = &self.page {
            packet.append_field("Location", url);
        }

        if let Some(media_type) = self.page.media_type() {
            packet.append_field("Content-Type", media_type);
        }

        {
            let body = self.page.into_body();
            packet.append_field("Content-Length", &body.len().to_string());
            packet.append_body(body);
        }

        stream.write_all(&packet)
    }
}
