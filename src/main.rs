#[macro_use]
extern crate diesel;

use base64::{engine::general_purpose, Engine as _};
use diesel::{
    query_dsl::methods::{FindDsl, LimitDsl},
    ExpressionMethods, RunQueryDsl,
};
use rocket::{
    catch, catchers, delete, get,
    http::Status,
    post, put, request, routes,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_sync_db_pools::database;

mod models;
mod schema;

use models::{NewProduct, Product};
use schema::products;

#[database("sqlite_database")]
struct DbConn(diesel::SqliteConnection);

// Basic authentication, for example:
// Basic base64(username:password)
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn from_header(header: &str) -> Option<Self> {
        let header = header.trim_start_matches("Basic ");
        let decoded = general_purpose::STANDARD.decode(header).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let mut split = decoded.splitn(2, ':');
        let username = split.next()?.to_string();
        let password = split.next()?.to_string();
        Some(Self::new(username, password))
    }
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(header) = request.headers().get_one("Authorization") {
            if let Some(auth) = Self::from_header(header) {
                return request::Outcome::Success(auth);
            }
        }
        request::Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_products(auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(|c| {
        let products = products::table
            .limit(100)
            .load::<Product>(c)
            .expect("Error loading products");
        json!(products)
    })
    .await
}

#[get("/<id>")]
async fn get_product(id: i32, auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(move |c| {
        // let product = products::table.find(id).get_result::<Product>(c).expect("error");
        let product = products::table
            .find(id)
            .first::<Product>(c)
            .expect("Error loading product");
        json!(product)
    })
    .await
}

#[post("/", format = "json", data = "<new_product>")]
async fn create_product(auth: BasicAuth, conn: DbConn, new_product: Json<NewProduct>) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let count = diesel::insert_into(products::table)
            .values(new_product.into_inner())
            .execute(c)
            .expect("Error create new product");
        json!(count)
    })
    .await
}

#[put("/<id>", format = "json", data = "<product>")]
async fn update_product(id: i32, product: Json<Product>, auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let count = diesel::update(products::table.find(id))
            .set((
                products::name.eq(&product.name),
                products::description.eq(&product.description),
            ))
            .execute(c)
            .expect("Error update product");
        json!(count)
    })
    .await
}

#[delete("/<id>")]
async fn delete_product(id: i32, auth: BasicAuth, conn: DbConn) -> Value {
    let _ = auth;
    conn.run(move |c| {
        let count = diesel::delete(products::table.find(id))
            .execute(c)
            .expect("Error delete product");
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
