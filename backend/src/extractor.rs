use std::sync::Arc;

use crate::{BoxFuture, FuncReturn};

#[derive(Clone, Debug)]
pub struct Id(pub String);

impl From<String> for Id {
    #[inline]
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<Id> for String {
    #[inline]
    fn from(id: Id) -> Self {
        id.0
    }
}

#[derive(Clone)]
pub struct IdConfig {
    pub token_to_id_function: Arc<dyn Fn(String) -> FuncReturn + Send + Sync>,
}

impl std::fmt::Debug for IdConfig {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for IdConfig {
    fn default() -> Self {
        unreachable!("No ID extractor specified");
    }
}

impl actix_web::FromRequest for Id {
    // TODO
    type Error = ();

    type Future = BoxFuture<Result<Self, Self::Error>>;

    type Config = IdConfig;

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = match req.headers().get("Authorization") {
            Some(t) => match t.to_str() {
                Ok(t) => {
                    if t.starts_with("Bearer ") {
                        t.trim_start_matches("Bearer ").to_owned()
                    } else {
                        // The authorization field doesn't contain a token e.g. Basic authorization
                        return Box::pin(futures::future::ready(Err(())));
                    }
                }
                // Non-ASCII characters
                Err(_) => return Box::pin(futures::future::ready(Err(()))),
            },
            // No authorization header
            None => return Box::pin(futures::future::ready(Err(()))),
        };
        let f = match req.app_data::<IdConfig>() {
            Some(f) => f.token_to_id_function.clone(),
            None => unreachable!("No ID extractor specified"),
        };

        let result = async move { (f)(token).await.map(Self::from).map_err(|_| ()) };
        Box::pin(result)
    }
}
