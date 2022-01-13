use crate::graphql::schema;

use actix_web::{
    error::JsonPayloadError,
    http::Method,
    web::{Data, Query},
    FromRequest, HttpMessage, HttpResponse,
};
use juniper::{
    http::{GraphQLBatchRequest, GraphQLRequest},
    ScalarValue,
};
use juniper_actix::{graphiql_handler, playground_handler};

// NOTE: A lot of this file is copy pasted from juniper-actix with slight modifications.
// I need to put in a pull request as the require RootNode to have a 'static lifetime
// associated with it when it does not need to have one. Until then this file is pretty
// fat.

pub(crate) async fn graphiql_route() -> actix_web::Result<HttpResponse> {
    graphiql_handler("/graphql", None).await
}

pub(crate) async fn playground_route() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

pub(crate) async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    user: crate::extractor::User,
    pool: Data<sqlx::PgPool>,
) -> actix_web::Result<HttpResponse> {
    let pool = pool.get_ref();
    let ctx = crate::context::Context { user, pool };

    // TODO: Currently we are recreating the schema everytime so we can use contexts with lifetimes.
    // We can alternatively do this:
    // https://github.com/graphql-rust/juniper/issues/143#issuecomment-494471440
    let schema = schema();

    let context = &ctx;
    match *req.method() {
        Method::POST => {
            let req = match req.content_type() {
                "application/json" => {
                    let body = String::from_request(&req, &mut payload.into_inner()).await?;
                    serde_json::from_str::<GraphQLBatchRequest<_>>(&body)
                        .map_err(JsonPayloadError::Deserialize)
                }
                "application/graphql" => {
                    let body = String::from_request(&req, &mut payload.into_inner()).await?;
                    Ok(GraphQLBatchRequest::Single(GraphQLRequest::new(
                        body, None, None,
                    )))
                }
                _ => Err(JsonPayloadError::ContentType),
            }?;
            let gql_batch_response = req.execute(&schema, context).await;
            let gql_response = serde_json::to_string(&gql_batch_response)?;
            let mut response = match gql_batch_response.is_ok() {
                true => HttpResponse::Ok(),
                false => HttpResponse::BadRequest(),
            };
            Ok(response.content_type("application/json").body(gql_response))
        }
        Method::GET => {
            let get_req = Query::<GetGraphQLRequest>::from_query(req.query_string())?;
            let req = GraphQLRequest::from(get_req.into_inner());
            let gql_response = req.execute(&schema, context).await;
            let body_response = serde_json::to_string(&gql_response)?;
            let mut response = match gql_response.is_ok() {
                true => HttpResponse::Ok(),
                false => HttpResponse::BadRequest(),
            };
            Ok(response
                .content_type("application/json")
                .body(body_response))
        }
        _ => Err(actix_web::error::UrlGenerationError::ResourceNotFound.into()),
    }
}

#[derive(serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
struct GetGraphQLRequest {
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    variables: Option<String>,
}

impl<S> From<GetGraphQLRequest> for GraphQLRequest<S>
where
    S: ScalarValue,
{
    fn from(get_req: GetGraphQLRequest) -> Self {
        let GetGraphQLRequest {
            query,
            operation_name,
            variables,
        } = get_req;
        // TODO
        let variables =
            variables.map(|s| serde_json::from_str(&s).expect("failed deserializing variables"));
        Self::new(query, operation_name, variables)
    }
}
