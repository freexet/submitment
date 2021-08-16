use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    Error, HttpResponse,
};
use juniper::http::{playground::playground_source, GraphQLRequest};

use crate::graphql::schema::{create_schema, Context, Schema};
use crate::service::Service;

pub fn graphql_handlers(config: &mut ServiceConfig) {
    config
        .app_data(Data::new(create_schema()))
        .route("/graphql", web::get().to(graphql_playground))
        .route("/graphql", web::post().to(graphql));
}

async fn graphql(
    service: Data<Service>,
    schema: Data<Schema>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        service: service.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;
    let res = serde_json::to_string(&res)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", None))
}
