use rocket::{
    catch, catchers, delete, get, post, put, routes,
    serde::json::{serde_json::json, Value},
};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_products() -> Value {
    json!("Product::all()")
}

#[get("/<id>")]
async fn get_product(id: i32) -> Value {
    json!("Product::find(id)")
}

#[post("/")]
async fn create_product() -> Value {
    json!("Product::create()")
}

#[put("/<id>")]
async fn update_product(id: i32) -> Value {
    json!("Product::update(id)")
}

#[delete("/<id>")]
async fn delete_product(id: i32) -> Value {
    json!("Product::delete(id)")
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
        .launch()
        .await?;
    Ok(())
}
