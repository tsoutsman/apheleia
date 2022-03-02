use std::marker::PhantomData;

use diesel::{
    backend::Backend,
    sql_types,
    types::{FromSql, ToSql},
    FromSqlRow,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, AsExpression, FromSqlRow, Serialize, Deserialize,
)]
#[sql_type = "diesel::sql_types::Uuid"]
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
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<'_, W, DB>,
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
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
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
