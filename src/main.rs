#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rocket::fs::NamedFile;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

/* Crate modules */
mod db;
mod models;
mod routes;
mod schema;

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

#[get("/<file..>", rank = 5)]
async fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    //    let conn = SqliteConnection::establish(&db_url).unwrap();
    //
    //    let testimonial = models::NewTestimonial {
    //        name: String::from("VentGrey"),
    //        testimonial: String::from("This testimonial text is a test. The cake is a lie!"),
    //        workplace: String::from("UpVent Technologies"),
    //        website: String::from("https://upvent.codes/"),
    //    };
    //
    //    if models::Testimonial::insert(testimonial, &conn) {
    //        println!("Testimonial inserted correctly!");
    //    } else {
    //        println!("Something failed while inserting the testimonial!");
    //    }

    let pool = db::init_pool(db_url);

    rocket::build().manage(pool).mount("/", routes![all, index])
}
