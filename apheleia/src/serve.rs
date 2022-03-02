use std::{future::Future, sync::Arc};

use crate::{auth, Error, FuncReturn, Result};

use actix_web::{middleware, web::Data, App, HttpServer};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

/// Entry point for the server.
///
/// Takes in a single paramater, `token_to_id`, a function which converts an
/// unverified token into some verified ID. For example, an OAuth access token
/// into a user ID.
#[inline]
pub(crate) async fn serve<Func, Fut>(token_to_id_function: Func) -> Result<()>
where
    Func: Fn(String) -> Fut + 'static + Send + Sync + Clone,
    Fut: Future<Output = std::result::Result<u32, Box<dyn std::error::Error>>> + 'static + Send,
{
    let wrapper = move |token| -> FuncReturn { Box::pin(token_to_id_function(token)) };
    let config = auth::Config {
        token_to_id_function: Arc::new(wrapper),
    };
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://username:password@localhost/db_name");
    let db_pool = Pool::new(manager).map_err(|_| Error::R2d2)?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(config.clone())
            // TODO: CORS?
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
    })
    .bind("0.0.0.0:8000")
    .map_err(|e| -> Error { e.into() })?;

    log::info!("Listening on: {:?}", server.addrs());

    server.run().await.map_err(|e| -> Error { e.into() })
}