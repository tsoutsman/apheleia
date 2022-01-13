use std::marker::PhantomData;

use crate::{
    graphql::{
        archetype::Archetype,
        item::{Item, ItemsType},
        role::Role,
        settings::Settings,
        SubjectArea,
    },
    Context,
};

use juniper::{graphql_object, FieldResult};

pub(crate) struct Query<'a>(PhantomData<&'a ()>);

impl<'a> Query<'a> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

#[graphql_object(context = Context<'a>)]
impl<'a> Query<'a> {
    fn api_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    // User Queries

    fn items(
        _ctx: &Context<'a>,
        _ty: ItemsType,
        _subject_areas: Option<Vec<SubjectArea>>,
    ) -> FieldResult<Vec<Item>> {
        todo!();
    }

    fn settings(_ctx: &Context<'a>) -> FieldResult<Settings> {
        todo!();
    }

    fn logs(_ctx: &Context<'a>, _subject_areas: Option<Vec<SubjectArea>>) -> FieldResult<Settings> {
        todo!();
    }

    // Privileged Queries

    fn archetypes(_ctx: &Context<'a>, _subject_area: SubjectArea) -> FieldResult<Vec<Archetype>> {
        todo!();
    }

    // Admin Queries

    fn roles(_ctx: &Context<'a>, _subject_area: SubjectArea) -> FieldResult<Vec<Role>> {
        todo!();
    }
}
