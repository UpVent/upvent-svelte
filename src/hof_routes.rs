use crate::db::Conn as DbConn;
use crate::models::{NewHOF, HOF};
use rocket::serde::json::{json, Json, Value};

#[get("/hofs", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let hofs: Vec<HOF> = HOF::all(&conn);

    Json(json!({
        "status": 200,
        "result": hofs,
    }))
}

#[post("/hofs", format = "application/json", data = "<new_hof>")]
pub fn new(conn: DbConn, new_hof: Json<NewHOF>) -> Json<Value> {
    Json(json!({
        "status": HOF::insert(new_hof.into_inner(), &conn),
        "result": HOF::all(&conn).first(),
    }))
}

#[get("/hofs/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<HOF> = HOF::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/hofs/<id>", format = "application/json", data = "<hof>")]
pub fn update(conn: DbConn, id: i32, hof: Json<NewHOF>) -> Json<Value> {
    let status: i32 = if HOF::update_by_id(id, &conn, hof.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/hofs/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if HOF::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/hofs/names/<name>", format = "application/json")]
pub fn name(name: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": HOF::all_by_name(name, &conn),
    }))
}
