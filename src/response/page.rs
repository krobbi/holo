use super::Status;

/// An HTTP response's page.
pub enum Page {
    /// File with MIME type and content.
    File(Option<String>, Vec<u8>),

    /// Redirect to URL.
    Redirect(String),

    /// Error message.
    Error(Status),
}

impl Page {
    /// Get the status.
    pub(super) fn status(&self) -> &Status {
        match self {
            Page::File(_, _) => &Status::Ok,
            Page::Redirect(_) => &Status::MovedPermanently,
            Page::Error(status) => status,
        }
    }

    /// Get the MIME type essence.
    pub(super) fn mime(&self) -> Option<&str> {
        match self {
            Page::File(mime, _) => mime.as_deref(),
            Page::Redirect(_) => Some("text/html"),
            Page::Error(_) => Some("text/plain"),
        }
    }

    /// Convert the page into content.
    pub(super) fn into_content(self) -> Vec<u8> {
        match self {
            Page::File(_, content) => content,
            Page::Redirect(url) => redirect_content(&url),
            Page::Error(status) => Vec::from(status.reason()),
        }
    }
}

/// Create new redirect content from a target URL.
fn redirect_content(url: &str) -> Vec<u8> {
    static TEMPLATE: &str = include_str!("../../res/redirect.html");
    TEMPLATE.replace("{{url}}", url).into_bytes()
}
