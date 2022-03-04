use crate::{
    db::schema::archetype,
    id::{Archetype, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type SubjectArea = Select<Find<archetype::table, Id<Archetype>>, archetype::subject_area>;

impl Id<Archetype> {
    pub(crate) fn subject_area(&self) -> SubjectArea {
        archetype::table.find(*self).select(archetype::subject_area)
    }
}
