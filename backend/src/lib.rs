#![deny(
    non_ascii_idents,
    // missing_docs,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]
#![cfg_attr(not(test), deny(clippy::expect_used, clippy::unwrap_used))]

use std::{future::Future, sync::Arc};

use crate::extractor::Id;

use actix_web::{web, App, HttpResponse, HttpServer};
use juniper_actix::{graphiql_handler, graphql_handler};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub mod error;
mod extractor;

pub(crate) type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
pub(crate) type FuncReturn = BoxFuture<Result<String, Box<dyn std::error::Error>>>;

struct Context<'a> {
    pub id: Id,
    pub pool: &'a PgPool,
}

impl<'a> juniper::Context for Context<'a> {}

// async fn graphiql_route() -> actix_web::Result<HttpResponse> {
//     graphiql_handler("/graphql", None).await
// }

async fn graphql_route(
    _req: actix_web::HttpRequest,
    _payload: actix_web::web::Payload,
    // schema: web::Data<Schema>,
    id: extractor::Id,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    let pool = pool.get_ref();
    let _ctx = Context { id, pool };
    // graphql_handler(&schema, &context, req, payload).await
    todo!();
}

/// Entry point for the server.
///
/// Takes in a single paramater, `token_to_id`, a function which converts an
/// unverified token into some verified ID. For example, an OAuth access token
/// into a user ID.
#[inline]
pub fn run<F, Fut>(token_to_id_function: F) -> error::Result<()>
where
    F: Fn(String) -> Fut + 'static + Send + Sync + Clone,
    Fut: Future<Output = Result<String, Box<dyn std::error::Error>>> + 'static + Send,
{
    actix_web::rt::System::new("main").block_on(async move {
        let wrapper = move |token| -> FuncReturn { Box::pin(token_to_id_function(token)) };
        let config = extractor::IdConfig {
            token_to_id_function: Arc::new(wrapper),
        };
        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect("postgres address")
            .await
            .map_err(|e| -> error::Error { e.into() })?;

        HttpServer::new(move || {
            App::new().data(db_pool.clone()).service(
                web::resource("/graphql")
                    .route(web::get().to(graphql_route))
                    .route(web::post().to(graphql_route))
                    .app_data(config.clone()),
            )
        })
        .bind("127.0.0.1:8000")
        .map_err(|e| -> error::Error { e.into() })?
        .run()
        .await
        .map_err(|e| -> error::Error { e.into() })
    })
}
