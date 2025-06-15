use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::{get, launch, routes};
use std::path::{Path, PathBuf};

#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let webroot = Path::new("webroot");
    let mut path = webroot.join(&file);

    // If the path is a directory, try to serve index.html
    if path.is_dir() {
        path = path.join("index.html");
    }

    // If no file segments provided, try to serve webroot/index.html
    if file.components().count() == 0 {
        path = webroot.join("index.html");
    }

    match NamedFile::open(&path).await {
        Ok(file) => Ok(file),
        Err(_) => Err(NotFound(format!("File not found: {}", path.display()))),
    }
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("webroot/index.html");
    match NamedFile::open(path).await {
        Ok(file) => Ok(file),
        Err(_) => Err(NotFound("index.html not found".to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
