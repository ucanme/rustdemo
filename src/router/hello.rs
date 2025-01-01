use std::process::id;
use actix_web::{get, web, HttpResponse, Responder};
use mysql::serde_json;
use crate::model::post;
use sea_orm::{ConnectionTrait, EntityTrait};

#[get("/")]
pub async fn hello(db:web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let posts: Vec<post::Model> = post::Entity::find_by_id(1).all(db.get_ref()).await.unwrap();
    println!("表中的所有帖子:");

    let str = serde_json::to_string(&posts).unwrap();
    HttpResponse::Ok().body(str)
}
