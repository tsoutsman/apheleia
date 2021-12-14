pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    DatabaseConnection,
    Io,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}

impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Self::DatabaseConnection
    }
}
