use std::marker::PhantomData;

use crate::{
    graphql::{
        archetype::{Archetype, ArchetypeInput},
        item::Item,
        role::{Role, RoleInput},
        settings::{Settings, SettingsInput},
        SubjectArea,
    },
    Context,
};

use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub(crate) struct Mutation<'a>(PhantomData<&'a ()>);

impl<'a> Mutation<'a> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

#[graphql_object(context = Context<'a>)]
impl<'a> Mutation<'a> {
    // User Mutations

    async fn update_settings(_ctx: &Context<'a>, _input: SettingsInput) -> FieldResult<Settings> {
        todo!();
    }

    // Privileged Mutations

    async fn loan_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item<'_>> {
        todo!();
    }

    async fn return_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item<'_>> {
        todo!();
    }

    async fn create_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item<'_>> {
        todo!();
    }

    async fn delete_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item<'_>> {
        todo!();
    }

    // TODO: Add some way to update item archetype field values.

    async fn create_archetype(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: ArchetypeInput,
    ) -> FieldResult<Archetype<'_>> {
        todo!();
    }

    async fn update_archetype(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: ArchetypeInput,
    ) -> FieldResult<Archetype<'_>> {
        todo!();
    }

    // Admin Mutations

    async fn create_role(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: RoleInput,
    ) -> FieldResult<Role<'_>> {
        todo!();
    }

    async fn update_role(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: RoleInput,
    ) -> FieldResult<Role<'_>> {
        todo!();
    }
}
