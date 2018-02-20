#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate serde;

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
extern crate time;
extern crate rand;

mod pg_pool;
pub use pg_pool::DbConn;

mod schema;
mod controllers;
mod models;
mod guards;
mod requests;
mod libraries;
mod resources;

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
            user_controller::password,
            user_controller::delete,
            folder_controller::index,
            folder_controller::show,
            folder_controller::children,
            folder_controller::store,
            folder_controller::update,
            folder_controller::delete,
            file_controller::index,
            file_controller::show,
            file_controller::download,
            file_controller::store_file,
            file_controller::store,
            file_controller::update,
            file_controller::delete,
        ])
        .catch(errors![
            error_controller::bad_request,
            error_controller::unauthorized,
            error_controller::forbidden,
            error_controller::not_found,
            error_controller::internal_server_error,
        ])
        .launch();
}
