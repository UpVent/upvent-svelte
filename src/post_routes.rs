use crate::db::Conn as DbConn;
use crate::models::{NewPost, Post};
use rocket::serde::json::{json, Json, Value};

#[get("/posts", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let posts = Post::all(&conn);

    Json(json!({
        "status": 200,
        "result": posts,
    }))
}

#[post("/posts", format = "application/json", data = "<new_post>")]
pub fn new(conn: DbConn, new_post: Json<NewPost>) -> Json<Value> {
    Json(json!({
        "status": Post::insert(new_post.into_inner(), &conn),
        "result": Post::all(&conn).first(),
    }))
}

#[get("/posts/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<Post> = Post::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update(conn: DbConn, id: i32, post: Json<NewPost>) -> Json<Value> {
    let status: i32 = if Post::update_by_id(id, &conn, post.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/posts/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if Post::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/posts/names/<title>", format = "application/json")]
pub fn title(title: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Post::all_by_title(title, &conn),
    }))
}
