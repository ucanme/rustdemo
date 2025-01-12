use std::fmt::Debug;
use std::io::read_to_string;
use std::ptr::{null, null_mut};
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
#[derive(Serialize)]
struct RespMsg{
    code: i32,
    message: String,
}

#[get("/")]
pub async fn hello(db:web::Data<sea_orm::DatabaseConnection>) ->HttpResponse {
    let result = post::Entity::find_by_id(1).all(db.get_ref()).await;
    match result {
        Ok(posts) => {
            // let str = serde_json::to_string(&posts).unwrap();
            HttpResponse::Ok().json(posts)
        }
        Err(D    ) =>{
            let msg =  RespMsg{
                code:1000,
                message: "123".to_string(),
            };
            HttpResponse::Ok().json(msg)
        }
    }
}

