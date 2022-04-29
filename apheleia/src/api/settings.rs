use actix_web::{get, put, HttpResponse, Responder};

pub(crate) fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_settings)
        .service(modify_settings);
}

#[get("/settings")]
async fn get_settings() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/settings")]
async fn modify_settings() -> impl Responder {
    HttpResponse::Ok()
}
