use rocket::fs::NamedFile;
use rocket::response::{content::RawHtml, status::NotFound};
use rocket::{get, launch, routes};
use std::fs;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<FileResponse, NotFound<String>> {
    let webroot = Path::new("webroot");
    let mut path = webroot.join(&file);

    // If the path is a directory, try to serve index.html or index.md
    if path.is_dir() {
        let html_index = path.join("index.html");
        let md_index = path.join("index.md");

        if html_index.exists() {
            path = html_index;
        } else if md_index.exists() {
            path = md_index;
        } else {
            return Err(NotFound(format!(
                "No index file found in directory: {}",
                path.display()
            )));
        }
    }

    // If no file segments provided, try to serve webroot/index.html or webroot/index.md
    if file.components().count() == 0 {
        let html_index = webroot.join("index.html");
        let md_index = webroot.join("index.md");

        if html_index.exists() {
            path = html_index;
        } else if md_index.exists() {
            path = md_index;
        } else {
            return Err(NotFound("No index file found".to_string()));
        }
    }

    // Check if the file exists
    if !path.exists() {
        return Err(NotFound(format!("File not found: {}", path.display())));
    }

    // Handle markdown files
    if let Some(extension) = path.extension() {
        if extension == "md" {
            return serve_markdown_file(&path).await;
        }
    }

    // Serve regular files
    match NamedFile::open(&path).await {
        Ok(file) => Ok(FileResponse::Static(file)),
        Err(_) => Err(NotFound(format!("Could not open file: {}", path.display()))),
    }
}

#[get("/")]
async fn index() -> Result<FileResponse, NotFound<String>> {
    let webroot = Path::new("webroot");
    let html_index = webroot.join("index.html");
    let md_index = webroot.join("index.md");

    if html_index.exists() {
        match NamedFile::open(&html_index).await {
            Ok(file) => Ok(FileResponse::Static(file)),
            Err(_) => Err(NotFound("Could not open index.html".to_string())),
        }
    } else if md_index.exists() {
        serve_markdown_file(&md_index).await
    } else {
        Err(NotFound("No index file found".to_string()))
    }
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
