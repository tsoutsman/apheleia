use crate::{
    auth::User,
    db::{model, schema::role, tokio::AsyncRunQueryDsl, DbPool},
    id::{self, Id},
    Result,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::QueryDsl;
use serde::Deserialize;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_role)
        .service(get_roles)
        .service(add_role)
        .service(modify_role)
        .service(delete_role);
}

#[get("/roles/{id}")]
async fn get_role(
    _: User,
    role_id: web::Path<Id<id::Role>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let role = role::table
        .find(*role_id)
        .first::<model::Role>(&pool)
        .await?;

    Result::Ok(HttpResponse::Ok().json(role))
}

#[get("/roles")]
async fn get_roles(pool: web::Data<DbPool>, _: User) -> impl Responder {
    // TODO: Pagination
    let roles = role::table.load::<model::Role>(&pool).await?;
    Result::Ok(HttpResponse::Ok().json(roles))
}

#[derive(Clone, Debug, Deserialize)]
struct AddRole {
    name: String,
    subject_area: Id<id::SubjectArea>,
}

#[post("/roles")]
async fn add_role(
    user: User,
    request: web::Json<AddRole>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    if user.is_admin_of(&pool, request.subject_area).await? {
        let request = request.into_inner();

        let role = model::Role {
            id: Id::new(),
            name: request.name,
            subject_area: request.subject_area,
        };

        diesel::insert_into(role::table)
            .values(role)
            .execute(&pool)
            .await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[derive(Clone, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = role)]
struct ModifyRole {
    name: Option<String>,
    subject_area: Option<Id<id::SubjectArea>>,
}

#[put("/roles/{id}")]
async fn modify_role(
    user: User,
    role_id: web::Path<Id<id::Role>>,
    request: web::Json<ModifyRole>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let role_subject_area = role_id.subject_area().first(&pool).await?;
    if user.is_admin_of(&pool, role_subject_area).await? {
        let request = request.into_inner();

        let target = role::table.find(*role_id);
        // This is safe: https://github.com/diesel-rs/diesel/issues/885
        diesel::update(target).set(request).execute(&pool).await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[delete("/roles/{id}")]
async fn delete_role(
    user: User,
    role_id: web::Path<Id<id::Role>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let role_subject_area = role_id.subject_area().first(&pool).await?;
    if user.is_admin_of(&pool, role_subject_area).await? {
        let target = role::table.find(*role_id);
        diesel::delete(target).execute(&pool).await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}
