use crate::{
    db::schema::item,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type Archetype = Select<Find<item::table, Id<id::Item>>, item::archetype>;

impl Id<id::Item> {
    #[allow(dead_code)]
    pub(crate) fn archetype(&self) -> Archetype {
        item::table.find(*self).select(item::archetype)
    }
}
