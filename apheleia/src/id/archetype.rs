use crate::{
    db::schema::archetype,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type SubjectArea = Select<Find<archetype::table, Id<id::Archetype>>, archetype::subject_area>;

impl Id<id::Archetype> {
    pub(crate) fn subject_area(&self) -> SubjectArea {
        archetype::table.find(*self).select(archetype::subject_area)
    }
}
