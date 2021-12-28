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

mod models;

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

/// Get ALL testimonials
#[get("/testimonials")]
fn all_testimonials() -> Option<Json<Vec<Testimonial>>> {
    //TODO: Cosas
}

/// Get a specific testimonial (Providing it's corresponding UUID)
#[get("/testimonial/<id>", format = "application/json")]
fn get_testimonial(id: Uuid) {
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
