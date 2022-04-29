use crate::{
    auth::User,
    db::{model, schema::archetype, tokio::AsyncRunQueryDsl, DbPool},
    id::{self, Id},
    Result,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::QueryDsl;
use serde::Deserialize;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_archetype)
        .service(get_archetypes)
        .service(add_archetype)
        .service(modify_archetype)
        .service(delete_archetype);
}

#[get("/archetypes/{id}")]
async fn get_archetype(
    archetype_id: web::Path<Id<id::Archetype>>,
    pool: web::Data<DbPool>,
    _: User,
) -> impl Responder {
    let archetype = archetype::table
        .find(*archetype_id)
        .first::<model::Archetype>(&pool)
        .await?;

    Result::Ok(HttpResponse::Ok().json(archetype))
}

#[get("/archetypes")]
async fn get_archetypes(pool: web::Data<DbPool>, _: User) -> impl Responder {
    // TODO: Pagination
    let archetypes = archetype::table.load::<model::Archetype>(&pool).await?;
    Result::Ok(HttpResponse::Ok().json(archetypes))
}

#[derive(Clone, Debug, Deserialize)]
struct AddArchetype {
    name: String,
    subject_area: Id<id::SubjectArea>,
    schema: serde_json::Value,
}

#[post("/archetypes")]
async fn add_archetype(
    pool: web::Data<DbPool>,
    user: User,
    request: web::Json<AddArchetype>,
) -> impl Responder {
    if user.is_admin_of(&pool, request.subject_area).await? {
        let request = request.into_inner();
        let archetype = model::Archetype {
            id: Id::<id::Archetype>::new(),
            name: request.name,
            subject_area: request.subject_area,
            schema: request.schema,
        };

        diesel::insert_into(archetype::table)
            .values(archetype)
            .execute(&pool)
            .await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[derive(Clone, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = archetype)]
struct ModifyArchetype {
    name: Option<String>,
    schema: Option<serde_json::Value>,
}

#[put("/archetypes/{id}")]
async fn modify_archetype(
    archetype_id: web::Path<Id<id::Archetype>>,
    pool: web::Data<DbPool>,
    user: User,
    request: web::Json<ModifyArchetype>,
) -> impl Responder {
    let archetype_subject_area = archetype_id.subject_area().first(&pool).await?;
    if user.is_admin_of(&pool, archetype_subject_area).await? {
        let request = request.into_inner();
        let target = archetype::table.find(*archetype_id);
        // This is safe: https://github.com/diesel-rs/diesel/issues/885
        diesel::update(target).set(request).execute(&pool).await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[delete("/archetypes/{id}")]
async fn delete_archetype(
    archetype_id: web::Path<Id<id::Archetype>>,
    pool: web::Data<DbPool>,
    user: User,
) -> impl Responder {
    let archetype_subject_area = archetype_id.subject_area().first(&pool).await?;
    if user.is_admin_of(&pool, archetype_subject_area).await? {
        let target = archetype::table.find(*archetype_id);
        diesel::delete(target).execute(&pool).await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}
