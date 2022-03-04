// use crate::{
//     auth::{Permission, User},
//     db::{model, schema::loan, DbPool},
//     id::{self, Id},
//     Result,
// };
//
// use actix_web::{get, post, put, web, HttpResponse, Responder};
// use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
// use serde::Deserialize;
//
// #[get("/loans/{id}")]
// async fn get_loan(
//     loan_id: web::Path<Id<id::Item>>,
//     pool: web::Data<DbPool>,
//     _: User,
// ) -> impl Responder {
//     panic!();
//     // TODO: Should everyone have access to loan.
//     // let loan = loan::table
//     //     .find(*loan_id)
//     //     .first::<model::Loan>(&pool)
//     //     .await?;
//
//     // Result::Ok(HttpResponse::Ok().json(loan))
// }
//
// #[derive(Copy, Clone, Debug, Deserialize)]
// struct GetLoansQueryParams {
//     role: GetLoansFilter,
// }
//
// #[derive(Copy, Clone, Debug, Deserialize)]
// enum GetLoansFilter {
//     Loanee,
//     Loaner,
//     Manager,
// }
//
// #[get("/loans")]
// async fn get_loans(
//     pool: web::Data<DbPool>,
//     user: User,
//     params: web::Query<GetLoansQueryParams>,
// ) -> impl Responder {
//     panic!();
//     // Result::Ok(match params.role {
//     //     GetLoansFilter::Loanee => {
//     //         let loans = loan::table
//     //             .filter(loan::loanee.eq(user))
//     //             .load::<model::Loan>(&pool)
//     //             .await?;
//     //         HttpResponse::Ok().json(loans)
//     //     }
//     //     GetLoansFilter::Loaner => {
//     //         let loans = loan::table
//     //             .filter(loan::loaner.eq(user))
//     //             .load::<model::Loan>(&pool)
//     //             .await?;
//     //         HttpResponse::Ok().json(loans)
//     //     }
//     //     GetLoansFilter::Manager => {
//     //         let loans = loan::table;
//     //         todo!();
//     //     }
//     // })
// }
//
// #[post("/loans")]
// async fn add_loan() -> impl Responder {
//     HttpResponse::Ok()
// }
//
// #[put("/loans/{id}")]
// async fn modify_loan() -> impl Responder {
//     HttpResponse::Ok()
// }
//
// // #[delete("/loans/{id}")]
// // async fn delete_loan() -> impl Responder {
// //     HttpResponse::Ok()
// // }
