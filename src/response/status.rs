/// An HTTP response status.
pub enum Status {
    /// The request succeeded.
    Ok,

    /// The URL of the requested resource has been changed permanently.
    MovedPermanently,

    /// The client does not have access rights to the content.
    Forbidden,

    /// The server cannot find the requested resource.
    NotFound,

    /// The server has encountered a situation it does not know how to handle.
    InternalServerError,
}

impl Status {
    /// Get the status code.
    pub(super) fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::MovedPermanently => 301,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::InternalServerError => 500,
        }
    }

    /// Get the reason phrase.
    pub(super) fn reason(&self) -> &'static str {
        match self {
            Status::Ok => "OK",
            Status::MovedPermanently => "Moved Permanently",
            Status::Forbidden => "Forbidden",
            Status::NotFound => "Not Found",
            Status::InternalServerError => "Internal Server Error",
        }
    }
}
