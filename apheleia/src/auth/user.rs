use crate::{
    auth::Permission,
    db::{
        schema::{item, role_permission, subject_area, user_id, user_role},
        DbPool,
    },
    BoxFuture, Error, Id,
};

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl};
use tokio_diesel::AsyncRunQueryDsl;

#[derive(Clone, Debug)]
pub(crate) struct User(Id);

impl From<u32> for User {
    #[inline]
    fn from(v: u32) -> Self {
        Self(Id(v))
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
            Ok(id.into())
        };
        Box::pin(result)
    }
}

impl User {
    pub(crate) async fn is_authorised(
        &self,
        pool: &DbPool,
        item_id: Id,
        permission: Permission,
    ) -> crate::Result<bool> {
        let archetype_id = item::table
            // TODO: Remove this i32::from.
            .find(i32::from(item_id))
            .select(item::archetype)
            .first_async::<Option<i32>>(pool)
            .await?
            .ok_or(Error::NotFound)?;

        Ok(user_id::table
            .find(i32::from(self.0))
            .inner_join(user_role::table)
            .inner_join(role_permission::table.on(role_permission::role.eq(user_role::role)))
            .filter(role_permission::archetype.eq(archetype_id))
            .select((
                role_permission::loan,
                role_permission::borrow,
                role_permission::modify,
            ))
            .load_async::<(bool, bool, bool)>(pool)
            .await?
            .into_iter()
            .any(|(loan, borrow, modify)| match permission {
                Permission::Loan => loan,
                Permission::Borrow => borrow,
                Permission::Modify => modify,
            }))
    }

    pub(crate) async fn is_admin(&self, pool: &DbPool, subject_area_id: Id) -> crate::Result<bool> {
        let admin_id = subject_area::table
            // TODO: Remove this i32::from.
            .filter(subject_area::id.eq(i32::from(subject_area_id)))
            .select(subject_area::admin)
            .first_async::<Id>(pool)
            .await?;

        Ok(admin_id == self.0)
    }
}
