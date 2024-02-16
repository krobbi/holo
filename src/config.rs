use clap::{arg, command};

/// Configuration data for Holo.
pub struct Config {
    /// Whether to cross-origin isolation is enabled.
    cross_origin_isolation: bool,
}

impl Config {
    /// Create new configuration data using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(-i --coi "Enable cross-origin isolation"))
            .get_matches();

        let cross_origin_isolation = args.get_flag("coi");

        Config {
            cross_origin_isolation,
        }
    }

    /// Get whether cross origin isolation is enabled.
    pub fn cross_origin_isolation(&self) -> bool {
        self.cross_origin_isolation
    }
}
