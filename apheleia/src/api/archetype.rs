use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/archetypes/{id}")]
async fn get_archetype() -> impl Responder {
    todo!("get_archetype");
    HttpResponse::Ok()
}

#[get("/archetypes")]
async fn get_archetypes() -> impl Responder {
    todo!("get_archetypes");
    HttpResponse::Ok()
}

#[post("/archetypes")]
async fn add_archetype() -> impl Responder {
    todo!("add_archetype");
    HttpResponse::Ok()
}

#[put("/archetypes/{id}")]
async fn modify_archetype() -> impl Responder {
    todo!("modify_archetype");
    HttpResponse::Ok()
}

#[delete("/archetypes/{id}")]
async fn delete_archetype() -> impl Responder {
    todo!("delete_archetype");
    HttpResponse::Ok()
}
