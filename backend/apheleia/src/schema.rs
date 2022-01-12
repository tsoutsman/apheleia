use std::marker::PhantomData;

use crate::{
    settings::{Settings, SettingsInput},
    subject_area::SubjectArea,
    Context,
};

use juniper::{graphql_object, EmptySubscription, GraphQLEnum, RootNode};

#[derive(GraphQLEnum)]
enum ItemsType {
    /// Items the user has lent out to other users.
    LentOut,
    /// Items that have been lent to the user.
    LentTo,
    /// Items that the user has write access to (i.e. items that the user can lend out).
    Subordinate,
}

pub(crate) struct Query<'a>(PhantomData<&'a ()>);

#[graphql_object(context = Context<'a>)]
impl<'a> Query<'a> {
    fn api_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn settings(_ctx: &Context<'a>) -> Settings {
        todo!();
    }

    fn items(_ctx: &Context<'a>, _ty: ItemsType) -> String {
        todo!();
    }
}

pub(crate) struct Mutation<'a>(PhantomData<&'a ()>);

#[graphql_object(context = Context<'a>)]
impl<'a> Mutation<'a> {
    fn settings(_ctx: &Context<'a>, _settings: SettingsInput) -> Settings {
        todo!();
    }

    fn loan_item(_ctx: &Context<'a>, _subject_areas: Option<Vec<SubjectArea>>) -> String {
        todo!();
    }

    fn return_item(_ctx: &Context<'a>, _subject_areas: Option<Vec<SubjectArea>>) -> String {
        todo!();
    }
}

// TODO static lifetimes
type Schema =
    RootNode<'static, Query<'static>, Mutation<'static>, EmptySubscription<Context<'static>>>;

pub(crate) fn schema() -> Schema {
    Schema::new(
        Query(PhantomData),
        Mutation(PhantomData),
        EmptySubscription::<Context<'static>>::new(),
    )
}
