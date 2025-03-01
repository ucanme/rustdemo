use std::fmt::Debug;
use std::io::read_to_string;
use std::ptr::{null, null_mut};
use actix_web::{get, web, HttpResponse, Result, Responder};
use actix_web::http::StatusCode;
use futures::future::err;
use mysql::serde_json;
use mysql::serde_json::error;
use crate::model::post;
use sea_orm::{EntityTrait};
use serde::Serialize;
use crate::error::error::CustomError::SeaOrmDbError;
use crate::lib::http;
use crate::lib::http::{ApiResponse, RespMsg};

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
pub async fn hello(db:web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let result = post::Entity::find().all(db.get_ref()).await;
    match result {
        Ok(posts) => {
            http::build_success_response( posts)
        }
        Err(D    ) =>{
            http::build_error_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,SeaOrmDbError(D))
        }
    }
}

