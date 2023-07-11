use actix_web::{web, HttpRequest, HttpResponse};

use crate::middleware::auth;
use crate::middleware::state::AppState;
use crate::utils::handler::ApiResponse;

use super::request::{ArticleListQueryParameter, CreateArticleRequest};
use super::response::{MultipleArticleResponse, SingleArticleResponse};
use super::service;

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

pub async fn create_article(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateArticleRequest>,
) -> ApiResponse {
    let conn = &mut state.conn()?;
    let current_user = auth::get_current_user(&req)?;

    let (article, profile, favorite_info, tags) = service::create_article(
        conn,
        &service::CreateArticleService {
            current_user,
            title: form.article.title.clone(),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
            tag_name_list: form.article.tags_list.clone(),
        },
    )?;

    let res = SingleArticleResponse::from((article, profile, favorite_info, tags));
    Ok(HttpResponse::Ok().json(res))
}
