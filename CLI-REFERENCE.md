# 🍺 Barleywine CLI Quick Reference

## Usage Syntax

```bash
barleywine [FLAGS] [OPTIONS]
```

## Flags

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-h` | `--help` | Print help information and exit |
| `-V` | `--version` | Print version information and exit |
| | `--verify` | Verify configuration without starting server |

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-c, --config` | `<FILE>` | none | Specify configuration file path |
| `--loglevel` | `<LEVEL>` | `info` | Set log level (error/warn/info/debug/trace) |
| `--log` | `<FILE>` | stdout | Specify log file path |

## Quick Examples

### Basic Usage
```bash
# Start server with defaults
cargo run

# Show help
cargo run -- --help

# Show version
cargo run -- --version
```

### Configuration & Verification
```bash
# Verify setup before running
cargo run -- --verify

# Use custom config file
cargo run -- --config myconfig.toml

# Verify custom config
cargo run -- --config prod.toml --verify
```

### Logging
```bash
# Set log level
cargo run -- --loglevel debug
cargo run -- --loglevel error

# Log to file
cargo run -- --log logs/server.log

# Combined logging options
cargo run -- --loglevel warn --log logs/warnings.log
```

### Production Examples
```bash
# Production server
cargo run -- --config production.toml --loglevel error --log logs/error.log

# Development server
cargo run -- --loglevel debug --log logs/dev.log

# Testing configuration
cargo run -- --config test.toml --verify
```

## Log Levels

| Level | Description | Use Case |
|-------|-------------|----------|
| `error` | Only errors | Production (minimal logging) |
| `warn` | Warnings + errors | Production (standard logging) |
| `info` | General info | Default (balanced logging) |
| `debug` | Detailed debugging | Development (verbose logging) |
| `trace` | Very verbose | Troubleshooting (maximum detail) |

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Error (invalid arguments, missing files, etc.) |

## Validation Rules

### Log Level
- Must be one of: `error`, `warn`, `info`, `debug`, `trace`
- Case insensitive (`INFO`, `Debug`, etc. accepted)

### Config File
- File must exist if specified
- Path can be relative or absolute
- TOML format expected

### Log File
- Parent directory must exist
- Will be created if doesn't exist
- Path can be relative or absolute

## Common Patterns

### Development Workflow
```bash
# 1. Verify setup
cargo run -- --verify

# 2. Start with debug logging
cargo run -- --loglevel debug

# 3. Test config changes
cargo run -- --config new-config.toml --verify
```

### Production Deployment
```bash
# 1. Verify production config
cargo run -- --config prod.toml --verify

# 2. Start production server
cargo run -- --config prod.toml --loglevel warn --log logs/prod.log
```

### Troubleshooting
```bash
# Maximum verbosity
cargo run -- --loglevel trace --log logs/trace.log

# Check configuration
cargo run -- --verify

# Test specific config
cargo run -- --config debug.toml --verify
```

## File Structure Requirements

### Minimum Setup
```
project/
├── src/main.rs
├── Cargo.toml
└── webroot/           # Required directory
    └── index.html     # or index.md
```

### With Configuration
```
project/
├── config.toml        # Custom config
├── logs/              # Log directory (if using --log)
├── webroot/           # Content directory
└── src/main.rs
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `ROCKET_PORT` | Override default port | `ROCKET_PORT=3000` |
| `ROCKET_ADDRESS` | Override bind address | `ROCKET_ADDRESS=0.0.0.0` |
| `ROCKET_LOG_LEVEL` | Rocket's internal logging | `ROCKET_LOG_LEVEL=debug` |

## Error Messages & Solutions

### "Configuration file does not exist"
```bash
# Check file path
ls -la config.toml

# Use absolute path if needed
cargo run -- --config /full/path/to/config.toml
```

### "Invalid log level"
```bash
# Use valid log level
cargo run -- --loglevel info  # ✅ Valid
cargo run -- --loglevel INFO  # ✅ Case insensitive
cargo run -- --loglevel invalid  # ❌ Invalid
```

### "Log file directory does not exist"
```bash
# Create directory first
mkdir -p logs
cargo run -- --log logs/server.log
```

### "webroot directory not found"
```bash
# Create webroot directory
mkdir webroot
echo "# Hello World" > webroot/index.md
```

## Integration Examples

### Docker
```dockerfile
# Build stage
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/barleywine /usr/local/bin/
COPY --from=builder /app/webroot /app/webroot
COPY --from=builder /app/config.toml /app/
WORKDIR /app
CMD ["barleywine", "--config", "config.toml", "--loglevel", "info"]
```

### Systemd Service
```ini
[Unit]
Description=Barleywine Server
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/barleywine
ExecStart=/usr/local/bin/barleywine --config production.toml --log /var/log/barleywine/server.log
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Shell Script
```bash
#!/bin/bash
# start-server.sh

CONFIG_FILE="${1:-config.toml}"
LOG_LEVEL="${2:-info}"
LOG_FILE="${3:-logs/server.log}"

# Verify configuration first
if ! cargo run -- --config "$CONFIG_FILE" --verify; then
    echo "❌ Configuration verification failed"
    exit 1
fi

# Start server
echo "🍺 Starting Barleywine..."
cargo run -- \
    --config "$CONFIG_FILE" \
    --loglevel "$LOG_LEVEL" \
    --log "$LOG_FILE"
```

## Tips & Best Practices

1. **Always verify first**: Use `--verify` before starting production servers
2. **Log to files in production**: Use `--log` to capture server logs
3. **Use appropriate log levels**: `error` for production, `debug` for development
4. **Organize configs**: Keep separate config files for different environments
5. **Check webroot**: Ensure `webroot/` directory exists and has content
6. **Monitor logs**: Regularly check log files for issues
7. **Test configurations**: Use `--verify` when changing config files

---

**Quick Help**: `cargo run -- --help`
**Version Info**: `cargo run -- --version`
**Full Documentation**: See `README.md` and `EXAMPLES.md`
