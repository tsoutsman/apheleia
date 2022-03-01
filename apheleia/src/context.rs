use sqlx::PgPool;

pub(crate) struct Context<'a> {
    pub(crate) user: crate::extractor::User,
    pub(crate) pool: &'a PgPool,
}
