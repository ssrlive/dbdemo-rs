#[macro_use]
extern crate diesel;

use rocket::{
    catch, catchers, delete, get, post, put, routes,
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

#[database("sqlite_database")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_products(auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(|c| {
        let products = ProductRepository::get_products(c).expect("Error loading products");
        json!(products)
    })
    .await
}

#[get("/<id>")]
async fn get_product(id: i32, auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let product = ProductRepository::get_product(id, c).expect("Error loading product");
        json!(product)
    })
    .await
}

#[post("/", format = "json", data = "<new_product>")]
async fn create_product(auth: BasicAuth, conn: DbConn, new_product: Json<NewProduct>) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let product = ProductRepository::create_product(new_product.into_inner(), c)
            .expect("Error create product");
        json!(product)
    })
    .await
}

#[put("/<id>", format = "json", data = "<product>")]
async fn update_product(id: i32, product: Json<Product>, auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let mut product = product.into_inner();
        product.id = id;
        let product = ProductRepository::update_product(product, c).expect("Error update product");
        json!(product)
    })
    .await
}

#[delete("/<id>")]
async fn delete_product(id: i32, auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let count = ProductRepository::delete_product(id, c).expect("Error delete product");
        json!(count)
    })
    .await
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
