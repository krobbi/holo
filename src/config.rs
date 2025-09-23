use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueHint};

use crate::error::{Error, Result};

/// Configuration data for Holo.
pub struct Config {
    /// The command line arguments providing configuration data.
    args: Args,
}

impl Config {
    /// Creates new configuration data from command line arguments.
    pub fn try_new() -> Result<Self> {
        let args = Args::try_parse().map_err(Error::Command)?;
        Ok(Self { args })
    }

    /// Returns the canonical [`Path`] to the root directory for serving files.
    pub fn root(&self) -> &Path {
        &self.args.root
    }

    /// Returns the desired TCP port.
    pub fn port(&self) -> u16 {
        self.args.port
    }
}

/// Command line arguments.
#[derive(Parser)]
#[command(bin_name("holo"), version, about)]
struct Args {
    /// The canonical path to the root directory for serving files.
    #[arg(
        value_hint(ValueHint::DirPath),
        value_parser = parse_root,
        help = "Server root directory",
        default_value = "."
    )]
    root: PathBuf,

    /// The desired TCP port.
    #[arg(help = "TCP port", short, long, default_value_t = 8080)]
    port: u16,
}

/// Parses a canonical path to a root directory.
fn parse_root(root: &str) -> Result<PathBuf> {
    let root = fs::canonicalize(root).map_err(Error::RootNotExist)?;

    if !root.is_dir() {
        return Err(Error::RootNotDirectory);
    }

    Ok(root)
}
