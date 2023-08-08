use crate::{middleware::state::AppState, utils::handler::ApiResponse};
use actix_web::{web, HttpResponse};

use super::{response::MultipleCommentsResponse, service};

type ArticleTitleSlug = String;

pub async fn get_article_comments(
    state: web::Data<AppState>,
    path: web::Path<ArticleTitleSlug>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let slug = path.into_inner();

    let list =
        service::fetch_article_comments(conn, &service::FetchArticleCommentsService { slug })?;

    let res = MultipleCommentsResponse::from(list);
    Ok(HttpResponse::Ok().json(res))
}
