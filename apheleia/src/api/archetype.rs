use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/archetypes/{id}")]
async fn get_archetype() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/archetypes")]
async fn get_archetypes() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/archetypes")]
async fn add_archetype() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/archetypes/{id}")]
async fn modify_archetype() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/archetypes/{id}")]
async fn delete_archetype() -> impl Responder {
    HttpResponse::Ok()
}
