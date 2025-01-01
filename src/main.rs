mod router;
mod db;
mod model;

use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use mysql::Pool;
use sea_orm::ConnectionTrait;
use crate::router::hello::hello;
use db::db::{connect};
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let sql_conn = connect().await;
    let db_data = web::Data::new(sql_conn);
    println!("Connected");
    HttpServer::new( move ||{
        App::new()
            .service(hello) .app_data(db_data.clone())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}