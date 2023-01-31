use base64::{engine::general_purpose, Engine as _};
use rocket::{
    catch, catchers, delete, get,
    http::Status,
    log, post, put, request, routes,
    serde::json::{serde_json::json, Value},
};

// Basic authentication, for example:
// Basic base64(username:password)
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
async fn get_products(auth: BasicAuth) -> Value {
    println!("{}:{}", auth.username, auth.password);
    json!("Product::all()")
}

#[get("/<id>")]
async fn get_product(id: i32, auth: BasicAuth) -> Value {
    json!("Product::find(id)")
}

#[post("/")]
async fn create_product(auth: BasicAuth) -> Value {
    json!("Product::create()")
}

#[put("/<id>")]
async fn update_product(id: i32, auth: BasicAuth) -> Value {
    json!("Product::update(id)")
}

#[delete("/<id>")]
async fn delete_product(id: i32, auth: BasicAuth) -> Value {
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
