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
            Page::Redirect(_) | Page::Error(_) => Some("text/html"),
        }
    }

    /// Convert the page into content.
    pub(super) fn into_content(self) -> Vec<u8> {
        match self {
            Page::File(_, content) => content,
            Page::Redirect(url) => redirect_content(&url),
            Page::Error(status) => error_content(&status),
        }
    }
}

/// Create new redirect content using a target URL.
fn redirect_content(url: &str) -> Vec<u8> {
    static TEMPLATE: &str = include_str!("../../res/redirect.html");
    TEMPLATE.replace("{{url}}", url).into_bytes()
}

/// Create new error content using an error status.
fn error_content(status: &Status) -> Vec<u8> {
    static TEMPLATE: &str = include_str!("../../res/error.html");
    let code = status.code();
    let reason = status.reason();

    TEMPLATE
        .replace("{{code}}", &code.to_string())
        .replace("{{reason}}", reason)
        .into_bytes()
}
