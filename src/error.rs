use actix_web::{http::StatusCode, ResponseError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error authenticating caller")]
    Authentication,
    #[error("caller has insufficient rights")]
    Authorisation,
    #[error("unknown I/O error")]
    Io(#[from] std::io::Error),
    #[error("request timed out")]
    Timeout(#[from] tokio::time::error::Elapsed),
    #[error("unknown database connection pool error")]
    R2d2(#[from] r2d2::Error),
    #[error("error occured during database migration")]
    Migration,
    #[error("not found")]
    NotFound(diesel::result::Error),
    #[error("database error")]
    Database(diesel::result::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Authentication => StatusCode::UNAUTHORIZED,
            Error::Authorisation => StatusCode::FORBIDDEN,
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Timeout(_) => StatusCode::BAD_REQUEST,
            Error::R2d2(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Migration => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => Self::NotFound(e),
            _ => Self::Database(e),
        }
    }
}
