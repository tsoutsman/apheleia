use crate::{
    db::{schema::archetype, DbPool},
    id::{self, Id},
    Result,
};

use diesel::QueryDsl;
use tokio_diesel::AsyncRunQueryDsl;

pub(crate) async fn get_archetypes_subject_area(
    pool: &DbPool,
    archetype_id: Id<id::Archetype>,
) -> Result<Id<id::SubjectArea>> {
    archetype::table
        .find(archetype_id)
        .select(archetype::subject_area)
        .first_async::<Id<id::SubjectArea>>(pool)
        .await
        .map_err(|e| e.into())
}
