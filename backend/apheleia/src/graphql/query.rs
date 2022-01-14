use std::marker::PhantomData;

use crate::{
    graphql::{
        archetype::Archetype,
        item::Item,
        loan::{Loan, LoanType},
        role::Role,
        settings::Settings,
        util, SubjectArea,
    },
    Context,
};

use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

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

    async fn loans(
        _ctx: &Context<'a>,
        _subject_areas: Option<Vec<SubjectArea>>,
        _ty: LoanType,
    ) -> FieldResult<Vec<Loan<'_>>> {
        todo!();
    }

    async fn logs(
        _ctx: &Context<'a>,
        _subject_areas: Option<Vec<SubjectArea>>,
    ) -> FieldResult<Settings> {
        todo!();
    }

    async fn settings(_ctx: &Context<'a>) -> FieldResult<Settings> {
        todo!();
    }

    // Privileged Queries

    async fn item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item: Uuid,
    ) -> FieldResult<Item<'_>> {
        todo!();
    }

    async fn items(
        _ctx: &Context<'a>,
        subject_areas: Option<Vec<SubjectArea>>,
    ) -> FieldResult<Vec<Item<'_>>> {
        let result = Vec::new();

        for _schema in util::schemas(subject_areas) {
            // TODO: Pagination
            todo!();
        }

        Ok(result)
    }

    async fn archetypes(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
    ) -> FieldResult<Vec<Archetype<'_>>> {
        todo!();
    }

    // Admin Queries

    async fn roles(_ctx: &Context<'a>, _subject_area: SubjectArea) -> FieldResult<Vec<Role<'_>>> {
        todo!();
    }
}
