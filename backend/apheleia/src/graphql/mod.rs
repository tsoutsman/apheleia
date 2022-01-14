mod archetype;
mod item;
mod loan;
mod mutation;
mod query;
mod role;
mod route;
mod settings;
mod subject_area;
mod user;
mod util;

pub(crate) use route::{graphiql_route, graphql_route, playground_route};
pub(crate) use subject_area::SubjectArea;

// TODO max query depth

type Schema<'a> = juniper::RootNode<
    'a,
    query::Query<'a>,
    mutation::Mutation<'a>,
    juniper::EmptySubscription<crate::Context<'a>>,
>;

pub(crate) fn schema<'a>() -> Schema<'a> {
    Schema::new(
        query::Query::new(),
        mutation::Mutation::new(),
        juniper::EmptySubscription::<crate::Context<'a>>::new(),
    )
}
