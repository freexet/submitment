mod error;
mod graphql;
mod handler;
mod repository;
mod schema;
mod service;
mod util;

use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Responder};
use color_eyre::Result;
use dotenv::dotenv;

use crate::handler::graphql::graphql_handlers;
use crate::repository::Repository;
use crate::service::{auth::AuthService, Service};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let repository = Repository::new().await?;
    let service = Service {
        auth: AuthService::new(repository),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .wrap(Logger::default())
            .configure(graphql_handlers)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;
    println!("Hello");

    Ok(())
}
