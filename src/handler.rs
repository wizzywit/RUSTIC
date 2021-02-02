use actix_web::{ web, Responder, HttpResponse};
use crate::models::{Status, ResultResponse};
use crate::models::CreateTodoList;
use deadpool_postgres::{Pool, Client};
use crate::db;
use std::io::ErrorKind::Other;

pub async fn status() -> impl Responder{
    web::HttpResponse::Ok()
    .json(Status {status: "UP".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_todo_items(db_pool: web::Data<Pool>, path: web::Path<(i32, )>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");
    let result = db::get_todo_items(&client, path.0).await;
    match result {
        Ok(todo_items) => HttpResponse::Ok().json(todo_items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");
    let result = db::create_todo(&client, json.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");
    let result = db::check_item(&client, path.0, path.1).await;
    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse {success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse {success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}