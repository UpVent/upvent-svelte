#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::sqlite::SqliteConnection;
use rocket::fs::NamedFile;
use rocket::serde::uuid::Uuid;
use std::io;
use std::path::{Path, PathBuf};
use upventrs_server::connect_sqlite;
use uuid::Uuid;

mod models;

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

/// Get all testimonials (defined in the testimonials model)
#[get("/testimonial/<id>", format = "application/json")]
fn get_testimonial(id: String) {
    let conn: SqliteConnection = connect_sqlite();
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
