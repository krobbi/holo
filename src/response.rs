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
            packet.put_status_line(code, reason);
        }

        packet.put_header("Connection", "close");

        if config.cross_origin_isolation() {
            packet.put_header("Cross-Origin-Embedder-Policy", "require-corp");
            packet.put_header("Cross-Origin-Opener-Policy", "same-origin");
        }

        if let Page::Redirect(url) = &self.page {
            packet.put_header("Location", url);
        }

        if let Some(mime) = self.page.mime() {
            packet.put_header("Content-Type", mime);
        }

        {
            let content = self.page.into_content();
            packet.put_header("Content-Length", &content.len().to_string());
            packet.put_end_of_headers();
            packet.put_content(content);
        }

        stream.write_all(&packet)
    }
}
