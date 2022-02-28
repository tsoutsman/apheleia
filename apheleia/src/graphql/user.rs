use std::marker::PhantomData;

use crate::{graphql::loan::Loan, Context};

use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub(crate) struct User<'a> {
    pub(crate) id: Uuid,
    pub(crate) loans: Vec<Uuid>,
    __phantom_data: PhantomData<&'a ()>,
}

#[graphql_object(context = Context<'a>)]
impl<'a> User<'a> {
    fn id(&self) -> Uuid {
        self.id
    }

    async fn loans(&self, _ctx: &Context<'a>) -> FieldResult<Vec<Loan<'_>>> {
        todo!("populate loans");
    }
}
