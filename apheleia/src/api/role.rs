use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/roles/{id}")]
async fn get_role() -> impl Responder {
    todo!("get_role");
    HttpResponse::Ok()
}

#[get("/roles")]
async fn get_roles() -> impl Responder {
    todo!("get_roles");
    HttpResponse::Ok()
}

#[post("/roles")]
async fn add_role() -> impl Responder {
    todo!("add_role");
    HttpResponse::Ok()
}

#[put("/roles/{id}")]
async fn modify_role() -> impl Responder {
    todo!("modify_role");
    HttpResponse::Ok()
}

#[delete("/roles/{id}")]
async fn delete_role() -> impl Responder {
    todo!("delete_role");
    HttpResponse::Ok()
}
