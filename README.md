# 🍺 Barleywine Static File Server

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
   cargo run
   ```

4. **Access your files**
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

## Configuration

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

### Changing the Webroot Directory

To serve files from a different directory, modify `src/main.rs`:

```rust
let webroot = Path::new("your-custom-directory");
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

**Files not found:**

- Verify files exist in the `webroot` directory
- Check file permissions
- Ensure correct file paths (case-sensitive on Unix systems)

**MIME types incorrect:**

- File extension should match expected MIME type
- Some browsers cache MIME type information

**Markdown not converting:**

- Ensure `.md` files are in the `webroot` directory
- Check that markdown content is valid
- Verify the `markdown` crate dependency is installed

### Getting Help

If you encounter issues:

1. Check the console output for error messages
2. Verify your file paths and permissions
3. Ensure your `webroot` directory structure is correct
4. Test with simple HTML files first
5. Try creating a simple `.md` file to test markdown conversion
6. Check that both HTML and markdown index files work in directories

---

Happy serving! 🚀🍺
