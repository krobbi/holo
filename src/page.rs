use crate::http::{Respond, Status};

/// A page that can be sent as an HTTP response.
pub enum Page {
    /// A test `Page` for a [`Request`](crate::http::Request)'s URI.
    Test(String),

    /// An error `Page` for an HTTP response [`Status`] code.
    Error(Status),
}

impl Respond for Page {
    fn status(&self) -> Status {
        match self {
            Self::Test(_) => Status::default(),
            Self::Error(status) => *status,
        }
    }

    fn media_type(&self) -> Option<impl AsRef<str>> {
        match self {
            Self::Test(_) => Some("text/plain; charset=utf-8"),
            Self::Error(_) => Some("text/html; charset=utf-8"),
        }
    }

    fn body(&self) -> impl AsRef<[u8]> {
        match self {
            Self::Test(uri) => format!("Requested URI: '{}'\n", uri.escape_default()),
            Self::Error(status) => render_error(*status),
        }
    }
}

/// Renders an error HTML document from an HTTP response [`Status`] code.
fn render_error(status: Status) -> String {
    let title = format!("{} - {}", status.code(), status.reason());
    let content = "<p>An error occurred.</p>";
    render_html(&title, content)
}

/// Renders an HTML document from a title and content.
fn render_html(title: &str, content: &str) -> String {
    static BASE: &str = include_str!("../res/base.html");
    BASE.replace("{{title}}", title)
        .replace("{{content}}", content)
}
