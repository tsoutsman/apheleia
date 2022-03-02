use actix_web::{get, put, HttpResponse, Responder};

#[get("/settings")]
async fn get_settings() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/settings")]
async fn modify_settings() -> impl Responder {
    HttpResponse::Ok()
}
