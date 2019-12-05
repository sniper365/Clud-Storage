pub mod builders;
pub mod models;
mod pool;
pub mod presentation;
pub mod query;
pub mod relationships;

use self::models::Model;
pub use self::pool::DbPool;
use self::query::Query;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;

pub struct DbFacade;

impl DbFacade {
    pub fn connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        DbPool::connection()
    }
}

pub trait Entity: Model + Query {}

impl Entity for models::User {}
impl Entity for models::Folder {}
impl Entity for models::File {}
