mod router;
mod db;

use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use mysql::Pool;
use crate::router::hello::hello;
use db::db::connect;
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
    let sql_conn = connect();
    if sql_conn.is_err(){
        println!("sqlConnErr");
    }
   let arg = env::args();
    let shared_data = web::Data::new(sql_conn.unwrap());
    HttpServer::new(move || {
        App::new()
            .service(hello).app_data(web::Data::new(shared_data.clone()))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}