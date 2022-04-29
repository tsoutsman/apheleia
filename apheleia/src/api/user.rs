use crate::{
    auth::User,
    db::{schema::user, tokio::AsyncRunQueryDsl, DbPool},
    Result,
};

use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::QueryDsl;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user).service(add_user);
}

#[get("/users/{id}")]
async fn get_user(user_id: web::Path<User>, pool: web::Data<DbPool>, _: User) -> impl Responder {
    let user = user::table
        .find(*user_id)
        .select(user::id)
        .first::<User>(&pool)
        .await?;
    Result::Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn add_user(pool: web::Data<DbPool>, user: User) -> impl Responder {
    // FIXME: What if user already exists?
    // TODO: Get and verify invite link.

    diesel::insert_into(user::table)
        .values(user)
        .execute(&pool)
        .await?;
    Result::Ok(HttpResponse::Ok())
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::header,
        test::{self, TestRequest},
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn test_user() {
        let (app, _pool) = crate::test::init_test_service().await;

        let req = TestRequest::post().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let req = TestRequest::get().uri("/users/1234").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        let req = TestRequest::get()
            .uri("/users/1234")
            .insert_header((header::AUTHORIZATION, "Bearer 1234"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);

        let req = TestRequest::post()
            .uri("/users")
            .insert_header((header::AUTHORIZATION, "Bearer 1234"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let req = TestRequest::get()
            .uri("/users/1234")
            .insert_header((header::AUTHORIZATION, "Bearer 1234"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let req = TestRequest::get()
            .uri("/users/5678")
            .insert_header((header::AUTHORIZATION, "Bearer 1234"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);

        let req = TestRequest::get()
            .uri("/users/1234")
            .insert_header((header::AUTHORIZATION, "Bearer 5678"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
