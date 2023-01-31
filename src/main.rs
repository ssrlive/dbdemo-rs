#[macro_use]
extern crate diesel;

use rocket::{
    catch, catchers, delete, get,
    http::Status,
    post, put,
    response::status,
    routes,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_sync_db_pools::database;

mod basic_auth;
mod models;
mod repositories;
mod schema;

use basic_auth::BasicAuth;
use models::{NewProduct, Product};
use repositories::ProductRepository;

type ResultValue = Result<Value, status::Custom<Value>>;

#[database("sqlite_database")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_products(auth: BasicAuth, conn: DbConn) -> ResultValue {
    let _ = auth;
    conn.run(|c| {
        ProductRepository::get_products(c)
            .map(|p| json!(p))
            .map_err(error_handler)
    })
    .await
}

#[get("/<id>")]
async fn get_product(id: i32, auth: BasicAuth, conn: DbConn) -> ResultValue {
    let _ = auth;
    conn.run(move |c| {
        ProductRepository::get_product(id, c)
            .map(|p| json!(p))
            .map_err(error_handler)
    })
    .await
}

#[post("/", format = "json", data = "<new_product>")]
async fn create_product(
    auth: BasicAuth,
    conn: DbConn,
    new_product: Json<NewProduct>,
) -> ResultValue {
    let _ = auth;
    conn.run(move |c| {
        ProductRepository::create_product(new_product.into_inner(), c)
            .map(|p| json!(p))
            .map_err(error_handler)
    })
    .await
}

#[put("/<id>", format = "json", data = "<product>")]
async fn update_product(
    id: i32,
    product: Json<Product>,
    auth: BasicAuth,
    conn: DbConn,
) -> ResultValue {
    let _ = auth;
    conn.run(move |c| {
        let mut product = product.into_inner();
        product.id = id;
        ProductRepository::update_product(product, c)
            .map(|p| json!(p))
            .map_err(error_handler)
    })
    .await
}

#[delete("/<id>")]
async fn delete_product(id: i32, auth: BasicAuth, conn: DbConn) -> ResultValue {
    let _ = auth;
    conn.run(move |c| {
        ProductRepository::delete_product(id, c)
            .map(|p| json!(p))
            .map_err(error_handler)
    })
    .await
}

fn error_handler(e: diesel::result::Error) -> status::Custom<Value> {
    status::Custom(Status::InternalServerError, json!(e.to_string()))
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("404")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/products",
            routes![
                get_products,
                get_product,
                create_product,
                update_product,
                delete_product
            ],
        )
        .register("/", catchers![not_found_url])
        .attach(DbConn::fairing())
        .launch()
        .await?;
    Ok(())
}
