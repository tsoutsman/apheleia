use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/items/{id}")]
async fn get_item() -> impl Responder {
    todo!("get_item");
    HttpResponse::Ok()
}

#[get("/items")]
async fn get_items() -> impl Responder {
    todo!("get_items");
    HttpResponse::Ok()
}

#[post("/items")]
async fn add_item() -> impl Responder {
    todo!("add_item");
    HttpResponse::Ok()
}

#[put("/items/{id}")]
async fn modify_item() -> impl Responder {
    todo!("modify_item");
    HttpResponse::Ok()
}

#[delete("/items/{id}")]
async fn delete_item() -> impl Responder {
    todo!("delete_item");
    HttpResponse::Ok()
}
