#![feature(decl_macro, proc_macro_hygiene)]
#![feature(rustc_private)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod posts;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    posts::router::create_routes();
}
