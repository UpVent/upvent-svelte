#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use crate::routes::*;
use dotenv::dotenv;
use std::env;

/* Crate modules */
mod db;
mod models;
mod routes;
mod schema;
mod static_files;

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

    rocket::build()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author, update],
        )
        .mount(
            "/",
            routes![crate::static_files::all, crate::static_files::index],
        )
        .register("/", catchers![not_found])
        .register("/api/v1/", catchers![not_found])
}
