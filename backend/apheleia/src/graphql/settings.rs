use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
pub(crate) struct Settings {
    pub(crate) theme: Theme,
}

#[derive(GraphQLInputObject)]
pub(crate) struct SettingsInput {
    pub(crate) theme: Theme,
}

#[derive(GraphQLEnum)]
pub(crate) enum Theme {
    Light,
    Dark,
}
