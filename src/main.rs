#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Per directory rust nightly build override
// rustup override set nightly
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

// linking with `cc` failed docker build rust:
// https://users.rust-lang.org/t/linking-error-in-docker/29563
// You will need to install libpq inside the container, as diesel depends on it to connect to a PostgreSQL database.