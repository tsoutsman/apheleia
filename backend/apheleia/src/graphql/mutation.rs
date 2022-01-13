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

    fn update_settings(_ctx: &Context<'a>, _input: SettingsInput) -> FieldResult<Settings> {
        todo!();
    }

    // Privileged Mutations

    fn loan_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item> {
        todo!();
    }

    fn return_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item> {
        todo!();
    }

    fn create_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item> {
        todo!();
    }

    fn delete_item(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _item_id: Uuid,
    ) -> FieldResult<Item> {
        todo!();
    }

    // TODO: Add some way to update item archetype field values.

    fn create_archetype(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: ArchetypeInput,
    ) -> FieldResult<Archetype> {
        todo!();
    }

    fn update_archetype(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: ArchetypeInput,
    ) -> FieldResult<Archetype> {
        todo!();
    }

    // Admin Mutations

    fn create_role(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: RoleInput,
    ) -> FieldResult<Role> {
        todo!();
    }

    fn update_role(
        _ctx: &Context<'a>,
        _subject_area: SubjectArea,
        _input: RoleInput,
    ) -> FieldResult<Role> {
        todo!();
    }
}
