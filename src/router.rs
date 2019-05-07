use rocket;

pub fn create_routes() {
    rocket::ignite()
        .mount("/hello", routes![world])
        .mount("/mystring", routes![new_string])
        .launch();
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<string>")]
fn new_string(string: String) -> String {
    format!("Hello, {}", string.as_str())
}