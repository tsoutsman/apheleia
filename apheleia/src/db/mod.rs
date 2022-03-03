pub(crate) mod model;
pub(crate) mod schema;

mod func;
pub(crate) use func::*;

pub(crate) type DbPool =
    diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;
