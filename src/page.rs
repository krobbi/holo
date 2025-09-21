use crate::http::{Respond, Status};

/// A page that can be sent as an HTTP response.
pub enum Page {
    /// A test page.
    Test,
}

impl Respond for Page {
    fn status(&self) -> Status {
        match self {
            Self::Test => Status::Ok,
        }
    }
}
