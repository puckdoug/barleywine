use crate::log;
use ::log::info;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Cli {
    /// Specify a configuration file
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Verify the configuration without running the server
    #[structopt(long)]
    pub verify: bool,

    /// Set the log level (error, warn, info, debug, trace)
    #[structopt(long, default_value = "info")]
    pub loglevel: String,

    /// Specify a log directory
    #[structopt(long, parse(from_os_str))]
    pub log: Option<PathBuf>,
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Cli::from_args()
    }

    /// Validate the parsed arguments
    pub fn validate(&self) -> Result<(), String> {
        // Validate log level
        if !log::is_valid_log_level(&self.loglevel) {
            return Err(format!(
                "Invalid log level '{}'. Valid levels are: {}",
                self.loglevel,
                log::valid_log_levels().join(", ")
            ));
        }

        // Validate config file exists if specified
        if let Some(ref config_path) = self.config {
            if !config_path.exists() {
                return Err(format!(
                    "Configuration file '{}' does not exist",
                    config_path.display()
                ));
            }
        }

        // Validate log directory exists if specified
        if let Some(ref log_dir) = self.log {
            if !log_dir.exists() {
                return Err(format!(
                    "Cannot run without a log directory. The directory '{}' does not exist",
                    log_dir.display()
                ));
            }
            if !log_dir.is_dir() {
                return Err(format!(
                    "Cannot run without a log directory. The path '{}' is not a directory",
                    log_dir.display()
                ));
            }
        }

        info!("CLI arguments validated successfully");
        Ok(())
    }
}
