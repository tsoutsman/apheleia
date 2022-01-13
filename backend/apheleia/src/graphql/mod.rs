mod archetype;
mod item;
mod mutation;
mod query;
mod role;
mod route;
mod settings;
mod subject_area;

use crate::Context;
use mutation::Mutation;
use query::Query;

use juniper::{EmptySubscription, RootNode};

pub(crate) use route::{graphiql_route, graphql_route, playground_route};
pub(crate) use subject_area::SubjectArea;

// TODO static lifetimes
type Schema<'a> = RootNode<'a, Query<'a>, Mutation<'a>, EmptySubscription<Context<'a>>>;

pub(crate) fn schema<'a>() -> Schema<'a> {
    Schema::new(
        Query::new(),
        Mutation::new(),
        EmptySubscription::<Context<'a>>::new(),
    )
}
