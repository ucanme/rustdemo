use std::process::id;
use actix_web::{get, web, HttpResponse, Responder};
use crate::model::post;
use sea_orm::{ConnectionTrait, EntityTrait};

#[get("/")]
pub async fn hello(db:web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let posts: Vec<post::Model> = post::Entity::find_by_id(1).all(db.get_ref()).await.unwrap();
    println!("表中的所有帖子:");
    for post in posts {
        println!("id: {}, title: {}", post.id, post.content);
    }
    HttpResponse::Ok().body("Hello world!")
}
