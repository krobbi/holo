use crate::{
    http::{Request, Status},
    page::Page,
};

/// Finds a [`Page`] to return as a response to an HTTP [`Request`].
pub fn find_page(request: &Request) -> Page {
    if request.is_local() {
        Page::Test(request.uri().into())
    } else {
        Page::Error(Status::Forbidden)
    }
}
