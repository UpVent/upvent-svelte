use crate::db::Conn as DbConn;
use crate::models::{NewTestimonial, Testimonial};
use rocket::serde::json::{json, Json, Value};

#[get("/testimonials", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let testimonials = Testimonial::all(&conn);

    Json(json!({
        "status": 200,
        "result": testimonials,
    }))
}

#[post(
    "/testimonials",
    format = "application/json",
    data = "<new_testimonial>"
)]
pub fn new(conn: DbConn, new_testimonial: Json<NewTestimonial>) -> Json<Value> {
    Json(json!({
        "status": Testimonial::insert(new_testimonial.into_inner(), &conn),
        "result": Testimonial::all(&conn).first(),
    }))
}

#[get("/testimonials/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Testimonial::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put(
    "/testimonials/<id>",
    format = "application/json",
    data = "<testimonial>"
)]
pub fn update(conn: DbConn, id: i32, testimonial: Json<NewTestimonial>) -> Json<Value> {
    let status = if Testimonial::update_by_id(id, &conn, testimonial.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/testimonials/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Testimonial::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/testimonials/names/<name>", format = "application/json")]
pub fn author(name: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Testimonial::all_by_name(name, &conn),
    }))
}
