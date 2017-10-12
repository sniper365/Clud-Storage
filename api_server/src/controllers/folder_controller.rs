use rocket::http::Cookie;
use rocket::http::Cookies;
use rocket_contrib::Json;

use diesel;
use diesel::LoadDsl;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;

use bcrypt::verify;

use pg_pool::DbConn;

use models::user::User;

use guards::auth_guard::Token;

use requests::folder_request;
use responses::folder_response;

#[get("/folders")]
fn index(conn: DbConn, auth: Auth) -> Json<folder_response::FolderList>
