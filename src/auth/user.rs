use crate::{
    auth::Permission,
    db::{
        schema::{item, role_permissions, subject_area, user, user_roles},
        tokio::AsyncRunQueryDsl,
        DbPool,
    },
    id::{self, Id},
    BoxFuture, Error,
};

use diesel::{
    backend::{Backend, HasRawValue},
    deserialize::{FromSql, FromSqlRow},
    dsl::{Eq, Find, InnerJoin, InnerJoinOn},
    expression::AsExpression,
    query_dsl::JoinOnDsl,
    serialize::ToSql,
    sql_types::Integer,
    ExpressionMethods, Identifiable, Insertable, QueryDsl,
};
use serde::{Deserialize, Serialize};

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
    Deserialize,
)]
#[diesel(sql_type = Integer, table_name = user)]
pub(crate) struct User(#[diesel(column_name = id)] pub(crate) i32);

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
    fn to_sql<'a>(
        &'a self,
        out: &mut diesel::serialize::Output<'a, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for User
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: <DB as HasRawValue<'_>>::RawValue) -> diesel::deserialize::Result<Self> {
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
            .first::<Id<id::Archetype>>(pool)
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
        Ok(self
            .permissions()
            .filter(role_permissions::archetype.eq(archetype_id))
            .select((
                role_permissions::meta,
                role_permissions::loan,
                role_permissions::receive,
            ))
            .load::<(bool, bool, bool)>(pool)
            .await?
            .into_iter()
            .any(|(meta, loan, receive)| match permission {
                Permission::Meta => meta,
                Permission::Loan => loan,
                Permission::Receive => receive,
            }))
    }

    pub(crate) async fn is_admin_of(
        &self,
        pool: &DbPool,
        subject_area_id: Id<id::SubjectArea>,
    ) -> crate::Result<bool> {
        let admin_id = subject_area::table
            .filter(subject_area::id.eq(subject_area_id))
            .select(subject_area::admin)
            .first::<User>(pool)
            .await?;
        Ok(admin_id.0 == self.0)
    }

    pub(crate) fn is_root(&self, root: crate::auth::Root) -> bool {
        self.0 == root.0
    }
}

type Permissions = InnerJoinOn<
    InnerJoin<Find<user::table, User>, user_roles::table>,
    role_permissions::table,
    Eq<user_roles::role, role_permissions::role>,
>;

impl User {
    pub(crate) fn permissions(&self) -> Permissions {
        user::table
            .find(*self)
            .inner_join(user_roles::table)
            .inner_join(role_permissions::table.on(user_roles::role.eq(role_permissions::role)))
    }
}
