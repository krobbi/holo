use super::Status;

/// An HTTP response's page.
pub enum Page {
    /// File with MIME type and content.
    File(Option<String>, Vec<u8>),

    /// Error message.
    Error(Status),
}

impl Page {
    /// Get the status.
    pub(super) fn status(&self) -> &Status {
        match self {
            Page::File(_, _) => &Status::Ok,
            Page::Error(status) => status,
        }
    }

    /// Get the MIME type essence.
    pub(super) fn mime(&self) -> Option<&str> {
        match self {
            Page::File(mime, _) => mime.as_deref(),
            Page::Error(_) => Some("text/plain"),
        }
    }

    /// Convert the page into content.
    pub(super) fn into_content(self) -> Vec<u8> {
        match self {
            Page::File(_, content) => content,
            Page::Error(status) => Vec::from(status.reason()),
        }
    }
}
