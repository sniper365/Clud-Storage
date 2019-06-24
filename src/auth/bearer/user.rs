use super::Bearer;
use super::Error;
use db::models::User;
use db::DbPool;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use schema::*;
use serde_json::Map;
use serde_json::Value;

impl Bearer for User {
    fn header(&self) -> Value {
        Value::Object(Map::new())
    }

    fn verify(payload: Value) -> Result<Self, Error> {
        let conn = DbPool::connection();

        let user_id: i32 = payload
            .get("user_id")
            .unwrap_or(&Value::Null)
            .as_u64()
            .unwrap_or(0) as i32;

        match Self::all().filter(users::id.eq(user_id)).first(&conn) {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::JWTInvalid),
        }
    }
}
