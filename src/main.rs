#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::fs::NamedFile;
use std::io;

/* Crate modules */
mod models;
mod schema;

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
