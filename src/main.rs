#[macro_use]
extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Serialize};

mod models;

#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct Testimonial {
    name: String,
    testimonial: String,
    workplace: String,
    website: String,
}

#[get("/testimonials")]
fn testimonials() -> Json<Testimonial> {
    Json(Testimonial {
        name: "Juan Camaney López Rodríguez".to_string(),
        testimonial: "Todo 10/10".to_string(),
        workplace: "CEO".to_string(),
        website: "https://github.com".to_string(),
    })
}

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
    rocket::build().mount("/", routes![index, files, testimonials])
}
