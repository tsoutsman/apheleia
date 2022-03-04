use crate::{
    auth::User,
    db::schema::{archetype, item, loan, role, role_permissions, subject_area, user_roles},
    id::{self, Id},
};

use chrono::{offset::Utc, DateTime};
use serde::Serialize;

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = archetype)]
pub(crate) struct Archetype {
    pub(crate) id: Id<id::Archetype>,
    pub(crate) name: String,
    pub(crate) subject_area: Id<id::SubjectArea>,
    pub(crate) schema: serde_json::Value,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = item)]
pub(crate) struct Item {
    pub(crate) id: Id<id::Item>,
    pub(crate) note: Option<String>,
    pub(crate) archetype: Id<id::Archetype>,
    pub(crate) archetype_data: serde_json::Value,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = loan)]
pub(crate) struct Loan {
    pub(crate) id: Id<id::Loan>,
    pub(crate) return_requested: bool,
    pub(crate) item: Id<id::Item>,
    pub(crate) loaner: User,
    pub(crate) loanee: User,
    pub(crate) note: Option<String>,
    pub(crate) date_loaned: DateTime<Utc>,
    pub(crate) date_due: Option<DateTime<Utc>>,
    pub(crate) date_returned: Option<DateTime<Utc>>,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = role)]
pub(crate) struct Role {
    pub(crate) id: Id<id::Role>,
    pub(crate) name: String,
    pub(crate) subject_area: Id<id::SubjectArea>,
}

#[derive(Copy, Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = role_permissions, primary_key(role, archetype))]
pub(crate) struct RolePermission {
    pub(crate) role: Id<id::Role>,
    pub(crate) archetype: Id<id::Archetype>,
    pub(crate) loan: bool,
    pub(crate) receive: bool,
    pub(crate) create: bool,
    pub(crate) modify: bool,
    pub(crate) delete: bool,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = subject_area)]
pub(crate) struct SubjectArea {
    pub(crate) id: Id<id::SubjectArea>,
    pub(crate) name: String,
    pub(crate) admin: User,
}

#[derive(Copy, Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[diesel(table_name = user_roles, primary_key(user, role))]
pub(crate) struct UserRole {
    pub(crate) user: User,
    pub(crate) role: Id<id::Role>,
}

// User struct is in auth::user
