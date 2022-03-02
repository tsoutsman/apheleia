use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/loans/{id}")]
async fn get_loan() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/loans/managed")]
async fn get_managed_loans() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/loans/borrowed")]
async fn get_borrowed_loans() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/loans")]
async fn add_loan() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/loans/{id}")]
async fn modify_loan() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/loans/{id}")]
async fn delete_loan() -> impl Responder {
    HttpResponse::Ok()
}
