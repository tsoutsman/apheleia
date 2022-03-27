use crate::{
    db::schema::loan,
    id::{self, Id},
};

use diesel::{
    dsl::{Find, Select},
    QueryDsl,
};

type FindQuery = Find<loan::table, Id<id::Loan>>;

impl Id<id::Loan> {
    pub(crate) fn find(&self) -> FindQuery {
        loan::table.find(*self)
    }
}
