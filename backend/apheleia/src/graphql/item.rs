use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLEnum)]
pub(crate) enum ItemsType {
    /// Items the user has lent out to other users.
    LentOut,
    /// Items that have been lent to the user.
    LentTo,
    /// Items that the user has write access to (i.e. items that the user can lend out).
    Subordinate,
}

#[derive(GraphQLObject)]
pub(crate) struct Item {
    pub(crate) id: Uuid,
    // Some other fields
}
