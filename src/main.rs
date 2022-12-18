#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{
    error::Error,
    net::Ipv4Addr,
};
use mongodb::bson::{oid::ObjectId};
use utoipa::{
    OpenApi,
   };
   use utoipa_swagger_ui::SwaggerUi;
   
mod db;
use db::connect_database;
mod routes;
use routes::*;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let db = connect_database().await;
    let db = web::Data::new(db);


    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::create,
            routes::read,
            routes::update,
            routes::delete
        ),
        components(
            schemas(routes::Todo, routes::TaskName)
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
        
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
        .app_data(db.clone())
        .configure(routes::configure(db.clone()))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
        )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await;

    Ok(())
}


