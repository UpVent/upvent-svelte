#[macro_use]
extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::{content, status};

use rocket::serde::{json::Json, Serialize};

#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct Testimonial {
    name: String,
    title: String,
    website: String,
}

#[get("/testimonials")]
fn testimonials() -> Json<Testimonial> {
    Json(Testimonial {
        name: "Juan Camaney López Rodríguez".to_string(),
        title: "CEO".to_string(),
        website: "https://github.com".to_string(),
    })
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html").await
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files, testimonials])
}
