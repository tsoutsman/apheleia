use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLObject)]
pub(crate) struct Archetype {
    id: Uuid,
}

#[derive(GraphQLInputObject)]
pub(crate) struct ArchetypeInput {
    id: Uuid,
}
