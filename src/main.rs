#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod router;

fn main() {
    router::create_routes();
}