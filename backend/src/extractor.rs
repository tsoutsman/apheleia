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
        panic!("No ID extractor specified");
    }
}

impl<F> actix_web::FromRequest for Id<F>
where
    F: Fn(String) -> FuncReturn + Clone + 'static,
{
    type Error = ();

    type Future = BoxFuture<Result<Self, Self::Error>>;

    type Config = IdConfig<F>;

    #[inline]
    fn from_request(
        _req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        todo!();
    }
}
