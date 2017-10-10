use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use pg_pool::DbConn;
use models::user::User;

use models::*;
use schema::users::dsl::*;

use rocket_contrib::Json;

use bcrypt::{DEFAULT_COST, hash, verify};

#[get("/login")]
fn login(conn: DbConn) -> Json<Vec<User>> {
    users.load::<User>(&*conn)
        .map(| u | Json(u))
        .unwrap()
}
