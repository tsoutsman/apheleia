#![deny(
    non_ascii_idents,
    // missing_docs,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    nonstandard_style,
    unreachable_pub,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    rustdoc::broken_intra_doc_links
)]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

mod api;
mod auth;
mod db;
mod error;
mod id;
mod serve;
#[cfg(test)]
mod test;

pub use crate::auth::Root;
pub use error::{Error, Result};
pub use crate::serve::serve;

pub(crate) type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
pub(crate) type FuncReturn = BoxFuture<std::result::Result<u32, Box<dyn std::error::Error>>>;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = embed_migrations!("./migrations");
