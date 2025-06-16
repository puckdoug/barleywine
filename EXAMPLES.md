# 🍺 Barleywine Examples

This guide provides comprehensive examples of how to use Barleywine to create different types of websites and applications.

## Table of Contents

- [Quick Start Example](#quick-start-example)
- [Personal Blog](#personal-blog)
- [Documentation Site](#documentation-site)
- [Portfolio Website](#portfolio-website)
- [Mixed Content Site](#mixed-content-site)
- [Command-Line Usage Examples](#command-line-usage-examples)
- [Configuration Examples](#configuration-examples)
- [Advanced Use Cases](#advanced-use-cases)

## Quick Start Example

### 1. Basic Static Site

Create a simple website with mixed HTML and Markdown content:

```
webroot/
├── index.html              # Main homepage
├── about.md               # About page in Markdown
├── contact.html           # Contact form
├── blog/
│   ├── index.md          # Blog index
│   ├── post-1.md         # First blog post
│   └── post-2.md         # Second blog post
├── assets/
│   ├── style.css         # Site styles
│   ├── script.js         # Site JavaScript
│   └── images/
│       ├── logo.png      # Site logo
│       └── hero.jpg      # Hero image
└── docs/
    ├── index.md          # Documentation home
    ├── getting-started.md
    └── api-reference.md
```

**webroot/index.html:**
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Awesome Site</title>
    <link rel="stylesheet" href="/assets/style.css">
</head>
<body>
    <header>
        <nav>
            <a href="/">Home</a>
            <a href="/about.md">About</a>
            <a href="/blog/">Blog</a>
            <a href="/docs/">Docs</a>
            <a href="/contact.html">Contact</a>
        </nav>
    </header>

    <main>
        <h1>Welcome to My Site</h1>
        <p>This is a hybrid site using both HTML and Markdown!</p>

        <section class="features">
            <h2>Why Barleywine?</h2>
            <ul>
                <li>✨ Instant markdown conversion</li>
                <li>🚀 Zero configuration</li>
                <li>📱 Mobile-friendly</li>
                <li>⚡ Lightning fast</li>
            </ul>
        </section>
    </main>

    <script src="/assets/script.js"></script>
</body>
</html>
```

**webroot/about.md:**
```markdown
# About Me

Hi! I'm a developer who loves **simple solutions** to complex problems.

## My Story

I discovered Barleywine when I needed a simple way to serve both static HTML and dynamic Markdown content without complex build processes.

### What I Do

- 💻 Web Development
- 📝 Technical Writing
- 🚀 Performance Optimization
- 🛠️ Tool Development

## Contact

Feel free to [reach out](/contact.html) if you want to collaborate!

*This page demonstrates how Markdown files are automatically converted to beautiful HTML.*
```

**Run the server:**
```bash
cargo run -- --loglevel info
```

Visit:
- `http://localhost:8000/` - HTML homepage
- `http://localhost:8000/about.md` - Markdown about page
- `http://localhost:8000/blog/` - Blog index (Markdown)

## Personal Blog

### Directory Structure

```
webroot/
├── index.md               # Blog homepage
├── about.md              # About the author
├── archive.md            # Post archive
├── posts/
│   ├── 2024/
│   │   ├── 01-getting-started.md
│   │   ├── 02-advanced-tips.md
│   │   └── 03-performance-guide.md
│   └── 2023/
│       ├── 12-year-review.md
│       └── 11-thanksgiving.md
├── categories/
│   ├── tech.md
│   ├── personal.md
│   └── tutorials.md
├── assets/
│   ├── blog.css
│   ├── blog.js
│   └── images/
│       ├── avatars/
│       └── posts/
└── rss.xml               # RSS feed (static)
```

**webroot/index.md:**
```markdown
# My Developer Blog

Welcome to my corner of the internet! 👋

## Latest Posts

### [Getting Started with Rust Web Development](/posts/2024/01-getting-started.md)
*January 15, 2024*

Learn how to build fast, safe web applications with Rust and modern frameworks.

### [Advanced Performance Tips](/posts/2024/02-advanced-tips.md)
*January 10, 2024*

Deep dive into optimization techniques that make your web apps lightning fast.

### [The Complete Performance Guide](/posts/2024/03-performance-guide.md)
*January 5, 2024*

Everything you need to know about web performance in 2024.

## Categories

- [💻 Tech](/categories/tech.md) - Technical articles and tutorials
- [👨‍💻 Personal](/categories/personal.md) - Personal thoughts and experiences
- [📚 Tutorials](/categories/tutorials.md) - Step-by-step guides

## About

I'm a passionate developer who loves sharing knowledge. [Learn more about me](/about.md).

---

*Subscribe to my [RSS feed](/rss.xml) to stay updated!*
```

**webroot/posts/2024/01-getting-started.md:**
```markdown
# Getting Started with Rust Web Development

*Published on January 15, 2024 • 10 min read*

Rust has revolutionized systems programming, and now it's making waves in web development too! 🦀

## Why Rust for Web?

1. **Memory Safety** - No more segfaults or buffer overflows
2. **Performance** - Blazing fast execution
3. **Concurrency** - Fearless parallel processing
4. **Ecosystem** - Growing collection of web frameworks

## Popular Web Frameworks

### Rocket 🚀
```rust
use rocket::{get, routes, launch};

#[get("/")]
fn hello() -> &'static str {
    "Hello, Rust web world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
```

### Axum ⚡
```rust
use axum::{response::Html, routing::get, Router};

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Axum!</h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    // ...
}
```

## Getting Started

1. **Install Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Create Project**: `cargo new my-web-app`
3. **Add Dependencies**: `cargo add rocket`
4. **Build & Run**: `cargo run`

## Next Steps

- [Advanced Performance Tips](/posts/2024/02-advanced-tips.md)
- [Database Integration Guide](/posts/2024/04-database-guide.md)
- [Deployment Strategies](/posts/2024/05-deployment.md)

---

**Tags:** #rust #web-development #tutorial #beginner

[← Back to Blog](/) | [Next Post →](/posts/2024/02-advanced-tips.md)
```

**Run with custom config:**
```bash
cargo run -- --config blog-config.toml --loglevel info
```

## Documentation Site

### Structure

```
webroot/
├── index.md              # Documentation home
├── getting-started/
│   ├── index.md         # Getting started guide
│   ├── installation.md
│   ├── first-steps.md
│   └── examples.md
├── api/
│   ├── index.md         # API overview
│   ├── authentication.md
│   ├── endpoints.md
│   └── examples.md
├── guides/
│   ├── index.md
│   ├── best-practices.md
│   ├── troubleshooting.md
│   └── deployment.md
├── reference/
│   ├── index.md
│   ├── configuration.md
│   ├── cli.md
│   └── changelog.md
└── assets/
    ├── docs.css
    ├── prism.css        # Syntax highlighting
    ├── prism.js
    └── search.js
```

**webroot/index.md:**
```markdown
# Barleywine Documentation

Welcome to the official Barleywine documentation! 📚

## Quick Navigation

### 🚀 [Getting Started](/getting-started/)
New to Barleywine? Start here for installation and basic setup.

### 📖 [API Reference](/api/)
Complete API documentation with examples and best practices.

### 📋 [Guides](/guides/)
In-depth guides for common use cases and advanced features.

### 🔧 [Reference](/reference/)
Configuration options, CLI commands, and technical specifications.

## What is Barleywine?

Barleywine is a high-performance static file server built with Rust that automatically converts Markdown files to beautifully styled HTML.

### Key Features

- ✨ **Zero Configuration** - Works out of the box
- 📝 **Markdown Support** - Automatic HTML conversion
- 🎨 **Beautiful Styling** - Responsive design included
- ⚡ **Lightning Fast** - Built with Rust for speed
- 🔒 **Secure** - Safe file serving with path protection

## Quick Start

```bash
# Install and run
cargo install barleywine
mkdir webroot
echo "# Hello World" > webroot/index.md
barleywine

# Visit http://localhost:8000
```

## Recent Updates

- **v0.1.0** - Initial release with markdown conversion
- **v0.0.9** - Added CLI interface and configuration
- **v0.0.8** - Performance improvements and bug fixes

## Community

- 📧 **Email**: docs@barleywine.dev
- 💬 **Discord**: [Join our server](https://discord.gg/barleywine)
- 🐛 **Issues**: [GitHub Issues](https://github.com/barleywine/barleywine/issues)
- 💡 **Discussions**: [GitHub Discussions](https://github.com/barleywine/barleywine/discussions)

---

*Last updated: January 2024 • [Edit on GitHub](https://github.com/barleywine/docs)*
```

## Portfolio Website

### Structure

```
webroot/
├── index.html           # Landing page (HTML for custom design)
├── about.md            # About page
├── projects/
│   ├── index.md        # Projects overview
│   ├── project-1.md    # Individual projects
│   ├── project-2.md
│   └── project-3.md
├── blog/
│   ├── index.md        # Blog home
│   └── posts/
│       ├── 2024-01-post.md
│       └── 2024-02-post.md
├── contact.html        # Contact form
├── resume.md           # Resume in Markdown
├── assets/
│   ├── portfolio.css
│   ├── animations.css
│   ├── main.js
│   └── images/
│       ├── projects/
│       ├── avatar.jpg
│       └── backgrounds/
└── downloads/
    ├── resume.pdf
    └── portfolio.pdf
```

## Mixed Content Site

### Structure for E-commerce + Blog

```
webroot/
├── index.html           # Custom homepage
├── shop/
│   ├── index.html      # Product catalog (HTML)
│   ├── product-1.html  # Product pages (HTML)
│   └── cart.html       # Shopping cart (HTML)
├── blog/
│   ├── index.md        # Blog (Markdown)
│   ├── tutorials/
│   │   ├── index.md
│   │   └── how-to-*.md
│   └── news/
│       ├── index.md
│       └── *.md
├── support/
│   ├── index.md        # Help center
│   ├── faq.md
│   ├── shipping.md
│   └── returns.md
├── legal/
│   ├── privacy.md      # Legal pages in Markdown
│   ├── terms.md
│   └── cookies.md
└── assets/
    ├── shop.css        # E-commerce styles
    ├── blog.css        # Blog styles
    ├── shop.js         # Shopping functionality
    └── images/
```

## Command-Line Usage Examples

### Basic Usage

```bash
# Start server with defaults
cargo run

# Show help
cargo run -- --help

# Show version
cargo run -- --version

# Verify configuration
cargo run -- --verify
```

### Logging Options

```bash
# Set log level to debug
cargo run -- --loglevel debug

# Log to file
cargo run -- --log logs/server.log

# Both together
cargo run -- --loglevel debug --log logs/debug.log

# Different log levels
cargo run -- --loglevel error   # Only errors
cargo run -- --loglevel warn    # Warnings + errors
cargo run -- --loglevel info    # Default level
cargo run -- --loglevel debug   # Detailed info
cargo run -- --loglevel trace   # Very verbose
```

### Configuration Files

```bash
# Use custom config
cargo run -- --config production.toml

# Verify config without running
cargo run -- --config prod.toml --verify

# Multiple options
cargo run -- --config prod.toml --loglevel warn --log logs/prod.log
```

### Development Workflow

```bash
# Development server with debug logging
cargo run -- --loglevel debug

# Production server with minimal logging
cargo run -- --config prod.toml --loglevel error --log logs/error.log

# Testing configuration
cargo run -- --config test.toml --verify
```

## Configuration Examples

### Basic Configuration (config.toml)

```toml
[server]
host = "127.0.0.1"
port = 8000
workers = 4

[logging]
level = "info"
access_log = true

[content]
webroot = "webroot"
markdown_enabled = true
index_files = ["index.html", "index.md"]
```

### Production Configuration (production.toml)

```toml
[server]
host = "0.0.0.0"
port = 80
workers = 8
timeout = 30

[logging]
level = "warn"
file = "/var/log/barleywine/server.log"
access_log = true
format = "json"

[content]
webroot = "/var/www/html"
markdown_enabled = true
index_files = ["index.html", "index.md"]

[security]
security_headers = true
blocked_extensions = [".env", ".git", ".svn", ".DS_Store", ".htaccess"]

[performance]
compression = true
compression_level = 6
http2 = true

[cache]
enabled = true
duration = 3600
cache_control = "public, max-age=3600"
```

### Development Configuration (dev.toml)

```toml
[server]
host = "127.0.0.1"
port = 3000
workers = 2

[logging]
level = "debug"
access_log = true
format = "pretty"

[content]
webroot = "src/webroot"
markdown_enabled = true

[development]
dev_mode = true
hot_reload = true
debug_routes = true
pretty_json = true
```

## Advanced Use Cases

### Multi-Language Documentation

```
webroot/
├── index.md             # Default (English)
├── en/
│   ├── index.md
│   ├── guide.md
│   └── api.md
├── es/
│   ├── index.md
│   ├── guia.md
│   └── api.md
├── fr/
│   ├── index.md
│   ├── guide.md
│   └── api.md
└── assets/
    ├── i18n.js
    └── flags/
```

### API Documentation with Examples

```
webroot/
├── index.md             # API overview
├── authentication/
│   ├── index.md
│   ├── oauth.md
│   └── api-keys.md
├── endpoints/
│   ├── index.md
│   ├── users.md
│   ├── posts.md
│   └── comments.md
├── examples/
│   ├── index.md
│   ├── curl.md
│   ├── javascript.md
│   ├── python.md
│   └── rust.md
├── sdks/
│   ├── index.md
│   ├── javascript.md
│   ├── python.md
│   └── rust.md
└── assets/
    ├── api-docs.css
    ├── code-examples.js
    └── syntax-highlighting.css
```

### Knowledge Base

```
webroot/
├── index.md             # Search and categories
├── categories/
│   ├── getting-started/
│   │   ├── index.md
│   │   ├── installation.md
│   │   └── first-steps.md
│   ├── troubleshooting/
│   │   ├── index.md
│   │   ├── common-issues.md
│   │   └── error-codes.md
│   └── advanced/
│       ├── index.md
│       ├── configuration.md
│       └── customization.md
├── search.html          # Search interface
├── tags/
│   ├── beginner.md
│   ├── advanced.md
│   └── troubleshooting.md
└── assets/
    ├── search.js
    ├── knowledge-base.css
    └── icons/
```

### Team Wiki

```
webroot/
├── index.md             # Wiki home
├── projects/
│   ├── project-alpha/
│   │   ├── index.md
│   │   ├── requirements.md
│   │   ├── architecture.md
│   │   └── deployment.md
│   └── project-beta/
│       ├── index.md
│       └── README.md
├── processes/
│   ├── index.md
│   ├── onboarding.md
│   ├── code-review.md
│   └── deployment.md
├── resources/
│   ├── index.md
│   ├── tools.md
│   ├── links.md
│   └── templates/
│       ├── meeting-notes.md
│       └── project-proposal.md
└── team/
    ├── index.md
    ├── directory.md
    └── contact-info.md
```

## Performance Tips

### Optimize Large Sites

```bash
# Use compression for better performance
cargo run -- --config production.toml

# Monitor with detailed logging
cargo run -- --loglevel debug --log logs/performance.log

# Production setup with minimal logging
cargo run -- --loglevel error --log logs/error.log
```

### Directory Organization Best Practices

1. **Keep related content together** - Group related markdown files in directories
2. **Use consistent naming** - `index.md` for directory landing pages
3. **Optimize images** - Store images in `assets/images/` subdirectories
4. **Separate concerns** - Keep CSS, JS, and images in `assets/`
5. **Use meaningful URLs** - Structure directories to create intuitive URLs

### Deployment Script Example

```bash
#!/bin/bash
# deploy.sh

echo "🚀 Deploying Barleywine site..."

# Build optimized version
cargo build --release

# Copy content to server directory
rsync -av webroot/ /var/www/html/

# Start server with production config
./target/release/barleywine \
  --config production.toml \
  --loglevel info \
  --log /var/log/barleywine/server.log

echo "✅ Deployment complete!"
```

## Conclusion

Barleywine's flexibility allows you to create everything from simple static sites to complex documentation platforms. The combination of HTML and Markdown support, along with comprehensive CLI options, makes it suitable for a wide variety of use cases.

Key advantages:

- **Zero build step** - Edit markdown files and see changes immediately
- **Flexible structure** - Mix HTML and Markdown as needed
- **Performance** - Rust-powered speed with minimal resource usage
- **Simple deployment** - Single binary with configuration files
- **Developer-friendly** - Comprehensive CLI and logging options

Start with a simple structure and grow your site organically as your needs evolve!
