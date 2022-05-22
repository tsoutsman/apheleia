use crate::{
    db::schema::role,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type SubjectArea = Select<Find<role::table, Id<id::Role>>, role::subject_area>;

impl Id<id::Role> {
    pub(crate) fn subject_area(&self) -> SubjectArea {
        role::table.find(*self).select(role::subject_area)
    }
}
