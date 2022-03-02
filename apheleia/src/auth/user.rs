use crate::{
    auth::Permission,
    db::{
        schema::{item, role_permissions, subject_area, user, user_roles},
        DbPool,
    },
    id::{self, Id},
    BoxFuture, Error,
};

use diesel::{
    backend::Backend,
    sql_types::Integer,
    types::{FromSql, ToSql},
    ExpressionMethods, Identifiable, Insertable, JoinOnDsl, QueryDsl,
};
use serde::Serialize;
use tokio_diesel::AsyncRunQueryDsl;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    FromSqlRow,
    Insertable,
    AsExpression,
    Identifiable,
    Serialize,
)]
#[sql_type = "Integer"]
#[table_name = "user"]
pub(crate) struct User(#[column_name = "id"] i32);

impl From<i32> for User {
    #[inline]
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl<DB> ToSql<Integer, DB> for User
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<'_, W, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for User
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(Self(i32::from_sql(bytes)?))
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
            // TODO
            Ok(Self(id as i32))
        };
        Box::pin(result)
    }
}

impl User {
    pub(crate) async fn is_authorised_by_item(
        &self,
        pool: &DbPool,
        item_id: Id<id::Item>,
        permission: Permission,
    ) -> crate::Result<bool> {
        let archetype_id = item::table
            .find(item_id)
            .select(item::archetype)
            .first_async::<Id<id::Archetype>>(pool)
            .await?;

        self.is_authorised_by_archetype(pool, archetype_id, permission)
            .await
    }

    pub(crate) async fn is_authorised_by_archetype(
        &self,
        pool: &DbPool,
        archetype_id: Id<id::Archetype>,
        permission: Permission,
    ) -> crate::Result<bool> {
        Ok(user::table
            .find(*self)
            .inner_join(user_roles::table)
            .inner_join(role_permissions::table.on(role_permissions::role.eq(user_roles::role)))
            .filter(role_permissions::archetype.eq(archetype_id))
            .select((
                role_permissions::loan,
                role_permissions::borrow,
                role_permissions::create,
                role_permissions::modify,
                role_permissions::delete,
            ))
            .load_async::<(bool, bool, bool, bool, bool)>(pool)
            .await?
            .into_iter()
            .any(|(loan, borrow, create, modify, delete)| match permission {
                Permission::Loan => loan,
                Permission::Borrow => borrow,
                Permission::Create => create,
                Permission::Modify => modify,
                Permission::Delete => delete,
            }))
    }

    #[allow(dead_code)]
    pub(crate) async fn is_admin(
        &self,
        pool: &DbPool,
        subject_area_id: Id<id::SubjectArea>,
    ) -> crate::Result<bool> {
        let admin_id = subject_area::table
            .filter(subject_area::id.eq(subject_area_id))
            .select(subject_area::admin)
            .first_async::<User>(pool)
            .await?;

        Ok(admin_id.0 == self.0)
    }
}
