use diesel::{
    backend::Backend,
    sql_types::Integer,
    types::{FromSql, ToSql},
    Queryable,
};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Id(pub(crate) u32);

impl From<i32> for Id {
    fn from(v: i32) -> Self {
        Self(u32::from_be_bytes(v.to_be_bytes()))
    }
}

impl From<Id> for i32 {
    fn from(v: Id) -> Self {
        i32::from_be_bytes(v.0.to_be_bytes())
    }
}

impl<DB, ST> Queryable<ST, DB> for Id
where
    DB: Backend,
    i32: Queryable<ST, DB>,
{
    type Row = <i32 as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        i32::build(row).into()
    }
}

impl<DB> ToSql<Integer, DB> for Id
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<'_, W, DB>,
    ) -> diesel::serialize::Result {
        i32::from(*self).to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for Id
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(Self::from(i32::from_sql(bytes)?))
    }
}
