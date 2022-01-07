use crate::db::Conn as DbConn;
use crate::models::{NewPrivacyPolicy, PrivacyPolicy};
use rocket::serde::json::{json, Json, Value};

#[get("/privacypolicies", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let fsprojects: Vec<PrivacyPolicy> = PrivacyPolicy::all(&conn);

    Json(json!({
        "status": 200,
        "result": fsprojects,
    }))
}

#[post(
    "/privacypolicies",
    format = "application/json",
    data = "<new_privacypolicy>"
)]
pub fn new(conn: DbConn, new_privacypolicy: Json<NewPrivacyPolicy>) -> Json<Value> {
    Json(json!({
        "status": PrivacyPolicy::insert(new_privacypolicy.into_inner(), &conn),
        "result": PrivacyPolicy::all(&conn).first(),
    }))
}

#[get("/privacypolicies/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<PrivacyPolicy> = PrivacyPolicy::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put(
    "/privacypolicies/<id>",
    format = "application/json",
    data = "<privacypolicy>"
)]
pub fn update(conn: DbConn, id: i32, privacypolicy: Json<NewPrivacyPolicy>) -> Json<Value> {
    let status: i32 = if PrivacyPolicy::update_by_id(id, &conn, privacypolicy.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/privacypolicies/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if PrivacyPolicy::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/privacypolicies/names/<title>", format = "application/json")]
pub fn title(title: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": PrivacyPolicy::all_by_title(title, &conn),
    }))
}
