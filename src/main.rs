#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[cfg(test)]
mod unit_tests;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Rust-Rocket!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

fn main() {
    rocket::ignite().mount("/", routes![index, ping]).launch();
}
