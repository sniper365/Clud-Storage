#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate lazy_static;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate chrono;
extern crate bcrypt;

mod pg_pool;
pub use pg_pool::DbConn;

mod schema;
mod controllers;
mod models;
mod guards;
mod requests;
mod responses;

use dotenv::dotenv;
use std::env;

use controllers::*;

fn main() {
    // initialize env
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", routes![
            session_controller::login,
            session_controller::logout,
        ])
        .launch();
}
