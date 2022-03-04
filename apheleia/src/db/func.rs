use crate::{
    auth::User,
    db::schema::{role_permissions, user, user_roles},
};

use diesel::{
    dsl::{Eq, Find, InnerJoin, InnerJoinOn},
    ExpressionMethods, JoinOnDsl, QueryDsl,
};

type Permissions = InnerJoinOn<
    InnerJoin<Find<user::table, User>, user_roles::table>,
    role_permissions::table,
    Eq<role_permissions::role, user_roles::role>,
>;

impl User {
    pub(crate) fn permissions(&self) -> Permissions {
        user::table
            .find(*self)
            .inner_join(user_roles::table)
            .inner_join(role_permissions::table.on(role_permissions::role.eq(user_roles::role)))
    }
}
