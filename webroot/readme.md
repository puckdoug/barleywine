# 🍺 Barleywine Server

Welcome to **Barleywine** - a blazing-fast static file server built with Rust and Rocket that automatically converts Markdown files to beautifully styled HTML!

## ✨ Live Markdown Conversion Demo

You're currently viewing a **`.md` file** that has been automatically converted to HTML with beautiful styling applied. This demonstrates one of Barleywine's key features: seamless markdown-to-HTML conversion.

### 🚀 Key Features

- **Static File Serving**: Serves any file type with proper MIME detection
- **Markdown Conversion**: Automatically converts `.md` files to styled HTML
- **Directory Indexes**: Auto-serves `index.html` or `index.md` for directories
- **Zero Configuration**: Just drop files in `webroot` and go!
- **Lightning Fast**: Built with Rust for maximum performance
- **Security**: Files served only from designated webroot directory

## 📁 File Structure Demo

```
webroot/
├── index.html          # Traditional HTML (served at /)
├── readme.md          # This file! (served at /readme.md)
├── sample.md          # Markdown demo with advanced features
├── blog.md           # Blog-style markdown content
├── blog/             # Directory with markdown index
│   ├── index.md      # Auto-served when visiting /blog/
│   └── post-1.md     # Individual blog post
├── subdir/           # Mixed content directory
│   └── index.html    # HTML takes precedence over markdown
├── styles.css        # CSS with proper MIME type
├── script.js         # JavaScript with proper MIME type
└── test.txt          # Plain text files
```

## 🎯 Test the Features

### Markdown Files
- [📝 Sample Markdown](/sample.md) - Advanced markdown features demo
- [📰 Blog Content](/blog.md) - Blog-style content
- [📁 Blog Directory](/blog/) - Directory with markdown index
- [📖 Blog Post](/blog/post-1.md) - Individual blog post

### Static Files
- [🎨 CSS Stylesheet](/styles.css) - Served as `text/css`
- [⚙️ JavaScript File](/script.js) - Served as `application/javascript`
- [📄 Text File](/test.txt) - Served as `text/plain`
- [📁 HTML Subdirectory](/subdir/) - Traditional HTML index

### Mixed Content
- [🏠 HTML Home](/) - Traditional HTML index page
- [📁 This Markdown File](/readme.md) - You are here!

## 💻 How It Works

### 1. **File Request Processing**
```rust
// Pseudo-code showing the request flow
if path.is_dir() {
    // Look for index.html first, then index.md
    serve_index_file(path)
} else if path.extension() == "md" {
    // Convert markdown to HTML with styling
    convert_markdown_to_html(path)
} else {
    // Serve static file with proper MIME type
    serve_static_file(path)
}
```

### 2. **Markdown Conversion Pipeline**
1. Read `.md` file from filesystem
2. Parse markdown using `markdown-rs` crate
3. Convert to HTML content
4. Extract title from first `# heading`
5. Wrap in responsive HTML template
6. Serve as `text/html` with proper styling

### 3. **MIME Type Detection**
| Extension | MIME Type | Example |
|-----------|-----------|---------|
| `.html`, `.htm` | `text/html` | Web pages |
| `.md` | `text/html` | Converted markdown |
| `.css` | `text/css` | Stylesheets |
| `.js` | `application/javascript` | Scripts |
| `.json` | `application/json` | Data files |
| `.png`, `.jpg` | `image/*` | Images |
| `.pdf` | `application/pdf` | Documents |

## 🛠️ Technical Implementation

### Server Architecture
- **Framework**: Rocket 0.5.1 (Rust web framework)
- **Markdown Parser**: `markdown` crate (CommonMark compliant)
- **Async Runtime**: Tokio for high-performance I/O
- **File Serving**: Zero-copy file serving with proper headers

### Performance Characteristics
- **Memory Efficient**: Streams files without loading entirely into memory
- **CPU Optimized**: Markdown conversion cached per request
- **Concurrent**: Handles thousands of simultaneous connections
- **Secure**: Path traversal protection built-in

## 🎨 Styling Features

The automatic HTML conversion includes:

### Typography
- **Clean fonts**: System font stack for optimal readability
- **Proper spacing**: Consistent margins and line heights
- **Responsive sizing**: Adapts to different screen sizes

### Code Highlighting
```rust
fn main() {
    println!("🍺 Hello from Barleywine!");
}
```

```javascript
console.log("Markdown converted to HTML! 🎉");
```

```bash
# Start the server
cargo run

# Visit in browser
open http://localhost:8000
```

### Interactive Elements
- **Copy buttons** on code blocks
- **Hover effects** on headings
- **Responsive tables** with proper formatting
- **Mobile-friendly** design

## 📊 Comparison with Alternatives

| Feature | Barleywine | Static Generators | Traditional CMS |
|---------|------------|-------------------|-----------------|
| **Setup Time** | ⚡ Instant | 🕐 Minutes | 🕒 Hours |
| **Live Editing** | ✅ Real-time | ❌ Build step | ✅ Database |
| **Performance** | 🚀 Excellent | 🚀 Excellent | 🐌 Variable |
| **Security** | 🔒 High | 🔒 High | ⚠️ Complex |
| **Deployment** | 📦 Single binary | 📁 Static files | 🗄️ Full stack |

## 🏃 Getting Started

### Quick Start
```bash
# Clone or create project
cd barleywine

# Add content to webroot
echo "# My Page" > webroot/my-page.md

# Start server
cargo run

# View your page
open http://localhost:8000/my-page.md
```

### Development Workflow
1. **Write content** in Markdown
2. **Save to webroot** directory
3. **Refresh browser** - see changes instantly!
4. **No build step** required

### Directory Organization
```
my-site/
├── webroot/
│   ├── index.md          # Homepage
│   ├── about.md         # About page
│   ├── blog/
│   │   ├── index.md     # Blog index
│   │   ├── post-1.md    # Blog posts
│   │   └── post-2.md
│   ├── assets/
│   │   ├── style.css    # Custom styles
│   │   └── images/      # Static images
│   └── docs/
│       └── api.md       # Documentation
└── src/
    └── main.rs          # Server code
```

## 🌟 Advanced Features

### Custom Styling
Override default styles by including CSS files:
```html
<!-- In your markdown, this gets preserved -->
<link rel="stylesheet" href="/custom.css">
```

### Mixed Content
Combine markdown and HTML seamlessly:
- Use `.html` for complex layouts
- Use `.md` for content-heavy pages
- Mix both in the same directory structure

### Template Customization
The HTML template includes:
- Responsive viewport meta tags
- Mobile-friendly CSS
- Print-friendly styles
- Accessibility features
- Interactive JavaScript enhancements

## 📈 Use Cases

### Perfect For:
- **Documentation sites** - Write docs in markdown
- **Personal blogs** - Simple content management
- **Project wikis** - Version-controlled content
- **Prototyping** - Quick static site development
- **Team knowledge bases** - Collaborative editing

### Example Workflows:
1. **Technical Documentation**: Write in markdown, commit to git, auto-deploy
2. **Blog Publishing**: Create `.md` files, organize in directories
3. **Team Wiki**: Collaborative editing with version history
4. **API Documentation**: Mix markdown content with HTML examples

## 🔧 Configuration Options

### Server Settings
```toml
# Rocket.toml
[default]
port = 8000
address = "127.0.0.1"

# Custom settings (if implemented)
[barleywine]
webroot = "content"
markdown_extensions = ["tables", "strikethrough"]
```

### Environment Variables
```bash
# Change port
ROCKET_PORT=3000 cargo run

# Custom webroot (if implemented)
WEBROOT_DIR=/path/to/content cargo run
```

## 🤝 Contributing

Want to enhance Barleywine? Ideas for contributions:

- **Syntax highlighting** for code blocks
- **Custom themes** support
- **Hot reload** during development
- **Markdown extensions** (math, diagrams)
- **Template customization** options
- **Asset processing** pipeline

## 📝 Conclusion

Barleywine demonstrates the power of combining:
- **Rust's performance** and safety
- **Rocket's simplicity** and features
- **Markdown's ease** of authoring
- **Modern web standards** for presentation

The result is a **zero-configuration** static site server that makes content creation as simple as writing markdown files!

---

## 🎯 Try It Now!

1. **Edit this file**: Modify `webroot/readme.md`
2. **Refresh the page**: See your changes instantly
3. **Create new files**: Add more `.md` files to explore
4. **Build something awesome**: Use Barleywine for your next project!

*This page was generated from `readme.md` using Barleywine's automatic markdown conversion.* 🍺

**System Info:**
- Server: Barleywine (Rust + Rocket)
- Markdown Parser: `markdown-rs` v1.0
- Styling: Custom responsive CSS template
- Rendered: On-demand from filesystem

[🏠 Back to HTML Home](/) | [📝 View More Samples](/sample.md) | [📁 Explore Blog](/blog/)
