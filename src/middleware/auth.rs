use actix_web::{HttpMessage, HttpRequest};
use serde_json::json;

use crate::{app::user::model::User, error::AppError};

pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
    req.extensions()
        .get::<User>()
        .map(|user| user.to_owned())
        .ok_or_else(|| {
            AppError::Unauthorized(json!({
                "error": "Unauthorize user. Need auth token on header"
            }))
        })
}
