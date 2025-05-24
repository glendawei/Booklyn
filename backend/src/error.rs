use thiserror;
use actix_web::error::ResponseError;
use sqlx::error::Error as SqlxError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    DatabaseError(String),
}

impl From<SqlxError> for Error {
    fn from(e: SqlxError) -> Error {
        Error::DatabaseError(e.to_string())
    }
}

impl ResponseError for Error {}
