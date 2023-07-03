use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    middleware::{auth, state::AppState},
    utils::handler::ApiResponse,
};

use super::{model::User, request, response::UserResponse};

//新規登録
pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::SignupForm>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let (user, token) = User::create(
        conn,
        &form.user.username,
        &form.user.email,
        &form.user.password,
    )?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

//ログイン
pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<request::SigninFrom>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let (user, token) = User::authenticate(conn, &form.user.email, &form.user.password)?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_user(req: HttpRequest) -> ApiResponse {
    let user = auth::get_current_user(&req)?;
    let token = user.gerenate_token()?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}
