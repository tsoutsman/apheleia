#![deny(
    non_ascii_idents,
    // missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]
#![cfg_attr(not(test), deny(clippy::expect_used, clippy::unwrap_used))]

use std::future::Future;

use actix_web::{web, App, HttpServer};

mod error;
mod extractor;

pub(crate) type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
pub(crate) type FuncReturn = BoxFuture<Result<String, Box<dyn std::error::Error>>>;

/// Entry point for the server.
///
/// Takes in a single paramater, `token_to_id`, a function which converts an
/// unverified token into some verified ID. For example, an OAuth access token
/// into a user ID.
#[inline]
pub fn run<F, Fut>(token_to_id_function: F) -> std::io::Result<()>
where
    F: Fn(String) -> Fut + 'static + Send + Clone,
    Fut: Future<Output = Result<String, Box<dyn std::error::Error>>> + 'static + Send,
{
    actix_web::rt::System::new("main").block_on(async move {
        let wrapper = move |token| -> FuncReturn { Box::pin(token_to_id_function(token)) };
        let config = extractor::IdConfig {
            token_to_id_function: wrapper,
        };

        HttpServer::new(move || {
            App::new().service(web::resource("/graphql").app_data(config.clone()))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
    })
}
