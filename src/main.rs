#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use rocket::serde::json::{json, Json, Value};
use std::env;

/* Crate modules */

// Database pool handling module
mod db;

// Database models + methods module
mod models;

// Database Schema autogenerated by Diesel
mod schema;

// Static file handler module
mod static_files;

// Testimonial Model routes
mod testimonial_routes;

// Project Routes
mod project_routes;

// FSProject Routes
mod fsproject_routes;

// License Routes
mod license_routes;

// Hall Of Fame Routes
mod hof_routes;

// Team Members Routes
// Privacy Policy Routes
// Terms Of Service Routes
// Blog Post Routes
// Product Routes

/* Universal 404 handler */
#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found"
    }))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let pool = db::init_pool(db_url);

    rocket::build()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![
                // Testimonial Routes
                crate::testimonial_routes::index,
                crate::testimonial_routes::new,
                crate::testimonial_routes::show,
                crate::testimonial_routes::delete,
                crate::testimonial_routes::author,
                crate::testimonial_routes::update,
                // Project Routes
                crate::project_routes::index,
                crate::project_routes::new,
                crate::project_routes::show,
                crate::project_routes::delete,
                crate::project_routes::title,
                crate::project_routes::update,
                // FSProject Routes
                crate::fsproject_routes::index,
                crate::fsproject_routes::new,
                crate::fsproject_routes::show,
                crate::fsproject_routes::delete,
                crate::fsproject_routes::title,
                crate::fsproject_routes::update,
                // License Routes
                crate::license_routes::index,
                crate::license_routes::new,
                crate::license_routes::show,
                crate::license_routes::delete,
                crate::license_routes::name,
                crate::license_routes::update,
                // Hall Of Fame Routes
                crate::hof_routes::index,
                crate::hof_routes::new,
                crate::hof_routes::show,
                crate::hof_routes::delete,
                crate::hof_routes::name,
                crate::hof_routes::update,
                // Team Members Routes
                // Privacy Policy Routes
                // Terms Of Service Routes
                // Blog Post Routes
                // Product Routes
            ],
        )
        .mount(
            "/",
            routes![crate::static_files::all, crate::static_files::index],
        )
}
