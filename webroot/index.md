# Welcome to Barleywine

**Barleywine** is a high-performance static file server that doesn't get in your way.

## Features

- 🚀 **Fast** - Built with Rust and Rocket for maximum performance
- 📝 **Markdown Support** - Automatically converts `.md` files to beautiful HTML
- 📊 **Comprehensive Logging** - Separate logs for server events and access requests
- 🎨 **Beautiful Rendering** - Clean, responsive design for markdown content
- ⚙️ **Simple Configuration** - Easy to set up and customize

## Getting Started

1. Place your files in the `webroot/` directory
2. Start the server with `cargo run`
3. Access your files at `http://localhost:8000`

## Logging

Barleywine maintains two separate log files:

- **`barleywine.log`** - Contains server startup, shutdown, and operational messages
- **`access.log`** - Contains detailed access logs for all web requests

### Example log entries:

**barleywine.log:**
```
[2024-01-15 10:30:00 UTC] Server started on port 8000 serving files from webroot/
[2024-01-15 10:30:05 UTC] Served markdown file: index.md
```

**access.log:**
```
[2024-01-15 10:30:05 UTC] 127.0.0.1 "GET" "/" 200 "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)"
[2024-01-15 10:30:10 UTC] 127.0.0.1 "GET" "/about.md" 200 "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)"
```

## Code Examples

Here's how simple it is to serve files:

```rust
use rocket::fs::NamedFile;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("webroot/").join(file)).await.ok()
}
```

## Configuration

You can customize Barleywine's behavior with command-line options:

- `--loglevel` - Set logging level (error, warn, info, debug, trace)
- `--log` - Specify custom log directory
- `--config` - Use a custom configuration file
- `--verify` - Verify configuration without starting the server

---

*Happy serving! 🍺*
