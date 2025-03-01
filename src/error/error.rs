#![feature(io_error_more)]
use std::error::Error;

#[derive(Debug,thiserror::Error)]
pub enum CustomError {
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    FsIOError(#[from] std::io::Error),
    #[error(transparent)]
    SqxCoreErr(#[from] sqlx::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    SeaOrmDbError(#[from] sea_orm::DbErr),
}