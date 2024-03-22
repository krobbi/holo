use super::Status;

/// An HTTP response's page.
pub enum Page {
    /// File with a media type and a message body.
    File(Option<&'static str>, Vec<u8>),

    /// Index of a URL.
    Index(String),

    /// Redirect to a URL.
    Redirect(String),

    /// Error message with a response status.
    Error(Status),
}

impl Page {
    /// Get the status.
    pub(super) fn status(&self) -> Status {
        match self {
            Page::File(_, _) | Page::Index(_) => Status::Ok,
            Page::Redirect(_) => Status::MovedPermanently,
            Page::Error(status) => *status,
        }
    }

    /// Get the media type.
    pub(super) fn media_type(&self) -> Option<&str> {
        match self {
            Page::File(media_type, _) => *media_type,
            Page::Index(_) | Page::Redirect(_) | Page::Error(_) => Some("text/html; charset=utf-8"),
        }
    }

    /// Convert the page into a message body.
    pub(super) fn into_body(self) -> Vec<u8> {
        match self {
            Page::File(_, body) => body,
            Page::Index(url) => index_body(&url),
            Page::Redirect(url) => redirect_body(&url),
            Page::Error(status) => error_body(status),
        }
    }
}

/// Create a new index message body using an index URL.
fn index_body(url: &str) -> Vec<u8> {
    static TEMPLATE: &str = include_str!("../../res/index.html");
    TEMPLATE.replace("{{url}}", url).into_bytes()
}

/// Create a new redirect message body using a target URL.
fn redirect_body(url: &str) -> Vec<u8> {
    static TEMPLATE: &str = include_str!("../../res/redirect.html");
    TEMPLATE.replace("{{url}}", url).into_bytes()
}

/// Create a new error message body from an error status.
fn error_body(status: Status) -> Vec<u8> {
    static TEMPLATE: &str = include_str!("../../res/error.html");
    let code = &status.code().to_string();
    let reason = status.reason();

    TEMPLATE
        .replace("{{code}}", code)
        .replace("{{reason}}", reason)
        .into_bytes()
}
