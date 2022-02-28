use std::marker::PhantomData;

use crate::{graphql::item::Item, Context};

use chrono::{offset::Utc, DateTime};
use juniper::{graphql_object, FieldResult, GraphQLEnum};
use uuid::Uuid;

use super::user::User;

#[derive(GraphQLEnum)]
pub(crate) enum LoanType {
    /// Items the user has lent out to other users.
    Out,
    /// Items that have been lent to the user.
    To,
    /// Items that the user has write access to (i.e. items that the user can lend out).
    Subordinate,
}

pub(crate) struct Loan<'a> {
    pub(crate) id: Uuid,
    pub(crate) item: Uuid,
    pub(crate) loaner: Uuid,
    pub(crate) loanee: Uuid,
    pub(crate) date_loaned: DateTime<Utc>,
    pub(crate) date_returned: Option<DateTime<Utc>>,
    __phantom_data: PhantomData<&'a ()>,
}

#[graphql_object(context = Context<'a>)]
impl<'a> Loan<'a> {
    // Does anything seem wrong? :)
    fn id() -> Uuid {
        self.id
    }

    async fn item(&self, _ctx: &Context<'a>) -> FieldResult<Item<'_>> {
        todo!("populate item");
    }

    fn loaner(&self) -> FieldResult<User<'_>> {
        todo!("populate loaner");
    }

    fn loanee(&self) -> FieldResult<User<'_>> {
        todo!("populate loanee");
    }

    fn date_loaned(&self) -> DateTime<Utc> {
        self.date_loaned
    }

    fn date_returned(&self) -> Option<DateTime<Utc>> {
        self.date_returned
    }
}
