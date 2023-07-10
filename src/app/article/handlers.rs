use actix_web::{web, HttpResponse};

use crate::{middleware::state::AppState, utils::handler::ApiResponse};

use super::{request::ArticleListQueryParameter, response::MultipleArticleResponse, service};

pub async fn get_articles(
    state: web::Data<AppState>,
    params: web::Query<ArticleListQueryParameter>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);

    let (articles_list, articles_count) = service::fetch_articles_list(
        conn,
        service::FetchArticlesList {
            tag: params.tag.clone(),
            author: params.author.clone(),
            favorited: params.favorited.clone(),
            offset,
            limit,
        },
    )?;

    let res = MultipleArticleResponse::from((articles_list, articles_count));
    Ok(HttpResponse::Ok().json(res))
}
