#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<string>")]
fn new_string(string: String) -> String {
    format!("Hello, {}", string.as_str())
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![world])
        .mount("/mystring", routes![new_string])
        .launch();
}