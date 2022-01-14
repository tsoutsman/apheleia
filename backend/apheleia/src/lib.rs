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

mod cli;
mod context;
mod error;
mod extractor;
mod graphql;
mod serve;

pub use cli::run;
pub use error::{Error, Result};

pub(crate) use context::Context;
pub(crate) use serve::serve;

pub(crate) type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
pub(crate) type FuncReturn = BoxFuture<std::result::Result<String, Box<dyn std::error::Error>>>;

mod db;
