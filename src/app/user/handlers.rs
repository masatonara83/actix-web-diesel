use actix_web::web;

use crate::{middleware::state::AppState, utils::handler::ApiResponse};

use super::request;

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::SignupForm>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    
}
