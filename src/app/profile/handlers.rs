use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    app::user::model::User,
    middleware::{auth, state::AppState},
    utils::handler::ApiResponse,
};

use super::response::ProfileResponse;

pub type UsernameSlug = String;

pub async fn get_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;
    let username = path.into_inner();

    let profile = {
        let followee = User::find_by_username(conn, &username)?;
        current_user.get_profile(conn, &followee.id)
    };

    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}
