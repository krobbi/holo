use clap::Parser;

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

    /// Returns the desired TCP port.
    pub fn port(&self) -> u16 {
        self.args.port
    }
}

/// Command line arguments.
#[derive(Parser)]
#[command(bin_name("holo"), version, about)]
struct Args {
    /// The desired TCP port.
    #[arg(help = "TCP port", short, long, default_value_t = 8080)]
    port: u16,
}
