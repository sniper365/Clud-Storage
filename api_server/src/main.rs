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
extern crate crypto;
extern crate frank_jwt;

mod pg_pool;
pub use pg_pool::DbConn;

mod schema;
mod controllers;
mod models;
mod guards;
mod requests;
mod responses;
mod libraries;

use dotenv::dotenv;
use std::env;

use controllers::*;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/api", routes![
            session_controller::login,
            user_controller::index,
            user_controller::show,
            user_controller::store,
            user_controller::update,
            user_controller::delete,
            folder_controller::index,
            folder_controller::show,
            folder_controller::children,
        ])
        .launch();
}
