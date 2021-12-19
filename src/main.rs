#[macro_use]
extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;

mod models;

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
