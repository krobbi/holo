/// An HTTP response status.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Status {
    /// The request succeeded.
    Ok = 200,

    /// The URL of the requested resource has been changed permanently.
    MovedPermanently = 301,

    /// The client does not have access rights to the content.
    Forbidden = 403,

    /// The server cannot find the requested resource.
    NotFound = 404,

    /// The server has encountered a situation it does not know how to handle.
    InternalServerError = 500,
}

impl Status {
    /// Get the status code.
    pub(super) fn code(self) -> u16 {
        self as u16
    }

    /// Get the reason phrase.
    pub(super) fn reason(self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::MovedPermanently => "Moved Permanently",
            Status::Forbidden => "Forbidden",
            Status::NotFound => "Not Found",
            Status::InternalServerError => "Internal Server Error",
        }
    }
}
