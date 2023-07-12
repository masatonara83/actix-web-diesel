use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ArticleListQueryParameter {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct FeedQueryParameter {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub article: CreateArticleInner,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticleInner {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags_list: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticleInner,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleInner {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
