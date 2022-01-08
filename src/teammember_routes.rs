use crate::db::Conn as DbConn;
use crate::models::{NewTeamMember, TeamMember};
use rocket::serde::json::{json, Json, Value};

#[get("/teammembers", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let teammembers = TeamMember::all(&conn);

    Json(json!({
        "status": 200,
        "result": teammembers,
    }))
}

#[post("/teammembers", format = "application/json", data = "<new_teammember>")]
pub fn new(conn: DbConn, new_teammember: Json<NewTeamMember>) -> Json<Value> {
    Json(json!({
        "status": TeamMember::insert(new_teammember.into_inner(), &conn),
        "result": TeamMember::all(&conn).first(),
    }))
}

#[get("/teammembers/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<TeamMember> = TeamMember::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put(
    "/teammembers/<id>",
    format = "application/json",
    data = "<teammember>"
)]
pub fn update(conn: DbConn, id: i32, teammember: Json<NewTeamMember>) -> Json<Value> {
    let status: i32 = if TeamMember::update_by_id(id, &conn, teammember.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/teammembers/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if TeamMember::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/teammembers/names/<name>", format = "application/json")]
pub fn name(name: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": TeamMember::all_by_name(name, &conn),
    }))
}
