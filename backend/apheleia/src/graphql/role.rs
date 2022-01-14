use std::marker::PhantomData;

use crate::{graphql::user::User, Context};

use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use uuid::Uuid;

pub(crate) struct Role<'a> {
    id: Uuid,
    __phantom_data: PhantomData<&'a ()>,
}

#[graphql_object(context = Context<'a>)]
impl<'a> Role<'a> {
    fn id(&self) -> Uuid {
        self.id
    }

    async fn users(&self, _ctx: &Context<'a>) -> FieldResult<Vec<User<'_>>> {
        todo!("populate users");
    }
}

#[derive(GraphQLInputObject)]
pub(crate) struct RoleInput {
    id: Uuid,
}
