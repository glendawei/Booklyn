pub mod handlers;
pub mod database;
pub mod api_doc;
mod error;

use sqlx::postgres::PgPool;

pub struct AppData {
    pub db_conn: PgPool,
}
