use actix_web::{http::StatusCode, ResponseError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error authentication user")]
    Authentication,
    #[error("user has insufficient rights")]
    Authorisation,
    #[error("unknown I/O error")]
    Io(#[from] std::io::Error),
    #[error("request timed out")]
    Timeout(#[from] tokio::time::error::Elapsed),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Authentication => StatusCode::UNAUTHORIZED,
            Error::Authorisation => StatusCode::FORBIDDEN,
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Timeout(_) => StatusCode::BAD_REQUEST,
        }
    }
}
