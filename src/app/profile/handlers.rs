use actix_web::{web, HttpRequest};

use crate::{middleware::{state::AppState, auth}, utils::handler::ApiResponse, app::user::model::User};

pub type UsernameSlug = String;

pub async fn get_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req);
    let username = path.into_inner();

    let profile = {
        let followee = User::find_by_username(conn, &username);
        
    }
}
