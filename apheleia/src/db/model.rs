use crate::{db::schema::*, Id};

use chrono::{offset::Utc, DateTime};
use serde::Serialize;

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "archetype"]
pub(crate) struct Archetype {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) subject_area: Id,
    pub(crate) schema: String,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "item"]
pub(crate) struct Item {
    pub(crate) id: Id,
    pub(crate) note: Option<String>,
    pub(crate) archetype: Id,
    pub(crate) archetype_data: Option<serde_json::Value>,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "loan"]
pub(crate) struct Loan {
    pub(crate) id: Id,
    pub(crate) return_requested: bool,
    pub(crate) item: Id,
    pub(crate) loaner: Id,
    pub(crate) loanee: Id,
    pub(crate) note: Option<String>,
    pub(crate) date_loaned: DateTime<Utc>,
    pub(crate) date_due: Option<DateTime<Utc>>,
    pub(crate) date_returned: Option<DateTime<Utc>>,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "role"]
pub(crate) struct Role {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) subject_area: Id,
}

#[derive(Copy, Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "role_permissions"]
#[primary_key(role, archetype)]
pub(crate) struct RolePermission {
    pub(crate) role: Id,
    pub(crate) archetype: Id,
    pub(crate) loan: bool,
    pub(crate) borrow: bool,
    pub(crate) create: bool,
    pub(crate) modify: bool,
    pub(crate) delete: bool,
}

#[derive(Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "subject_area"]
pub(crate) struct SubjectArea {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) admin: Id,
}

#[derive(Copy, Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "user"]
pub(crate) struct User {
    pub(crate) id: Id,
}

#[derive(Copy, Clone, Queryable, Insertable, Debug, Identifiable, Serialize)]
#[table_name = "user_roles"]
#[primary_key(user, role)]
pub(crate) struct UserRole {
    pub(crate) user: Id,
    pub(crate) role: Id,
}
