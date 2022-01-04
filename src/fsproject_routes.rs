use crate::db::Conn as DbConn;
use crate::models::{FSProject, NewFSProject};
use rocket::serde::json::{json, Json, Value};

#[get("/fsprojects", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let fsprojects: Vec<FSProject> = FSProject::all(&conn);

    Json(json!({
        "status": 200,
        "result": fsprojects,
    }))
}

#[post("/fsprojects", format = "application/json", data = "<new_fsproject>")]
pub fn new(conn: DbConn, new_fsproject: Json<NewFSProject>) -> Json<Value> {
    Json(json!({
        "status": FSProject::insert(new_fsproject.into_inner(), &conn),
        "result": FSProject::all(&conn).first(),
    }))
}

#[get("/fsprojects/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<FSProject> = FSProject::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/fsprojects/<id>", format = "application/json", data = "<fsproject>")]
pub fn update(conn: DbConn, id: i32, fsproject: Json<NewFSProject>) -> Json<Value> {
    let status: i32 = if FSProject::update_by_id(id, &conn, fsproject.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/fsprojects/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if FSProject::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/fsprojects/names/<title>", format = "application/json")]
pub fn title(title: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": FSProject::all_by_title(title, &conn),
    }))
}
