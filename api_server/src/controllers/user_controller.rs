use rocket_contrib::Json;

use diesel::LoadDsl;

use pg_pool::DbConn;

use models::user::User;

use responses::user_response;

use guards::auth_guard::Auth;

#[get("/users")]
fn index(conn: DbConn, auth: Auth) -> Json<user_response::Index> {
    use schema::users::dsl;

    if !auth.user.is_admin(&conn) {
        return Json(user_response::Index {
            success: false,
            message: String::from("You are not an admin"),
            users: None,
        })
    }

    let users = match dsl::users.load::<User>(&*conn) {
        Ok(users) => users,
        Err(_) => return Json(user_response::Index {
            success: false,
            message: String::from("Failed to query users"),
            users: None,
        })
    };

    let response: Vec<user_response::Show> = users.into_iter().map(| user | {
        user_response::Show {
            success: true,
            message: String::from("User found"),
            user_id: Some(user.id),
            email: Some(user.email),
            first_name: Some(user.first_name),
            last_name: Some(user.last_name),
        }
    }).collect();

    return Json(user_response::Index {
        success: true,
        message: String::from("Found list of users"),
        users: Some(response),
    })
}
