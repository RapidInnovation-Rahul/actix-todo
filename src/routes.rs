use actix_web::{get, post, web, App, HttpServer, HttpResponse};
use mongodb::{ bson::{doc, Document},Client, options::ClientOptions, Collection};
use std::{process, vec};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Todo{
    _id : Option<ObjectId>,
    pub task : String,
    pub done : bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TaskName{
    pub name : String,
}

const DB_NAME: &str = "Actix-todo";
const COLL_NAME: &str = "todolist";


pub fn configure(client: web::Data<Client>) -> impl FnOnce(&mut web::ServiceConfig) {
    |config: &mut web::ServiceConfig| {
        config
            .app_data(client)
            .route("/create", web::post().to(create))
            .route("/read", web::get().to(read))
            .route("/update/{id}", web::put().to(update))
            .route("/delete/{id}", web::delete().to(delete));
    }
}

#[utoipa::path( 
    post,
    path = "/create",
    responses(
        (status = 200, description = "Successfully Created new task", body = Todo),
    ),
    params(
        ("info" = TaskName, description = "create new task"),
    )
)]

pub async fn create(client: web::Data<Client>, info : web::Json<TaskName>)-> HttpResponse{
    let collection: Collection<Document> = client.database(DB_NAME).collection(COLL_NAME);
    let id = collection.estimated_document_count(None).await.unwrap();
    let doc = doc! {
        "task": info.name.clone(),
        "done": false,
    };
    collection.insert_one(&doc, None).await;

    HttpResponse::Ok().json(doc)
}


#[utoipa::path(
    get,
    path = "/read",
    responses(
        (status = 200, description = "Successfull", body = Vec<Todo>),
    ),
    
)]
pub async fn read(client: web::Data<Client>)->HttpResponse{
    let collection = client.database(DB_NAME).collection::<Todo>(COLL_NAME);
    let mut cur = collection.find(None, None).await.unwrap();
    let mut res = Vec::new();
    while let Some(item) = cur.next().await{
        res.push(item.unwrap());
    };
    HttpResponse::Ok().json(res)
}


#[utoipa::path(
    put,
    path = "/update/{id}",
    responses(
        (status = 200, description = "updated Successfully"),
    ),
)]
pub async fn update(client: web::Data<Client>, id : web::Path<String>)-> HttpResponse{
    let collection = client.database(DB_NAME).collection::<Todo>(COLL_NAME);
    let id = id.into_inner();
    let as_obj_id = id.parse::<ObjectId>().unwrap();
    collection.update_one(doc!{"_id": as_obj_id}, doc!{"$set" : {"done":true}}, None).await.unwrap();
    HttpResponse::Ok().finish()
}


#[utoipa::path(
    delete,
    path = "/delete/{id}",
    responses(
        (status = 200, description = "Deleted Successfully"),
    ),
    
)]
pub async fn delete(client: web::Data<Client>, id : web::Path<String>)-> HttpResponse{
    let collection = client.database(DB_NAME).collection::<Todo>(COLL_NAME);
    let id = id.into_inner();
    let as_obj_id = id.parse::<ObjectId>().unwrap();
    collection.delete_one(doc!{"_id": as_obj_id},None).await.unwrap();
    HttpResponse::Ok().finish()
}