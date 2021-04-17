use actix_web::{
    guard::Get,
    post,
    web::{resource, Data},
    App, HttpResponse, HttpServer, Result,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use graphql::schema::TodoSchema;

mod graphql;
mod database;

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
    let schema = TodoSchema::default();
    let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(index)
            .service(resource("/").guard(Get()).to(playground))
    })
    .bind("localhost:3000");
    println!("Server is running");
    server?.run().await
}
