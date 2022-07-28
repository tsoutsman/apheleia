use crate::{
    db::schema::{archetype, item, loan},
    id::{self, Id},
};

use diesel::{
    dsl::{Eq, Find, InnerJoin, InnerJoinOn, Select},
    ExpressionMethods, JoinOnDsl, QueryDsl,
};

// type SubjectArea = Select<Find<role::table, Id<id::Role>>, role::subject_area>;
type SubjectArea = Select<
    InnerJoinOn<
        InnerJoin<Select<Find<loan::table, Id<id::Loan>>, loan::item>, item::table>,
        archetype::table,
        Eq<item::archetype, archetype::id>,
    >,
    archetype::subject_area,
>;

impl Id<id::Loan> {
    pub(crate) fn subject_area(&self) -> SubjectArea {
        loan::table
            .find(*self)
            .select(loan::item)
            .inner_join(item::table)
            .inner_join(archetype::table.on(item::archetype.eq(archetype::id)))
            .select(archetype::subject_area)
    }
}
