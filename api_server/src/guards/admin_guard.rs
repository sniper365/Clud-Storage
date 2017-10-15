use rocket::request::FromRequest;
use rocket::{Request, State, Outcome};
use rocket::request;
use rocket::http::Status;

use guards::auth_guard::Auth;

use r2d2;
use r2d2_diesel::ConnectionManager;
use std::ops::Deref;

use models::user::User;
use models::role::Role;
use models::role_user::RoleUser;

use diesel::pg::PgConnection;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;

type ManagedPgConn = ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ManagedPgConn>;

pub struct Admin {
    pub user: User,
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {
        // I want access to the database pool to check if the user is an admin
        let pool = request.guard::<State<Pool>>()?;

        // Get a connection; if one is not available, reply as such
        let conn = match pool.get() {
            Ok(conn) => conn,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        // Check if the user is authenticated:
        // If so: get the user, if not return Unauthorized
        let auth = request.guard::<Auth>()?;

        use schema::role_user::dsl::*;
        use schema::roles::dsl::*;

        let admin = match roles.filter(name.eq("admin"))
        .first::<Role>(conn.deref()) {
            Ok(role) => role,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        match role_user.filter(user_id.eq(auth.user.id))
        .filter(role_id.eq(admin.id))
        .first::<RoleUser>(conn.deref()) {
            Ok(_) => return Outcome::Success(Admin { user: auth.user }),
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
