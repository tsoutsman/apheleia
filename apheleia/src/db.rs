use apheleia_proc::queries;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// TODO: Juniper dataloader

use crate::{graphql::SubjectArea, Result};

#[inline]
pub(crate) async fn pool() -> Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:hunter2@db")
        .await
        .map_err(|e| e.into())
}

#[inline]
pub(crate) async fn init_schema(area: SubjectArea, pool: &Pool<Postgres>) -> Result<()> {
    // TODO: Transaction
    for query in queries!("sql/init_schema.sql", schema = area.schema_name()) {
        query.execute(pool).await?;
    }
    Ok(())
}

#[inline]
pub(crate) async fn schema_exists(area: SubjectArea, pool: &Pool<Postgres>) -> Result<bool> {
    Ok(sqlx::query(&format!(
        include_str!("sql/schema_exists.sql"),
        schema = area.schema_name()
    ))
    .fetch_optional(pool)
    .await?
    .is_some())
}

// IDK if I wanna use this

// #[allow(dead_code)]
// pub(crate) struct Query<T>(pub(crate) T)
// where
//     T: AsRef<str>;
//
// // I could make these methods generic but not worth the effort.
// macro_rules! impl_db_methods {
//     ($($async:ident $fn:ident $($id:ident)? () -> $ty:ty;)*) => {
//         $(
//             impl_db_methods!(@$async@$fn@$($id)?@$ty);
//         )*
//     };
//     (@fn@$f:ident@@$ty:ty) => {
//         #[inline]
//         pub(crate) async fn $f<'e>(&'e self, pool: &::sqlx::Pool<::sqlx::Postgres>) -> ::futures::stream::BoxStream<'e, ::sqlx::Result<$ty>>{
//             ::sqlx::query(self.0.as_ref())
//                 .$f(pool)
//         }
//     };
//     (@async@fn@$f:ident@$ty:ty) => {
//         #[inline]
//         pub(crate) async fn $f(&self, pool: &::sqlx::Pool<::sqlx::Postgres>) -> crate::Result<$ty> {
//             ::sqlx::query(self.0.as_ref())
//                 .$f(pool)
//                 .await
//                 .map_err(|e| e.into())
//         }
//     };
// }
//
// #[allow(dead_code)]
// impl<T> Query<T>
// where
//     T: AsRef<str>,
// {
//     impl_db_methods![
//         async fn execute() -> <Postgres as Database>::QueryResult;
//         async fn fetch_optional() -> Option<PgRow>;
//         async fn fetch_one() -> PgRow;
//         fn fetch() -> <Postgres as Database>::Row;
//         async fn fetch_all() -> Vec<<Postgres as Database>::Row>;
//     ];
// }
