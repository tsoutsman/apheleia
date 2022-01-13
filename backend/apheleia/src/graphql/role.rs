use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLObject)]
pub(crate) struct Role {
    id: Uuid,
}

#[derive(GraphQLInputObject)]
pub(crate) struct RoleInput {
    id: Uuid,
}
