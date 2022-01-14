use std::marker::PhantomData;

use crate::{graphql::user::User, Context};

use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub(crate) struct Item<'a> {
    pub(crate) id: Uuid,
    pub(crate) loaner: Uuid,
    pub(crate) loanee: Uuid,
    __phantom_data: PhantomData<&'a ()>,
}

#[graphql_object(context = Context<'a>)]
impl<'a> Item<'a> {
    fn id(&self) -> Uuid {
        self.id
    }

    async fn loaner(&self, _ctx: &Context<'a>) -> FieldResult<User<'_>> {
        todo!("populate loaner");
    }

    async fn loanee(&self, _ctx: &Context<'a>) -> FieldResult<User<'_>> {
        todo!("populate loaner");
    }
}
