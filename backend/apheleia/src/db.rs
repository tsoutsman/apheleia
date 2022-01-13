use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub(crate) async fn pool() -> crate::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:hunter2@db")
        .await
        .map_err(|e| e.into())
}
