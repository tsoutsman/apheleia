use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/loans/{id}")]
async fn get_loan() -> impl Responder {
    todo!("get_loan");
    HttpResponse::Ok()
}

#[get("/loans/managed")]
async fn get_managed_loans() -> impl Responder {
    todo!("get_managed_loans");
    HttpResponse::Ok()
}

#[get("/loans/borrowed")]
async fn get_borrowed_loans() -> impl Responder {
    todo!("get_borrowed_loans");
    HttpResponse::Ok()
}

#[post("/loans")]
async fn add_loan() -> impl Responder {
    todo!("add_loan");
    HttpResponse::Ok()
}

#[put("/loans/{id}")]
async fn modify_loan() -> impl Responder {
    todo!("modify_loan");
    HttpResponse::Ok()
}

#[delete("/loans/{id}")]
async fn delete_loan() -> impl Responder {
    todo!("delete_loan");
    HttpResponse::Ok()
}
