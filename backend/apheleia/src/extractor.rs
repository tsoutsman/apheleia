use std::sync::Arc;

use crate::{BoxFuture, FuncReturn, SubjectArea};

use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub admin_of: SmallVec<[SubjectArea; 1]>,
}

impl From<User> for String {
    #[inline]
    fn from(user: User) -> Self {
        user.id
    }
}

#[derive(Clone)]
pub struct UserConfig {
    pub token_to_id_function: Arc<dyn Fn(String) -> FuncReturn + Send + Sync>,
}

impl std::fmt::Debug for UserConfig {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        unreachable!("No ID extractor specified");
    }
}

impl actix_web::FromRequest for User {
    // TODO
    type Error = ();

    type Future = BoxFuture<Result<Self, Self::Error>>;

    type Config = UserConfig;

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
        let f = match req.app_data::<UserConfig>() {
            Some(f) => f.token_to_id_function.clone(),
            None => unreachable!("No ID extractor specified"),
        };

        let result = async move {
            let id = (f)(token).await.map_err(|_| ())?;
            let admin_of = SubjectArea::admin_of(&id);

            Ok(Self { id, admin_of })
        };
        Box::pin(result)
    }
}
