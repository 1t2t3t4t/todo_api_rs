use std::sync::Arc;

use actix_web::{
    guard::Get,
    post,
    web::{resource, Data},
    App, HttpResponse, HttpServer, Result,
};
use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::EmptySubscription;
use async_graphql_actix_web::{Request, Response};

use graphql::schema::TodoSchema;

use crate::database::{Database, FileSystemDatabase};

mod database;
mod graphql;
pub mod model;

#[post("/")]
async fn index(schema: Data<TodoSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database: Arc<dyn Database> = Arc::new(FileSystemDatabase::default());
    let schema = TodoSchema::build(
        Default::default(),
        Default::default(),
        EmptySubscription::default(),
    )
    .data(database.clone())
    .extension(ApolloTracing)
    .finish();
    let bind_addr = "0.0.0.0:3000";

    let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(index)
            .service(resource("/").guard(Get()).to(playground))
    })
    .bind(bind_addr);
    println!("Server is running at {}", bind_addr);

    server?.run().await
}
