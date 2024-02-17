use new_mime_guess::Mime;

use super::Status;

/// An HTTP response's content.
pub enum Content {
    /// Webpage with MIME type.
    Page(Option<Mime>, Vec<u8>),

    /// Error message.
    Error(Status),
}

impl Content {
    /// Get the status.
    pub(super) fn status(&self) -> &Status {
        match self {
            Content::Page(_, _) => &Status::Ok,
            Content::Error(status) => status,
        }
    }

    /// Get the MIME type essence.
    pub(super) fn mime(&self) -> Option<&str> {
        match self {
            Content::Page(Some(mime), _) => Some(mime.essence_str()),
            Content::Page(None, _) => None,
            Content::Error(_) => Some("text/plain"),
        }
    }

    /// Get the data.
    pub(super) fn data(&self) -> &[u8] {
        match self {
            Content::Page(_, data) => data,
            Content::Error(status) => status.reason().as_bytes(),
        }
    }
}
