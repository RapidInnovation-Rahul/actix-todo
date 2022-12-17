#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod db;
use db::connect_database;
mod routes;
use routes::*;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let db = connect_database().await;
    let db = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
        .app_data(db.clone())
        .route("/create", web::post().to(create))
        .route("/read", web::get().to(read))
        .route("/update/{name}", web::post().to(update))
        .route("/delete/{name}", web::delete().to(delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}



// this is for testing...
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test1() {
        let task1 = TaskName{
            name : "task2343".to_string(),
        };
        let client = connect_database().await;
        let create_operation = create(web::Data::new(client), web::Json(task1)).await;
        assert!(create_operation.status().is_success());
    }

    #[actix_web::test]
    async fn test2() {
        let task1 = TaskName{
            name : "task1".to_string(),
        };
        let client = connect_database().await;
        let read_operation = read(web::Data::new(client)).await;
        assert!(read_operation.status().is_success());
    }

    // #[actix_web::test]
    // async fn test3() {
    //     let id = "639dc11d48690460c1dfd532".to_string();
    //     let client = connect_database().await;
    //     let app = test::init_service(
    //         App::new()
    //             .app_data(web::Data::new(client))
    //             .route("/delete", web::delete().to(delete)),
    //     )
    //     .await;
    //     let req = test::TestRequest::delete().uri("").to_request();
    //     let resp = test::call_service(&app, req).await;
    //     assert!(resp.status().is_success());
    // }

}

