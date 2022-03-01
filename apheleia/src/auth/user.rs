use crate::{BoxFuture, Error};

#[derive(Clone, Debug)]
pub(crate) struct User(u32);

impl From<User> for u32 {
    #[inline]
    fn from(user: User) -> Self {
        user.0
    }
}

impl actix_web::FromRequest for User {
    type Error = Error;

    type Future = BoxFuture<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = match req.headers().get("Authorization") {
            Some(t) => match t.to_str() {
                Ok(t) => {
                    if t.starts_with("Bearer ") {
                        t.trim_start_matches("Bearer ").to_owned()
                    } else {
                        // The authorization field doesn't contain a token e.g. Basic authorization
                        return Box::pin(futures::future::ready(Err(Error::Authentication)));
                    }
                }
                // Non-ASCII characters
                Err(_) => return Box::pin(futures::future::ready(Err(Error::Authentication))),
            },
            // No authorization header
            None => return Box::pin(futures::future::ready(Err(Error::Authentication))),
        };
        let f = match req.app_data::<crate::auth::Config>() {
            Some(f) => f.token_to_id_function.clone(),
            None => unreachable!("No ID extractor specified"),
        };

        let result = async move {
            let id = (f)(token).await.map_err(|_| Error::Authentication)?;
            Ok(Self(id))
        };
        Box::pin(result)
    }
}
