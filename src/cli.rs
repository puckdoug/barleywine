use crate::log;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "barleywine",
    about = "A high-performance web platform that doesn't get in your way.",
    version = "0.1.0"
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

    /// Specify a different log file
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

        // Validate log file directory exists if specified
        if let Some(ref log_path) = self.log {
            if let Some(parent) = log_path.parent() {
                if !parent.exists() {
                    return Err(format!(
                        "Log file directory '{}' does not exist",
                        parent.display()
                    ));
                }
            }
        }

        Ok(())
    }

    /// Print configuration summary
    pub fn print_config(&self) {
        println!("Barleywine Configuration:");
        println!(
            "  Config file: {}",
            self.config
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "default".to_string())
        );
        println!("  Log level: {}", self.loglevel);
        println!(
            "  Log file: {}",
            self.log
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "stdout".to_string())
        );
        println!("  Verify only: {}", self.verify);
    }
}
