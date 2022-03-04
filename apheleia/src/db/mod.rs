pub(crate) mod model;
pub(crate) mod schema;
pub(crate) mod tokio;

mod func;


pub(crate) type DbPool =
    diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;
