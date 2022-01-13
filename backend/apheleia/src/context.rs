use crate::extractor::User;

use sqlx::PgPool;

pub(crate) struct Context<'a> {
    pub(crate) user: User,
    pub(crate) pool: &'a PgPool,
}

impl<'a> juniper::Context for Context<'a> {}
