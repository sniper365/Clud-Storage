#![feature(proc_macro_hygiene, decl_macro, specialization)]
#![feature(optin_builtin_traits)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate rand;
extern crate time;
#[macro_use]
extern crate lazy_static;
extern crate diesel_derive_enum;
extern crate frank_jwt;
extern crate postgres;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_multipart_form_data;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate fake;

#[cfg(test)]
#[macro_use]
mod test;

mod auth;
mod controllers;
mod db;
mod env;
mod policies;
mod schema;
mod services;
mod web;

fn main() {
    // Load .env file
    dotenv::dotenv().expect("Missing .env file");

    web::boot()
}
