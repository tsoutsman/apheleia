use crate::{
    auth::{Permission, User},
    db::{
        model,
        schema::{archetype, item, loan, role_permissions},
        tokio::AsyncRunQueryDsl,
        DbPool,
    },
    id::{self, Id},
    Result,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use diesel::{query_dsl::JoinOnDsl, ExpressionMethods, QueryDsl};
use serde::Deserialize;

#[get("/loans/{id}")]
async fn get_loan(
    loan_id: web::Path<Id<id::Item>>,
    pool: web::Data<DbPool>,
    _: User,
) -> impl Responder {
    // TODO: Should everyone have access to loan.
    let loan = loan::table
        .find(*loan_id)
        .first::<model::Loan>(&pool)
        .await?;

    Result::Ok(HttpResponse::Ok().json(loan))
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct GetLoansQueryParams {
    role: GetLoansFilter,
}

#[derive(Copy, Clone, Debug, Deserialize)]
enum GetLoansFilter {
    Loanee,
    Loaner,
    Manager,
}

#[get("/loans")]
async fn get_loans(
    pool: web::Data<DbPool>,
    user: User,
    params: web::Query<GetLoansQueryParams>,
) -> impl Responder {
    Result::Ok(match params.role {
        GetLoansFilter::Loanee => {
            let loans = loan::table
                .filter(loan::loanee.eq(user))
                .load::<model::Loan>(&pool)
                .await?;
            HttpResponse::Ok().json(loans)
        }
        GetLoansFilter::Loaner => {
            let loans = loan::table
                .filter(loan::loaner.eq(user))
                .load::<model::Loan>(&pool)
                .await?;
            HttpResponse::Ok().json(loans)
        }
        GetLoansFilter::Manager => {
            let loans = loan::table
                .inner_join(item::table)
                .inner_join(archetype::table.on(item::archetype.eq(archetype::id)))
                .filter(
                    // TODO: Specific role? i.e. specify meta, loan, return
                    archetype::id.eq_any(user.permissions().select(role_permissions::archetype)),
                )
                .select(loan::all_columns)
                .load::<model::Loan>(&pool)
                .await?;
            HttpResponse::Ok().json(loans)
        }
    })
}

#[derive(Clone, Debug, Deserialize)]
struct AddLoanRequest {
    loanee: User,
    item: Id<id::Item>,
    note: Option<String>,
    date_due: Option<DateTime<Utc>>,
}

#[post("/loans")]
async fn add_loans(
    pool: web::Data<DbPool>,
    user: User,
    request: web::Json<AddLoanRequest>,
) -> impl Responder {
    if user
        .is_authorised_by_item(&pool, request.item, Permission::Loan)
        .await?
    {
        let request = request.into_inner();

        let loan = model::Loan {
            id: Id::new(),
            // TODO
            return_requested: false,
            item: request.item,
            loaner: user,
            loanee: request.loanee,
            note: request.note,
            date_loaned: Utc::now(),
            date_due: request.date_due,
            date_returned: None,
        };

        diesel::insert_into(loan::table)
            .values(loan)
            .execute(&pool)
            .await?;
        Result::Ok(HttpResponse::Ok())
    } else {
        Result::Ok(HttpResponse::Forbidden())
    }
}

#[derive(Clone, Debug, Deserialize, AsChangeset)]
#[diesel(table_name = item)]
struct ModifyLoanRequest {
    note: Option<String>,
    archetype: Option<Id<id::Archetype>>,
    archetype_data: Option<serde_json::Value>,
}

#[put("/loans/{id}")]
async fn modify_loan(// loan_id: web::Path<Id<id::Item>>,
    // pool: web::Data<DbPool>,
    // user: User,
    // request: web::Json<ModifyLoanRequest>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/loans/{id}")]
async fn delete_loan() -> impl Responder {
    HttpResponse::Ok()
}
