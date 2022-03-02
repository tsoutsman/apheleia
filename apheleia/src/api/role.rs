use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/roles/{id}")]
async fn get_role() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/roles")]
async fn get_roles() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/roles")]
async fn add_role() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/roles/{id}")]
async fn modify_role() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/roles/{id}")]
async fn delete_role() -> impl Responder {
    HttpResponse::Ok()
}
