# Barleywine Logging System

Barleywine includes a comprehensive logging system that tracks both server operations and web access requests. The logging system creates two separate log files to help you monitor and debug your static file server.

## Log Files

### 1. `barleywine.log` - Server Operations Log

This file contains all operational messages about the Barleywine server itself, including:

- Server startup and shutdown events
- File serving operations
- Error messages and warnings
- Configuration information
- Internal server events

**Example entries:**
```
[2024-01-15 10:30:00 UTC] Barleywine logging started at 2024-01-15 10:30:00 UTC
[2024-01-15 10:30:00 UTC] Server started on port 8000 serving files from webroot/
[2024-01-15 10:30:05 UTC] Served markdown file: /home/user/project/webroot/index.md
[2024-01-15 10:30:10 UTC] Served static file: /home/user/project/webroot/style.css
[2024-01-15 10:35:20 UTC] ERROR: Could not read file: permission denied
[2024-01-15 10:40:00 UTC] Server shutting down
```

### 2. `access.log` - HTTP Access Log

This file contains detailed information about every HTTP request made to the server, similar to Apache/Nginx access logs:

- Client IP addresses
- HTTP methods and requested URIs
- Response status codes
- User agent strings
- Timestamps for each request

**Example entries:**
```
[2024-01-15 10:30:05 UTC] 127.0.0.1 "GET" "/" 200 "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"
[2024-01-15 10:30:10 UTC] 192.168.1.100 "GET" "/about.md" 200 "curl/7.68.0"
[2024-01-15 10:30:15 UTC] 10.0.0.1 "GET" "/nonexistent.html" 404 "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
[2024-01-15 10:30:20 UTC] 172.16.0.50 "GET" "/data.json" 500 "-"
```

## Configuration

### Command Line Options

#### `--log <DIRECTORY>`
Specify a custom directory for log files. If not provided, logs will be written to a `logs/` directory in the current working directory.

```bash
# Use default logs directory
cargo run

# Use custom log directory
cargo run -- --log /var/log/barleywine

# Use relative path
cargo run -- --log ./custom-logs
```

#### `--loglevel <LEVEL>`
Set the logging verbosity level. This affects both console output and the `barleywine.log` file.

Available levels (from least to most verbose):
- `error` - Only error messages
- `warn` - Warnings and errors
- `info` - General information (default)
- `debug` - Detailed debugging information
- `trace` - Very detailed tracing information

```bash
# Set log level to debug
cargo run -- --loglevel debug

# Set log level to error (quiet)
cargo run -- --loglevel error
```

### Automatic Directory Creation

If the specified log directory doesn't exist, Barleywine will automatically create it (including parent directories) when the server starts.

## Log Format Details

### Barleywine Log Format
```
[TIMESTAMP] MESSAGE
```

- **TIMESTAMP**: UTC timestamp in format `YYYY-MM-DD HH:MM:SS UTC`
- **MESSAGE**: Human-readable message describing the event

### Access Log Format
```
[TIMESTAMP] CLIENT_IP "METHOD" URI "STATUS" "USER_AGENT"
```

- **TIMESTAMP**: UTC timestamp in format `YYYY-MM-DD HH:MM:SS UTC`
- **CLIENT_IP**: IP address of the client making the request
- **METHOD**: HTTP method (GET, POST, etc.)
- **URI**: Requested URI path
- **STATUS**: HTTP response status code
- **USER_AGENT**: Client's User-Agent header (or "-" if not provided)

## Integration with Application

The logging system is automatically initialized when Barleywine starts and integrates seamlessly with the web server. Every request is logged, including:

- Successful file serves (200 status)
- File not found errors (404 status)
- Server errors (500 status)
- Both static files and markdown conversions

## Log Rotation

Currently, Barleywine appends to log files indefinitely. For production use, you may want to implement log rotation using external tools like `logrotate` on Linux systems.

Example logrotate configuration (`/etc/logrotate.d/barleywine`):
```
/path/to/your/logs/*.log {
    daily
    missingok
    rotate 30
    compress
    notifempty
    copytruncate
}
```

## Monitoring and Analysis

### Real-time Monitoring
```bash
# Watch barleywine operations
tail -f logs/barleywine.log

# Watch HTTP access requests
tail -f logs/access.log

# Watch both files simultaneously
tail -f logs/*.log
```

### Log Analysis Examples
```bash
# Count requests by IP address
awk '{print $3}' logs/access.log | sort | uniq -c | sort -nr

# Find 404 errors
grep '"404"' logs/access.log

# Find requests from specific IP
grep "192.168.1.100" logs/access.log

# Count requests by hour
awk '{print $2}' logs/access.log | cut -d: -f1-2 | sort | uniq -c
```

## Performance Considerations

- Log files are opened once at startup and kept open for performance
- Each log entry is immediately flushed to disk for reliability
- Logging operations are designed to have minimal impact on request handling performance
- The logging system uses thread-safe mechanisms to handle concurrent requests

## Troubleshooting

### Common Issues

**Log files not created:**
- Check that the specified log directory exists and is writable
- Verify that the application has permission to create files in the directory

**Empty log files:**
- Ensure the server is actually receiving requests for access logs
- Check that the log level isn't set too high (e.g., `error` when you want `info` messages)

**Permission errors:**
- Make sure the user running Barleywine has write permissions to the log directory
- Consider using a directory in the user's home folder or `/tmp` for testing

### Debug Mode

Run with `--loglevel debug` to see more detailed information about the logging system initialization and operation:

```bash
cargo run -- --loglevel debug --log logs
```

This will show additional information such as:
- Log system initialization messages
- File creation and opening operations
- More detailed error messages

## Examples

### Basic Usage
```bash
# Start server with default logging
cargo run

# Check logs
cat logs/barleywine.log
cat logs/access.log
```

### Production Setup
```bash
# Create dedicated log directory
sudo mkdir -p /var/log/barleywine
sudo chown $(whoami):$(whoami) /var/log/barleywine

# Start with custom log location and appropriate log level
cargo run -- --log /var/log/barleywine --loglevel warn
```

### Development Setup
```bash
# Use debug level for development
cargo run -- --loglevel debug --log ./dev-logs

# Monitor logs during development
tail -f dev-logs/*.log
```
