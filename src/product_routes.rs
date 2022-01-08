use crate::db::Conn as DbConn;
use crate::models::{NewProduct, Product};
use rocket::serde::json::{json, Json, Value};

#[get("/teammebers", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let products = Product::all(&conn);

    Json(json!({
        "status": 200,
        "result": products,
    }))
}

#[post("/products", format = "application/json", data = "<new_product>")]
pub fn new(conn: DbConn, new_product: Json<NewProduct>) -> Json<Value> {
    Json(json!({
        "status": Product::insert(new_product.into_inner(), &conn),
        "result": Product::all(&conn).first(),
    }))
}

#[get("/products/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<Product> = Product::show(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/products/<id>", format = "application/json", data = "<product>")]
pub fn update(conn: DbConn, id: i32, product: Json<NewProduct>) -> Json<Value> {
    let status: i32 = if Product::update_by_id(id, &conn, product.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/products/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status: i32 = if Product::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/products/names/<name>", format = "application/json")]
pub fn name(name: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Product::all_by_name(name, &conn),
    }))
}
