use crate::{
    db::schema::archetype,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type FindQuery = Find<archetype::table, Id<id::Archetype>>;
type SubjectAreaQuery = Select<FindQuery, archetype::subject_area>;

impl Id<id::Archetype> {
    pub(crate) fn find(&self) -> FindQuery {
        archetype::table.find(*self)
    }

    pub(crate) fn subject_area(&self) -> SubjectAreaQuery {
        self.find().select(archetype::subject_area)
    }
}
