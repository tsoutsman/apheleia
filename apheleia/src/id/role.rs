use crate::{
    db::schema::role,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type FindQuery = Find<role::table, Id<id::Role>>;
type SubjectAreaQuery = Select<FindQuery, role::subject_area>;

impl Id<id::Role> {
    pub(crate) fn find(&self) -> FindQuery {
        role::table.find(*self)
    }

    pub(crate) fn subject_area(&self) -> SubjectAreaQuery {
        self.find().select(role::subject_area)
    }
}
