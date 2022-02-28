use std::marker::PhantomData;

use crate::{graphql::item::Item, Context};

use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use uuid::Uuid;

pub(crate) struct Archetype<'a> {
    id: Uuid,
    __phantom_data: PhantomData<&'a ()>,
}

#[graphql_object(context = Context<'a>)]
impl<'a> Archetype<'a> {
    fn id(&self) -> Uuid {
        self.id
    }

    async fn items(&self, _ctx: &Context<'a>) -> FieldResult<Vec<Item<'_>>> {
        todo!("populate items");
    }
}

#[derive(GraphQLInputObject)]
pub(crate) struct ArchetypeInput {
    id: Uuid,
}
