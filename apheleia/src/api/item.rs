use crate::{
    auth::{Permission, User},
    db::{model, schema::item, DbPool},
    Id, Result,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::QueryDsl;
use serde::Deserialize;
use tokio_diesel::AsyncRunQueryDsl;

#[get("/items/{id}")]
async fn get_item(item_id: web::Path<Id>, pool: web::Data<DbPool>, _: User) -> impl Responder {
    let item = item::table
        .find(*item_id)
        .first_async::<model::Item>(&pool)
        .await?;
    Result::Ok(HttpResponse::Ok().json(item))
}

#[get("/items")]
async fn get_items(pool: web::Data<DbPool>, _: User) -> impl Responder {
    // TODO: Pagination
    let items = item::table.load_async::<model::Item>(&pool).await?;
    Result::Ok(HttpResponse::Ok().json(items))
}

#[derive(Clone, Debug, Deserialize)]
struct AddItemRequest {
    note: Option<String>,
    archetype: Id,
    archetype_data: Option<serde_json::Value>,
}

#[post("/items")]
async fn add_item(
    pool: web::Data<DbPool>,
    request: web::Json<AddItemRequest>,
    user: User,
) -> impl Responder {
    if user
        .is_authorised_by_archetype(&pool, request.archetype, Permission::Create)
        .await?
    {
        let request = request.into_inner();

        let item = model::Item {
            // TODO
            id: 1.into(),
            note: request.note,
            archetype: request.archetype,
            archetype_data: request.archetype_data,
        };

        diesel::insert_into(item::table)
            .values(item)
            .execute_async(&pool)
            .await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        // TODO is Result::Ok correct here?
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[put("/items/{id}")]
async fn modify_item(
    item_id: web::Path<Id>,
    pool: web::Data<DbPool>,
    user: User,
) -> impl Responder {
    if user
        .is_authorised_by_item(&pool, *item_id, Permission::Modify)
        .await?
    {
        todo!();
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[delete("/items/{id}")]
async fn delete_item() -> impl Responder {
    HttpResponse::Ok()
}
