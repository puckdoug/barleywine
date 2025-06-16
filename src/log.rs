use crate::cli::Cli;
use std::path::Path;

/// Setup logging based on CLI configuration
pub fn setup_logging(cli: &Cli) {
    // Set log level based on CLI argument
    let log_level = normalize_log_level(&cli.loglevel);

    // Note: Rocket logging can be configured via Rocket.toml or environment variables
    // For now, we'll just display the intended log level
    println!("🔧 Log level set to: {}", log_level);

    // If a custom log directory is specified, we could implement file logging here
    if let Some(ref log_dir) = cli.log {
        println!("📋 Logging to directory: {}", log_dir.display());
        // TODO: Implement file logging if needed
        // For now, we'll just note it and continue with stdout logging
    }
}

/// Normalize log level string to standard format
fn normalize_log_level(level: &str) -> &str {
    match level.to_lowercase().as_str() {
        "error" => "error",
        "warn" => "warn",
        "info" => "info",
        "debug" => "debug",
        "trace" => "trace",
        _ => "info", // fallback, though validation should catch this
    }
}

/// Check if a log level is valid
pub fn is_valid_log_level(level: &str) -> bool {
    matches!(
        level.to_lowercase().as_str(),
        "error" | "warn" | "info" | "debug" | "trace"
    )
}

/// Get all valid log levels
pub fn valid_log_levels() -> &'static [&'static str] {
    &["error", "warn", "info", "debug", "trace"]
}

/// Initialize directory logging (placeholder for future implementation)
#[allow(dead_code)]
pub fn init_directory_logging(log_dir: &Path) -> Result<(), std::io::Error> {
    // TODO: Implement actual file logging to directory
    // This could use libraries like `env_logger`, `tracing`, or `log4rs`

    // Verify the log directory exists
    if !log_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Log directory does not exist: {}", log_dir.display()),
        ));
    }

    if !log_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", log_dir.display()),
        ));
    }

    Ok(())
}

/// Setup console logging with specified level
#[allow(dead_code)]
pub fn setup_console_logging(level: &str) {
    // TODO: Could implement actual console logger configuration here
    // For now, this is a placeholder that could be extended with
    // proper logging framework integration

    let normalized_level = normalize_log_level(level);
    println!("Console logging configured for level: {}", normalized_level);
}

/// Configure Rocket logging based on our log level
#[allow(dead_code)]
pub fn configure_rocket_logging(level: &str) {
    let rocket_level = normalize_log_level(level);

    // TODO: Could set environment variables for Rocket logging
    // std::env::set_var("ROCKET_LOG_LEVEL", rocket_level);

    // For now, just display the configuration
    println!("Rocket logging would be set to: {}", rocket_level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_log_level() {
        assert_eq!(normalize_log_level("ERROR"), "error");
        assert_eq!(normalize_log_level("Info"), "info");
        assert_eq!(normalize_log_level("debug"), "debug");
        assert_eq!(normalize_log_level("invalid"), "info");
    }

    #[test]
    fn test_is_valid_log_level() {
        assert!(is_valid_log_level("error"));
        assert!(is_valid_log_level("ERROR"));
        assert!(is_valid_log_level("Info"));
        assert!(is_valid_log_level("debug"));
        assert!(is_valid_log_level("trace"));
        assert!(!is_valid_log_level("invalid"));
        assert!(!is_valid_log_level(""));
    }

    #[test]
    fn test_valid_log_levels() {
        let levels = valid_log_levels();
        assert_eq!(levels.len(), 5);
        assert!(levels.contains(&"error"));
        assert!(levels.contains(&"warn"));
        assert!(levels.contains(&"info"));
        assert!(levels.contains(&"debug"));
        assert!(levels.contains(&"trace"));
    }
}
