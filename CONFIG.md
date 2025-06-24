# Barleywine Configuration System

The Barleywine static file server uses a comprehensive configuration system that allows you to customize every aspect of the server's behavior. Configuration is handled through TOML files with sensible defaults and CLI override options.

## Configuration Files

### Default Configuration Files
Barleywine automatically looks for configuration files in the following order:
1. `barleywine.toml` (primary default)
2. `config.toml` (secondary default)
3. `barleywine.conf` (tertiary default)

If none of these files exist, Barleywine uses built-in default values.

### Custom Configuration Files
You can specify a custom configuration file using the `--config` CLI option:
```bash
cargo run -- --config /path/to/custom-config.toml
```

## Configuration Structure

The configuration file is organized into sections that control different aspects of the server:

### [server] - Server Configuration
Controls the basic server settings:

```toml
[server]
host = "127.0.0.1"           # IP address to bind to
port = 8000                  # Port number to listen on
workers = 4                  # Number of worker threads
timeout = 30                 # Request timeout in seconds
max_request_size = 10485760  # Maximum request size in bytes (10MB)
```

**Options:**
- `host`: IP address to bind to (default: "127.0.0.1")
- `port`: Port number (default: 8000)
- `workers`: Number of worker threads (default: 4)
- `timeout`: Request timeout in seconds (default: 30)
- `max_request_size`: Maximum request size in bytes (default: 10MB)

### [logging] - Logging Configuration
Controls how and where logs are written:

```toml
[logging]
level = "info"                    # Log level
file = "logs/barleywine.log"     # Log file path (optional)
access_log = true                # Enable access logging
format = "pretty"                # Log format: compact, pretty, json
```

**Options:**
- `level`: Log level - "error", "warn", "info", "debug", "trace" (default: "info")
- `file`: Optional log file path. If not specified, logs go to stdout
- `access_log`: Enable HTTP access logging (default: true)
- `format`: Log format - "compact", "pretty", "json" (default: "pretty")

### [content] - Content Serving Configuration
Controls how static files and content are served:

```toml
[content]
webroot = "webroot"                              # Root directory for static files
index_files = ["index.html", "index.md"]        # Default index files
markdown_enabled = true                          # Enable markdown processing
markdown_extensions = ["tables", "strikethrough", "task_lists", "autolinks"]
```

**Options:**
- `webroot`: Root directory for static files (default: "webroot")
- `index_files`: List of index files to look for in directories (default: ["index.html", "index.md"])
- `markdown_enabled`: Enable automatic markdown to HTML conversion (default: true)
- `markdown_extensions`: List of markdown extensions to enable

### [cache] - Caching Configuration
Controls HTTP caching behavior:

```toml
[cache]
enabled = true                        # Enable caching
duration = 3600                       # Cache duration in seconds
cache_control = "public, max-age=3600" # Cache control header
```

**Options:**
- `enabled`: Enable HTTP caching (default: true)
- `duration`: Cache duration in seconds (default: 3600 = 1 hour)
- `cache_control`: Cache-Control header value (default: "public, max-age=3600")

### [security] - Security Configuration
Controls security features and file access:

```toml
[security]
security_headers = true                              # Enable security headers
allowed_extensions = []                              # Allowed file extensions (empty = allow all)
blocked_extensions = [".env", ".git", ".svn", ".DS_Store"] # Blocked file extensions
cors_enabled = false                                 # Enable CORS
cors_origins = ["*"]                                # CORS allowed origins
```

**Options:**
- `security_headers`: Add security headers to responses (default: true)
- `allowed_extensions`: List of allowed file extensions. Empty list allows all (default: [])
- `blocked_extensions`: List of blocked file extensions (default: [".env", ".git", ".svn", ".DS_Store"])
- `cors_enabled`: Enable Cross-Origin Resource Sharing (default: false)
- `cors_origins`: List of allowed CORS origins (default: ["*"])

### [performance] - Performance Configuration
Controls performance optimization features:

```toml
[performance]
compression = true          # Enable compression
compression_level = 6       # Compression level (1-9)
min_compress_size = 1024    # Minimum file size to compress
http2 = true               # Enable HTTP/2
```

**Options:**
- `compression`: Enable gzip compression (default: true)
- `compression_level`: Compression level from 1 (fastest) to 9 (best) (default: 6)
- `min_compress_size`: Minimum file size in bytes to compress (default: 1024)
- `http2`: Enable HTTP/2 support (default: true)

### [development] - Development Configuration
Controls development-specific features:

```toml
[development]
dev_mode = false        # Enable development mode
hot_reload = false      # Enable hot reloading
debug_routes = false    # Enable debug routes
pretty_json = true      # Pretty print JSON responses
```

**Options:**
- `dev_mode`: Enable development mode features (default: false)
- `hot_reload`: Enable automatic reloading on file changes (default: false)
- `debug_routes`: Enable debug and diagnostic routes (default: false)
- `pretty_json`: Pretty print JSON responses (default: true)

### [limits] - Request Limits Configuration
Controls request size limits for different content types:

```toml
[limits]
forms = "1MiB"     # Form data limit
json = "1MiB"      # JSON payload limit
file = "10MiB"     # File upload limit
data = "10MiB"     # General data limit
```

**Options:**
- `forms`: Maximum form data size (default: "1MiB")
- `json`: Maximum JSON payload size (default: "1MiB")
- `file`: Maximum file upload size (default: "10MiB")
- `data`: Maximum general data size (default: "10MiB")

### [template] - Template Configuration
Controls HTML template and styling options:

```toml
[template]
custom_template = "templates/custom.html"    # Custom HTML template (optional)
custom_css = "assets/custom.css"            # Custom CSS file (optional)
custom_js = "assets/custom.js"              # Custom JavaScript file (optional)
site_title = "Barleywine Server"     # Default site title
site_description = "A fast static file server with markdown support"
```

**Options:**
- `custom_template`: Path to custom HTML template for markdown rendering (optional)
- `custom_css`: Path to custom CSS file to include in generated pages (optional)
- `custom_js`: Path to custom JavaScript file to include in generated pages (optional)
- `site_title`: Default title for generated pages (default: "Barleywine Server")
- `site_description`: Default description for generated pages

### [routes] - Custom Routes Configuration
Defines custom route mappings and redirects:

```toml
[routes]
# Custom route mappings
"/api" = "api-docs.md"
"/docs" = "documentation/"

# Redirect rules
"/old-page" = "/new-page"
```

**Options:**
- Route mappings: Map URL paths to specific files or directories
- Redirects: Define HTTP redirects from old URLs to new ones

### [middleware] - Middleware Configuration
Controls which middleware components are enabled:

```toml
[middleware]
request_logging = true      # Enable request logging middleware
cors = false               # Enable CORS middleware
compression = true         # Enable compression middleware
security = true            # Enable security headers middleware
custom = ["rate_limiting"] # Custom middleware (optional)
```

**Options:**
- `request_logging`: Enable request logging middleware (default: true)
- `cors`: Enable CORS middleware (default: false)
- `compression`: Enable compression middleware (default: true)
- `security`: Enable security headers middleware (default: true)
- `custom`: List of custom middleware to enable (optional)

## CLI Override Options

Command-line options can override configuration file settings:

- `--config <FILE>`: Specify custom configuration file
- `--log <DIRECTORY>`: Override log directory
- `--loglevel <LEVEL>`: Override log level

Example:
```bash
# Use custom config with debug logging
cargo run -- --config production.toml --loglevel debug

# Override log directory
cargo run -- --log /var/log/barleywine
```

## Configuration Examples

### Development Configuration
```toml
[server]
host = "127.0.0.1"
port = 3000

[logging]
level = "debug"
format = "pretty"

[development]
dev_mode = true
hot_reload = true
debug_routes = true

[security]
cors_enabled = true
cors_origins = ["http://localhost:3000"]
```

### Production Configuration
```toml
[server]
host = "0.0.0.0"
port = 80
workers = 16
timeout = 30

[logging]
level = "warn"
file = "/var/log/barleywine/barleywine.log"
format = "json"

[cache]
enabled = true
duration = 86400  # 24 hours

[security]
security_headers = true
cors_enabled = false
blocked_extensions = [".env", ".git", ".svn", ".DS_Store", ".htaccess", ".config"]

[performance]
compression = true
compression_level = 9
http2 = true

[development]
dev_mode = false
hot_reload = false
debug_routes = false
```

### Markdown-Focused Configuration
```toml
[content]
webroot = "docs"
index_files = ["README.md", "index.md", "index.html"]
markdown_enabled = true
markdown_extensions = [
  "tables",
  "strikethrough",
  "task_lists",
  "autolinks",
  "footnotes",
  "definition_lists"
]

[template]
site_title = "Documentation Site"
site_description = "Project documentation powered by Barleywine"
custom_css = "assets/docs.css"

[cache]
enabled = true
duration = 3600
```

## Configuration Validation

Barleywine validates configuration on startup and will report errors for:

- Invalid log levels
- Non-existent directories or files
- Invalid port numbers or host addresses
- Invalid compression levels
- Missing required files (webroot, custom templates, etc.)

Use the `--verify` flag to validate configuration without starting the server:
```bash
cargo run -- --config myconfig.toml --verify
```

## Global Configuration Access

The configuration is available throughout the application via the global config system:

```rust
use crate::config;

// Initialize configuration (done automatically in main)
config::init_config(Some(Path::new("myconfig.toml")))?;

// Access configuration anywhere in the code
let config = config::get_config();
println!("Server port: {}", config.server.port);
println!("Webroot: {}", config.content.webroot.display());
```

## Environment Integration

While Barleywine primarily uses TOML configuration files, you can create wrapper scripts that generate configuration from environment variables:

```bash
#!/bin/bash
# generate-config.sh
cat > barleywine.toml << EOF
[server]
host = "${BARLEYWINE_HOST:-127.0.0.1}"
port = ${BARLEYWINE_PORT:-8000}

[logging]
level = "${LOG_LEVEL:-info}"
file = "${LOG_FILE:-logs/barleywine.log}"

[content]
webroot = "${WEBROOT:-webroot}"
EOF

cargo run
```

## Best Practices

1. **Version Control**: Keep configuration files in version control, but use different files for different environments
2. **Secrets**: Never put sensitive information directly in configuration files. Use environment variables or external secret management
3. **Validation**: Always test configuration changes with `--verify` before deploying
4. **Documentation**: Document any custom configuration options or non-standard setups
5. **Backup**: Keep backups of working configuration files
6. **Monitoring**: Monitor log files to ensure configuration changes work as expected

## Troubleshooting

### Common Configuration Issues

**Server won't start:**
- Check that the host address is valid and bindable
- Ensure the port isn't already in use
- Verify the webroot directory exists

**Logging not working:**
- Check that the log directory exists and is writable
- Verify log level is valid
- Ensure file paths are correct

**Files not serving:**
- Verify webroot path is correct
- Check file extension filtering rules
- Ensure proper file permissions

**Configuration not loading:**
- Check TOML syntax with `--verify`
- Ensure file paths are correct
- Verify file permissions

Use the `--verify` command to validate configuration:
```bash
cargo run -- --config myconfig.toml --verify
```

This will show the complete configuration that will be used and validate all settings.
