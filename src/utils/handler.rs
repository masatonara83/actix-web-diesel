use actix_web::HttpResponse;

use crate::error::AppError;

pub type ApiResponse = Result<HttpResponse, AppError>;
