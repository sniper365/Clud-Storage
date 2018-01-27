use rocket_contrib::Json;

use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::FirstDsl;

use bcrypt::verify;

use pg_pool::DbConn;

use models::user::User;

use requests::session_request;
use models::session::Session;

use libraries::jwt::Token;

use rocket::Response;
use rocket::http::{ Status, ContentType };
use std::io::Cursor;

use resources::AsResource;

#[post("/login", data="<request>")]
fn login(conn: DbConn, request: Json<session_request::Login>) -> Response<'static> {
    use schema::users::dsl::{users, email as user_email };

    // Find the user in the database that they claim to be
    let user = match users.filter(user_email.eq(request.0.email))
        .first::<User>(&*conn) {
        Ok(user) => user,
        Err(_) => return Response::build()
            .status(Status::Unauthorized)
            .header(ContentType::JSON)
            .finalize(),
    };

    // Check their password: if it fails to encrypt: their password is unacceptable
    let matched = match verify(&request.0.password, &user.password) {
        Ok(matched) => matched,
        Err(_) => return Response::build()
            .status(Status::Unauthorized)
            .header(ContentType::JSON)
            .finalize(),
    };

    // If it matched: make a token of their identity and return it
    if matched {
        let token = Token::new(user.id, user.name.clone());

        let response = Session::new(token, user);

        return Response::build()
            .status(Status::Ok)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(response.as_response()))
            .finalize();
    }

    Response::build()
        .status(Status::Unauthorized)
        .header(ContentType::JSON)
        .finalize()
}
