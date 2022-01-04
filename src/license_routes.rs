use crate::db::Conn as DbConn;
use crate::models::{License, NewLicense};
use rocket::serde::json::{json, Json, Value};

#[get("/licenses", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let licenses: Vec<License> = License::all(&conn);

    Json(json!({
        "status": 200,
        "result": licenses,
    }))
}

#[post("/licenses", format = "application/json", data = "<new_license>")]
pub fn new(conn: DbConn, new_license: Json<NewLicense>) -> Json<Value> {
    Json(json!({
        "status": License::insert(new_license.into_inner(), &conn),
        "result": License::all(&conn).first(),
    }))
}

#[get("/licenses/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<License> = License::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/licenses/<id>", format = "application/json", data = "<license>")]
pub fn update(conn: DbConn, id: i32, license: Json<NewLicense>) -> Json<Value> {
    let status: i32 = if License::update_by_id(id, &conn, license.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/licenses/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if License::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/licenses/names/<name>", format = "application/json")]
pub fn name(name: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": License::all_by_name(name, &conn),
    }))
}
