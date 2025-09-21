use crate::http::Respond;

/// A page that can be sent as an HTTP response.
pub enum Page {
    /// A test page.
    Test,
}

impl Respond for Page {
    fn media_type(&self) -> Option<impl AsRef<str>> {
        match self {
            Self::Test => Some("text/html; charset=utf-8"),
        }
    }

    fn body(&self) -> impl AsRef<[u8]> {
        match self {
            Self::Test => "<h1>Hello from Page::Test!</h1>\n",
        }
    }
}
