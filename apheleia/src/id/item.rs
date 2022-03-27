use crate::{
    db::schema::item,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type FindQuery = Find<item::table, Id<id::Item>>;
type ArchetypeQuery = Select<FindQuery, item::archetype>;

impl Id<id::Item> {
    pub(crate) fn find(&self) -> FindQuery {
        item::table.find(*self)
    }

    #[allow(dead_code)]
    pub(crate) fn archetype(&self) -> ArchetypeQuery {
        self.find().select(item::archetype)
    }
}
