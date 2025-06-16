use crate::cli::Cli;
use chrono::Utc;
use log::{error, info, warn};
use simplelog::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Global logger instances for different log types
static BARLEYWINE_LOGGER: Mutex<Option<std::fs::File>> = Mutex::new(None);
static ACCESS_LOGGER: Mutex<Option<std::fs::File>> = Mutex::new(None);

/// Setup logging based on CLI configuration
pub fn setup_logging(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Determine log directory - use CLI option or default to "logs"
    let log_dir = cli.log.clone().unwrap_or_else(|| PathBuf::from("logs"));

    // Create log directory if it doesn't exist
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)?;
        println!("📁 Created log directory: {}", log_dir.display());
    }

    // Set up the main application logger
    let log_level = parse_log_level(&cli.loglevel);
    setup_main_logger(&log_dir, log_level)?;

    // Initialize file loggers
    init_file_loggers(&log_dir)?;

    info!(
        "🔧 Logging initialized - Level: {}, Directory: {}",
        cli.loglevel,
        log_dir.display()
    );
    log_barleywine(&format!(
        "Barleywine logging started at {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    Ok(())
}

/// Initialize the main application logger using simplelog
fn setup_main_logger(log_dir: &Path, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = log_dir.join("barleywine.log");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_file)?;

    CombinedLogger::init(vec![
        TermLogger::new(
            level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(level, Config::default(), file),
    ])?;

    Ok(())
}

/// Initialize separate file loggers for barleywine and access logs
fn init_file_loggers(log_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize barleywine.log file handle
    let barleywine_log = log_dir.join("barleywine.log");
    let barleywine_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&barleywine_log)?;

    // Initialize access.log file handle
    let access_log = log_dir.join("access.log");
    let access_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&access_log)?;

    // Store file handles in global mutexes
    *BARLEYWINE_LOGGER.lock().unwrap() = Some(barleywine_file);
    *ACCESS_LOGGER.lock().unwrap() = Some(access_file);

    println!("📋 Log files initialized:");
    println!("  - General log: {}", barleywine_log.display());
    println!("  - Access log: {}", access_log.display());

    Ok(())
}

/// Log a message to the barleywine.log file
pub fn log_barleywine(message: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}\n", timestamp, message);

    if let Ok(mut logger) = BARLEYWINE_LOGGER.lock() {
        if let Some(ref mut file) = *logger {
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write to barleywine.log: {}", e);
            } else {
                let _ = file.flush();
            }
        }
    }
}

/// Log an access entry to the access.log file
pub fn log_access(
    remote_addr: &str,
    method: &str,
    uri: &str,
    status: u16,
    user_agent: Option<&str>,
) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let user_agent_str = user_agent.unwrap_or("-");
    let log_entry = format!(
        "[{}] {} \"{}\" {} \"{}\" \"{}\"\n",
        timestamp, remote_addr, method, uri, status, user_agent_str
    );

    if let Ok(mut logger) = ACCESS_LOGGER.lock() {
        if let Some(ref mut file) = *logger {
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write to access.log: {}", e);
            } else {
                let _ = file.flush();
            }
        }
    }
}

/// Log server startup information
pub fn log_server_startup(port: u16, webroot: &str) {
    let message = format!(
        "Server started on port {} serving files from {}",
        port, webroot
    );
    info!("{}", message);
    log_barleywine(&message);
}

/// Log server shutdown information
pub fn log_server_shutdown() {
    let message = "Server shutting down";
    info!("{}", message);
    log_barleywine(&message);
}

/// Log file serving information
pub fn log_file_served(path: &str, file_type: &str) {
    let message = format!("Served {} file: {}", file_type, path);
    info!("{}", message);
    log_barleywine(&message);
}

/// Log errors to both console and barleywine.log
pub fn log_error(error_msg: &str) {
    error!("{}", error_msg);
    log_barleywine(&format!("ERROR: {}", error_msg));
}

/// Log warnings to both console and barleywine.log
pub fn log_warning(warning_msg: &str) {
    warn!("{}", warning_msg);
    log_barleywine(&format!("WARNING: {}", warning_msg));
}

/// Parse log level string to LevelFilter
fn parse_log_level(level: &str) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
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

/// Initialize directory logging (now implemented)
pub fn init_directory_logging(log_dir: &Path) -> Result<(), std::io::Error> {
    // Verify the log directory exists or create it
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
    }

    if !log_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a directory: {}", log_dir.display()),
        ));
    }

    Ok(())
}

/// Flush all log files
pub fn flush_logs() {
    if let Ok(mut logger) = BARLEYWINE_LOGGER.lock() {
        if let Some(ref mut file) = *logger {
            let _ = file.flush();
        }
    }

    if let Ok(mut logger) = ACCESS_LOGGER.lock() {
        if let Some(ref mut file) = *logger {
            let _ = file.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_log_level() {
        assert_eq!(parse_log_level("error"), LevelFilter::Error);
        assert_eq!(parse_log_level("ERROR"), LevelFilter::Error);
        assert_eq!(parse_log_level("info"), LevelFilter::Info);
        assert_eq!(parse_log_level("Debug"), LevelFilter::Debug);
        assert_eq!(parse_log_level("invalid"), LevelFilter::Info);
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

    #[test]
    fn test_init_directory_logging() {
        let temp_dir = std::env::temp_dir().join("barleywine_test_logs");

        // Clean up if it exists
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }

        // Test creating directory
        assert!(init_directory_logging(&temp_dir).is_ok());
        assert!(temp_dir.exists());
        assert!(temp_dir.is_dir());

        // Test with existing directory
        assert!(init_directory_logging(&temp_dir).is_ok());

        // Clean up
        fs::remove_dir_all(&temp_dir).ok();
    }
}
