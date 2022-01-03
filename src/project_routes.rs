use crate::db::Conn as DbConn;
use crate::models::{NewProject, Project};
use rocket::serde::json::{json, Json, Value};

#[get("/projects", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let projects: Vec<Project> = Project::all(&conn);

    Json(json!({
        "status": 200,
        "result": projects,
    }))
}

#[post("/projects", format = "application/json", data = "<new_project>")]
pub fn new(conn: DbConn, new_project: Json<NewProject>) -> Json<Value> {
    Json(json!({
        "status": Project::insert(new_project.into_inner(), &conn),
        "result": Project::all(&conn).first(),
    }))
}

#[get("/projects/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<Project> = Project::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/projects/<id>", format = "application/json", data = "<project>")]
pub fn update(conn: DbConn, id: i32, project: Json<NewProject>) -> Json<Value> {
    let status: i32 = if Project::update_by_id(id, &conn, project.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/projects/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if Project::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/projects/names/<title>", format = "application/json")]
pub fn title(title: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Project::all_by_title(title, &conn),
    }))
}
