use actix_web::{http::StatusCode, HttpResponse};
use diesel::r2d2::PoolError;
use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    //401
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    //403
    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    //404
    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    //422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    //500
    #[error("Internet Server Error")]
    InternetServerError,
}

impl actix_web::error::ResponseError for AppError {
    //エラーレスポンスの定義
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::Forbidden(ref msg) => HttpResponse::Forbidden().json(msg),
            AppError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            AppError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            AppError::InternetServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternetServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<PoolError> for AppError {
    fn from(_: PoolError) -> Self {
        AppError::InternetServerError
    }
}
