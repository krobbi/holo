use clap::{value_parser, Arg, ArgAction, Command};

/// Configuration data for Holo.
pub struct Config {
    /// The TCP port.
    port: u16,

    /// Whether cross-origin isolation is enabled.
    cross_origin_isolation: bool,
}

impl Config {
    /// Create new configuration data using command line arguments.
    pub fn new() -> Config {
        let args = Command::new("holo")
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .value_parser(value_parser!(u16))
                    .default_value("8080")
                    .help("TCP port"),
            )
            .arg(
                Arg::new("cross_origin_isolation")
                    .short('i')
                    .long("coi")
                    .action(ArgAction::SetTrue)
                    .help("Enable cross-origin isolation"),
            )
            .get_matches();

        let port = args.get_one::<u16>("port").unwrap().to_owned();
        let cross_origin_isolation = args.get_flag("cross_origin_isolation");

        Config {
            port,
            cross_origin_isolation,
        }
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
