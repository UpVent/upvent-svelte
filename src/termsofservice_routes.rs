use crate::db::Conn as DbConn;
use crate::models::{NewTermsOfService, TermsOfService};
use rocket::serde::json::{json, Json, Value};

#[get("/termsofservices", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let terms: Vec<TermsOfService> = TermsOfService::all(&conn);

    Json(json!({
        "status": 200,
        "result": terms,
    }))
}

#[post("/termsofservices", format = "application/json", data = "<new_terms>")]
pub fn new(conn: DbConn, new_terms: Json<NewTermsOfService>) -> Json<Value> {
    Json(json!({
        "status": TermsOfService::insert(new_terms.into_inner(), &conn),
        "result": TermsOfService::all(&conn).first(),
    }))
}

#[get("/termsofservices/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<TermsOfService> = TermsOfService::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/termsofservices/<id>", format = "application/json", data = "<terms>")]
pub fn update(conn: DbConn, id: i32, terms: Json<NewTermsOfService>) -> Json<Value> {
    let status: i32 = if TermsOfService::update_by_id(id, &conn, terms.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/termsofservices/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if TermsOfService::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/termsofservices/names/<title>", format = "application/json")]
pub fn title(title: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": TermsOfService::all_by_title(title, &conn),
    }))
}
