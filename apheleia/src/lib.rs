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
mod serve;

pub use error::{Error, Result};

pub(crate) use serve::serve;

pub(crate) type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
pub(crate) type FuncReturn = BoxFuture<std::result::Result<u32, Box<dyn std::error::Error>>>;

pub fn run<Func, Fut>(token_to_id_function: Func) -> Result<()>
where
    Func: Fn(String) -> Fut + 'static + Send + Sync + Clone,
    Fut: std::future::Future<Output = std::result::Result<u32, Box<dyn std::error::Error>>>
        + 'static
        + Send,
{
    actix_web::rt::System::new().block_on(async move { serve(token_to_id_function).await })
}

#[macro_use]
extern crate diesel_migrations;
diesel_migrations::embed_migrations!("migrations");

#[macro_use]
extern crate diesel;
