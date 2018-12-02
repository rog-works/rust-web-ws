#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rocket_contrib;
extern crate serde;

#[macro_use] extern crate serde_derive;
// extern crate chrono;

mod models;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, devices])
        .launch();
}
