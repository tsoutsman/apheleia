use crate::{
    auth::User,
    db::{model, schema::subject_area, tokio::AsyncRunQueryDsl, DbPool},
    id::{self, Id},
    Result, Root,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::QueryDsl;
use serde::Deserialize;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_subject_area)
        .service(get_subject_areas)
        .service(add_subject_area)
        .service(modify_subject_area)
        .service(delete_subject_area);
}

#[get("/subject_areas/{id}")]
async fn get_subject_area() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/subject_areas")]
async fn get_subject_areas() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Clone, Debug, Deserialize)]
struct AddSubjectArea {
    name: String,
    admin: User,
}

#[post("/subject_areas")]
async fn add_subject_area(
    pool: web::Data<DbPool>,
    root: web::Data<Root>,
    user: User,
    request: web::Json<AddSubjectArea>,
) -> impl Responder {
    if user.is_root(*root.into_inner()) {
        let request = request.into_inner();
        let subject_area = model::SubjectArea {
            id: Id::<id::SubjectArea>::new(),
            name: request.name,
            admin: request.admin,
        };

        diesel::insert_into(subject_area::table)
            .values(subject_area)
            .execute(&pool)
            .await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[derive(Clone, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = subject_area)]
struct ModifySubjectArea {
    name: Option<String>,
    admin: Option<User>,
}

#[put("/subject_areas/{id}")]
async fn modify_subject_area(
    subject_area_id: web::Path<Id<id::SubjectArea>>,
    pool: web::Data<DbPool>,
    root: web::Data<Root>,
    user: User,
    request: web::Json<ModifySubjectArea>,
) -> impl Responder {
    if user.is_root(*root.into_inner()) || user.is_admin_of(&pool, *subject_area_id).await? {
        let request = request.into_inner();
        let target = subject_area::table.find(*subject_area_id);
        // This is safe: https://github.com/diesel-rs/diesel/issues/885
        diesel::update(target).set(request).execute(&pool).await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[delete("/subject_areas/{id}")]
async fn delete_subject_area(
    subject_area_id: web::Path<Id<id::SubjectArea>>,
    pool: web::Data<DbPool>,
    root: web::Data<Root>,
    user: User,
) -> impl Responder {
    if user.is_root(*root.into_inner()) || user.is_admin_of(&pool, *subject_area_id).await? {
        let target = subject_area::table.find(*subject_area_id);
        diesel::delete(target).execute(&pool).await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}
