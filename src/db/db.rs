use sea_orm::SqlxMySqlConnector;
use sea_orm::{DatabaseConnection};
use sqlx::MySqlPool;
use std::env;
use crate::error::error::CustomError;

pub async fn connect() -> Result<DatabaseConnection,CustomError>{
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

    let pool = MySqlPool::connect_with(sqlx_opts).await?;
    let db = SqlxMySqlConnector::from_sqlx_mysql_pool(pool);
    Ok(db)
}