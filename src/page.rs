use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter, Write},
};

use crate::http::{self, Respond, Status};

/// A page that can be sent as an HTTP response.
pub enum Page {
    /// A file `Page` with an optional media type and contents.
    File(Option<&'static str>, Vec<u8>),

    /// An index `Page` of a URI.
    Index(String),

    /// A redirection `Page` to an encoded URI.
    Redirect(String),

    /// An error `Page` for an HTTP response [`Status`] code.
    Error(Status),
}

impl Respond for Page {
    fn status(&self) -> Status {
        match self {
            Self::File(_, _) | Self::Index(_) => Status::Ok,
            Self::Redirect(_) => Status::Found,
            Self::Error(status) => *status,
        }
    }

    fn location(&self) -> Option<impl AsRef<str>> {
        match self {
            Self::File(_, _) | Self::Index(_) | Self::Error(_) => None,
            Self::Redirect(uri) => Some(uri),
        }
    }

    fn media_type(&self) -> Option<impl AsRef<str>> {
        match self {
            Self::File(media_type, _) => *media_type,
            Self::Index(_) | Self::Redirect(_) | Self::Error(_) => Some("text/html; charset=utf-8"),
        }
    }

    fn body(&self) -> impl AsRef<[u8]> {
        match self {
            Self::File(_, contents) => Cow::from(contents),
            Self::Index(uri) => render_index(uri).into(),
            Self::Redirect(uri) => render_redirect(uri).into(),
            Self::Error(status) => render_error(*status).into(),
        }
    }
}

/// Text to be escaped for use in HTML.
struct HtmlText<'a>(&'a str);

impl Display for HtmlText<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for char in self.0.chars() {
            match char {
                '"' => f.write_str("&quot;"),
                '&' => f.write_str("&amp;"),
                '\'' => f.write_str("&#39;"),
                '<' => f.write_str("&lt;"),
                '>' => f.write_str("&gt;"),
                char => f.write_char(char),
            }?;
        }

        Ok(())
    }
}

/// Renders an index HTML document from a URI.
fn render_index(uri: &str) -> Vec<u8> {
    let title = format!("Index of {}", HtmlText(uri));
    let content = "<p><strong>TODO:</strong> Implement directory listing.</p>";
    render_html(&title, content)
}

/// Renders a redirect HTML document from an encoded URI.
fn render_redirect(uri: &str) -> Vec<u8> {
    let title = "Redirecting";

    let content = format!(
        "<p>You are being redirected to <a href=\"{uri}\">{}</a>.</p>",
        HtmlText(&http::decode_uri(uri))
    );

    render_html(title, &content)
}

/// Renders an error HTML document from an HTTP response [`Status`] code.
fn render_error(status: Status) -> Vec<u8> {
    let title = format!("{} - {}", status.code(), status.reason());
    let content = "<p>An error occurred.</p>";
    render_html(&title, content)
}

/// Renders an HTML document from a title and content.
fn render_html(title: &str, content: &str) -> Vec<u8> {
    static BASE: &str = include_str!("../res/base.html");
    BASE.replace("{{title}}", title)
        .replace("{{content}}", content)
        .into_bytes()
}
