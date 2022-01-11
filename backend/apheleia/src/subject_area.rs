#[derive(Copy, Clone, Debug, Hash)]
// ../config.toml is taken from the root of the workspace and so it is referencing
// config.toml in the root directory of the project.
#[apheleia_proc::area_list("../config.toml")]
pub enum SubjectArea {}
