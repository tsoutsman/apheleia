#![feature(allocator_api)]
use std::future::Future;

mod error;
mod extractor;

pub use actix_web::main;

/// Entry point for the server.
///
/// Takes in a single paramater, `token_to_id`, a function which converts an
/// unverified token into some verified ID. For example, an OAuth access token
/// into a user ID.
pub async fn run<F, Fut>(_token_to_id_function: F)
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = Result<String, Box<dyn std::error::Error>>>,
{
    todo!();
}
