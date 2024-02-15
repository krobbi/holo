use std::{
    error,
    fmt::{self, Display, Formatter},
    io, result,
};

/// A result type for Holo.
pub type Result<T> = result::Result<T, Error>;

/// An error caught by Holo.
#[derive(Debug)]
pub enum Error {
    /// An IO error occurred.
    Io(io::Error),

    /// TCP stream is not an HTTP request.
    StreamNotHttpRequest,

    /// HTTP request path is not UTF-8.
    RequestPathNotUtf8,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Io(error) => error.fmt(f),
            Error::StreamNotHttpRequest => write!(f, "Connection is not HTTP."),
            Error::RequestPathNotUtf8 => write!(f, "Requested path is not UTF-8."),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
