use rocket::fs::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
pub async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

#[get("/<file..>", rank = 5)]
pub async fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}
