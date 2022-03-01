use actix_web::{get, HttpResponse, Responder};

#[get("/users/{id}")]
async fn get_user() -> impl Responder {
    todo!("get_user");
    HttpResponse::Ok()
}
