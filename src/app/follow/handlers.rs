use crate::{
    app::{
        profile::{handlers::UsernameSlug, response::ProfileResponse},
        user::model::User,
    },
    middleware::{auth, state::AppState},
    utils::handler::ApiResponse,
};

use actix_web::{web, HttpRequest, HttpResponse};

use super::model::Follow;

pub async fn create_follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;

    let username = path.into_inner();
    let followee = User::find_by_username(conn, &username)?;
    let profile = Follow::follow(conn, &current_user, &followee.id)?;

    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}
