use crate::{config, log};
use rocket::fs::NamedFile;
use rocket::http::uri::Origin;
use rocket::response::{content::RawHtml, status::NotFound};
use rocket::{get, routes};
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

// Custom response type to handle both static files and generated HTML
pub enum FileResponse {
    Static(NamedFile),
    Markdown(RawHtml<String>),
}

impl<'r> rocket::response::Responder<'r, 'static> for FileResponse {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            FileResponse::Static(file) => file.respond_to(request),
            FileResponse::Markdown(html) => html.respond_to(request),
        }
    }
}

#[get("/<file..>")]
async fn files(
    file: PathBuf,
    remote_addr: Option<SocketAddr>,
    origin: &Origin<'_>,
) -> Result<FileResponse, NotFound<String>> {
    let config = config::get_config();
    let mut path = config.content.webroot.join(&file);

    // If the path is a directory, try to serve configured index files
    if path.is_dir() {
        let mut found_index = false;
        for index_file in &config.content.index_files {
            let index_path = path.join(index_file);
            if index_path.exists() {
                path = index_path;
                found_index = true;
                break;
            }
        }

        if !found_index {
            return Err(NotFound(format!(
                "No index file found in directory: {}. Looking for: {:?}",
                path.display(),
                config.content.index_files
            )));
        }
    }

    // If no file segments provided, try to serve configured index files from webroot
    if file.components().count() == 0 {
        let mut found_index = false;
        for index_file in &config.content.index_files {
            let index_path = config.content.webroot.join(index_file);
            if index_path.exists() {
                path = index_path;
                found_index = true;
                break;
            }
        }

        if !found_index {
            return Err(NotFound(format!(
                "No index file found. Looking for: {:?}",
                config.content.index_files
            )));
        }
    }

    // Check if the file exists
    if !path.exists() {
        // Log access attempt for non-existent file
        let addr_str = remote_addr
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        log::log_access(&addr_str, "GET", origin.path().as_str(), 404, None);

        return Err(NotFound(format!("File not found: {}", path.display())));
    }

    // Log successful access
    let addr_str = remote_addr
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    log::log_access(&addr_str, "GET", origin.path().as_str(), 200, None);

    // Handle markdown files (if enabled)
    if config.content.markdown_enabled {
        if let Some(extension) = path.extension() {
            if extension == "md" {
                log::log_file_served(&path.display().to_string(), "markdown");
                return serve_markdown_file(&path).await;
            }
        }
    }

    // Serve regular files
    match NamedFile::open(&path).await {
        Ok(file) => {
            log::log_file_served(&path.display().to_string(), "static");
            Ok(FileResponse::Static(file))
        }
        Err(_) => {
            // Log access attempt for file that couldn't be opened
            let addr_str = remote_addr
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "unknown".to_string());
            log::log_access(&addr_str, "GET", origin.path().as_str(), 500, None);
            Err(NotFound(format!("Could not open file: {}", path.display())))
        }
    }
}

#[get("/")]
async fn index(remote_addr: Option<SocketAddr>) -> Result<FileResponse, NotFound<String>> {
    let config = config::get_config();

    // Log access to root
    let addr_str = remote_addr
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Try each configured index file in order
    for index_file in &config.content.index_files {
        let index_path = config.content.webroot.join(index_file);

        if index_path.exists() {
            log::log_access(&addr_str, "GET", "/", 200, None);

            // Check if it's a markdown file and markdown is enabled
            if config.content.markdown_enabled && index_file.ends_with(".md") {
                log::log_file_served(index_file, "markdown");
                return serve_markdown_file(&index_path).await;
            } else if !index_file.ends_with(".md") {
                // Serve as static file
                match NamedFile::open(&index_path).await {
                    Ok(file) => {
                        log::log_file_served(index_file, "static");
                        return Ok(FileResponse::Static(file));
                    }
                    Err(_) => {
                        log::log_access(&addr_str, "GET", "/", 500, None);
                        return Err(NotFound(format!("Could not open {}", index_file)));
                    }
                }
            }
        }
    }

    // No index file found
    log::log_access(&addr_str, "GET", "/", 404, None);
    Err(NotFound(format!(
        "No index file found. Looking for: {:?}",
        config.content.index_files
    )))
}

async fn serve_markdown_file(path: &Path) -> Result<FileResponse, NotFound<String>> {
    // Read the markdown file
    let markdown_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            return Err(NotFound(format!(
                "Could not read markdown file: {}",
                path.display()
            )))
        }
    };

    // Convert markdown to HTML
    let html_content = markdown::to_html(&markdown_content);

    // Extract title from the first # heading if present
    let title = extract_title(&markdown_content);

    // Wrap in HTML template
    let full_html = create_html_template(&html_content, &title);

    Ok(FileResponse::Markdown(RawHtml(full_html)))
}

fn extract_title(markdown: &str) -> String {
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return trimmed[2..].trim().to_string();
        }
    }
    "Untitled".to_string()
}

fn create_html_template(content: &str, title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #fff;
        }}

        h1, h2, h3, h4, h5, h6 {{
            color: #2c3e50;
            margin-top: 30px;
            margin-bottom: 15px;
        }}

        h1 {{
            border-bottom: 2px solid #3498db;
            padding-bottom: 10px;
        }}

        h2 {{
            border-bottom: 1px solid #bdc3c7;
            padding-bottom: 5px;
        }}

        p {{
            margin-bottom: 15px;
        }}

        code {{
            background-color: #f8f9fa;
            padding: 2px 4px;
            border-radius: 3px;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 0.9em;
        }}

        pre {{
            background-color: #f8f9fa;
            border: 1px solid #e9ecef;
            border-radius: 5px;
            padding: 15px;
            overflow-x: auto;
            margin: 20px 0;
        }}

        pre code {{
            background-color: transparent;
            padding: 0;
        }}

        blockquote {{
            border-left: 4px solid #3498db;
            margin: 20px 0;
            padding: 10px 20px;
            background-color: #f8f9fa;
            font-style: italic;
        }}

        ul, ol {{
            margin-bottom: 15px;
            padding-left: 30px;
        }}

        li {{
            margin-bottom: 5px;
        }}

        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 20px 0;
        }}

        th, td {{
            border: 1px solid #ddd;
            padding: 12px;
            text-align: left;
        }}

        th {{
            background-color: #f8f9fa;
            font-weight: bold;
        }}

        a {{
            color: #3498db;
            text-decoration: none;
        }}

        a:hover {{
            text-decoration: underline;
        }}

        img {{
            max-width: 100%;
            height: auto;
            border-radius: 5px;
            margin: 10px 0;
        }}

        hr {{
            border: none;
            border-top: 1px solid #bdc3c7;
            margin: 30px 0;
        }}

        .markdown-body {{
            margin-top: 20px;
        }}

        .back-link {{
            display: inline-block;
            margin-bottom: 20px;
            color: #6c757d;
            font-size: 0.9em;
        }}

        .back-link:hover {{
            color: #3498db;
        }}

        @media (max-width: 768px) {{
            body {{
                padding: 15px;
            }}

            h1 {{
                font-size: 1.8em;
            }}

            pre {{
                padding: 10px;
                font-size: 0.85em;
            }}

            table {{
                font-size: 0.9em;
            }}
        }}
    </style>
</head>
<body>
    <div class="markdown-body">
        {}
    </div>

    <script>
        // Add some interactivity for better UX
        document.addEventListener('DOMContentLoaded', function() {{
            // Add anchor links to headings
            const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
            headings.forEach(function(heading) {{
                if (heading.id) {{
                    const anchor = document.createElement('a');
                    anchor.href = '#' + heading.id;
                    anchor.innerHTML = '#';
                    anchor.className = 'header-anchor';
                    anchor.style.cssText = 'margin-left: 10px; color: #bdc3c7; text-decoration: none; font-weight: normal;';
                    anchor.style.display = 'none';

                    heading.appendChild(anchor);

                    heading.addEventListener('mouseenter', function() {{
                        anchor.style.display = 'inline';
                    }});

                    heading.addEventListener('mouseleave', function() {{
                        anchor.style.display = 'none';
                    }});
                }}
            }});

            // Add copy button to code blocks
            const codeBlocks = document.querySelectorAll('pre code');
            codeBlocks.forEach(function(codeBlock) {{
                const pre = codeBlock.parentElement;
                const button = document.createElement('button');
                button.textContent = 'Copy';
                button.style.cssText = 'position: absolute; top: 10px; right: 10px; background: #6c757d; color: white; border: none; padding: 5px 10px; border-radius: 3px; font-size: 0.8em; cursor: pointer;';

                pre.style.position = 'relative';
                pre.appendChild(button);

                button.addEventListener('click', function() {{
                    navigator.clipboard.writeText(codeBlock.textContent).then(function() {{
                        button.textContent = 'Copied!';
                        setTimeout(function() {{
                            button.textContent = 'Copy';
                        }}, 2000);
                    }});
                }});
            }});

            console.log('📝 Markdown rendered successfully with Barleywine!');
        }});
    </script>
</body>
</html>"#,
        title, content
    )
}

pub fn build_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, files])
}
