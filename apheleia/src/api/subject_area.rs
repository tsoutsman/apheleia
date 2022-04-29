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
async fn get_subject_area(
    _: User,
    subject_area_id: web::Path<Id<id::SubjectArea>>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let subject_area = subject_area::table
        .find(*subject_area_id)
        .first::<model::SubjectArea>(&pool)
        .await?;
    Result::Ok(HttpResponse::Ok().json(subject_area))
}

#[get("/subject_areas")]
async fn get_subject_areas(_: User, pool: web::Data<DbPool>) -> impl Responder {
    let subject_areas = subject_area::table
        .load::<model::SubjectArea>(&pool)
        .await?;
    Result::Ok(HttpResponse::Ok().json(subject_areas))
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
struct AddSubjectArea {
    name: String,
    admin: User,
}

#[post("/subject_areas")]
async fn add_subject_area(
    user: User,
    request: web::Json<AddSubjectArea>,
    pool: web::Data<DbPool>,
    root: web::Data<Root>,
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
#[cfg_attr(test, derive(serde::Serialize))]
#[diesel(table_name = subject_area)]
struct ModifySubjectArea {
    name: Option<String>,
    admin: Option<User>,
}

#[put("/subject_areas/{id}")]
async fn modify_subject_area(
    user: User,
    subject_area_id: web::Path<Id<id::SubjectArea>>,
    request: web::Json<ModifySubjectArea>,
    pool: web::Data<DbPool>,
    root: web::Data<Root>,
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
    user: User,
    subject_area_id: web::Path<Id<id::SubjectArea>>,
    pool: web::Data<DbPool>,
    root: web::Data<Root>,
) -> impl Responder {
    if user.is_root(*root.into_inner()) || user.is_admin_of(&pool, *subject_area_id).await? {
        let target = subject_area::table.find(*subject_area_id);
        diesel::delete(target).execute(&pool).await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::header,
        test::{self, TestRequest},
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn test_unauthenticated_subject_area_access() {
        let (app, _pool) = crate::test::init_test_service().await;

        let req = TestRequest::get()
            .uri("/subject_areas/30d6efc1-f093-4292-af2c-1d5718403d0c")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let req = TestRequest::get().uri("/subject_areas").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let req = TestRequest::post()
            .uri("/subject_areas")
            .set_json(AddSubjectArea {
                name: "name".to_owned(),
                admin: 1.into(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let req = TestRequest::put()
            .uri("/subject_areas/30d6efc1-f093-4292-af2c-1d5718403d0c")
            .set_json(ModifySubjectArea {
                name: None,
                admin: None,
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let req = TestRequest::delete()
            .uri("/subject_areas/30d6efc1-f093-4292-af2c-1d5718403d0c")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_invalid_subject_area_uuid() {
        let (app, _pool) = crate::test::init_test_service().await;

        let req = TestRequest::get()
            .uri("/subject_areas/z")
            .insert_header((header::AUTHORIZATION, "Bearer 1234"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        // NOTE: I don't think 404 is the correct status code, but it's what
        // Actix Web spits out when web::Path fails to deserialize and wouldn't
        // be trivial to change.
        // https://github.com/actix/actix-web/issues/2517
        assert_eq!(resp.status(), 404);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_subject_area() {
        let (app, _pool) = crate::test::init_test_service().await;

        let req = TestRequest::get()
            .uri("/subject_areas/30d6efc1-f093-4292-af2c-1d5718403d0c")
            .insert_header((header::AUTHORIZATION, "Bearer 1234"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);

        // TODO: Add more tests.
    }
}
