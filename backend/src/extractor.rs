use std::marker::PhantomData;

use crate::{BoxFuture, FuncReturn};

#[derive(Clone, Debug)]
pub struct Id<F>(
    pub String,
    /// I think this is the correct use of PhantomData. I need a generic for IdConfig in the
    /// FromRequest implementation for Id, but if the generic is not directly used by Id,
    /// the compiler says that the generic is unconstrained.
    PhantomData<F>,
)
where
    F: Fn(String) -> FuncReturn + Clone;

impl<F> From<String> for Id<F>
where
    F: Fn(String) -> FuncReturn + Clone,
{
    #[inline]
    fn from(id: String) -> Self {
        Self(id, PhantomData)
    }
}

impl<F> From<Id<F>> for String
where
    F: Fn(String) -> FuncReturn + Clone,
{
    #[inline]
    fn from(id: Id<F>) -> Self {
        id.0
    }
}

#[derive(Clone, Debug)]
pub struct IdConfig<F>
where
    F: Fn(String) -> FuncReturn + Clone,
{
    pub token_to_id_function: F,
}

impl<F> Default for IdConfig<F>
where
    F: Fn(String) -> FuncReturn + Clone,
{
    fn default() -> Self {
        unreachable!("No ID extractor specified");
    }
}

impl<F> actix_web::FromRequest for Id<F>
where
    F: Fn(String) -> FuncReturn + Clone + 'static + Send + Sync,
{
    // TODO
    type Error = ();

    type Future = BoxFuture<Result<Self, Self::Error>>;

    type Config = IdConfig<F>;

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
        let f = match req.app_data::<IdConfig<F>>() {
            // TODO is cloning here expensive?
            Some(f) => f.clone(),
            None => unreachable!("No ID extractor specified"),
        };

        let result = async move {
            (f.token_to_id_function)(token)
                .await
                .map(Self::from)
                .map_err(|_| ())
        };
        Box::pin(result)
    }
}
