use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
pub struct Settings {
    theme: Theme,
}

#[derive(GraphQLInputObject)]
pub struct SettingsInput {
    theme: Theme,
}

#[derive(GraphQLEnum)]
pub enum Theme {
    Light,
    Dark,
}
