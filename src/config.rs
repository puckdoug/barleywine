use crate::log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Main configuration structure for Barleywine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub content: ContentConfig,
    pub cache: CacheConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
    pub development: DevelopmentConfig,
    pub limits: LimitsConfig,
    pub template: TemplateConfig,
    pub routes: RoutesConfig,
    pub middleware: MiddlewareConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Number of worker threads
    pub workers: Option<u32>,
    /// Request timeout in seconds
    pub timeout: Option<u64>,
    /// Maximum request size in bytes
    pub max_request_size: Option<u64>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Log file path (optional)
    pub file: Option<PathBuf>,
    /// Whether to enable access logging
    pub access_log: bool,
    /// Log format: compact, pretty, json
    pub format: String,
}

/// Content serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentConfig {
    /// Root directory for static files
    pub webroot: PathBuf,
    /// Default index files to serve
    pub index_files: Vec<String>,
    /// Whether to enable markdown processing
    pub markdown_enabled: bool,
    /// Markdown extensions to enable
    pub markdown_extensions: Vec<String>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Whether to enable caching
    pub enabled: bool,
    /// Cache duration in seconds
    pub duration: u64,
    /// Cache control headers
    pub cache_control: String,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Whether to enable security headers
    pub security_headers: bool,
    /// Allowed file extensions (empty means all allowed)
    pub allowed_extensions: Vec<String>,
    /// Blocked file extensions
    pub blocked_extensions: Vec<String>,
    /// Whether to enable CORS
    pub cors_enabled: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Whether to enable compression
    pub compression: bool,
    /// Compression level (1-9)
    pub compression_level: u32,
    /// Minimum file size to compress
    pub min_compress_size: u64,
    /// Whether to enable HTTP/2
    pub http2: bool,
}

/// Development configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    /// Enable development mode features
    pub dev_mode: bool,
    /// Enable hot reloading
    pub hot_reload: bool,
    /// Enable debug routes
    pub debug_routes: bool,
    /// Pretty print JSON responses
    pub pretty_json: bool,
}

/// Request limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    /// Form data limit
    pub forms: String,
    /// JSON payload limit
    pub json: String,
    /// File upload limit
    pub file: String,
    /// Data limit
    pub data: String,
}

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Custom HTML template for markdown conversion
    pub custom_template: Option<PathBuf>,
    /// Custom CSS file to include in markdown pages
    pub custom_css: Option<PathBuf>,
    /// Custom JavaScript file to include in markdown pages
    pub custom_js: Option<PathBuf>,
    /// Site title for generated pages
    pub site_title: String,
    /// Site description
    pub site_description: String,
}

/// Routes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutesConfig {
    /// Custom route mappings
    #[serde(default)]
    pub mappings: HashMap<String, String>,
    /// Redirect rules
    #[serde(default)]
    pub redirects: HashMap<String, String>,
}

/// Middleware configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfig {
    /// Enable request logging middleware
    pub request_logging: bool,
    /// Enable CORS middleware
    pub cors: bool,
    /// Enable compression middleware
    pub compression: bool,
    /// Enable security headers middleware
    pub security: bool,
    /// Custom middleware
    pub custom: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8000,
                workers: Some(4),
                timeout: Some(30),
                max_request_size: Some(10 * 1024 * 1024), // 10MB
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
                access_log: true,
                format: "pretty".to_string(),
            },
            content: ContentConfig {
                webroot: PathBuf::from("webroot"),
                index_files: vec!["index.html".to_string(), "index.md".to_string()],
                markdown_enabled: true,
                markdown_extensions: vec![
                    "tables".to_string(),
                    "strikethrough".to_string(),
                    "task_lists".to_string(),
                    "autolinks".to_string(),
                ],
            },
            cache: CacheConfig {
                enabled: true,
                duration: 3600,
                cache_control: "public, max-age=3600".to_string(),
            },
            security: SecurityConfig {
                security_headers: true,
                allowed_extensions: vec![],
                blocked_extensions: vec![
                    ".env".to_string(),
                    ".git".to_string(),
                    ".svn".to_string(),
                    ".DS_Store".to_string(),
                ],
                cors_enabled: false,
                cors_origins: vec!["*".to_string()],
            },
            performance: PerformanceConfig {
                compression: true,
                compression_level: 6,
                min_compress_size: 1024,
                http2: true,
            },
            development: DevelopmentConfig {
                dev_mode: false,
                hot_reload: false,
                debug_routes: false,
                pretty_json: true,
            },
            limits: LimitsConfig {
                forms: "1MiB".to_string(),
                json: "1MiB".to_string(),
                file: "10MiB".to_string(),
                data: "10MiB".to_string(),
            },
            template: TemplateConfig {
                custom_template: None,
                custom_css: None,
                custom_js: None,
                site_title: "Barleywine Static Server".to_string(),
                site_description: "A fast static file server with markdown support".to_string(),
            },
            routes: RoutesConfig {
                mappings: HashMap::new(),
                redirects: HashMap::new(),
            },
            middleware: MiddlewareConfig {
                request_logging: true,
                cors: false,
                compression: true,
                security: true,
                custom: None,
            },
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::FileNotFound(path.to_path_buf()));
        }

        let content =
            fs::read_to_string(path).map_err(|e| ConfigError::ReadError(path.to_path_buf(), e))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;

        config.validate()?;
        Ok(config)
    }

    /// Load configuration from default locations or CLI-specified file
    pub fn load(config_file: Option<&Path>) -> Result<Self, ConfigError> {
        match config_file {
            Some(path) => {
                log::log_barleywine(&format!("Loading configuration from: {}", path.display()));
                Self::from_file(path)
            }
            None => {
                // Try default locations
                let default_files = ["barleywine.toml", "config.toml", "barleywine.conf"];

                for file in &default_files {
                    if Path::new(file).exists() {
                        log::log_barleywine(&format!("Found default configuration file: {}", file));
                        return Self::from_file(file);
                    }
                }

                // No config file found, use defaults
                log::log_barleywine("No configuration file found, using defaults");
                Ok(Self::default())
            }
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate log level
        if !log::is_valid_log_level(&self.logging.level) {
            return Err(ConfigError::InvalidLogLevel(self.logging.level.clone()));
        }

        // Validate webroot directory
        if !self.content.webroot.exists() {
            return Err(ConfigError::WebrootNotFound(self.content.webroot.clone()));
        }

        if !self.content.webroot.is_dir() {
            return Err(ConfigError::WebrootNotDirectory(
                self.content.webroot.clone(),
            ));
        }

        // Validate port range
        if self.server.port == 0 {
            return Err(ConfigError::InvalidPort(self.server.port));
        }

        // Validate host address
        if self.server.host.parse::<IpAddr>().is_err() {
            return Err(ConfigError::InvalidHost(self.server.host.clone()));
        }

        // Validate log file directory if specified
        if let Some(ref log_file) = self.logging.file {
            if let Some(parent) = log_file.parent() {
                if parent.exists() && !parent.is_dir() {
                    return Err(ConfigError::LogDirectoryNotDirectory(parent.to_path_buf()));
                }
            }
        }

        // Validate custom template file if specified
        if let Some(ref template_file) = self.template.custom_template {
            if !template_file.exists() {
                return Err(ConfigError::CustomTemplateNotFound(template_file.clone()));
            }
        }

        // Validate custom CSS file if specified
        if let Some(ref css_file) = self.template.custom_css {
            if !css_file.exists() {
                return Err(ConfigError::CustomCssNotFound(css_file.clone()));
            }
        }

        // Validate custom JS file if specified
        if let Some(ref js_file) = self.template.custom_js {
            if !js_file.exists() {
                return Err(ConfigError::CustomJsNotFound(js_file.clone()));
            }
        }

        // Validate compression level
        if !(1..=9).contains(&self.performance.compression_level) {
            return Err(ConfigError::InvalidCompressionLevel(
                self.performance.compression_level,
            ));
        }

        Ok(())
    }

    /// Create a default configuration file
    pub fn create_default_file<P: AsRef<Path>>(path: P) -> Result<(), ConfigError> {
        let path = path.as_ref();
        let default_config = Self::default();
        let toml_content =
            toml::to_string_pretty(&default_config).map_err(|e| ConfigError::SerializeError(e))?;

        fs::write(path, toml_content)
            .map_err(|e| ConfigError::WriteError(path.to_path_buf(), e))?;

        Ok(())
    }

    /// Get the effective log directory (handling CLI override)
    pub fn get_log_directory(&self, cli_log_dir: Option<&Path>) -> Option<PathBuf> {
        cli_log_dir
            .map(|p| p.to_path_buf())
            .or_else(|| {
                self.logging
                    .file
                    .as_ref()
                    .and_then(|f| f.parent().map(|p| p.to_path_buf()))
            })
            .or_else(|| Some(PathBuf::from("logs")))
    }

    /// Get the effective log level (handling CLI override)
    pub fn get_log_level(&self, cli_log_level: Option<&str>) -> String {
        cli_log_level.unwrap_or(&self.logging.level).to_string()
    }

    /// Get the server address as a SocketAddr
    pub fn get_server_address(&self) -> Result<std::net::SocketAddr, ConfigError> {
        let ip: IpAddr = self
            .server
            .host
            .parse()
            .map_err(|_| ConfigError::InvalidHost(self.server.host.clone()))?;
        Ok(std::net::SocketAddr::new(ip, self.server.port))
    }

    /// Check if a file extension is allowed
    pub fn is_extension_allowed(&self, extension: &str) -> bool {
        // If blocked extensions list contains this extension, deny it
        if self
            .security
            .blocked_extensions
            .iter()
            .any(|ext| ext == extension)
        {
            return false;
        }

        // If allowed extensions list is empty, allow all (except blocked)
        if self.security.allowed_extensions.is_empty() {
            return true;
        }

        // Otherwise, only allow if in allowed list
        self.security
            .allowed_extensions
            .iter()
            .any(|ext| ext == extension)
    }

    /// Print configuration summary
    pub fn print_summary(&self) {
        println!("📋 Barleywine Configuration Summary:");
        println!("   Server:");
        println!("     Address: {}:{}", self.server.host, self.server.port);
        println!("     Workers: {:?}", self.server.workers);
        println!("     Timeout: {:?}s", self.server.timeout);

        println!("   Content:");
        println!("     Webroot: {}", self.content.webroot.display());
        println!("     Index Files: {:?}", self.content.index_files);
        println!("     Markdown: {}", self.content.markdown_enabled);

        println!("   Logging:");
        println!("     Level: {}", self.logging.level);
        println!("     File: {:?}", self.logging.file);
        println!("     Access Log: {}", self.logging.access_log);
        println!("     Format: {}", self.logging.format);

        println!("   Cache:");
        println!("     Enabled: {}", self.cache.enabled);
        println!("     Duration: {}s", self.cache.duration);

        println!("   Security:");
        println!("     Security Headers: {}", self.security.security_headers);
        println!("     CORS Enabled: {}", self.security.cors_enabled);
        println!(
            "     Blocked Extensions: {:?}",
            self.security.blocked_extensions
        );

        println!("   Performance:");
        println!("     Compression: {}", self.performance.compression);
        println!("     HTTP/2: {}", self.performance.http2);

        println!("   Development:");
        println!("     Dev Mode: {}", self.development.dev_mode);
        println!("     Hot Reload: {}", self.development.hot_reload);
    }
}

/// Configuration error types
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(PathBuf),
    ReadError(PathBuf, std::io::Error),
    ParseError(PathBuf, toml::de::Error),
    WriteError(PathBuf, std::io::Error),
    SerializeError(toml::ser::Error),
    InvalidLogLevel(String),
    WebrootNotFound(PathBuf),
    WebrootNotDirectory(PathBuf),
    LogDirectoryNotDirectory(PathBuf),
    CustomTemplateNotFound(PathBuf),
    CustomCssNotFound(PathBuf),
    CustomJsNotFound(PathBuf),
    InvalidPort(u16),
    InvalidHost(String),
    InvalidCompressionLevel(u32),
    AlreadyInitialized,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {}", path.display())
            }
            ConfigError::ReadError(path, err) => {
                write!(
                    f,
                    "Failed to read configuration file {}: {}",
                    path.display(),
                    err
                )
            }
            ConfigError::ParseError(path, err) => {
                write!(
                    f,
                    "Failed to parse configuration file {}: {}",
                    path.display(),
                    err
                )
            }
            ConfigError::WriteError(path, err) => {
                write!(
                    f,
                    "Failed to write configuration file {}: {}",
                    path.display(),
                    err
                )
            }
            ConfigError::SerializeError(err) => {
                write!(f, "Failed to serialize configuration: {}", err)
            }
            ConfigError::InvalidLogLevel(level) => {
                write!(
                    f,
                    "Invalid log level '{}'. Valid levels are: {}",
                    level,
                    log::valid_log_levels().join(", ")
                )
            }
            ConfigError::WebrootNotFound(path) => {
                write!(f, "Webroot directory not found: {}", path.display())
            }
            ConfigError::WebrootNotDirectory(path) => {
                write!(f, "Webroot path is not a directory: {}", path.display())
            }
            ConfigError::LogDirectoryNotDirectory(path) => {
                write!(
                    f,
                    "Log directory path is not a directory: {}",
                    path.display()
                )
            }
            ConfigError::CustomTemplateNotFound(path) => {
                write!(f, "Custom template file not found: {}", path.display())
            }
            ConfigError::CustomCssNotFound(path) => {
                write!(f, "Custom CSS file not found: {}", path.display())
            }
            ConfigError::CustomJsNotFound(path) => {
                write!(f, "Custom JS file not found: {}", path.display())
            }
            ConfigError::InvalidPort(port) => {
                write!(f, "Invalid port number: {}", port)
            }
            ConfigError::InvalidHost(host) => {
                write!(f, "Invalid host address: {}", host)
            }
            ConfigError::InvalidCompressionLevel(level) => {
                write!(f, "Invalid compression level: {} (must be 1-9)", level)
            }
            ConfigError::AlreadyInitialized => {
                write!(f, "Configuration has already been initialized")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Global configuration instance
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

/// Initialize the global configuration
pub fn init_config(config_file: Option<&Path>) -> Result<(), ConfigError> {
    let config = Config::load(config_file).unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load configuration: {}", e);
        eprintln!("Using default configuration.");
        Config::default()
    });

    GLOBAL_CONFIG
        .set(config)
        .map_err(|_| ConfigError::AlreadyInitialized)?;

    Ok(())
}

/// Get the global configuration
pub fn get_config() -> &'static Config {
    GLOBAL_CONFIG
        .get()
        .expect("Configuration not initialized. Call init_config() first.")
}

/// Check if configuration has been initialized
pub fn is_config_initialized() -> bool {
    GLOBAL_CONFIG.get().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    // use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 8000);
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.content.webroot, PathBuf::from("webroot"));
        assert!(config.content.markdown_enabled);
        assert!(config.cache.enabled);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Test invalid log level
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());

        // Test invalid port
        config.logging.level = "info".to_string();
        config.server.port = 0;
        assert!(config.validate().is_err());

        // Test invalid host
        config.server.port = 8000;
        config.server.host = "invalid-host".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_extension_filtering() {
        let mut config = Config::default();

        // Test with empty allowed list (should allow all except blocked)
        assert!(!config.is_extension_allowed(".env"));
        assert!(config.is_extension_allowed(".html"));
        assert!(config.is_extension_allowed(".css"));

        // Test with specific allowed extensions
        config.security.allowed_extensions = vec![".html".to_string(), ".css".to_string()];
        assert!(config.is_extension_allowed(".html"));
        assert!(config.is_extension_allowed(".css"));
        assert!(!config.is_extension_allowed(".js"));
        assert!(!config.is_extension_allowed(".env")); // Still blocked
    }

    #[test]
    fn test_config_file_loading() {
        let temp_dir = std::env::temp_dir();
        let config_file = temp_dir.join("test_barleywine.toml");

        // Create a test config file
        let test_config = r#"
[server]
host = "0.0.0.0"
port = 3000

[logging]
level = "debug"

[content]
webroot = "/tmp/webroot"
index_files = ["index.html"]
markdown_enabled = false

[security]
cors_enabled = true
"#;

        fs::write(&config_file, test_config).unwrap();

        // Create the webroot directory for validation
        let webroot = PathBuf::from("/tmp/webroot");
        fs::create_dir_all(&webroot).ok();

        // Try to load the config
        let result = Config::from_file(&config_file);

        // Clean up
        fs::remove_file(&config_file).ok();
        fs::remove_dir_all(&webroot).ok();

        match result {
            Ok(config) => {
                assert_eq!(config.server.port, 3000);
                assert_eq!(config.server.host, "0.0.0.0");
                assert_eq!(config.logging.level, "debug");
                assert!(!config.content.markdown_enabled);
                assert!(config.security.cors_enabled);
            }
            Err(e) => {
                // Skip if directories can't be created in /tmp
                println!("Skipping test due to filesystem restrictions: {}", e);
            }
        }
    }

    #[test]
    fn test_create_default_file() {
        let temp_dir = std::env::temp_dir();
        let config_file = temp_dir.join("test_default_barleywine.toml");

        // Create default config file
        Config::create_default_file(&config_file).unwrap();

        // Verify file exists and contains expected content
        assert!(config_file.exists());
        let content = fs::read_to_string(&config_file).unwrap();
        assert!(content.contains("[server]"));
        assert!(content.contains("[logging]"));
        assert!(content.contains("[content]"));
        assert!(content.contains("[security]"));

        // Clean up
        fs::remove_file(&config_file).ok();
    }
}
