use std::{
    error,
    fmt::{self, Display, Formatter},
    io::{self, Write},
    process::ExitCode,
    result,
};

/// A specialized [`Result`](result::Result) type for Holo.
pub type Result<T> = result::Result<T, Error>;

/// An error caught by Holo.
#[derive(Debug)]
pub enum Error {
    /// A command line argument parsing error or a help or version message.
    Command(clap::Error),

    /// An error caused by failing to open a [`Server`](crate::http::Server).
    ServerOpen(io::Error),

    /// An error caused by failing to query a [`Server`](crate::http::Server)'s
    /// TCP/IP address.
    ServerAddressQuery(io::Error),

    /// An error caused by failing to establish a connection with a client.
    Connect(io::Error),

    /// An error caused by failing to read a request.
    RequestRead(io::Error),

    /// An error caused by a request not being an HTTP GET request.
    RequestNotHttpGet,

    /// An error caused by failing to send an HTTP response.
    ResponseSend(io::Error),
}

impl Error {
    /// Prints the `Error`. Any I/O errors encountered while printing will be
    /// ignored.
    pub fn print(&self) {
        let _ = match self {
            Self::Command(error) => error.print(),
            error => writeln!(io::stderr(), "error: {error}"),
        };
    }

    /// Returns the [`ExitCode`] associated with the `Error`.
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Command(error) => error.exit_code().try_into().unwrap_or(1).into(),
            _ => ExitCode::FAILURE,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Command(error) => Some(error),
            Self::ServerOpen(error)
            | Self::ServerAddressQuery(error)
            | Self::Connect(error)
            | Self::RequestRead(error)
            | Self::ResponseSend(error) => Some(error),
            Self::RequestNotHttpGet => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command(error) => error.fmt(f),
            Self::ServerOpen(error) => write!(f, "failed to open server: {error}"),
            Self::ServerAddressQuery(error) => write!(f, "failed to query server address: {error}"),
            Self::Connect(error) => write!(f, "failed to connect: {error}"),
            Self::RequestRead(error) => write!(f, "failed to read request: {error}"),
            Self::RequestNotHttpGet => f.write_str("request is not an HTTP GET request"),
            Self::ResponseSend(error) => write!(f, "failed to send response: {error}"),
        }
    }
}
