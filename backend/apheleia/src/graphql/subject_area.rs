// ../config.toml is taken from the root of the workspace and so it is referencing
// config.toml in the root directory of the project.
#[apheleia_proc::area_list("./config.toml")]
#[derive(Copy, Clone, Debug, Hash, juniper::GraphQLEnum)]
pub enum SubjectArea {}
