use sea_orm::{ConnectionTrait, EntityTrait};
use sea_orm::DbBackend;
use sea_orm::SqlxMySqlConnector;
use sea_orm::{FromQueryResult, Statement as sea_statment,DatabaseConnection};
use sqlx::MySqlPool;
use std::env;
use model::post;
use crate::model;
use crate::model::post::Model;

pub async fn connect() -> DatabaseConnection{
    println!("Connecting to database");
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let db_port = db_port.parse().unwrap();

    let sqlx_opts = sqlx::mysql::MySqlConnectOptions::new()
        .host(&db_host)
        .port(db_port)
        .database(&db_name)
        .username(&db_user)
        .password(&db_password)
        .ssl_ca("./isrgrootx1.pem");

    let pool = MySqlPool::connect_with(sqlx_opts).await.unwrap();
    let db = SqlxMySqlConnector::from_sqlx_mysql_pool(pool);
    let rs = db
        .query_all(sea_statment::from_string(
            db.get_database_backend(),
            "select * from POST;".to_string(),
        ))
        .await;



    let posts: Vec<post::Model> = post::Entity::find_by_id(1).all(&db).await.unwrap();

    println!("表中的所有帖子:");
    for post in posts {
        println!("id: {}, title: {}", post.id, post.content);
    }
    // rs.unwrap().iter().for_each(|row| {
    //     println!("Row: {:?}",row );
    //     let content = row.try_get::<String>("","content");
    //     println!("{:?}", content);
    // });
    db
}