use std::io::read_to_string;
use std::ptr::null;
use actix_web::{get, web, HttpResponse, Result, Responder};
use mysql::serde_json;
use crate::model::post;
use sea_orm::{EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
struct Resp{
    code: i32,
    message: String,
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
pub async fn hello(db:web::Data<sea_orm::DatabaseConnection>) ->HttpResponse {
    let result = post::Entity::find_by_id(1).all(db.get_ref()).await;
    match result {
        Ok(posts) => {
            // let str = serde_json::to_string(&posts).unwrap();
            HttpResponse::Ok().json(posts)
        }
        Err(e) =>{
            let obj = MyObj{name:"".to_string()};
            HttpResponse::Ok().json(obj)
        }
    }
}

