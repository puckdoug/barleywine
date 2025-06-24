# 🍺 Barleywine Server

A fast and efficient static file server built with Rust and the Rocket web framework. Barleywine serves files from a `webroot` directory with automatic MIME type detection, index.html support for directory requests, and **automatic markdown-to-HTML conversion**.

## Features

- ✨ **Static File Serving**: Serves any file from the `webroot` directory
- 📝 **Markdown Conversion**: Automatically converts `.md` files to beautifully styled HTML
- 🏠 **Automatic Index Pages**: Serves `index.html` or `index.md` when directories are requested
- 🎯 **MIME Type Detection**: Automatically detects and sets correct MIME types based on file extensions
- ⚡ **High Performance**: Built with Rust and Rocket for maximum efficiency
- 🔒 **Security**: Files are served only from the designated webroot directory
- 📱 **Cross-Platform**: Works on Windows, macOS, and Linux
- 🎨 **Beautiful Styling**: Markdown files get responsive, clean HTML templates automatically

## Quick Start

### Prerequisites

- Rust (1.70+ recommended)
- Cargo (comes with Rust)

### Running the Server

1. **Clone or navigate to the project directory**

   ```bash
   cd barleywine
   ```

2. **Add your static files to the `webroot` directory**

   ```bash
   # The webroot directory is where all your static files go
   ls webroot/
   ```

3. **Run the server**

   ```bash
   # Basic usage
   cargo run

   # With custom options
   cargo run -- --loglevel debug
   cargo run -- --config myconfig.toml
   cargo run -- --log logs/server.log
   ```

4. **Verify configuration (optional)**

   ```bash
   # Test your setup without starting the server
   cargo run -- --verify
   ```

5. **Access your files**
   - Open your browser to `http://localhost:8000`
   - The server will automatically serve `webroot/index.html` or `webroot/index.md` if they exist
   - Access any file directly: `http://localhost:8000/filename.ext`
   - Access subdirectories: `http://localhost:8000/subdir/` (serves `subdir/index.html` or `subdir/index.md`)
   - Markdown files are automatically converted to HTML: `http://localhost:8000/document.md`

## File Structure

```
barleywine/
├── src/
│   └── main.rs          # Main server implementation
webroot/             # Static files directory
│   ├── index.html       # Main page (served at /)
│   ├── index.md         # Markdown main page (alternative to HTML)
│   ├── sample.md        # Markdown files (auto-converted to HTML)
│   ├── blog.md          # More markdown content
│   ├── styles.css       # CSS files
│   ├── script.js        # JavaScript files
│   ├── test.txt         # Text files
│   ├── blog/            # Subdirectories
│   │   ├── index.md     # Markdown directory index
│   │   └── post-1.md    # Blog posts in markdown
│   └── subdir/          # Mixed content directories
│       └── index.html   # HTML directory index pages
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Supported MIME Types

Barleywine automatically detects MIME types based on file extensions:

| Extension        | MIME Type                | Description                                  |
| ---------------- | ------------------------ | -------------------------------------------- |
| `.html`, `.htm`  | `text/html`              | HTML documents                               |
| `.md`            | `text/html`              | Markdown files (converted to HTML)           |
| `.css`           | `text/css`               | Stylesheets                                  |
| `.js`            | `application/javascript` | JavaScript files                             |
| `.json`          | `application/json`       | JSON data                                    |
| `.txt`           | `text/plain`             | Plain text                                   |
| `.png`           | `image/png`              | PNG images                                   |
| `.jpg`, `.jpeg`  | `image/jpeg`             | JPEG images                                  |
| `.gif`           | `image/gif`              | GIF images                                   |
| `.svg`           | `image/svg+xml`          | SVG images                                   |
| `.pdf`           | `application/pdf`        | PDF documents                                |
| `.mp4`           | `video/mp4`              | MP4 videos                                   |
| `.mp3`           | `audio/mpeg`             | MP3 audio                                    |
| And many more... |                          | Rocket's NamedFile handles most common types |

## Usage Examples

### Basic HTML Page

Create `webroot/index.html`:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>My Site</title>
    <link rel="stylesheet" href="styles.css" />
  </head>
  <body>
    <h1>Welcome to My Site</h1>
    <script src="script.js"></script>
  </body>
</html>
```

### Markdown Page

Create `webroot/index.md`:

````markdown
# Welcome to My Site

This **markdown file** will be automatically converted to HTML with beautiful styling!

## Features

- Easy to write and edit
- Automatic HTML conversion
- Responsive design
- Syntax highlighting for code blocks

```rust
fn main() {
    println!("Hello from Barleywine!");
}
```
````

Visit [/sample.md](/sample.md) for more examples.

````

### Subdirectory with Index

Create `webroot/blog/index.html`:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>My Blog</title>
  </head>
  <body>
    <h1>Blog Posts</h1>
  </body>
</html>
````

Or create `webroot/blog/index.md`:

```markdown
# My Blog

Welcome to my blog! Here are the latest posts:

## Recent Posts

- [Getting Started with Rust](post-1.md)
- [Web Development Tips](post-2.md)

_This directory index is written in markdown and automatically converted to HTML._
```

Access at: `http://localhost:8000/blog/`

### Direct File Access

- `http://localhost:8000/styles.css` - Serves CSS with `text/css` MIME type
- `http://localhost:8000/script.js` - Serves JavaScript with `application/javascript`
- `http://localhost:8000/document.md` - Converts markdown to HTML with `text/html`
- `http://localhost:8000/image.png` - Serves PNG with `image/png`

## Command Line Interface

Barleywine provides a comprehensive command-line interface for configuration and control:

### Usage

```bash
barleywine [FLAGS] [OPTIONS]
```

### Flags

- `-h, --help` - Print help information
- `-V, --version` - Print version information
- `--verify` - Verify the configuration without running the server

### Options

- `-c, --config <FILE>` - Specify a configuration file
- `--loglevel <LEVEL>` - Set the log level (error, warn, info, debug, trace) [default: info]
- `--log <FILE>` - Specify a different log file

### Examples

```bash
# Show help
cargo run -- --help

# Show version
cargo run -- --version

# Verify configuration without starting server
cargo run -- --verify

# Run with debug logging
cargo run -- --loglevel debug

# Use custom config file
cargo run -- --config production.toml

# Log to file
cargo run -- --log logs/barleywine.log

# Combine multiple options
cargo run -- --config prod.toml --loglevel warn --log logs/prod.log
```

### Configuration Verification

The `--verify` flag allows you to test your configuration without starting the server:

```bash
$ cargo run -- --verify

🍺 Barleywine Server
Configuration verification:
Barleywine Configuration:
  Config file: default
  Log level: info
  Log file: stdout
  Verify only: true
  Webroot directory: ✅ Found at 'webroot/'
  Markdown support: ✅ Available
  Rocket framework: ✅ Ready

Configuration is valid! ✅
Run without --verify to start the server.
```

## Configuration

### Configuration File

Barleywine supports TOML configuration files for advanced settings. Use the `--config` flag to specify a custom configuration file:

```bash
cargo run -- --config myconfig.toml
```

Example configuration file:

```toml
[server]
host = "127.0.0.1"
port = 8000
workers = 4

[logging]
level = "info"
access_log = true
format = "pretty"

[content]
webroot = "webroot"
index_files = ["index.html", "index.md"]
markdown_enabled = true

[security]
security_headers = true
blocked_extensions = [".env", ".git", ".DS_Store"]

[performance]
compression = true
http2 = true
```

### Changing the Port

Rocket uses port 8000 by default. To change it, you can:

1. **Environment variable**:

   ```bash
   ROCKET_PORT=3000 cargo run
   ```

2. **Rocket.toml configuration file**:

   ```toml
   [default]
   port = 3000
   address = "127.0.0.1"
   ```

3. **Custom configuration file** (if implemented):
   ```toml
   [server]
   port = 3000
   host = "127.0.0.1"
   ```

### Logging Configuration

Control logging behavior with command-line options:

```bash
# Set log level
cargo run -- --loglevel debug

# Log to file instead of stdout
cargo run -- --log logs/server.log

# Combine with other options
cargo run -- --loglevel warn --log logs/warnings.log
```

Available log levels:

- `error` - Only errors
- `warn` - Warnings and errors
- `info` - General information (default)
- `debug` - Detailed debugging info
- `trace` - Very verbose tracing

### Changing the Webroot Directory

To serve files from a different directory, modify `src/main.rs`:

```rust
let webroot = Path::new("your-custom-directory");
```

Or use a configuration file (if implemented):

```toml
[content]
webroot = "my-content"
```

## Development

### Building for Release

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Checking Code

```bash
cargo check
cargo clippy
```

## Error Handling

- **404 Not Found**: Returned when a requested file doesn't exist
- **Directory without Index**: Returns 404 if no `index.html` exists in the directory
- **File Access Errors**: Properly handled with descriptive error messages

## Security Considerations

- Files are served only from the designated `webroot` directory
- Path traversal attacks (e.g., `../../../etc/passwd`) are prevented by Rocket's path handling
- No directory listing is provided for directories without index files

## Dependencies

- **Rocket 0.5.1**: Web framework for serving HTTP requests
- **Tokio**: Async runtime for file operations
- **markdown 1.0**: CommonMark compliant markdown parser for converting .md files to HTML

## License

This project is licensed under the terms specified in the LICENSE file.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Troubleshooting

### Common Issues

**Server won't start:**

- Check if port 8000 is already in use
- Ensure Rust and Cargo are properly installed
- Use `--verify` to check configuration: `cargo run -- --verify`

**Configuration issues:**

- Use `--verify` flag to validate your setup before running
- Check that config file exists if using `--config`
- Ensure log file directory exists if using `--log`

**Files not found:**

- Verify files exist in the `webroot` directory
- Check file permissions
- Ensure correct file paths (case-sensitive on Unix systems)
- Use `--verify` to confirm webroot directory is detected

**MIME types incorrect:**

- File extension should match expected MIME type
- Some browsers cache MIME type information

**Markdown not converting:**

- Ensure `.md` files are in the `webroot` directory
- Check that markdown content is valid
- Verify the `markdown` crate dependency is installed
- Use `--verify` to confirm markdown support is enabled

**Command-line issues:**

- Use `--help` to see all available options
- Check log level is valid: error, warn, info, debug, trace
- Ensure file paths are correct and accessible

### Getting Help

If you encounter issues:

1. **Use the verify command**: `cargo run -- --verify`
2. Check the console output for error messages
3. Use `--help` to see all available options
4. Verify your file paths and permissions
5. Ensure your `webroot` directory structure is correct
6. Test with simple HTML files first
7. Try creating a simple `.md` file to test markdown conversion
8. Check that both HTML and markdown index files work in directories
9. Run with `--loglevel debug` for more detailed output

---

Happy serving! 🚀🍺
