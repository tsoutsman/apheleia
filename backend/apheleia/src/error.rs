use actix_web::{http::StatusCode, ResponseError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error authentication user")]
    Authentication,
    #[error("user has insufficient rights")]
    Authorisation,
    #[error("unknown database error")]
    Database(#[from] sqlx::Error),
    #[error("unknown I/O error")]
    Io(#[from] std::io::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Authentication => StatusCode::UNAUTHORIZED,
            Error::Authorisation => StatusCode::FORBIDDEN,
            Error::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
