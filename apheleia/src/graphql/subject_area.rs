// ../config.toml is taken from the root of the workspace and so it is referencing
// config.toml in the root directory of the project.
#[apheleia_proc::subject_area("./config.toml")]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, juniper::GraphQLEnum)]
pub enum SubjectArea {}
