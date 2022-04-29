use crate::{
    auth::{Permission, User},
    db::{model, schema::item, tokio::AsyncRunQueryDsl, DbPool},
    id::{self, Id},
    Result,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::QueryDsl;
use serde::Deserialize;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_item).service(get_items).service(add_item).service(modify_item).service(delete_item);
}

#[get("/items/{id}")]
async fn get_item(
    item_id: web::Path<Id<id::Item>>,
    pool: web::Data<DbPool>,
    _: User,
) -> impl Responder {
    let item = item::table
        .find(*item_id)
        .first::<model::Item>(&pool)
        .await?;

    Result::Ok(HttpResponse::Ok().json(item))
}

#[get("/items")]
async fn get_items(pool: web::Data<DbPool>, _: User) -> impl Responder {
    // TODO: Pagination
    let items = item::table.load::<model::Item>(&pool).await?;
    Result::Ok(HttpResponse::Ok().json(items))
}

#[derive(Clone, Debug, Deserialize)]
struct AddItem {
    note: Option<String>,
    archetype: Id<id::Archetype>,
    archetype_data: serde_json::Value,
}

#[post("/items")]
async fn add_item(
    pool: web::Data<DbPool>,
    user: User,
    request: web::Json<AddItem>,
) -> impl Responder {
    if user
        .is_authorised_by_archetype(&pool, request.archetype, Permission::Meta)
        .await?
    {
        let request = request.into_inner();

        let item = model::Item {
            id: Id::new(),
            note: request.note,
            archetype: request.archetype,
            archetype_data: request.archetype_data,
        };

        diesel::insert_into(item::table)
            .values(item)
            .execute(&pool)
            .await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[derive(Clone, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = item)]
struct ModifyItem {
    note: Option<String>,
    archetype: Option<Id<id::Archetype>>,
    archetype_data: Option<serde_json::Value>,
}

#[put("/items/{id}")]
async fn modify_item(
    item_id: web::Path<Id<id::Item>>,
    pool: web::Data<DbPool>,
    user: User,
    request: web::Json<ModifyItem>,
) -> impl Responder {
    if user
        .is_authorised_by_item(&pool, *item_id, Permission::Meta)
        .await?
    {
        let request = request.into_inner();

        let target = item::table.find(*item_id);
        // This is safe: https://github.com/diesel-rs/diesel/issues/885
        diesel::update(target).set(request).execute(&pool).await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[delete("/items/{id}")]
async fn delete_item(
    item_id: web::Path<Id<id::Item>>,
    pool: web::Data<DbPool>,
    user: User,
) -> impl Responder {
    if user
        .is_authorised_by_item(&pool, *item_id, Permission::Meta)
        .await?
    {
        let target = item::table.find(*item_id);
        diesel::delete(target).execute(&pool).await?;

        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::TestDbPool;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_item() {
        let _pool = TestDbPool::new().await.expect("failed to create db pool");
    }
}
