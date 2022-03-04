use crate::db::schema::archetype;

use std::marker::PhantomData;

use diesel::{
    backend::{Backend, HasRawValue},
    deserialize::{FromSql, FromSqlRow},
    dsl::{Find, Select},
    expression::AsExpression,
    serialize::ToSql,
    sql_types, QueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, AsExpression, FromSqlRow, Serialize, Deserialize,
)]
#[diesel(sql_type = diesel::sql_types::Uuid)]
// AsExpression derive doesn't allow where clauses.
pub(crate) struct Id<T: Sealed>(Uuid, PhantomData<T>);

impl<T> Id<T>
where
    T: Sealed,
{
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4(), PhantomData)
    }
}

impl<DB, T> ToSql<sql_types::Uuid, DB> for Id<T>
where
    DB: Backend,
    Uuid: ToSql<sql_types::Uuid, DB>,
    T: Sealed + std::fmt::Debug,
{
    fn to_sql<'a>(
        &'a self,
        out: &mut diesel::serialize::Output<'a, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

impl<DB, T> FromSql<sql_types::Uuid, DB> for Id<T>
where
    DB: Backend,
    Uuid: FromSql<sql_types::Uuid, DB>,
    T: Sealed,
{
    fn from_sql(bytes: <DB as HasRawValue<'_>>::RawValue) -> diesel::deserialize::Result<Self> {
        Ok(Self(Uuid::from_sql(bytes)?, PhantomData))
    }
}

pub(crate) trait Sealed: private::Private {}

mod private {
    pub(crate) trait Private {}
}

macro_rules! id_struct {
    ($($id:ident),*$(,)?) => {
        $(
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub(crate) struct $id;
            impl private::Private for $id {}
            impl Sealed for $id {}
        )*
    };
}

id_struct![SubjectArea, Role, Archetype, Item, Loan];

type SubjectAreaQuery = Select<Find<archetype::table, Id<Archetype>>, archetype::subject_area>;

impl Id<Archetype> {
    pub(crate) fn subject_area(&self) -> SubjectAreaQuery {
        archetype::table.find(*self).select(archetype::subject_area)
    }
}
