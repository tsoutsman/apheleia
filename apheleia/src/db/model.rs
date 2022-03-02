use crate::db::schema::*;
use chrono::{offset::Utc, DateTime};

#[derive(Clone, Queryable, Debug, Identifiable)]
#[table_name = "archetype"]
pub struct Archetype {
    pub id: i32,
    pub name: String,
    pub subject_area: i32,
    pub schema: String,
}

#[derive(Clone, Queryable, Debug, Identifiable)]
#[table_name = "item"]
pub struct Item {
    pub id: i32,
    pub note: Option<String>,
    pub archetype: Option<i32>,
    pub archetype_data: Option<serde_json::Value>,
}

#[derive(Clone, Queryable, Debug, Identifiable)]
#[table_name = "loan"]
pub struct Loan {
    pub id: i32,
    pub return_requested: bool,
    pub item: i32,
    pub loaner: i32,
    pub loanee: i32,
    pub note: Option<String>,
    pub date_loaned: DateTime<Utc>,
    pub date_due: Option<DateTime<Utc>>,
    pub date_returned: Option<DateTime<Utc>>,
}

#[derive(Clone, Queryable, Debug, Identifiable)]
#[table_name = "role"]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub subject_area: i32,
}

#[derive(Copy, Clone, Queryable, Debug, Identifiable)]
#[table_name = "role_permission"]
#[primary_key(role, archetype)]
pub struct RolePermission {
    pub role: i32,
    pub archetype: i32,
    pub loan: bool,
    pub borrow: bool,
    pub modify: bool,
}

#[derive(Clone, Queryable, Debug, Identifiable)]
#[table_name = "subject_area"]
pub struct SubjectArea {
    pub id: i32,
    pub name: String,
    pub admin: i32,
}

#[derive(Copy, Clone, Queryable, Debug, Identifiable)]
#[table_name = "user_id"]
pub struct UserId {
    pub id: i32,
}

#[derive(Copy, Clone, Queryable, Debug, Identifiable)]
#[table_name = "user_role"]
#[primary_key(user_id, role)]
pub struct UserRole {
    pub user_id: i32,
    pub role: i32,
}
