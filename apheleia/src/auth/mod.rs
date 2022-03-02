mod config;
mod user;

pub(crate) use config::Config;
pub(crate) use user::User;

#[allow(dead_code)]
pub(crate) enum Permission {
    Loan,
    Borrow,
    Create,
    Modify,
    Delete,
}
