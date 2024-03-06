use std::path::{Path, PathBuf};

use clap::{arg, command, error::ErrorKind, value_parser, ValueHint};

/// Configuration data for Holo.
pub struct Config {
    /// The server root directory.
    root: PathBuf,

    /// The TCP port.
    port: u16,

    /// Whether cross-origin isolation is enabled.
    cross_origin_isolation: bool,
}

impl Config {
    /// Create new configuration data using command line arguments.
    pub fn new() -> Config {
        let mut cmd = command!()
            .arg(
                arg!([root] "Server root directory")
                    .value_parser(value_parser!(PathBuf))
                    .value_hint(ValueHint::DirPath)
                    .default_value("."),
            )
            .arg(
                arg!(-p --port <port> "TCP port")
                    .value_parser(value_parser!(u16))
                    .default_value("8080"),
            )
            .arg(arg!(-i --coi "Enable cross-origin isolation"));

        let matches = cmd.get_matches_mut();

        let Ok(root) = matches.get_one::<PathBuf>("root").unwrap().canonicalize() else {
            cmd.error(ErrorKind::ValueValidation, "could not find server root")
                .exit();
        };

        if !root.is_dir() {
            cmd.error(ErrorKind::ValueValidation, "server root is not a directory")
                .exit();
        }

        let port = matches.get_one::<u16>("port").unwrap().to_owned();
        let cross_origin_isolation = matches.get_flag("coi");

        Config {
            root,
            port,
            cross_origin_isolation,
        }
    }

    /// Get the server root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get the TCP port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get whether cross origin isolation is enabled.
    pub fn cross_origin_isolation(&self) -> bool {
        self.cross_origin_isolation
    }
}
