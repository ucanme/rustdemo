use actix_web::{
    http::StatusCode,
    HttpResponse,
    body::BoxBody,
};
use serde::Serialize;
use crate::error::error::CustomError;

// 定义一个通用的响应结构体
#[derive(Serialize)]
pub struct RespMsg<T>{
    pub(crate) code: i32,
    pub(crate) message: String,
    pub Data: T,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    // 构造成功响应
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    // 构造失败响应
    pub fn error(message: CustomError) -> Self {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

// 构建成功的 JSON 响应
pub fn build_success_response<T: Serialize>(data: T) -> HttpResponse {
    let response = ApiResponse::success(data);
    HttpResponse::Ok().json(response)
}

// 构建失败的 JSON 响应
pub fn build_error_response(status_code: StatusCode, message: CustomError) -> HttpResponse {
    let response:ApiResponse<&str> = ApiResponse::error(message);
    HttpResponse::build(status_code).json(response)
}