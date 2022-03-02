use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/subject_areas/{id}")]
async fn get_subject_area() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/subject_areas")]
async fn get_subject_areas() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subject_areas")]
async fn add_subject_area() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/subject_areas/{id}")]
async fn modify_subject_area() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/subject_areas/{id}")]
async fn delete_subject_area() -> impl Responder {
    HttpResponse::Ok()
}
