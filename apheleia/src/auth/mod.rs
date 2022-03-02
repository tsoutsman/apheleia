mod config;
mod user;

pub(crate) use config::Config;
pub(crate) use user::User;

pub(crate) enum Permission {
    Loan,
    Borrow,
    Modify,
}
