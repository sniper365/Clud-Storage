use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use pg_pool::DbConn;
use models::user::User;

use models::*;
use schema::users::dsl::*;

use rocket_contrib::Json;

// #[get("/register")]
